#!/usr/bin/env python3
import re
from pathlib import Path
import logging
import os

logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)


WORKING_DIR = Path.cwd() / "opensearch-client"
os.chdir(WORKING_DIR)

REGEX_CHANGES = [
    (r"\smatch: None,", """r#match: None,"""),
    (r"\bstatic: None,", """r#static: None,"""),
    (
        r"DefaultResponse\(crate::models::security::Created\),\s+DefaultResponse\(crate::models::security::Ok\)",
        "DefaultResponse(crate::models::security::Ok)",
    ),
    (r"\bFieldsValue", "Fields"),
    (
        r"\s+if let Some\(ref local_var_user_agent\)\s*=\s*local_var_configuration\s*.user_agent\s*\{\s+local_var_req_builder\s*=\s*local_var_req_builder\s*.header\(reqwest::header::USER_AGENT,\s*local_var_user_agent.clone\(\)\s*\);\s*\}",
        "",
    ),
]
TEXT_CHANGES = [
    ("""(crate::models::String)""", """(String)"""),
    ("""Option<common::filter_path>""", """Option<common::filter_path::FilterPath>"""),
    (
        """Option<common::_source_excludes>""",
        """Option<common::source_excludes::SourceExcludes>""",
    ),
    (
        """Option<common::_source_includes>""",
        """Option<common::source_includes::SourceIncludes>""",
    ),
    ("""_source_excludes""", """source_excludes"""),
    ("""_source_includes""", """source_includes"""),
    ("""pub match:""", """pub r#match:"""),
    ("""pub static: Option<bool>""", """pub r#static: Option<bool>"""),
    (
        """Option<common::wait_for_active_shards>""",
        """Option<common::wait_for_active_shards::WaitForActiveShards>""",
    ),
    (""", match: u32,""", """, r#match: u32,"""),
    ("""= type {""", """= r#type {"""),
    ("""crate::models::String""", """String"""),
    ("delete_by_queryOKJson", "DeleteByQueryOKJson"),
    ("reindexOKJson", "ReindexOKJson"),
    ("update_by_queryOKJson", "UpdateByQueryOKJson"),
    ("pub if: Option<String>", "pub r#if: Option<String>"),
    ("Option<common::refresh>", "Option<common::Refresh>"),
    ("Option<common::wait_for_nodes>", "Option<common::WaitForNodes>"),
    ("Option<common::wait_for_status>", "Option<common::WaitForStatus>"),
    ("Option<common::stored_fields>", "Option<common::StoredFields>"),
    ("Option<common::search_type>", "Option<common::SearchType>"),
    ("Option<common::sort>", "Option<common::Sort>"),
    ("Option<common::track_total_hits>", "Option<common::TrackTotalHits>"),
    ("Option<common::docvalue_fields>", "Option<common::DocvalueFields>"),
    ("Option<common::fielddata_fields>", "Option<common::FielddataFields>"),
    ("Option<common::completion_fields>", "Option<common::CompletionFields>"),
    ("Option<common::wait_for_status>", "Option<common::WaitForStatus>"),
    ("Option<common::wait_for_status>", "Option<common::WaitForStatus>"),
    ("enum track_total_hits", "enum TrackTotalHits"),
    ("indices::openOKJson", "indices::OpenOkJson"),
    ("GeoLocationValue", "GeoLocation"),
    ("TermsQueryField", "TermsLookupValue"),
    ("LatLonGeoLocationValue", "LatLonGeoLocation"),
    ("GeoHashLocationValue", "GeoHashLocation"),
    ("InlineScriptValue", "InlineScript"),
    ("ReplicationTaskStatusValue", "ReplicationTaskStatus"),
    ("RawTaskStatusValue", "RawTaskStatus"),
    ("PersistentTaskStatusValue", "PersistentTaskStatus"),
    ("CoordsGeoBoundsValue", "CoordsGeoBounds"),
    (
        "ExecuteLocalSampleCalculatorResponseValue",
        "ExecuteLocalSampleCalculatorResponse",
    ),
    ("common::slices", "common::Slices"),
    ("ShardStoreStatusValue", "ShardStoreStatus"),
    # recursive fix
    (
        """pub nested: Option<core::search::NestedIdentity>""",
        """pub nested: Option<Box<core::search::NestedIdentity>>""",
    ),
    # simplify to strings
    ("""pub relation: core::search::TotalHitsRelation,""", """pub relation: String,"""),
    ("""relation: core::search::TotalHitsRelation,""", """relation: String,"""),
    ("""common::time""", """common::Time"""),
    ("""common::health_status""", """common::HealthStatus"""),
    ("""common::health""", """common::Health"""),
    ("""common::multi_term_query_rewrite""", """common::MultiTermQueryRewrite"""),
    ("""common::node_role""", """common::NodeRole"""),
    # simplify to bool
    ("""Option<core::search::TrackHits>""", """Option<bool>"""),
    ("""Option<core::track_total_hits>""", """Option<bool>"""),
    # simplify to string or integer
    ("""common::fuzziness""", """common::Fuzziness"""),
    ("""common::minimum_should_match""", """common::MinimumShouldMatch"""),
    # simplify to string or long
    ("""common::bytes""", """common::Bytes"""),
    # simplify to string or miilins
    ("""common::date_time""", """common::DateTime"""),
    # simplify to string/stringList
    ("""common::actions""", """common::Actions"""),
    ("""common::metric""", """common::Metric"""),
    ("""common::groups""", """common::Groups"""),
    ("""common::routing""", """common::Routing"""),
    ("""common::filter_path::FilterPath""", """common::FilterPath"""),
    ("""common::expand_wildcards""", """common::ExpandWildcards"""),
    ("""common::node_ids""", """common::NodeIds"""),
    ("""common::fields""", """common::Fields"""),
    ("""common::node_names""", """common::NodeNames"""),
    ("""common::repository""", """common::Repository"""),
    ("""common::source_excludes::SourceExcludes""", """common::SourceExcludes"""),
    ("""common::source_includes::SourceIncludes""", """common::SourceIncludes"""),
    ("""common::ids""", """common::Ids"""),
    # simplify to json
    ("""Option<common::Object>""", """Option<serde_json::Value>"""),
    ("""common::Object""", """serde_json::Value"""),
    ("""Error::ResponseError""", """Error::ApiError"""),
    ("""ResolveIndexOkJson""", """ResolveIndexResponse"""),
    ("""OkJson""", """Response"""),
    ("""_ok_json""", """_response"""),
    ("""crate::models::""", """crate::"""),
    ("""use crate::models;""", """"""),
    ("""use models::""", """use crate::"""),
    ("""crate::apis::Error""", """crate::Error"""),
    # regafactory single clients:
    ("""<configuration::Configuration>""", """<crate::Configuration>"""),
    ("use async_trait::async_trait;", "use bon::bon;"),
    (
        """let local_var_uri_str = format!("{}/""",
        """let local_var_uri_str = format!("{}""",
    ),
    (""""{}/""", """"{}"""),
    ("""common::query_dsl::QueryContainer""", """crate::dsl::Query"""),
    
]

