use std::{
  collections::HashMap,
  fs::{self},
  path::PathBuf,
};

use serde_json::Value;
use opensearch::{Error, OsClient};
use tokio::fs::File;
use tracing::info;

pub async fn write_json_to_file(path: &PathBuf, json_value: &Value) -> anyhow::Result<()> {
  let json_string = serde_json::to_string_pretty(json_value)?;
  fs::write(path, json_string)?;
  info!("Wrote file: {}", path.display());
  Ok(())
}

pub async fn save_named_map(path: &PathBuf, data: HashMap<String, Value>) -> anyhow::Result<()> {
  // we create the dir in not exists
  fs::create_dir_all(&path).unwrap_or_else(|error| {
    eprintln!("Failed to create directory: {}", error);
  });
  // we iterate over the pipelines and dump them
  for (name, value) in data.iter() {
    let value_file = path.join(format!("{}.json", name));
    write_json_to_file(&value_file, &value).await?;
  }
  Ok(())
}

pub async fn dump_pipelines(client: &OsClient, output: PathBuf) -> anyhow::Result<()> {
  let pipelines = client.ingest().get_pipeline().send().await?.into_inner();
  let pipeline_path = output.join("pipelines");
  save_named_map(&pipeline_path, pipelines).await?;
  Ok(())
}

pub async fn dump_index_templates(client: &OsClient, output: PathBuf) -> anyhow::Result<()> {
  let data = client.indices().get_index_template().send().await?.into_inner();
  let data_path = output.join("templates");
  save_named_map(&data_path, data).await?;
  Ok(())
}

pub async fn dump_index_components(client: &OsClient, output: PathBuf) -> anyhow::Result<()> {
  let data = client.indices().get_component_template().send().await?.into_inner();
  let data_path = output.join("component_templates");
  save_named_map(&data_path, data).await?;
  Ok(())
}
