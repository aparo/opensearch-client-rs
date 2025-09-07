use crate::{search::*, util::*};

/// Filters documents based on a provided
/// [script](https://www.elastic.co/guide/en/opensearch/reference/current/modules-scripting-using.html).
/// The script query is typically used in a
/// [filter context](https://www.elastic.co/guide/en/opensearch/reference/current/query-filter-context.html).
///
/// To create script query:
/// ```
/// # use opensearch_dsl::queries::*;
/// # use opensearch_dsl::queries::params::*;
/// # let query =
/// Query::script(Script::source("return doc['amount'].value < 10;"));
/// ```
/// <https://www.elastic.co/guide/en/opensearch/reference/current/query-dsl-script-query.html>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(remote = "Self")]
pub struct ScriptQuery {
    script: Script,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    boost: Option<f32>,

    #[serde(default, skip_serializing_if = "ShouldSkip::should_skip")]
    _name: Option<String>,
}

impl Query {
    /// Creates an instance of [`ScriptQuery`]
    ///
    /// - `script` - Contains a script to run as a query. This script must
    /// return a boolean value, `true` or `false`
    pub fn script(script: Script) -> ScriptQuery {
        ScriptQuery {
            script,
            boost: None,
            _name: None,
        }
    }
}

impl ScriptQuery {
    add_boost_and_name!();
}

impl ShouldSkip for ScriptQuery {}

serialize_with_root!("script": ScriptQuery);
deserialize_with_root!("script": ScriptQuery);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_serialize_query(
            Query::script(
                Script::source("doc['numberOfCommits'].value > params.param1").param("param1", 50),
            )
            .name("_named_query")
            .boost(1.1),
            json!({
                "script": {
                    "_name": "_named_query",
                    "boost": 1.1,
                    "script": {
                        "source": "doc['numberOfCommits'].value > params.param1",
                        "params": {
                            "param1": 50
                        }
                    }
                }
            }),
        );
    }
}
