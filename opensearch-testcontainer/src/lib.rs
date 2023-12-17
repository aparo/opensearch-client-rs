use std::collections::HashMap;

use testcontainers::{core::WaitFor, Image};

const NAME: &str = "opensearchproject/opensearch";
const TAG: &str = "2.11.1";

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

  pub fn username(&self) -> &str {
    self.username.as_str()
  }

  pub fn password(&self) -> &str {
    self.password.as_str()
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
  type Args = ();

  fn name(&self) -> String {
    NAME.to_owned()
  }

  fn tag(&self) -> String {
    self.tag.to_owned()
  }

  fn ready_conditions(&self) -> Vec<WaitFor> {
    vec![WaitFor::message_on_stdout("[YELLOW] to [GREEN]")]
  }

  fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
    Box::new(self.env_vars.iter())
  }

  fn expose_ports(&self) -> Vec<u16> {
    vec![9200, 9300, 9600]
  }
}

#[cfg(test)]
mod tests {
  use testcontainers::{clients, RunnableImage};

  use crate::OpenSearch as OpenSearchImage;

  #[test]
  fn opensearch_default() {
    let docker = clients::Cli::default();
    let os_image = OpenSearchImage::default();
    let node = docker.run(os_image.clone());
    let host_port = node.get_host_port_ipv4(9200);

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

    assert_eq!(response["version"]["number"], "2.11.1");
  }
}
