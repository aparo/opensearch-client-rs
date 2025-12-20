use opensearch_dsl::Query;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::{
    common, common::DocumentDeleteResponse, get_opensearch, Error, IndexResponse, UpdateActionBody,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: String,
    pub os_type: String,
    pub aggregatable: bool,
    pub searchable: bool,
    pub sub_fields: Vec<Box<Field>>,
}

#[async_trait::async_trait]
pub trait Document: Serialize + DeserializeOwned + Sized + std::clone::Clone {
    /// The Elasticsearch index name where documents of this type live
    fn index_name() -> String;
    /// Return the unique ID of this document
    fn id(&self) -> String;

    fn columns() -> Vec<Field>;

    /// Create or update this document in Elasticsearch
    async fn save(&self) -> Result<IndexResponse, Error> {
        let doc_id = self.id().to_string();
        let doc_body = serde_json::to_value(self)?;

        get_opensearch()
            .index_document(&Self::index_name(), &doc_body, Some(doc_id))
            .await
    }

    /// Fetch a document by ID
    async fn get(id: &str) -> Result<Self, Error> {
        let index = Self::index_name();
        let result = get_opensearch().get_typed::<Self>(&index, id).await?;
        match result.source {
            None => {
                return Err(Error::DocumentNotFoundError(
                    index.to_owned(),
                    id.to_owned(),
                ))
            }
            Some(value) => Ok(value),
        }
    }

    /// Delete a document by ID
    async fn delete(id: &str) -> Result<DocumentDeleteResponse, Error> {
        let index = Self::index_name();
        get_opensearch().delete().index(index).id(id).call().await
    }

    /// Update a document with partial data and return the updated document
    async fn update(id: &str, partial_doc: &Value) -> Result<IndexResponse, Error> {
        let index = Self::index_name();
        let update_body = UpdateActionBody {
            doc: Some(partial_doc.clone()),
            doc_as_upsert: Some(false),
            ..Default::default()
        };

        get_opensearch()
            .update_document(&index, id, &update_body)
            .await
    }

    /// Refresh this document instance with the latest data from Elasticsearch
    async fn refresh(&mut self) -> Result<(), Error> {
        let updated_doc = Self::get(&self.id()).await?;
        *self = updated_doc;
        Ok(())
    }

    /// Find documents using query parameters
    async fn find(search: common::Search) -> Result<crate::search::TypedSearchResult<Self>, Error> {
        let index = Self::index_name();
        get_opensearch().search_typed::<Self>(&index, search).await
    }

    /// Find all documents (with optional limit)
    async fn find_all(
        limit: Option<usize>,
    ) -> Result<crate::search::TypedSearchResult<Self>, Error> {
        let mut params = common::Search::default();
        if let Some(l) = limit {
            params = params.size(l as u64);
        }
        Self::find(params).await
    }

    /// Find one document matching the query parameters
    async fn find_one(params: common::Search) -> Result<Option<Self>, Error> {
        let single_params = params.size(1);

        let results = Self::find(single_params).await?;
        Ok(results
            .hits
            .hits
            .into_iter()
            .next()
            .and_then(|f| f.source.clone()))
    }

    /// Count documents matching the query
    async fn count(params: Option<Query>) -> Result<u32, Error> {
        let index = Self::index_name();
        let query: Query = params.unwrap_or(Query::MatchAll(Query::match_all()));

        let response = get_opensearch()
            .count()
            .index(index)
            .query(query)
            .call()
            .await?;
        Ok(response.count)
    }
}