FILES_TO_DELETE = [
    # unused
    "src/models/common/script.rs",
    "src/models/common/status.rs",
    "src/models/common/fields_value.rs",
    "src/models/common/aggregations/calendar_interval.rs",
    "src/models/common/script_language.rs",
    "src/models/common/byte_unit.rs",
    "src/models/core/search/context_value.rs",
    "src/models/common/analysis/stop_words.rs",
    "src/models/common/data_stream_names.rs",
    "src/models/indices/open_ok_json.rs",
    "src/models/common/wait_for_status.rs",
    "src/models/common/sort_options_value.rs",
    "src/models/common/delete_by_query_ok_json.rs",
    "src/models/common/reindex_ok_json.rs",
    "src/models/common/update_by_query_ok_json.rs",
    "src/models/tasks/task_response.rs",
    "src/models/common/sort_options.rs",
    "src/models/common/lat_lon_geo_location_value.rs",
    "src/models/common/geo_hash_location_value.rs",
    "src/models/common/inline_script_value.rs",
    "src/models/common/sort.rs",
    "src/models/common/slices.rs",
    "src/models/common/stored_fields.rs",
    "src/models/common/fielddata_fields.rs",
    "src/models/common/docvalue_fields.rs",
    "src/models/common/completion_fields.rs",
    "src/models/common/coords_geo_bounds_value.rs",
    "src/models/indices/shard_stores/shard_store_status_value.rs",
    "src/models/indices/shard_stores/shard_store_status.rs",
    "src/models/common/geo_location_value.rs",
    "src/models/common/query_dsl/terms_query_field.rs",
    "src/models/common/mapping/geo_orientation.rs",
    "src/models/core/search/builtin_highlighter_type_value.rs",
    "src/models/tasks/persistent_task_status_value.rs",
    "src/models/tasks/replication_task_status_value.rs",
    "src/models/tasks/raw_task_status_value.rs",
    "src/models/ml/execute_local_sample_calculator_response_value.rs",
    "src/models/core/msearch/multisearch_body_value.rs",
    "src/models/core/mget/multi_get_error_value.rs",
    "src/models/common/expand_wildcard_value.rs",
    "src/models/common/expand_wildcard.rs",
    "src/models/core/search/total_hits_relation.rs",
    "src/models/core/search/track_hits.rs",
    "src/models/common/object.rs",
    "src/models/common/empty_object.rs",
    "src/models/common/time.rs",
    "src/models/common/time_unit.rs",
    "src/models/common/indices.rs",
    "src/models/common/index.rs",
    "src/models/common/routing_in_query_string.rs",
    "src/models/common/search_type.rs",
    "src/models/common/query_dsl/field_and_format.rs",
    "src/models/common/wkt_geo_bounds_value.rs",
    "src/models/common/top_right_bottom_left_geo_bounds_value.rs",
    "src/models/common/top_left_bottom_right_geo_bounds_value.rs",
    "src/models/common/analysis/char_filter_definition_value.rs",
    "src/models/common/analysis/cjk_analyzer_value.rs",
    "src/models/common/analysis/custom_analyzer_value.rs",
    "src/models/common/analysis/custom_normalizer_value.rs",
    "src/models/common/analysis/dutch_analyzer_value.rs",
    "src/models/common/analysis/fingerprint_analyzer_value.rs",
    "src/models/common/analysis/icu_analyzer_value.rs",
    "src/models/common/analysis/keyword_analyzer_value.rs",
    "src/models/common/analysis/kuromoji_analyzer_value.rs",
    "src/models/common/analysis/language_analyzer_value.rs",
    "src/models/common/analysis/lowercase_normalizer_value.rs",
    "src/models/common/analysis/nori_analyzer_value.rs",
    "src/models/common/analysis/pattern_analyzer_value.rs",
    "src/models/common/analysis/phone_analyzer_value.rs",
    "src/models/common/analysis/simple_analyzer_value.rs",
    "src/models/common/analysis/smartcn_analyzer_value.rs",
    "src/models/common/analysis/snowball_analyzer_value.rs",
    "src/models/common/analysis/standard_analyzer_value.rs",
    "src/models/common/analysis/stop_analyzer_value.rs",
    "src/models/common/analysis/token_filter_definition_value.rs",
    "src/models/common/analysis/tokenizer_definition_value.rs",
    "src/models/common/analysis/whitespace_analyzer_value.rs",
    "src/models/core/msearch/multisearch_header_value.rs",
    "src/models/core/msearch/multisearch_body_value.rs"
    "src/models/core/msearch_template/template_config_value.rs",
    # to int or string
    "src/models/common/fuzziness.rs",
    "src/models/common/health_status.rs",
    "src/models/common/health.rs",
    "src/models/common/minimum_should_match.rs",
    "src/models/common/node_role.rs",
    # long
    "src/models/common/bytes.rs",
    # datetime
    "src/models/common/date_time.rs",
    # to json/list of json
    "src/models/common/aggregations/aggregate_order.rs",
    "src/models/common/aggregations/buckets_adjacency_matrix_bucket.rs",
    "src/models/common/aggregations/buckets_composite_bucket.rs",
    "src/models/common/aggregations/buckets_date_histogram_bucket.rs",
    "src/models/common/aggregations/buckets_double_terms_bucket.rs",
    "src/models/common/aggregations/buckets_filters_bucket.rs",
    "src/models/common/aggregations/buckets_geo_hash_grid_bucket.rs",
    "src/models/common/aggregations/buckets_geo_tile_grid_bucket.rs",
    "src/models/common/aggregations/buckets_histogram_bucket.rs",
    "src/models/common/aggregations/buckets_ip_range_bucket.rs",
    "src/models/common/aggregations/buckets_long_rare_terms_bucket.rs",
    "src/models/common/aggregations/buckets_long_terms_bucket.rs",
    "src/models/common/aggregations/buckets_multi_terms_bucket.rs",
    "src/models/common/aggregations/buckets_range_bucket.rs",
    "src/models/common/aggregations/buckets_significant_long_terms_bucket.rs",
    "src/models/common/aggregations/buckets_significant_string_terms_bucket.rs",
    "src/models/common/aggregations/buckets_string_rare_terms_bucket.rs",
    "src/models/common/aggregations/buckets_string_terms_bucket.rs",
    "src/models/common/aggregations/buckets_variable_width_histogram_bucket.rs",
    "src/models/common/aggregations/buckets_void.rs",
    # to string/boolean
    "src/models/common/refresh.rs",
    # to string/stringlist
    "src/models/common/metric.rs",
    "src/models/common/actions.rs",
    "src/models/common/groups.rs",
    "src/models/common/routing.rs",
    "src/models/common/filter_path.rs",
    "src/models/common/fields.rs",
    "src/models/common/ids.rs",
    "src/models/common/expand_wildcards.rs",
    "src/models/common/names.rs",
    "src/models/common/node_ids.rs",
    "src/models/common/node_names.rs",
    "src/models/common/repository.rs",
    "src/models/common/source_excludes.rs",
    "src/models/common/source_includes.rs",
    "src/models/common/scroll_ids.rs",
    "src/models/common/multi_term_query_rewrite.rs",
    "src/models/indices/storage_type.rs",
    "src/models/indices/analyze_mod/text_to_analyze.rs",
]

