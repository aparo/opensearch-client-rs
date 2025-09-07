/*
 * Copyright 2023-2025 Alberto Paro
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Hit;
use crate::{
    search::aggregations::Aggregation as RequestAggregation, search::params::GeoLocation, Map,
};

/// Main aggregation trait equivalent
pub trait AggregationTrait {
    /// Meta information of aggregation
    fn meta(&self) -> Option<&Value>;

    /// Aggregation source
    fn source_aggregation(&self) -> Option<&RequestAggregation>;

    /// Set aggregation source
    fn set_source_aggregation(&mut self, agg: Option<RequestAggregation>);

    /// If the aggregation is empty
    fn is_empty(&self) -> bool;

    /// If the aggregation is not empty
    fn non_empty(&self) -> bool {
        !self.is_empty()
    }

    /// Extract label and count from an aggregation
    fn extract_label_values(&self) -> (Vec<String>, Vec<f64>) {
        (vec![], vec![])
    }
}

/// Main aggregation response enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AggregationResponse {
    /// Bucket aggregation response
    Bucket(BucketAggregation),
    /// Multi-bucket aggregation response  
    MultiBucket(MultiBucketAggregation),
    /// Document count aggregation response
    DocCount(DocCountAggregation),
    /// Geo bounds aggregation response
    GeoBounds(GeoBoundsValue),
    /// Extended metric statistics aggregation response
    MetricExtendedStats(MetricExtendedStats),
    /// Metric statistics aggregation response
    MetricStats(MetricStats),
    /// Metric value aggregation response
    MetricValue(MetricValue),
    /// Top hits aggregation response
    TopHits(TopHitsStats),
    /// Simple aggregation response
    Simple(Simple),
}

impl AggregationTrait for AggregationResponse {
    fn meta(&self) -> Option<&Value> {
        match self {
            AggregationResponse::Bucket(agg) => agg.meta.as_ref(),
            AggregationResponse::MultiBucket(agg) => agg.meta.as_ref(),
            AggregationResponse::DocCount(agg) => agg.meta.as_ref(),
            AggregationResponse::GeoBounds(agg) => agg.meta.as_ref(),
            AggregationResponse::MetricExtendedStats(agg) => agg.meta.as_ref(),
            AggregationResponse::MetricStats(agg) => agg.meta.as_ref(),
            AggregationResponse::MetricValue(agg) => agg.meta.as_ref(),
            AggregationResponse::TopHits(agg) => agg.meta.as_ref(),
            AggregationResponse::Simple(agg) => agg.meta.as_ref(),
        }
    }

    fn source_aggregation(&self) -> Option<&RequestAggregation> {
        match self {
            AggregationResponse::Bucket(agg) => agg.source_aggregation.as_ref(),
            AggregationResponse::MultiBucket(agg) => agg.source_aggregation.as_ref(),
            AggregationResponse::DocCount(agg) => agg.source_aggregation.as_ref(),
            AggregationResponse::GeoBounds(agg) => agg.source_aggregation.as_ref(),
            AggregationResponse::MetricExtendedStats(agg) => agg.source_aggregation.as_ref(),
            AggregationResponse::MetricStats(agg) => agg.source_aggregation.as_ref(),
            AggregationResponse::MetricValue(agg) => agg.source_aggregation.as_ref(),
            AggregationResponse::TopHits(agg) => agg.source_aggregation.as_ref(),
            AggregationResponse::Simple(agg) => agg.source_aggregation.as_ref(),
        }
    }

    fn set_source_aggregation(&mut self, agg: Option<RequestAggregation>) {
        match self {
            AggregationResponse::Bucket(ref mut a) => a.source_aggregation = agg,
            AggregationResponse::MultiBucket(ref mut a) => a.source_aggregation = agg,
            AggregationResponse::DocCount(ref mut a) => a.source_aggregation = agg,
            AggregationResponse::GeoBounds(ref mut a) => a.source_aggregation = agg,
            AggregationResponse::MetricExtendedStats(ref mut a) => a.source_aggregation = agg,
            AggregationResponse::MetricStats(ref mut a) => a.source_aggregation = agg,
            AggregationResponse::MetricValue(ref mut a) => a.source_aggregation = agg,
            AggregationResponse::TopHits(ref mut a) => a.source_aggregation = agg,
            AggregationResponse::Simple(ref mut a) => a.source_aggregation = agg,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            AggregationResponse::Bucket(agg) => agg.buckets.is_empty(),
            AggregationResponse::MultiBucket(_) => true,
            AggregationResponse::DocCount(_) => false,
            AggregationResponse::GeoBounds(_) => false,
            AggregationResponse::MetricExtendedStats(_) => false,
            AggregationResponse::MetricStats(_) => false,
            AggregationResponse::MetricValue(_) => false,
            AggregationResponse::TopHits(_) => false,
            AggregationResponse::Simple(_) => false,
        }
    }

    fn extract_label_values(&self) -> (Vec<String>, Vec<f64>) {
        match self {
            AggregationResponse::Bucket(agg) => {
                let labels: Vec<String> = agg.buckets.iter().map(|b| b.key_to_string()).collect();
                let values: Vec<f64> = agg.buckets.iter().map(|b| b.doc_count as f64).collect();
                (labels, values)
            }
            _ => (vec![], vec![]),
        }
    }
}

/// TopHitsResult equivalent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopHitsResult {
    pub total: i64,
    #[serde(rename = "max_score")]
    pub max_score: Option<f64>,
    pub hits: Vec<Hit>,
}

impl Default for TopHitsResult {
    fn default() -> Self {
        Self {
            total: 0,
            max_score: None,
            hits: vec![],
        }
    }
}

/// Simple aggregation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Simple {
    #[serde(rename = "_source")]
    pub source_aggregation: Option<RequestAggregation>,
    pub meta: Option<Value>,
}

/// Bucket structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bucket {
    pub key: Value,
    #[serde(rename = "doc_count")]
    pub doc_count: i64,
    #[serde(rename = "bg_count")]
    pub bg_count: Option<i64>,
    pub score: Option<f64>,
    #[serde(rename = "key_as_string")]
    pub key_as_string: Option<String>,
    #[serde(flatten)]
    pub sub_aggs: Map<String, AggregationResponse>,
}

impl Bucket {
    pub fn key_to_string(&self) -> String {
        if let Some(ref key_as_string) = self.key_as_string {
            key_as_string.clone()
        } else {
            match &self.key {
                Value::String(s) => s.clone(),
                _ => self.key.to_string(),
            }
        }
    }
}

/// MultiBucketBucket structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultiBucketBucket {
    #[serde(rename = "doc_count")]
    pub doc_count: i64,
    pub buckets: Map<String, BucketAggregation>,
}

/// MultiBucketAggregation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultiBucketAggregation {
    pub buckets: Map<String, MultiBucketBucket>,
    #[serde(rename = "_source")]
    pub source_aggregation: Option<RequestAggregation>,
    pub meta: Option<Value>,
}

/// BucketAggregation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BucketAggregation {
    pub buckets: Vec<Bucket>,
    #[serde(rename = "doc_count_error_upper_bound")]
    pub doc_count_error_upper_bound: i64,
    #[serde(rename = "sum_other_doc_count")]
    pub sum_other_doc_count: i64,
    #[serde(rename = "_source")]
    pub source_aggregation: Option<RequestAggregation>,
    pub meta: Option<Value>,
}

impl Default for BucketAggregation {
    fn default() -> Self {
        Self {
            buckets: vec![],
            doc_count_error_upper_bound: 0,
            sum_other_doc_count: 0,
            source_aggregation: None,
            meta: None,
        }
    }
}

impl BucketAggregation {
    pub fn buckets_count_as_list(&self) -> Vec<(String, i64)> {
        self.buckets
            .iter()
            .map(|b| (b.key_to_string(), b.doc_count))
            .collect()
    }

    pub fn buckets_count_as_map(&self) -> Map<String, i64> {
        self.buckets
            .iter()
            .map(|b| (b.key_to_string(), b.doc_count))
            .collect()
    }
}

/// DocCountAggregation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocCountAggregation {
    #[serde(rename = "doc_count")]
    pub doc_count: f64,
    #[serde(flatten)]
    pub sub_aggs: Map<String, AggregationResponse>,
    #[serde(rename = "_source")]
    pub source_aggregation: Option<RequestAggregation>,
    pub meta: Option<Value>,
}

/// GeoBoundsValue
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoBoundsValue {
    #[serde(rename = "top_left")]
    pub top_left: GeoLocation,
    #[serde(rename = "bottom_right")]
    pub bottom_right: GeoLocation,
    #[serde(rename = "_source")]
    pub source_aggregation: Option<RequestAggregation>,
    pub meta: Option<Value>,
}

/// MetricExtendedStats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetricExtendedStats {
    pub count: i64,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub sum: f64,
    #[serde(rename = "sum_of_squares")]
    pub sum_of_squares: f64,
    pub variance: f64,
    #[serde(rename = "std_deviation")]
    pub std_deviation: f64,
    #[serde(rename = "_source")]
    pub source_aggregation: Option<RequestAggregation>,
    pub meta: Option<Value>,
}

impl Default for MetricExtendedStats {
    fn default() -> Self {
        Self {
            count: 0,
            min: 0.0,
            max: 0.0,
            avg: 0.0,
            sum: 0.0,
            sum_of_squares: 0.0,
            variance: 0.0,
            std_deviation: 0.0,
            source_aggregation: None,
            meta: None,
        }
    }
}

/// TopHitsStats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopHitsStats {
    pub hits: TopHitsResult,
    #[serde(rename = "_source")]
    pub source_aggregation: Option<RequestAggregation>,
    pub meta: Option<Value>,
}

/// MetricStats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetricStats {
    pub count: i64,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub sum: f64,
    #[serde(rename = "_source")]
    pub source_aggregation: Option<RequestAggregation>,
    pub meta: Option<Value>,
}

impl Default for MetricStats {
    fn default() -> Self {
        Self {
            count: 0,
            min: 0.0,
            max: 0.0,
            avg: 0.0,
            sum: 0.0,
            source_aggregation: None,
            meta: None,
        }
    }
}

/// MetricValue
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetricValue {
    pub value: f64,
    #[serde(rename = "_source")]
    pub source_aggregation: Option<RequestAggregation>,
    pub meta: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_metric_value_serialization() {
        let metric = MetricValue {
            value: 42.5,
            source_aggregation: None,
            meta: None,
        };

        let json = serde_json::to_value(&metric).unwrap();
        assert_eq!(json["value"], 42.5);
    }

    #[test]
    fn test_bucket_key_to_string() {
        let bucket = Bucket {
            key: json!("test_key"),
            doc_count: 10,
            bg_count: None,
            score: None,
            key_as_string: Some("test_key_string".to_string()),
            sub_aggs: Map::new(),
        };

        assert_eq!(bucket.key_to_string(), "test_key_string");

        let bucket2 = Bucket {
            key: json!("another_key"),
            doc_count: 5,
            bg_count: None,
            score: None,
            key_as_string: None,
            sub_aggs: Map::new(),
        };

        assert_eq!(bucket2.key_to_string(), "another_key");
    }

    #[test]
    fn test_aggregation_trait() {
        let bucket_agg = BucketAggregation {
            buckets: vec![],
            doc_count_error_upper_bound: 0,
            sum_other_doc_count: 0,
            source_aggregation: None,
            meta: None,
        };

        let agg = AggregationResponse::Bucket(bucket_agg);
        assert!(agg.is_empty());
        assert!(!agg.non_empty());
    }
}
