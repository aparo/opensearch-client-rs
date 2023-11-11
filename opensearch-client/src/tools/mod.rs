use std::{
  collections::HashMap,
  fs::{self},
  path::PathBuf,
};

use serde_json::Value;
use tracing::info;
use walkdir::WalkDir;

use crate::OsClient;

const PIPELINE_DIRECTORY: &str = "pipelines";
const TEMPLATE_DIRECTORY: &str = "templates";
const COMPONENT_DIRECTORY: &str = "components";

/// Tools is a struct that contains all the methods to dump and restore cluster
/// data
pub struct Tools<'a> {
  client: &'a OsClient,
}

impl<'a> Tools<'a> {
  pub fn new(client: &'a OsClient) -> Self {
    Self { client }
  }

  /// Asynchronously dumps the pipelines to the specified output path.
  ///
  /// # Arguments
  /// * `output` - The path to the output file.
  ///
  /// # Returns
  /// Returns `Ok(())` if the operation was successful, otherwise returns an
  /// `anyhow::Error`.
  pub async fn dump_pipelines(&self, output: PathBuf) -> anyhow::Result<()> {
    let pipelines = self.client.ingest().get_pipelines().send().await?.into_inner();
    let pipeline_path = output.join(PIPELINE_DIRECTORY);
    save_named_map(&pipeline_path, pipelines).await?;
    Ok(())
  }

  /// Asynchronously dumps the index templates to the specified output path.
  ///
  /// # Arguments
  /// * `output` - The path to the output file.
  ///
  /// # Returns
  /// Returns `Ok(())` if the operation was successful, otherwise returns an
  /// `anyhow::Error`.
  pub async fn dump_index_templates(&self, output: PathBuf) -> anyhow::Result<()> {
    let data = self.client.indices().get_index_templates().send().await?.into_inner();
    let data_path = output.join(TEMPLATE_DIRECTORY);
    save_named_map(&data_path, data).await?;
    Ok(())
  }

  /// Asynchronously dumps the index components to the specified output path.
  ///
  /// # Arguments
  /// * `output` - The path to the output file.
  ///
  /// # Returns
  /// Returns `Ok(())` if the operation was successful, otherwise returns an
  /// `anyhow::Error`.
  pub async fn dump_index_components(&self, output: PathBuf) -> anyhow::Result<()> {
    let data = self
      .client
      .indices()
      .get_component_templates()
      .send()
      .await?
      .into_inner();
    let data_path = output.join(COMPONENT_DIRECTORY);
    save_named_map(&data_path, data).await?;
    Ok(())
  }

  /// Asynchronously restores the pipelines from the specified path.
  ///
  /// # Arguments
  /// * `input_path` - The path to be used as source for the files.
  ///
  /// # Returns
  /// Returns `Ok(())` if the operation was successful, otherwise returns an
  /// `anyhow::Error`.
  pub async fn restore_pipelines(&self, input: PathBuf) -> anyhow::Result<()> {
    let files = get_json_file_recursive(&input.join(PIPELINE_DIRECTORY))?;
    let current_pipelines = self.client.ingest().get_pipelines().send().await?.into_inner();
    for entry in files {
      let name = entry
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .replace(".json", "");
      let pipeline = fs::read_to_string(entry)?;
      let pipeline: serde_json::Value = serde_json::from_str(&pipeline)?;
      self
        .update_pipeline_if_required(&name, pipeline, current_pipelines.clone())
        .await?;
    }
    Ok(())
  }

  pub async fn update_pipeline_if_required(
    &self,
    name: &String,
    body: serde_json::Value,
    current_pipelines: HashMap<String, Value>,
  ) -> anyhow::Result<()> {
    if current_pipelines.contains_key(name) {
      let old_pipeline = current_pipelines.get(name).unwrap();
      let version = old_pipeline["version"].as_u64().unwrap_or(0);
      let new_version = body["version"].as_u64().unwrap_or(0);
      if version >= new_version {
        info!("Pipeline {} is up to date", name);
        return Ok(());
      }
    }
    self.client.ingest().put_pipeline(name, body).send().await?;
    info!("Pipeline {} updated", name);
    Ok(())
  }

  /// Asynchronously restores the index templates from the specified path.
  ///
  /// # Arguments
  /// * `input_path` - The path to be used as source for the files.
  ///
  /// # Returns
  /// Returns `Ok(())` if the operation was successful, otherwise returns an
  /// `anyhow::Error`.
  pub async fn restore_index_templates(&self, input: PathBuf) -> anyhow::Result<()> {
    let files = get_json_file_recursive(&input.join(TEMPLATE_DIRECTORY))?;
    let current_templates = self.client.indices().get_index_templates().send().await?.into_inner();
    for entry in files {
      let name = entry
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .replace(".json", "");
      let body = fs::read_to_string(entry)?;
      let body: serde_json::Value = serde_json::from_str(&body)?;
      self
        .update_template_if_required(&name, body, current_templates.clone())
        .await?;
    }
    Ok(())
  }