MODS_TO_FIX: list[str] = [
    # "src/models/common/mod.rs",
    # "src/models/asynchronous_search/mod.rs",
    # "src/models/indices/mod.rs",
]


def delete_unrequired_files():
    """
    Deletes files that are not required anymore.
    """
    for file in FILES_TO_DELETE:
        file_path = Path(file)
        if file_path.exists():
            logging.info(f"Deleting {file_path}")
            file_path.unlink()


def process_rust_file():
    extensions = set(".rs".split())

    skipped = set()

    for filename in Path.cwd().rglob("*"):
        if (
            ".git" in str(filename)
            or ".husky" in str(filename)
            or ".venv" in str(filename)
            or "node_modules" in str(filename)
        ):
            continue
        if not filename.is_file():
            continue
        if filename.suffix not in extensions:
            skipped.add(filename.suffix)
            continue

        logging.debug(f"Checking {filename}")
        original = filename.read_text()

        content = original

        content = migrate_error_handling(content)

        for old_value, new_value in TEXT_CHANGES:
            content = content.replace(old_value, new_value)
        for rx, new_value in REGEX_CHANGES:
            matches = re.finditer(rx, content, re.MULTILINE)
            for match in matches:
                item = match.group()
                groups = match.groups()
                rep = new_value
                if len(groups) > 0:
                    for pos, g in enumerate(groups):
                        rep = rep.replace("$" + str(pos + 1), g)
                content = content.replace(item, rep)
        while "r#r#" in content:
            content = content.replace("r#r#", "r#")

        if original != content or filename.name.endswith("_ok_json.rs"):
            old_filename = filename
            if filename.name.endswith("_ok_json.rs"):
                new_name = filename.name.replace("_ok_json.rs", "_response.rs")
                filename = filename.parent / new_name
                if filename.exists():
                    logging.warning(f"File {filename} already exists, skipping.")
                    continue
                old_filename.rename(filename)
            logging.info(f"Updating {filename}")
            filename.write_text(content)

    # from pprint import pprint

    # pprint(skipped)


