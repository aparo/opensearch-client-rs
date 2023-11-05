use std::{
  fs::{self},
  path::PathBuf,
};

use serde_json::Value;
use opensearch::{Error, OsClient};
use tokio::fs::File;

pub async fn dump_pipelines(client: &OsClient, output: PathBuf) -> anyhow::Result<()> {
<<<<<<< Updated upstream
  let pipelines = client.ingest().get_pipeline().send().await?;
=======
  let pipelines = client.ingest().ingest_get_pipeline().send().await?;
>>>>>>> Stashed changes
  let pipeline_path = output.join("pipelines");
  // we create the dir in not exists
  fs::create_dir_all(&pipeline_path).unwrap_or_else(|error| {
    eprintln!("Failed to create directory: {}", error);
  });
  // we iterate over the pipelines and dump them
  for (name, pipeline) in pipelines.iter() {
    let pipeline_file = pipeline_path.join(format!("{}.json", name));
    write_json_to_file(&pipeline_file, &pipeline).await?;
  }

  Ok(())
}

async fn write_json_to_file(path: &PathBuf, json_value: &Value) -> anyhow::Result<()> {
  let json_string = serde_json::to_string(json_value)?;
  fs::write(path, json_string)?;
  Ok(())
}
