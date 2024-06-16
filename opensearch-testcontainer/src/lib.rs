use std::{borrow::Cow, collections::HashMap};

use testcontainers::{core::*, Image};

const NAME: &str = "opensearchproject/opensearch";
const TAG: &str = "2.14.0";

#[derive(Debug, Clone)]
pub struct OpenSearch {
  image_name: String,
  env_vars: HashMap<String, String>,
  tag: String,
  username: String,
  password: String,
}

impl OpenSearch {
  pub fn with_name(mut self, name: &str) -> Self {
    self.image_name = name.to_owned();
    self
  }

  pub fn with_tag(mut self, tag: &str) -> Self {
    self.tag = tag.to_owned();
    self
  }

  pub fn with_cluster_name(mut self, name: &str) -> Self {
    self.env_vars.insert("cluster.name".to_owned(), name.to_owned());
    self
  }

  pub fn with_env_var(mut self, key: &str, value: &str) -> Self {
    self.env_vars.insert(key.to_owned(), value.to_owned());
    self
  }

  pub fn username(&self) -> String {
    self.username.to_owned()
  }

  pub fn password(&self) -> String {
    self.password.to_owned()
  }
}

impl Default for OpenSearch {
  fn default() -> Self {
    let mut env_vars = HashMap::new();
    env_vars.insert("discovery.type".to_owned(), "single-node".to_owned());
    OpenSearch {
      image_name: NAME.to_owned(),
      env_vars,
      tag: TAG.to_owned(),
      username: "admin".to_owned(),
      password: "admin".to_owned(),
    }
  }
}

impl Image for OpenSearch {
  fn name(&self) -> &str {
    &self.image_name.as_str()
  }

  fn tag(&self) -> &str {
    self.tag.as_str()
  }

  fn ready_conditions(&self) -> Vec<WaitFor> {
    vec![
      WaitFor::message_on_stdout("[YELLOW] to [GREEN]"),
      WaitFor::message_on_stdout("ML configuration initialized successfully"),
    ]
  }

  fn env_vars(&self) -> impl IntoIterator<Item = (impl Into<Cow<'_, str>>, impl Into<Cow<'_, str>>)> {
    Box::new(self.env_vars.iter())
  }

  fn expose_ports(&self) -> &[ContainerPort] {
    &[
      ContainerPort::Tcp(9200),
      ContainerPort::Tcp(9300),
      ContainerPort::Tcp(9600),
    ]
  }
}

#[cfg(test)]
mod tests {
  use testcontainers::runners::AsyncRunner;

  use crate::OpenSearch as OpenSearchImage;

  #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
  async fn opensearch_default() {
    let os_image = OpenSearchImage::default();
    let opensearch = os_image.clone().start().await.unwrap();
    let host_port = opensearch.get_host_port_ipv4(9200).await.unwrap();

    let client = reqwest::blocking::Client::builder()
      .danger_accept_invalid_certs(true)
      .build()
      .unwrap();

    let response = client
      .get(format!("https://127.0.0.1:{host_port}"))
      .header("content-type", "application/json")
      .basic_auth(os_image.username(), Some(os_image.password()))
      .send()
      .unwrap();

    let response = response.text().unwrap();
    println!("response: {}", response);
    let response: serde_json::Value = serde_json::from_str(&response).unwrap();

    assert_eq!(response["version"]["number"], "2.14.0");
  }
}