def migrate_error_handling(content):
    """
    Migrate error handling to the new format.
    """
    content = re.sub(
        r"/// Struct for typed errors of method \[\`(\w|_)+\`\]\s*#\[derive\(Debug, Clone, Serialize, Deserialize\)\]\s*#\[serde\(untagged\)\]\s*pub enum (\w+Error)\s*\{\s*DefaultResponse\((\:|\w)+\)\s*,\s*UnknownValue\(serde_json::Value\)\s*,\s*\}",
        "",
        content,
    )
    content = re.sub(
        r"Error<(\w)+>",
        "Error",
        content,
    )
    content = re.sub(
        r"let local_var_entity: Option<(\w+)Error>\s*=\s*serde_json::from_str\(&local_var_content\).ok\(\)\s*;\s*let local_var_error\s*=\s*ResponseContent\s*\{\s*status\s*:\s*local_var_status\s*,\s*content: local_var_content,\s*entity\s*:\s*local_var_entity(,)*\s*\};",
        """let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
            };""",
        content,
    )

    content = re.sub(
        r"Result<ResponseContent<(?P<name>\w+)>,",
        """Result<\g<name>,""",
        content,
    )

    return content


def process_string_array_enum_tmpl():
    template_file = Path("fixes/enum_string_array.tmpl")
    if not template_file.exists():
        logging.error(f"Template file {template_file} does not exist.")
        return

    template_content = template_file.read_text()
    # Load the JSON file with enum definitions
    enums_file = Path("fixes/enums_multi.json")
    if not enums_file.exists():
        logging.error(f"Enums file {enums_file} does not exist.")
        return
    import json

    with open(enums_file, "r") as f:
        enums = json.load(f)
        if not isinstance(enums, list):
            logging.error(f"Enums file {enums_file} does not contain a list.")
            return
    for enum_def in enums:
        name = enum_def.get("name")
        target_path = enum_def.get("path")
        logging.debug(f"Processing enum definition: {name} -> {target_path}")
        if not name:
            logging.error(f"Enum definition missing 'name': {enum_def}")
            continue
        documentation = enum_def.get("documentation", "")
        single_name = enum_def.get("single_name", "StringValue")
        multi_name = enum_def.get("multi_name", "ArrayValue")

        content = template_content
        content = content.replace("$$NAME$$", name)
        content = content.replace("$$DOCUMENTATION$$", documentation)
        content = content.replace("$$SINGLEVALUE$$", single_name)
        content = content.replace("$$ARRAYVALUE$$", multi_name)

        output_file = Path(target_path)
        if not output_file.parent.exists():
            output_file.parent.mkdir(parents=True, exist_ok=True)
        if output_file.exists():
            # print(f"File {output_file} already exists, skipping.")
            previous_content = output_file.read_text()
            if previous_content == content:
                logging.debug(f"File {output_file} already up to date, skipping.")
                continue
        output_file.write_text(content)
        logging.info(f"Updated {output_file}")


