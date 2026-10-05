/*
 * opensearch-client
 *
 * Rust Client for OpenSearch
 * Tests for mapping types.
 */

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::{json, Value};

    use crate::common::mapping::{
        autocomplete_property::{CompletionProperty, SearchAsYouTypeProperty},
        boolean_property::BooleanProperty,
        date_property::{DateNanosProperty, DateProperty},
        dynamic_template::DynamicTemplate,
        enums::{
            DynamicMapping, IndexOptions, KnnEngine, KnnSpaceType, MatchPattern, Similarity,
            TermVector,
        },
        geo_property::{GeoPointProperty, GeoShapeProperty},
        ip_property::IpProperty,
        knn_vector_method::KnnVectorMethod,
        numeric_property::{NumericProperty, ScaledFloatProperty},
        object_property::{JoinProperty, NestedProperty, ObjectProperty},
        range_property::DateRangeProperty,
        specialized_property::{AggregateMetricDoubleProperty, AliasProperty, BinaryProperty},
        text_property::{
            ConstantKeywordProperty, KeywordProperty, TextProperty, TokenCountProperty,
            VersionProperty, WildcardProperty,
        },
        type_mapping::TypeMapping,
        vector_property::KnnVectorProperty,
        Property,
    };

    // ── Property serialization ─────────────────────────────────────────────────

    #[test]
    fn test_text_property_serializes_type_field() {
        let p = Property::text(TextProperty::new().with_analyzer("standard"));
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "text");
        assert_eq!(v["analyzer"], "standard");
    }

    #[test]
    fn test_keyword_property_serializes_type_field() {
        let p = Property::keyword(KeywordProperty::new().with_ignore_above(256));
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "keyword");
        assert_eq!(v["ignore_above"], 256);
    }

    #[test]
    fn test_integer_property_serializes_type_field() {
        let p = Property::integer();
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "integer");
    }

    #[test]
    fn test_float_property_roundtrip() {
        let p = Property::float();
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "float");
        let deserialized: Property = serde_json::from_value(v).unwrap();
        assert_eq!(deserialized.type_name(), "float");
    }

    #[test]
    fn test_boolean_property_serializes_type_field() {
        let mut bp = BooleanProperty::new();
        bp.null_value = Some(false);
        let p = Property::Boolean(Box::new(bp));
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "boolean");
        assert_eq!(v["null_value"], false);
    }

    #[test]
    fn test_date_property_with_format() {
        let dp = DateProperty::new().with_format("yyyy-MM-dd||epoch_millis");
        let p = Property::Date(Box::new(dp));
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "date");
        assert_eq!(v["format"], "yyyy-MM-dd||epoch_millis");
    }

    #[test]
    fn test_date_nanos_property() {
        let p = Property::date_nanos();
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "date_nanos");
    }

    #[test]
    fn test_ip_property() {
        let mut ip = IpProperty::new();
        ip.ignore_malformed = Some(true);
        let p = Property::Ip(Box::new(ip));
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "ip");
        assert_eq!(v["ignore_malformed"], true);
    }

    #[test]
    fn test_geo_point_property() {
        let p = Property::geo_point();
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "geo_point");
    }

    #[test]
    fn test_geo_shape_property() {
        let p = Property::geo_shape();
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "geo_shape");
    }

    #[test]
    fn test_scaled_float_requires_scaling_factor() {
        let sf = ScaledFloatProperty::new(100.0);
        let p = Property::ScaledFloat(Box::new(sf));
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "scaled_float");
        assert_eq!(v["scaling_factor"], 100.0);
    }

    #[test]
    fn test_wildcard_property() {
        let mut wp = WildcardProperty::new();
        wp.ignore_above = Some(1024);
        let p = Property::Wildcard(Box::new(wp));
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "wildcard");
        assert_eq!(v["ignore_above"], 1024);
    }

    #[test]
    fn test_constant_keyword_property() {
        let p = Property::ConstantKeyword(Box::new(ConstantKeywordProperty::new("logs")));
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "constant_keyword");
        assert_eq!(v["value"], "logs");
    }

    #[test]
    fn test_version_property() {
        let p = Property::Version(Box::new(VersionProperty::new()));
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "version");
    }

    // ── Range types ────────────────────────────────────────────────────────────

    #[test]
    fn test_integer_range_property() {
        let p = Property::IntegerRange(Box::default());
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "integer_range");
    }

    #[test]
    fn test_date_range_property_with_format() {
        let dr = DateRangeProperty::new().with_format("yyyy-MM-dd");
        let p = Property::DateRange(Box::new(dr));
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "date_range");
        assert_eq!(v["format"], "yyyy-MM-dd");
    }

    #[test]
    fn test_ip_range_property() {
        let p = Property::IpRange(Box::default());
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "ip_range");
    }

    // ── Object types ───────────────────────────────────────────────────────────

    #[test]
    fn test_object_property_with_subfields() {
        let obj = ObjectProperty::new().with_property("name", json!({"type": "keyword"}));
        let p = Property::object(obj);
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "object");
        assert_eq!(v["properties"]["name"]["type"], "keyword");
    }

    #[test]
    fn test_nested_property() {
        let nested = NestedProperty::new()
            .with_property("tag", json!({"type": "keyword"}))
            .with_property("score", json!({"type": "float"}));
        let p = Property::nested(nested);
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "nested");
        assert_eq!(v["properties"]["tag"]["type"], "keyword");
    }

    #[test]
    fn test_flat_object_property() {
        let p = Property::FlatObject(Box::default());
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "flat_object");
    }

    #[test]
    fn test_join_property() {
        let join = JoinProperty::new()
            .with_relation("question", json!("answer"));
        let p = Property::Join(Box::new(join));
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "join");
        assert_eq!(v["relations"]["question"], "answer");
    }

    // ── Autocomplete types ─────────────────────────────────────────────────────

    #[test]
    fn test_completion_property() {
        let mut cp = CompletionProperty::new();
        cp.analyzer = Some("simple".into());
        cp.max_input_length = Some(50);
        let p = Property::Completion(Box::new(cp));
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "completion");
        assert_eq!(v["analyzer"], "simple");
        assert_eq!(v["max_input_length"], 50);
    }

    #[test]
    fn test_search_as_you_type_property() {
        let mut sayt = SearchAsYouTypeProperty::new();
        sayt.max_shingle_size = Some(3);
        let p = Property::SearchAsYouType(Box::new(sayt));
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "search_as_you_type");
        assert_eq!(v["max_shingle_size"], 3);
    }

    // ── Vector types ───────────────────────────────────────────────────────────

    #[test]
    fn test_knn_vector_property_dimension() {
        let p = Property::knn_vector(128);
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "knn_vector");
        assert_eq!(v["dimension"], 128);
    }

    #[test]
    fn test_knn_vector_property_with_method() {
        let method = KnnVectorMethod::hnsw(KnnEngine::Faiss)
            .with_space_type(KnnSpaceType::L2)
            .with_parameters(json!({"ef_construction": 128, "m": 16}));
        let p = Property::KnnVector(Box::new(KnnVectorProperty::new(256).with_method(method)));
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "knn_vector");
        assert_eq!(v["dimension"], 256);
        assert_eq!(v["method"]["name"], "hnsw");
        assert_eq!(v["method"]["engine"], "faiss");
        assert_eq!(v["method"]["space_type"], "l2");
        assert_eq!(v["method"]["parameters"]["ef_construction"], 128);
    }

    #[test]
    fn test_rank_feature_property() {
        let p = Property::RankFeature(Box::default());
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "rank_feature");
    }

    #[test]
    fn test_rank_features_property() {
        let p = Property::RankFeatures(Box::default());
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "rank_features");
    }

    // ── Specialized types ──────────────────────────────────────────────────────

    #[test]
    fn test_binary_property() {
        let p = Property::Binary(Box::new(BinaryProperty::new()));
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "binary");
    }

    #[test]
    fn test_alias_property() {
        let p = Property::alias("user.name");
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "alias");
        assert_eq!(v["path"], "user.name");
    }

    #[test]
    fn test_percolator_property() {
        let p = Property::Percolator(Box::default());
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "percolator");
    }

    #[test]
    fn test_aggregate_metric_double_property() {
        let amd = AggregateMetricDoubleProperty::new(
            vec!["min".into(), "max".into(), "sum".into(), "value_count".into()],
            "max",
        );
        let p = Property::AggregateMetricDouble(Box::new(amd));
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "aggregate_metric_double");
        assert_eq!(v["default_metric"], "max");
    }

    // ── TypeMapping ────────────────────────────────────────────────────────────

    #[test]
    fn test_type_mapping_with_properties() {
        let mapping = TypeMapping::new()
            .with_dynamic(DynamicMapping::Strict)
            .with_property("title", Property::text(TextProperty::new().with_analyzer("standard")))
            .with_property("status", Property::keyword(KeywordProperty::new().with_ignore_above(64)))
            .with_property("score", Property::float())
            .with_property("created_at", Property::date());

        let v: Value = serde_json::to_value(&mapping).unwrap();
        assert_eq!(v["dynamic"], "strict");
        assert_eq!(v["properties"]["title"]["type"], "text");
        assert_eq!(v["properties"]["title"]["analyzer"], "standard");
        assert_eq!(v["properties"]["status"]["type"], "keyword");
        assert_eq!(v["properties"]["status"]["ignore_above"], 64);
        assert_eq!(v["properties"]["score"]["type"], "float");
        assert_eq!(v["properties"]["created_at"]["type"], "date");
    }

    #[test]
    fn test_type_mapping_roundtrip() {
        let original = TypeMapping::new()
            .with_property("id", Property::long())
            .with_property("name", Property::keyword(KeywordProperty::new()))
            .with_property("active", Property::boolean());

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TypeMapping = serde_json::from_str(&json).unwrap();

        let props = deserialized.properties.unwrap();
        assert_eq!(props["id"].type_name(), "long");
        assert_eq!(props["name"].type_name(), "keyword");
        assert_eq!(props["active"].type_name(), "boolean");
    }

    // ── DynamicTemplate ────────────────────────────────────────────────────────

    #[test]
    fn test_dynamic_template_strings_as_keyword() {
        let tmpl = DynamicTemplate::strings_as_keyword();
        let v: Value = serde_json::to_value(&tmpl).unwrap();
        assert_eq!(v["match_mapping_type"], "string");
        assert_eq!(v["mapping"]["type"], "keyword");
    }

    #[test]
    fn test_dynamic_template_strings_as_text() {
        let tmpl = DynamicTemplate::strings_as_text();
        let v: Value = serde_json::to_value(&tmpl).unwrap();
        assert_eq!(v["match_mapping_type"], "string");
        assert_eq!(v["mapping"]["type"], "text");
    }

    #[test]
    fn test_dynamic_template_with_match_pattern() {
        let tmpl = DynamicTemplate {
            r#match: Some("*_text".into()),
            match_pattern: Some(MatchPattern::Simple),
            mapping: Some(Property::text(TextProperty::new())),
            ..Default::default()
        };
        let v: Value = serde_json::to_value(&tmpl).unwrap();
        assert_eq!(v["match"], "*_text");
        assert_eq!(v["match_pattern"], "simple");
        assert_eq!(v["mapping"]["type"], "text");
    }

    // ── Enums ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_dynamic_mapping_serialization() {
        assert_eq!(
            serde_json::to_value(DynamicMapping::Strict).unwrap(),
            json!("strict")
        );
        assert_eq!(
            serde_json::to_value(DynamicMapping::True).unwrap(),
            json!("true")
        );
        assert_eq!(
            serde_json::to_value(DynamicMapping::False).unwrap(),
            json!("false")
        );
    }

    #[test]
    fn test_term_vector_serialization() {
        assert_eq!(
            serde_json::to_value(TermVector::WithPositionsOffsets).unwrap(),
            json!("with_positions_offsets")
        );
        assert_eq!(
            serde_json::to_value(TermVector::No).unwrap(),
            json!("no")
        );
    }

    #[test]
    fn test_index_options_serialization() {
        assert_eq!(
            serde_json::to_value(IndexOptions::Positions).unwrap(),
            json!("positions")
        );
        assert_eq!(
            serde_json::to_value(IndexOptions::Offsets).unwrap(),
            json!("offsets")
        );
    }

    #[test]
    fn test_similarity_serialization() {
        assert_eq!(
            serde_json::to_value(Similarity::Bm25).unwrap(),
            json!("BM25")
        );
        assert_eq!(
            serde_json::to_value(Similarity::Boolean).unwrap(),
            json!("boolean")
        );
    }

    #[test]
    fn test_knn_engine_serialization() {
        assert_eq!(
            serde_json::to_value(KnnEngine::Faiss).unwrap(),
            json!("faiss")
        );
        assert_eq!(
            serde_json::to_value(KnnEngine::Lucene).unwrap(),
            json!("lucene")
        );
    }

    #[test]
    fn test_knn_space_type_serialization() {
        assert_eq!(
            serde_json::to_value(KnnSpaceType::L2).unwrap(),
            json!("l2")
        );
        assert_eq!(
            serde_json::to_value(KnnSpaceType::Cosinesimil).unwrap(),
            json!("cosinesimil")
        );
    }

    // ── Property type_name ─────────────────────────────────────────────────────

    #[test]
    fn test_all_property_type_names() {
        let cases = vec![
            (Property::text(TextProperty::new()), "text"),
            (Property::keyword(KeywordProperty::new()), "keyword"),
            (Property::integer(), "integer"),
            (Property::long(), "long"),
            (Property::float(), "float"),
            (Property::double(), "double"),
            (Property::boolean(), "boolean"),
            (Property::date(), "date"),
            (Property::date_nanos(), "date_nanos"),
            (Property::ip(), "ip"),
            (Property::geo_point(), "geo_point"),
            (Property::geo_shape(), "geo_shape"),
            (Property::knn_vector(64), "knn_vector"),
            (Property::alias("other_field"), "alias"),
            (Property::object(ObjectProperty::new()), "object"),
            (Property::nested(NestedProperty::new()), "nested"),
        ];

        for (prop, expected) in cases {
            assert_eq!(prop.type_name(), expected, "type_name mismatch for {}", expected);
            assert_eq!(prop.to_string(), expected);
        }
    }

    // ── Deserialization ────────────────────────────────────────────────────────

    #[test]
    fn test_deserialize_text_property() {
        let json = json!({"type": "text", "analyzer": "standard", "norms": false});
        let p: Property = serde_json::from_value(json).unwrap();
        assert_eq!(p.type_name(), "text");
        if let Property::Text(tp) = p {
            assert_eq!(tp.analyzer.as_deref(), Some("standard"));
            assert_eq!(tp.norms, Some(false));
        } else {
            panic!("Expected Text variant");
        }
    }

    #[test]
    fn test_deserialize_knn_vector_with_method() {
        let json = json!({
            "type": "knn_vector",
            "dimension": 512,
            "space_type": "cosinesimil",
            "method": {
                "name": "hnsw",
                "engine": "lucene",
                "parameters": {"ef_construction": 200, "m": 24}
            }
        });
        let p: Property = serde_json::from_value(json).unwrap();
        assert_eq!(p.type_name(), "knn_vector");
        if let Property::KnnVector(knn) = p {
            assert_eq!(knn.dimension, 512);
            let method = knn.method.unwrap();
            assert_eq!(method.name, "hnsw");
        } else {
            panic!("Expected KnnVector variant");
        }
    }

    #[test]
    fn test_deserialize_join_property() {
        let json = json!({
            "type": "join",
            "relations": {"question": "answer"}
        });
        let p: Property = serde_json::from_value(json).unwrap();
        assert_eq!(p.type_name(), "join");
        if let Property::Join(jp) = p {
            let relations = jp.relations.unwrap();
            assert_eq!(relations["question"], json!("answer"));
        } else {
            panic!("Expected Join variant");
        }
    }

    // ── Token count ────────────────────────────────────────────────────────────

    #[test]
    fn test_token_count_property() {
        let tc = TokenCountProperty::new("standard");
        let p = Property::TokenCount(Box::new(tc));
        let v: Value = serde_json::to_value(&p).unwrap();
        assert_eq!(v["type"], "token_count");
        assert_eq!(v["analyzer"], "standard");
    }

    // ── Full index mapping example (from documentation) ────────────────────────

    /// Example mapping similar to the OpenSearch documentation quickstart.
    #[test]
    fn test_full_mapping_example() {
        let mapping = TypeMapping::new()
            .with_dynamic(DynamicMapping::Strict)
            .with_property("title", Property::text(TextProperty::new().with_analyzer("english")))
            .with_property(
                "status",
                Property::keyword(KeywordProperty::new().with_ignore_above(64)),
            )
            .with_property("priority", Property::Integer(Box::new(NumericProperty::new())))
            .with_property("score", Property::ScaledFloat(Box::new(ScaledFloatProperty::new(100.0))))
            .with_property("is_published", Property::boolean())
            .with_property("published_at", Property::date())
            .with_property("author_ip", Property::ip())
            .with_property("location", Property::geo_point())
            .with_property(
                "embedding",
                Property::knn_vector(384),
            )
            .with_property(
                "tags",
                Property::nested(
                    NestedProperty::new()
                        .with_property("name", json!({"type": "keyword"}))
                        .with_property("weight", json!({"type": "float"})),
                ),
            );

        let v: Value = serde_json::to_value(&mapping).unwrap();
        assert_eq!(v["dynamic"], "strict");
        assert_eq!(v["properties"]["title"]["type"], "text");
        assert_eq!(v["properties"]["title"]["analyzer"], "english");
        assert_eq!(v["properties"]["status"]["type"], "keyword");
        assert_eq!(v["properties"]["priority"]["type"], "integer");
        assert_eq!(v["properties"]["score"]["type"], "scaled_float");
        assert_eq!(v["properties"]["score"]["scaling_factor"], 100.0);
        assert_eq!(v["properties"]["is_published"]["type"], "boolean");
        assert_eq!(v["properties"]["published_at"]["type"], "date");
        assert_eq!(v["properties"]["author_ip"]["type"], "ip");
        assert_eq!(v["properties"]["location"]["type"], "geo_point");
        assert_eq!(v["properties"]["embedding"]["type"], "knn_vector");
        assert_eq!(v["properties"]["embedding"]["dimension"], 384);
        assert_eq!(v["properties"]["tags"]["type"], "nested");
        assert_eq!(v["properties"]["tags"]["properties"]["name"]["type"], "keyword");
    }
}
