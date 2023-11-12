use std::path::PathBuf;

use opensearch::OsClient;
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
  io::{self, stderr, stdin, stdout, AsyncWriteExt, BufReader, BufWriter},
};
use tokio::io::{
  AsyncReadExt as _,  // for `read_to_end`
  AsyncWriteExt as _, // for `write_all` and `shutdown`
};
use futures::{pin_mut, stream, StreamExt};

pub struct Dumper<'a> {
  pub client: &'a OsClient,
  pub compress: bool,
  pub output: PathBuf,
  pub indices: String,
  pub read_bulk: u64,
  pub max_record_for_file: u32,
}

impl<'a> Dumper<'a> {
  pub async fn dump(&self) -> anyhow::Result<()> {
    let resolve_response = self
      .client
      .indices()
      .resolve_index(&self.indices)
      .send()
      .await?
      .into_inner();
    let mut indices = resolve_response.get_open_indices();
    indices.sort();
    for index in indices {
      self.dump_mapping(index.as_str()).await?;
      self.dump_index(index.as_str()).await?;
    }
    Ok(())
  }

  pub async fn dump_mapping(&self, index: &str) -> anyhow::Result<()> {
    let mut path = self.output.clone();
    path.push(index);
    if !path.exists() {
      std::fs::create_dir_all(path.clone())?;
    }
    path.push(format!("{}.json", index));
    let file = File::create(&path).await?;
    let mut writer = BufWriter::new(file);
    let mapping = self.client.indices().get().index(index).send().await?.into_inner();
    let mut mapping = mapping.get(index).unwrap().clone();
    mapping = mapping.clean_for_create();
    writer
      .write_all(serde_json::to_string_pretty(&mapping).unwrap().as_bytes())
      .await?;
    writer.flush().await?;
    println!("Written file {}", path.to_str().unwrap());
    Ok(())
  }

  pub async fn dump_index(&self, index: &str) -> anyhow::Result<()> {
    let mut path = self.output.clone();
    path.push(index);
    if !path.exists() {
      std::fs::create_dir_all(path.clone())?;
    }
    path.push(format!("{}__data.zstd", index));
    let file = File::create(&path).await?;
    let writer = BufWriter::new(file);

    let mut encoder = ZstdEncoder::new(writer);
    let query = Query::match_all();
    let sort = SortCollection::new().field(FieldSort::ascending("_id"));

    let stream = self
      .client
      .search_stream::<serde_json::Value>(index, &query.into(), &sort, self.read_bulk)
      .await?;
    pin_mut!(stream);
    let mut count = 0;

    while let Some(hit) = stream.next().await {
      let header = Header {
        index: hit.index,
        id: hit.id,
      };
      let mut header = serde_json::to_vec(&header)?;
      header.push(b'\n');
      let source = hit.source;
      let source = serde_json::to_vec(&source)?;
      header.append(&mut source.clone());
      header.push(b'\n');
      encoder.write_all(&header).await?;
      count += 1;
    }
    // writer.flush().await?;
    encoder.shutdown().await?;
    println!("Written file {} with records {}", path.to_str().unwrap(), count);
    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
struct Header {
  #[serde(rename = "_index")]
  pub index: String,
  #[serde(rename = "_id")]
  pub id: String,
}