def process_string_int_enum_tmpl():
    template_file = Path("fixes/enum_string_number.tmpl")
    if not template_file.exists():
        logging.error(f"Template file {template_file} does not exist.")
        return

    template_content = template_file.read_text()
    # Load the JSON file with enum definitions
    enums_file = Path("fixes/enums_string_num.json")
    if not enums_file.exists():
        logging.error(f"Enums file {enums_file} does not exist.")
        return
    import json

    with open(enums_file, "r") as f:
        enums = json.load(f)
        if not isinstance(enums, list):
            logging.error(f"Enums file {enums_file} does not contain a list.")
            return
    for enum_def in enums:
        name = enum_def.get("name")
        target_path = enum_def.get("path")
        logging.debug(f"Processing enum definition: {name} -> {target_path}")
        if not name:
            logging.error(f"Enum definition missing 'name': {enum_def}")
            continue

        content = template_content
        content = content.replace("$$NAME$$", name)
        content = content.replace(
            "$$DOCUMENTATION$$", enum_def.get("documentation", "")
        )
        content = content.replace(
            "$$STRING_VALUE$$", enum_def.get("string_value", "StringValue")
        )
        content = content.replace(
            "$$NUMBER_VALUE$$", enum_def.get("number_value", "CountValue")
        )
        content = content.replace("$$NUMBER_TYPE$$", enum_def.get("number_type", "u32"))

        output_file = Path(target_path)
        if not output_file.parent.exists():
            output_file.parent.mkdir(parents=True, exist_ok=True)
        if output_file.exists():
            # print(f"File {output_file} already exists, skipping.")
            previous_content = output_file.read_text()
            if previous_content == content:
                logging.debug(f"File {output_file} already up to date, skipping.")
                continue
        output_file.write_text(content)
        logging.info(f"Updated {output_file}")


