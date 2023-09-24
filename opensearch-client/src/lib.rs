use std::sync::{Arc, Mutex};

use reqwest::Client;

use futures::{
    stream::{self, StreamExt},
    Stream,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use tracing::debug;
use tracing::info;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenSearchError {
    #[error("Fetching error: {0}")]
    FetchingError(String),
    #[error("Authentication error: {0}")]
    Authentication(String),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    JsonRequestError(#[from] serde_json::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexResponse {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_version")]
    pub version: u64,
    pub result: String,
    #[serde(rename = "_shards")]
    pub shards: Shards,
    #[serde(rename = "_seq_no")]
    pub seq_no: u64,
    #[serde(rename = "_primary_term")]
    pub primary_term: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetResponse<T> {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_version")]
    pub version: u64,
    #[serde(rename = "_seq_no")]
    pub seq_no: u64,
    #[serde(rename = "_primary_term")]
    pub primary_term: u64,
    pub found: bool,
    #[serde(rename = "_source")]
    pub source: Option<T>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult<T> {
    pub took: u64,
    #[serde(rename = "timed_out")]
    pub timed_out: bool,
    #[serde(rename = "_shards")]
    pub shards: Shards,
    pub hits: Hits<T>,
    pub aggregations: Option<Aggregations>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shards {
    pub total: u64,
    pub successful: u64,
    pub skipped: Option<u64>,
    pub failed: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hits<T> {
    pub total: Total,
    #[serde(rename = "max_score")]
    pub max_score: Option<f64>,
    pub hits: Vec<Hit<T>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Total {
    pub value: u64,
    pub relation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hit<T> {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_score")]
    pub score: Option<f64>,
    #[serde(rename = "_source")]
    pub source: T,
    #[serde(rename = "sort")]
    pub sort: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggregations {}

#[derive(Debug, Clone)]
pub struct OpenSearch {
    pub client: Client,
    server: String,
    user: String,
    password: String,
    pub bulker: Arc<Mutex<String>>,
    pub bulker_size: Arc<Mutex<u32>>,
    pub max_bulk_size: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Script {
    #[serde(default)]
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_as_upsert: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<Script>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BulkAction {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    #[serde(rename = "_index")]
    pub index: String,
    pub pipeline: Option<String>,
}

struct SearchAfterState {
    os: OpenSearch,
    index: String,
    stop: bool,
    size: u32,
    query: serde_json::Value,
    sort: serde_json::Value,
    search_after: Option<Value>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkResponse {
    took: u64,
    errors: bool,
    items: Vec<HashMap<String, BulkItemResponse>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BulkError {
    #[serde(rename = "_index")]
    pub index: Option<String>,
    #[serde(default)]
    pub index_uuid: Option<String>,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub shard: Option<String>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BulkItemResponse {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_index")]
    index: String,
    #[serde(rename = "_version")] version: Option<u32>,
    #[serde(default)]
    status: u16,
    found: Option<bool>,
    #[serde(rename = "_shards")] shards: Option<Shards>,
    #[serde(default)]
    error: Option<BulkError>,
    #[serde(default, rename = "_primary_term")]
    primary_term: Option<u32>,
    #[serde(default, rename = "_seq_no")]
    seq_no: Option<u32>,
}
// fn serialize_to<W: Write, T: ?Sized + Serialize>(mut writer: W, value: &T) -> Result<(), std::io::Error> {
//     serde_json::to_writer(&mut writer, value)?;
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

    pub async fn list_indices(&self) -> Result<Vec<String>, OpenSearchError> {
        let request_url = format!("{}/_cat/indices", self.server);
        let response = self
            .client
            .get(request_url)
            .query(&[("s", "i")])
            .basic_auth(self.user.as_str(), Some(self.password.as_str()))
            .send()
            .await?;
        let cat_result = response.text().await?;
        let values: Vec<String> = cat_result
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(|x| {
                let mut iterator = x.split_ascii_whitespace();
                iterator.nth(2).unwrap().to_owned()
            })
            .collect();
        Ok(values)
    }

    // async fn execute_call(&self, response:&Response) -> Result<Vec<String>, OpenSearchError> {
    //     match response.status() {
    //         reqwest::StatusCode::OK => {
    //             // on success, parse our JSON to an APIResponse
    //             match response.json::<APIResponse>().await {
    //                 Ok(parsed) => println!("Success! {:?}", parsed),
    //                 Err(_) => println!("Hm, the response didn't match the shape we expected."),
    //             };
    //         }
    //         reqwest::StatusCode::UNAUTHORIZED => {
    //             println!("Need to grab a new token");
    //         }
    //         other => {
    //             panic!("Uh oh! Something unexpected happened: {:?}", other);
    //         }
    //     };
    // }

    pub async fn bulk_index_document<T:Serialize>(
        &self,
        index: &String,
        id: Option<String>,
        body: &T,
    ) -> Result<serde_json::Value, OpenSearchError> {
        let body_json = serde_json::to_value(body)?;
        let action = BulkAction {
            index: index.clone(),
            id: id.clone(),
            pipeline: None,
        };
        self.bulk_action("index", action, Some(&body_json)).await
    }

    pub async fn bulk_action(
        &self,
        command: &str,
        action: BulkAction,
        body: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, OpenSearchError> {
        let mut m: serde_json::Map<String, Value> = serde_json::Map::new();
        m.insert(command.to_owned(), json!(action));
        let j = serde_json::to_string(&Value::Object(m.clone()))?;
        let bulker_arc = Arc::clone(&self.bulker);
        let mut bulker = bulker_arc.lock().unwrap();
        bulker.push_str(j.as_str());
        bulker.push('\n');
        match body {
            None => {}
            Some(js) => {
                let j = serde_json::to_string(js)?;
                bulker.push_str(j.as_str());
                bulker.push('\n');
            }
        }

        let bulker_size_arc = Arc::clone(&self.bulker_size);
        let mut bulker_size = bulker_size_arc.lock().unwrap();
        *bulker_size += 1;
        if *bulker_size >= self.max_bulk_size {
            drop(bulker_size);
            drop(bulker);
            self.flush_bulk().await?;
        }
        Ok(Value::Object(m))
    }

    pub async fn bulk_create_document<T: Serialize>(
        &self,
        index: &String,
        id: &String,
        body: &T,
    ) -> Result<serde_json::Value, OpenSearchError> {
        let body_json = serde_json::to_value(body)?;

        let action = BulkAction {
            index: index.clone(),
            id: Some(id.clone()),
            pipeline: None,
        };

        self.bulk_action("create", action, Some(&body_json)).await
    }

    pub async fn bulk_update_document(
        &self,
        index: &String,
        id: &String,
        body: &UpdateAction,
    ) -> Result<serde_json::Value, OpenSearchError> {
        let action = BulkAction {
            index: index.clone(),
            id: Some(id.clone()),
            pipeline: None,
        };
        let j = serde_json::to_value(body)?;
        self.bulk_action("update", action, Some(&j)).await
    }

    pub async fn flush_bulk(&self) -> Result<BulkResponse, OpenSearchError> {
        let bulker_size_arc = Arc::clone(&self.bulker_size);
        let mut bulker_size = bulker_size_arc.lock().unwrap();
        if *bulker_size > 0 {
            let bulker_arc = Arc::clone(&self.bulker);
            let mut bulker = bulker_arc.lock().unwrap();

            let request_url = format!("{}/_bulk", self.server);

            match self
                .client
                .post(request_url)
                .basic_auth(self.user.as_str(), Some(self.password.as_str()))
                .body(bulker.to_owned())
                .header("Content-Type", "application/x-ndjson")
                .send()
                .await
            {
                Ok(response) => {
                    let result:BulkResponse = response.json().await?;
                    *bulker = String::new();
                    *bulker_size = 0;
                    debug!("{:?}", &result);
                    if result.errors {
                        for map in &result.items {
                            for (_, value) in map.iter() {
                                if let Some(error)=&value.error {
                                    if !error.kind.eq_ignore_ascii_case("version_conflict_engine_exception") {
                                        info!("{:?}", &value);
                                    }
                                }
                            }
                        }
                    }

                    Ok(result)
                }
                Err(err) => {
                    println!("{:?}", &err);
                    Err(OpenSearchError::RequestError(err))
                }
            }
        } else {
            Ok(BulkResponse { took: 0, errors: false, items: vec!() })
        }
    }

    pub async fn index_document<T: Serialize>(
        &self,
        index: &String,
        body: &T,
        id: Option<String>,
    ) -> Result<serde_json::Value, OpenSearchError> {
        let request_url = match id {
            Some(real_id) => format!("{}/{}/_doc/{}", self.server, index, real_id),
            None => format!("{}/{}/_doc", self.server, index),
        };

        let body_json = serde_json::to_value(body)?;

        let response = self
            .client
            .post(request_url)
            .basic_auth(self.user.as_str(), Some(self.password.as_str()))
            .json(&body_json)
            .send()
            .await?;
        let result = response.json().await?;

        Ok(result)
    }

    pub async fn create_document<T: Serialize>(
        &self,
        index: &String,
        id: &String,
        body: &T,
    ) -> Result<serde_json::Value, OpenSearchError> {
        let request_url = format!("{}/{}/_doc/{}", self.server, index, id);
        let body_json = serde_json::to_value(body)?;

        let response = self
            .client
            .put(request_url)
            .basic_auth(self.user.as_str(), Some(self.password.as_str()))
            .json(&body_json)
            .send()
            .await?;
        let result = response.json().await?;

        Ok(result)
    }

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

        let stream = stream::unfold(start_state, move |state| async move {
            if state.stop {
                None
            } else {
                let result = stream_next::<T>(state).await;
                match result {
                    Ok((items, state)) => Some((stream::iter(items), state)),
                    Err(_err) => None,
                }
            }
        });

        Ok(stream.flatten())
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        index: &String,
        id: &String,
    ) -> Result<GetResponse<T>, OpenSearchError> {
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