  pub async fn update_template_if_required(
    &self,
    name: &String,
    body: serde_json::Value,
    current_templates: HashMap<String, Value>,
  ) -> anyhow::Result<()> {
    if current_templates.contains_key(name) {
      let old_template = current_templates.get(name).unwrap();
      let version = old_template["version"].as_u64().unwrap_or(0);
      let new_version = body["version"].as_u64().unwrap_or(0);
      if version >= new_version {
        info!("Index Template {} is up to date", name);
        return Ok(());
      }
    }
    self.client.indices().put_template(name, body).send().await?;
    info!("Index Template {} updated", name);
    Ok(())
  }

  /// Asynchronously restores the index components from the specified path.
  ///
  /// # Arguments
  /// * `input_path` - The path to be used as source for the files.
  ///
  /// # Returns
  /// Returns `Ok(())` if the operation was successful, otherwise returns an
  /// `anyhow::Error`.
  pub async fn restore_index_components(&self, input: PathBuf) -> anyhow::Result<()> {
    let files = get_json_file_recursive(&input.join(COMPONENT_DIRECTORY))?;
    let current_components = self
      .client
      .indices()
      .get_component_templates()
      .send()
      .await?
      .into_inner();
    for entry in files {
      let name = entry
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .replace(".json", "");
      let body = fs::read_to_string(entry)?;
      let body: serde_json::Value = serde_json::from_str(&body)?;
      self
        .update_component_if_required(&name, body, current_components.clone())
        .await?;
    }
    Ok(())
  }

  pub async fn update_component_if_required(
    &self,
    name: &String,
    body: serde_json::Value,
    current_templates: HashMap<String, Value>,
  ) -> anyhow::Result<()> {
    if current_templates.contains_key(name) {
      let old_template = current_templates.get(name).unwrap();
      let version = old_template["version"].as_u64().unwrap_or(0);
      let new_version = body["version"].as_u64().unwrap_or(0);
      if version >= new_version {
        info!("Index Component {} is up to date", name);
        return Ok(());
      }
    }
    self.client.indices().put_component_template(name, body).send().await?;
    info!("Index Component {} updated", name);
    Ok(())
  }

  /// Asynchronously fixes the pipelines from the specified path adding version
  /// if missing.
  pub async fn fix_pipelines(&self, input: PathBuf) -> anyhow::Result<()> {
    let files = get_json_file_recursive(&input.join(PIPELINE_DIRECTORY))?;
    for file in files {
      let body = fs::read_to_string(&file)?;
      let mut body: serde_json::Map<String, Value> = serde_json::from_str(&body)?;
      if !body.contains_key("version") {
        body.insert("version".to_string(), serde_json::Value::from(1));
        write_json_to_file(&file, &serde_json::Value::from(body)).await?;
      }
    }
    Ok(())
  }

  /// Asynchronously fixes the index templates from the specified path adding
  /// version if missing.
  pub async fn fix_index_templates(&self, input: PathBuf) -> anyhow::Result<()> {
    let files = get_json_file_recursive(&input.join(TEMPLATE_DIRECTORY))?;
    for file in files {
      let body = fs::read_to_string(&file)?;
      let mut body: serde_json::Map<String, Value> = serde_json::from_str(&body)?;
      if !body.contains_key("version") {
        body.insert("version".to_string(), serde_json::Value::from(1));
        write_json_to_file(&file, &serde_json::Value::from(body)).await?;
      }
    }
    Ok(())
  }

  /// Asynchronously fixes the index components from the specified path adding
  /// version if missing.
  pub async fn fix_components(&self, input: PathBuf) -> anyhow::Result<()> {
    let files = get_json_file_recursive(&input.join(COMPONENT_DIRECTORY))?;
    for file in files {
      let body = fs::read_to_string(&file)?;
      let mut body: serde_json::Map<String, Value> = serde_json::from_str(&body)?;
      if !body.contains_key("version") {
        body.insert("version".to_string(), serde_json::Value::from(1));
        write_json_to_file(&file, &serde_json::Value::from(body)).await?;
      }
    }
    Ok(())
  }
}

pub async fn write_json_to_file(path: &PathBuf, json_value: &Value) -> anyhow::Result<()> {
  let json_string = serde_json::to_string_pretty(json_value)?;
  if path.exists() {
    let old_data = fs::read_to_string(path)?;
    if old_data == json_string {
      info!("File {} already exists and is up to date", path.display());
      return Ok(());
    }
  }

  fs::write(path, json_string)?;
  info!("Wrote file: {}", path.display());
  Ok(())
}

pub async fn save_named_map(path: &PathBuf, data: HashMap<String, Value>) -> anyhow::Result<()> {
  // we create the dir in not exists
  fs::create_dir_all(path).unwrap_or_else(|error| {
    eprintln!("Failed to create directory: {}", error);
  });
  // we iterate over the pipelines and dump them
  for (name, value) in data.iter() {
    let value_file = path.join(format!("{}.json", name));
    write_json_to_file(&value_file, value).await?;
  }
  Ok(())
}

fn get_json_file_recursive(path: &PathBuf) -> anyhow::Result<Vec<PathBuf>> {
  if !path.exists() {
    return Ok(Vec::new());
  }
  let mut files = Vec::new();
  for entry in WalkDir::new(path) {
    let entry = entry?;
    if entry.path().is_file() {
      if entry.path().extension().unwrap_or_default() == "json" {
        files.push(entry.path().to_path_buf());
      }
    }
  }
  Ok(files)
}
