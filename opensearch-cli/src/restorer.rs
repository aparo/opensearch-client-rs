use std::path::PathBuf;

use clap::ValueEnum;
use opensearch::{indices::types::IndexTemplateMapping, OsClient};
use opensearch_dsl::{
  search::sort::{FieldSort, SortCollection},
  Query,
};
use async_compression::tokio::{
  bufread::ZstdDecoder,
  write::{GzipEncoder, ZstdEncoder},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::{
  fs::File,
  io::{self, stderr, stdin, stdout, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter, Lines},
};
use tokio::io::{
  AsyncBufRead as _,
  AsyncReadExt as _,  // for `read_to_end`
  AsyncWriteExt as _, // for `write_all` and `shutdown`
};
use futures::{pin_mut, stream, StreamExt};
use tracing::{debug, error};
type Decoder<T> = ZstdDecoder<T>;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum RestoreMode {
  Index,
  Create,
}

impl std::fmt::Display for RestoreMode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self
      .to_possible_value()
      .expect("no values are skipped")
      .get_name()
      .fmt(f)
  }
}

pub struct Restorer<'a> {
  pub client: &'a OsClient,
  pub input: PathBuf,
  pub rename_index: Option<String>,
  pub index: Option<String>,
  pub bulk_size: u32,
  pub skip_mapping: bool,
  pub skip_data: bool,
  pub mode: RestoreMode,
}

impl<'a> Restorer<'a> {
  pub async fn restore(&self) -> anyhow::Result<()> {
    // we list of files in input path
    let mut files = std::fs::read_dir(&self.input)?
      .map(|res| res.map(|e| e.path()))
      .collect::<Result<Vec<_>, std::io::Error>>()?;
    files.sort();
    // we first restore the mappings
    if !self.skip_mapping {
      for file in &files {
        if file.is_dir() {
          continue;
        }
        let file_name = file.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".json") {
          let index = file_name.replace(".json", "");
          if let Some(ref i) = self.index {
            if !index.starts_with(i) {
              continue;
            }
          }
          // self.restore_mapping(&file, &index).await?;
        }
      }
    }

    // Now we restore the data
    if !self.skip_data {
      for file in &files {
        if file.is_dir() {
          continue;
        }
        let file_name = file.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with("__data.zstd") {
          let index = file_name.replace("__data.zstd", "");
          if let Some(ref i) = self.index {
            if !index.starts_with(i) {
              continue;
            }
          }
          self.restore_index(&file, &index).await?;
        }
      }
    }
    Ok(())
  }

  pub async fn restore_mapping(&self, file: &PathBuf, index: &str) -> anyhow::Result<()> {
    let file = File::open(file).await?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).await?;
    let mapping = serde_json::from_slice::<IndexTemplateMapping>(&buffer)?;
    let new_index = self.rename_index.clone().unwrap_or(index.to_string());
    self.client.indices().create(&new_index, mapping).send().await?;

    println!("Restored mapping for index {}", index);
    Ok(())
  }

  pub async fn restore_index(&self, file: &PathBuf, index: &str) -> anyhow::Result<()> {
    let mut lines = create_lines(file).await;
    let mut total_count: u32 = 0;
    let mut header = Header::default();
    loop {
      let data = lines.next_line().await;
      match data {
        Ok(Some(line)) => {
          if total_count % 2 == 0 {
            let base_header = serde_json::from_str::<Header>(&line)?;
            header = base_header;
          } else {
            let body = serde_json::from_str::<serde_json::Value>(&line)?;
            let target_index = self.rename_index.clone().unwrap_or(header.index.clone());
            self
              .client
              .bulk_index_document(&target_index, Some(header.id.clone()), &body)
              .await?;
          }
          total_count += 1;
        }
        Ok(None) => {
          break;
        }
        Err(e) => {
          error!("Error in File Read {:?}", e);
          break;
        }
      }
    }
    self.client.flush_bulk().await?;
    println!(
      "Written index {} with records {}",
      self.rename_index.clone().unwrap_or(index.to_owned()),
      total_count / 2
    );
    Ok(())
  }
}

pub async fn create_lines(file_path: &PathBuf) -> Lines<BufReader<Decoder<BufReader<File>>>> {
  let file = File::open(file_path).await.unwrap();
  let reader = BufReader::new(file);
  let gzip_decoder = ZstdDecoder::new(reader);
  let buf_reader = tokio::io::BufReader::with_capacity(100000, gzip_decoder);
  let lines = buf_reader.lines();
  return lines;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
struct Header {
  #[serde(rename = "_index")]
  pub index: String,
  #[serde(rename = "_id")]
  pub id: String,
}
