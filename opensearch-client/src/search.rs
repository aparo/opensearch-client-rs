use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use reqwest::Client;
use futures::{
  stream::{self, StreamExt},
  Stream,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
// use tracing::{debug, info};
// use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum OpenSearchError {
//   #[error("Fetching error: {0}")]
//   FetchingError(String),
//   #[error("Authentication error: {0}")]
//   Authentication(String),
//   #[error(transparent)]
//   IOError(#[from] std::io::Error),
//   #[error(transparent)]
//   RequestError(#[from] reqwest::Error),
//   #[error(transparent)]
//   JsonRequestError(#[from] serde_json::Error),
//   #[error(transparent)]
//   Other(#[from] anyhow::Error),
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct IndexResponse {
//   #[serde(rename = "_index")]
//   pub index: String,
//   #[serde(rename = "_id")]
//   pub id: String,
//   #[serde(rename = "_version")]
//   pub version: u64,
//   pub result: String,
//   #[serde(rename = "_shards")]
//   pub shards: Shards,
//   #[serde(rename = "_seq_no")]
//   pub seq_no: u64,
//   #[serde(rename = "_primary_term")]
//   pub primary_term: u64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct GetResponse<T> {
//   #[serde(rename = "_index")]
//   pub index: String,
//   #[serde(rename = "_id")]
//   pub id: String,
//   #[serde(rename = "_version")]
//   pub version: u64,
//   #[serde(rename = "_seq_no")]
//   pub seq_no: u64,
//   #[serde(rename = "_primary_term")]
//   pub primary_term: u64,
//   pub found: bool,
//   #[serde(rename = "_source")]
//   pub source: Option<T>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct SearchResult<T> {
//   pub took: u64,
//   #[serde(rename = "timed_out")]
//   pub timed_out: bool,
//   #[serde(rename = "_shards")]
//   pub shards: Shards,
//   pub hits: Hits<T>,
//   pub aggregations: Option<Aggregations>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Shards {
//   pub total: u64,
//   pub successful: u64,
//   pub skipped: Option<u64>,
//   pub failed: u64,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Hits<T> {
//   pub total: Total,
//   #[serde(rename = "max_score")]
//   pub max_score: Option<f64>,
//   pub hits: Vec<Hit<T>>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Total {
//   pub value: u64,
//   pub relation: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Hit<T> {
//   #[serde(rename = "_index")]
//   pub index: String,
//   #[serde(rename = "_id")]
//   pub id: String,
//   #[serde(rename = "_score")]
//   pub score: Option<f64>,
//   #[serde(rename = "_source")]
//   pub source: T,
//   #[serde(rename = "sort")]
//   pub sort: Option<Value>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Aggregations {}

// #[derive(Debug, Clone)]
// pub struct OpenSearch {
//   pub client: Client,
//   server: String,
//   user: String,
//   password: String,
//   pub bulker: Arc<Mutex<String>>,
//   pub bulker_size: Arc<Mutex<u32>>,
//   pub max_bulk_size: u32,
// }


struct SearchAfterState {
  os: OpenSearch,
  index: String,
  stop: bool,
  size: u32,
  query: serde_json::Value,
  sort: serde_json::Value,
  search_after: Option<Value>,
}
// fn serialize_to<W: Write, T: ?Sized + Serialize>(mut writer: W, value: &T) ->
// Result<(), std::io::Error> {     serde_json::to_writer(&mut writer, value)?;
//     writer.write_all(b'\n')
// }

impl OpenSearch {
  pub fn new_from_environment() -> Result<OpenSearch, OpenSearchError> {
    let accept_invalid_certificates: bool = match std::env::var("OPENSEARCH_SSL_VERIFY") {
      Ok(value) => value.eq_ignore_ascii_case("value"),
      Err(_) => false,
    };
    let user: String = match std::env::var("OPENSEARCH_USER") {
      Ok(user) => user,
      Err(_) => "admin".into(),
    };
    let password: String = match std::env::var("OPENSEARCH_PASSWORD") {
      Ok(password) => password,
      Err(_) => "admin".into(),
    };

    let server = match std::env::var("OPENSEARCH_URL") {
      Ok(server) => server,
      Err(_) => "https://localhost:9200".into(),
    };

    let res = reqwest::Client::builder()
      .danger_accept_invalid_certs(accept_invalid_certificates)
      .build()?;
    Ok(OpenSearch {
      client: res,
      user,
      password,
      server,
      bulker: Arc::new(Mutex::new(String::new())),
      bulker_size: Arc::new(Mutex::new(0)),
      max_bulk_size: 10,
    })
  }


  // async fn execute_call(&self, response:&Response) -> Result<Vec<String>,
  // OpenSearchError> {     match response.status() {
  //         reqwest::StatusCode::OK => {
  //             // on success, parse our JSON to an APIResponse
  //             match response.json::<APIResponse>().await {
  //                 Ok(parsed) => println!("Success! {:?}", parsed),
  //                 Err(_) => println!("Hm, the response didn't match the shape
  // we expected."),             };
  //         }
  //         reqwest::StatusCode::UNAUTHORIZED => {
  //             println!("Need to grab a new token");
  //         }
  //         other => {
  //             panic!("Uh oh! Something unexpected happened: {:?}", other);
  //         }
  //     };
  // }






  pub async fn search<T: DeserializeOwned>(
    &self,
    index: &String,
    body: &serde_json::Value,
  ) -> Result<SearchResult<T>, OpenSearchError> {
    let request_url = format!("{}/{}/_search", self.server, index);

    tracing::info!("search {index} {body}");

    let response = self
      .client
      .post(request_url)
      .basic_auth(self.user.as_str(), Some(self.password.as_str()))
      .json(body)
      .send()
      .await?;

    match response.status() {
      reqwest::StatusCode::OK => {
        // on success, parse our JSON to an APIResponse
        let result = response.json::<SearchResult<T>>().await?;
        Ok(result)
      }
      reqwest::StatusCode::UNAUTHORIZED => {
        let text = response.text().await?;
        Err(OpenSearchError::Authentication(text))
      }
      _other => {
        let text = response.text().await?;
        Err(OpenSearchError::FetchingError(text))
      }
    }

    // let body=&response.text().await?;
    // // println!("body: {}",&body);
    // if response.status().is_success() {
    //     let result:SearchResult<T> = serde_json::from_str(body.as_str())?;
    //     return Ok(result);
    // }
    // Err(OpenSearchError::FetchingError(body.clone()))
    // let result = response.json::<SearchResult<T>>().await?;

    // Ok(result)
  }

  pub async fn search_stream<T: DeserializeOwned>(
    &self,
    index: &String,
    query: &serde_json::Value,
    sort: &serde_json::Value,
    size: u32,
  ) -> Result<impl Stream<Item = Hit<T>> + 'static, OpenSearchError> {
    let start_state = SearchAfterState {
      os: self.clone(),
      stop: false,
      search_after: None,
      index: index.clone(),
      query: query.clone(),
      sort: sort.clone(),
      size,
    };

    async fn stream_next<T: DeserializeOwned>(
      state: SearchAfterState,
    ) -> Result<(Vec<Hit<T>>, SearchAfterState), OpenSearchError> {
      let sort = {
        let mut values: Vec<Value> = match state.sort.clone() {
          Value::Null => vec![],
          Value::Bool(_) => vec![],
          Value::Number(_) => vec![],
          Value::String(_) => vec![],
          Value::Array(values) => values,
          Value::Object(x) => vec![Value::Object(x)],
        };
        values.push(json!({"_doc":{"order":"asc"}}));
        Value::Array(values)
      };

      let mut body = json!({
          "query": state.query,
          "size": state.size,
          "sort": sort
      });
      if state.search_after.is_some() {
        body["search_after"] = json!(state.search_after);
      }

      let response = state.os.search(&state.index, &body).await?;
      let hits = response.hits.hits;
      let next_state = SearchAfterState {
        stop: (hits.len() as u32) < state.size,
        search_after: hits.iter().last().and_then(|f| f.sort.clone()),
        ..state
      };

      Ok((hits, next_state))
    }

    let stream = stream::unfold(start_state, move |state| {
      async move {
        if state.stop {
          None
        } else {
          let result = stream_next::<T>(state).await;
          match result {
            Ok((items, state)) => Some((stream::iter(items), state)),
            Err(_err) => None,
          }
        }
      }
    });

    Ok(stream.flatten())
  }

  pub async fn get<T: DeserializeOwned>(&self, index: &String, id: &String) -> Result<GetResponse<T>, OpenSearchError> {
    let request_url = format!("{}/{}/_doc/{id}", self.server, index);

    let response = self
      .client
      .get(request_url)
      .basic_auth(self.user.as_str(), Some(self.password.as_str()))
      .send()
      .await?;
    let result = response.json::<GetResponse<T>>().await?;

    Ok(result)
  }

  pub async fn update(
    &self,
    index: &String,
    id: &String,
    action: &UpdateAction,
  ) -> Result<IndexResponse, OpenSearchError> {
    let request_url = format!("{}/{}/_update/{}", self.server, index, id);
    // let body = serde_json::to_string(&action).unwrap();
    let response = self
      .client
      .post(request_url)
      .basic_auth(self.user.as_str(), Some(self.password.as_str()))
      .json(action)
      .send()
      .await?;
    // let result=response.json::<IndexResponse>().await?;
    let result = response.text().await?;
    // println!("{:}", &result);
    let p: IndexResponse = match serde_json::from_str(&result) {
      Ok(idx) => idx,
      Err(_) => {
        println!("{:}", &result);
        IndexResponse {
          index: "".to_owned(),
          id: "".to_owned(),
          version: 1,
          result: "".to_owned(),
          shards: Shards {
            total: 0,
            successful: 0,
            skipped: None,
            failed: 0,
          },
          seq_no: 0,
          primary_term: 0,
        }
      }
    };
    Ok(p)
  }
}
