#[cfg(test)]
mod tests {
  use serde_json::json;
  use testcontainers::clients;
  use opensearch_testcontainer::*;
  use opensearch::{url::Url, OsClientBuilder};
  #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
  async fn opensearch_default() -> Result<(), Box<dyn std::error::Error>> {
    let docker = clients::Cli::default();
    let os_image = OpenSearch::default();
    let node = docker.run(os_image.clone());
    let host_port = node.get_host_port_ipv4(9200);

    let client = OsClientBuilder::new()
      .accept_invalid_certificates(true)
      .base_url(Url::parse(&format!("https://127.0.0.1:{host_port}")).unwrap())
      .basic_auth(os_image.username(), os_image.password())
      .build();

    let bulker = client.bulker().bulk_size(1000).max_concurrent_connections(10).build();
    for i in 0..10000 {
      bulker
        .index("test", &json!({"id":i}), Some(i.to_string()))
        .await
        .unwrap();
    }
    drop(bulker);

    let count = client.count().index("test").send().await.unwrap();
    assert_eq!(count.count, 10000);
    Ok(())
  }
}
