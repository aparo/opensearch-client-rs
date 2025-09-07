use super::ShardFailure;
use crate::util::ShouldSkip;

/// Number of shards touched with their states
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ShardStatistics {
    /// Total number of touched shards
    #[serde(default)]
    pub total: u32,

    /// Total number of successful shards
    #[serde(default)]
    pub successful: u32,

    /// Total number of skipped shards
    #[serde(default)]
    pub skipped: u32,

    /// Total number of failed shards
    #[serde(default)]
    pub failed: u32,

    /// Partial response failures
    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    pub failures: Vec<ShardFailure>,
}

impl Default for ShardStatistics {
    fn default() -> Self {
        Self {
            total: 1,
            successful: 1,
            skipped: 0,
            failed: 0,
            failures: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_successfully() {
        let value = json!({
          "total": 280,
          "successful": 277,
          "skipped": 0,
          "failed": 3,
          "failures": [
            {
              "shard": 1,
              "index": "nbs_comprehend-2021-w41",
              "node": "oGEHA-aRSnmwuEmqSZc6Kw",
              "reason": {
                "type": "script_exception",
                "reason": "runtime error",
                "script_stack": [
                  "org.opensearch.index.fielddata.ScriptDocValues$Longs.get(ScriptDocValues.java:121)",
                  "org.opensearch.index.fielddata.ScriptDocValues$Longs.getValue(ScriptDocValues.java:115)",
                  "doc['user.followers_count'].value > 9999 ? 1 : 0",
                  "                           ^---- HERE"
                ],
                "script": "doc['user.followers_count'].value > 9999 ? 1 : 0",
                "lang": "painless",
                "position": {
                  "offset": 27,
                  "start": 0,
                  "end": 48
                },
                "caused_by": {
                  "type": "illegal_state_exception",
                  "reason": "A document doesn't have a value for a field! Use doc[<field>].size()==0 to check if a document is missing a field!"
                }
              }
            }
          ]
        });

        let _ = serde_json::from_value::<ShardStatistics>(value).unwrap();
    }
}