def name_camel_case(name):
    """
    Convert a string to CamelCase.
    """
    return "".join(word.capitalize() for word in name.split("_"))


def fix_mod_missing():
    for mod_name in MODS_TO_FIX:
        mod_file = Path(mod_name)
        if not mod_file.exists():
            print(f"Mod file {mod_file} does not exist.")
            return

        content = mod_file.read_text()
        original = content
        files = mod_file.parent.glob("*.rs")
        for file in files:
            name = file.name.replace(".rs", "")
            if name == "mod":
                continue
            if content.find(f"pub mod {name};") != -1:
                # print(f"Module {name} already exists in common/mod.rs")
                continue
            name_camel = name_camel_case(name)
            content = (
                content + f"\n\npub mod {name};\npub use self::{name}::{name_camel};"
            )
        if original != content:
            mod_file.write_text(content)
            logging.info(f"Updated {mod_file}")


def add_bon_builder_to_methods(content: str, pos: int) -> str:
    """
    Add 'bon' to methods in the content.
    """
    # Find the end of the method definition
    start_pos = content[:pos].rfind("async fn ")
    section = content[:start_pos].strip()
    if not section.endswith("#[builder]"):
        # If the method is not public, we don't add 'bon'
        return content[:start_pos] + "#[builder]\n" + content[start_pos:]
    return content


def remove_struct_param(content: str, start_pos: int, end_pos: int) -> str:
    """
    Remove the struct parameter from the content.
    """
    # Find the start of the struct definition
    struct_start = content[:start_pos].rfind("}")
    if struct_start == -1:
        return content  # No struct found, return original content
    return content[: struct_start + 1] + content[end_pos + 1 :]


def fix_client(filename: Path):
    """
    Fix the client file to use the new Configuration.
    """
    content = filename.read_text()
    original = content
    # match a regular expression to find all the paramaeters name
    params = list(set(re.findall(r"\bparams\s*:\s*(?P<name>\w+)Params\b", content)))
    params.sort()
    # collecting the parameters list
    for param in params:
        print(f"Processing parameter: {param}")
        find_str = f"pub struct {param}Params {{"
        pos = content.find(find_str)
        fields = ""
        if pos != -1:
            end_pos = content.find("}", pos)
            if end_pos == -1:
                break
            fields = content[pos + len(find_str) : end_pos].strip().replace("pub ", "")

            params_placeholder = f"params: {param}Params"
            # struct from the content
            content = remove_struct_param(content, pos, end_pos)
            # add bon to methods
            finder_pos = 0
            while finder_pos != -1:
                finder_pos = content.find(params_placeholder, finder_pos)
                if finder_pos != -1:
                    content = add_bon_builder_to_methods(content, finder_pos)
                    finder_pos = finder_pos + 100

            # remove the fields from the content
            content = content.replace(params_placeholder, fields)

            find_to_replace = f"let {param}Params {{"
            pos = content.find(find_to_replace)
            if pos != -1:
                # replace the params with the parameters name
                # and remove the params from the content
                end_pos = content.find("} = params;", pos)
                if end_pos == -1:
                    break
                content = content[:pos] + content[end_pos + len("} = params;") :]
    content = content.replace("#[async_trait]", "#[bon]")
    content = content.replace(",,", ",")
    content = content.replace("async fn", "pub async fn")
    content = content.replace("async pub ", "pub async ")
    content = content.replace("pub pub ", "pub ")

    if original != content:
        filename.write_text(content)
        logging.info(f"Updated {filename}")


if __name__ == "__main__":
    # delete_unrequired_files()
    process_rust_file()
    ## process_string_array_enum_tmpl()
    ## process_string_int_enum_tmpl()
    # fix_mod_missing()
    # fix_client(Path("src/common/common_api.rs"))
    # fix_client(Path("src/cluster/cluster_api.rs"))
    # fix_client(Path("src/indices/indices_api.rs"))
    # fix_client(Path("src/ingest/ingest_api.rs"))
    # fix_client(Path("src/ml/ml_api.rs"))
    print("Done")
