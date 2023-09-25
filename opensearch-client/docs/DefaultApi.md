# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_post**](DefaultApi.md#bulk_post) | **POST** /_bulk | 
[**bulk_post_with_index**](DefaultApi.md#bulk_post_with_index) | **POST** /{index}/_bulk | 
[**bulk_put**](DefaultApi.md#bulk_put) | **PUT** /_bulk | 
[**bulk_put_with_index**](DefaultApi.md#bulk_put_with_index) | **PUT** /{index}/_bulk | 
[**cat_aliases**](DefaultApi.md#cat_aliases) | **GET** /_cat/aliases | 
[**cat_aliases_with_name**](DefaultApi.md#cat_aliases_with_name) | **GET** /_cat/aliases/{name} | 
[**cat_all_pit_segments**](DefaultApi.md#cat_all_pit_segments) | **GET** /_cat/pit_segments/_all | 
[**cat_allocation**](DefaultApi.md#cat_allocation) | **GET** /_cat/allocation | 
[**cat_allocation_with_node_id**](DefaultApi.md#cat_allocation_with_node_id) | **GET** /_cat/allocation/{node_id} | 
[**cat_cluster_manager**](DefaultApi.md#cat_cluster_manager) | **GET** /_cat/cluster_manager | 
[**cat_count**](DefaultApi.md#cat_count) | **GET** /_cat/count | 
[**cat_count_with_index**](DefaultApi.md#cat_count_with_index) | **GET** /_cat/count/{index} | 
[**cat_fielddata**](DefaultApi.md#cat_fielddata) | **GET** /_cat/fielddata | 
[**cat_fielddata_with_fields**](DefaultApi.md#cat_fielddata_with_fields) | **GET** /_cat/fielddata/{fields} | 
[**cat_health**](DefaultApi.md#cat_health) | **GET** /_cat/health | 
[**cat_help**](DefaultApi.md#cat_help) | **GET** /_cat | 
[**cat_indices**](DefaultApi.md#cat_indices) | **GET** /_cat/indices | 
[**cat_indices_with_index**](DefaultApi.md#cat_indices_with_index) | **GET** /_cat/indices/{index} | 
[**cat_master**](DefaultApi.md#cat_master) | **GET** /_cat/master | 
[**cat_nodeattrs**](DefaultApi.md#cat_nodeattrs) | **GET** /_cat/nodeattrs | 
[**cat_nodes**](DefaultApi.md#cat_nodes) | **GET** /_cat/nodes | 
[**cat_pending_tasks**](DefaultApi.md#cat_pending_tasks) | **GET** /_cat/pending_tasks | 
[**cat_pit_segments**](DefaultApi.md#cat_pit_segments) | **GET** /_cat/pit_segments | 
[**cat_plugins**](DefaultApi.md#cat_plugins) | **GET** /_cat/plugins | 
[**cat_recovery**](DefaultApi.md#cat_recovery) | **GET** /_cat/recovery | 
[**cat_recovery_with_index**](DefaultApi.md#cat_recovery_with_index) | **GET** /_cat/recovery/{index} | 
[**cat_repositories**](DefaultApi.md#cat_repositories) | **GET** /_cat/repositories | 
[**cat_segment_replication**](DefaultApi.md#cat_segment_replication) | **GET** /_cat/segment_replication | 
[**cat_segment_replication_with_index**](DefaultApi.md#cat_segment_replication_with_index) | **GET** /_cat/segment_replication/{index} | 
[**cat_segments**](DefaultApi.md#cat_segments) | **GET** /_cat/segments | 
[**cat_segments_with_index**](DefaultApi.md#cat_segments_with_index) | **GET** /_cat/segments/{index} | 
[**cat_shards**](DefaultApi.md#cat_shards) | **GET** /_cat/shards | 
[**cat_shards_with_index**](DefaultApi.md#cat_shards_with_index) | **GET** /_cat/shards/{index} | 
[**cat_snapshots**](DefaultApi.md#cat_snapshots) | **GET** /_cat/snapshots | 
[**cat_snapshots_with_repository**](DefaultApi.md#cat_snapshots_with_repository) | **GET** /_cat/snapshots/{repository} | 
[**cat_tasks**](DefaultApi.md#cat_tasks) | **GET** /_cat/tasks | 
[**cat_templates**](DefaultApi.md#cat_templates) | **GET** /_cat/templates | 
[**cat_templates_with_name**](DefaultApi.md#cat_templates_with_name) | **GET** /_cat/templates/{name} | 
[**cat_thread_pool**](DefaultApi.md#cat_thread_pool) | **GET** /_cat/thread_pool | 
[**cat_thread_pool_with_thread_pool_patterns**](DefaultApi.md#cat_thread_pool_with_thread_pool_patterns) | **GET** /_cat/thread_pool/{thread_pool_patterns} | 
[**change_password**](DefaultApi.md#change_password) | **PUT** /_plugins/_security/api/account | 
[**clear_scroll**](DefaultApi.md#clear_scroll) | **DELETE** /_search/scroll | 
[**clear_scroll_with_scroll_id**](DefaultApi.md#clear_scroll_with_scroll_id) | **DELETE** /_search/scroll/{scroll_id} | 
[**cluster_allocation_explain_get**](DefaultApi.md#cluster_allocation_explain_get) | **GET** /_cluster/allocation/explain | 
[**cluster_allocation_explain_post**](DefaultApi.md#cluster_allocation_explain_post) | **POST** /_cluster/allocation/explain | 
[**cluster_delete_component_template**](DefaultApi.md#cluster_delete_component_template) | **DELETE** /_component_template/{name} | 
[**cluster_delete_decommission_awareness**](DefaultApi.md#cluster_delete_decommission_awareness) | **DELETE** /_cluster/decommission/awareness/ | 
[**cluster_delete_voting_config_exclusions**](DefaultApi.md#cluster_delete_voting_config_exclusions) | **DELETE** /_cluster/voting_config_exclusions | 
[**cluster_delete_weighted_routing**](DefaultApi.md#cluster_delete_weighted_routing) | **DELETE** /_cluster/routing/awareness/weights | 
[**cluster_exists_component_template**](DefaultApi.md#cluster_exists_component_template) | **HEAD** /_component_template/{name} | 
[**cluster_get_component_template**](DefaultApi.md#cluster_get_component_template) | **GET** /_component_template | 
[**cluster_get_component_template_with_name**](DefaultApi.md#cluster_get_component_template_with_name) | **GET** /_component_template/{name} | 
[**cluster_get_decommission_awareness**](DefaultApi.md#cluster_get_decommission_awareness) | **GET** /_cluster/decommission/awareness/{awareness_attribute_name}/_status | 
[**cluster_get_settings**](DefaultApi.md#cluster_get_settings) | **GET** /_cluster/settings | 
[**cluster_get_weighted_routing**](DefaultApi.md#cluster_get_weighted_routing) | **GET** /_cluster/routing/awareness/{attribute}/weights | 
[**cluster_health**](DefaultApi.md#cluster_health) | **GET** /_cluster/health | 
[**cluster_health_with_index**](DefaultApi.md#cluster_health_with_index) | **GET** /_cluster/health/{index} | 
[**cluster_pending_tasks**](DefaultApi.md#cluster_pending_tasks) | **GET** /_cluster/pending_tasks | 
[**cluster_post_voting_config_exclusions**](DefaultApi.md#cluster_post_voting_config_exclusions) | **POST** /_cluster/voting_config_exclusions | 
[**cluster_put_component_template_post**](DefaultApi.md#cluster_put_component_template_post) | **POST** /_component_template/{name} | 
[**cluster_put_component_template_put**](DefaultApi.md#cluster_put_component_template_put) | **PUT** /_component_template/{name} | 
[**cluster_put_decommission_awareness**](DefaultApi.md#cluster_put_decommission_awareness) | **PUT** /_cluster/decommission/awareness/{awareness_attribute_name}/{awareness_attribute_value} | 
[**cluster_put_settings**](DefaultApi.md#cluster_put_settings) | **PUT** /_cluster/settings | 
[**cluster_put_weighted_routing**](DefaultApi.md#cluster_put_weighted_routing) | **PUT** /_cluster/routing/awareness/{attribute}/weights | 
[**cluster_remote_info**](DefaultApi.md#cluster_remote_info) | **GET** /_remote/info | 
[**cluster_reroute**](DefaultApi.md#cluster_reroute) | **POST** /_cluster/reroute | 
[**cluster_state**](DefaultApi.md#cluster_state) | **GET** /_cluster/state | 
[**cluster_state_with_index_metric**](DefaultApi.md#cluster_state_with_index_metric) | **GET** /_cluster/state/{metric}/{index} | 
[**cluster_state_with_metric**](DefaultApi.md#cluster_state_with_metric) | **GET** /_cluster/state/{metric} | 
[**cluster_stats**](DefaultApi.md#cluster_stats) | **GET** /_cluster/stats | 
[**cluster_stats_with_node_id**](DefaultApi.md#cluster_stats_with_node_id) | **GET** /_cluster/stats/nodes/{node_id} | 
[**count_get**](DefaultApi.md#count_get) | **GET** /_count | 
[**count_get_with_index**](DefaultApi.md#count_get_with_index) | **GET** /{index}/_count | 
[**count_post**](DefaultApi.md#count_post) | **POST** /_count | 
[**count_post_with_index**](DefaultApi.md#count_post_with_index) | **POST** /{index}/_count | 
[**create_action_group**](DefaultApi.md#create_action_group) | **PUT** /_plugins/_security/api/actiongroups/{action_group} | 
[**create_pit**](DefaultApi.md#create_pit) | **POST** /{index}/_search/point_in_time | 
[**create_post**](DefaultApi.md#create_post) | **POST** /{index}/_create/{id} | 
[**create_put**](DefaultApi.md#create_put) | **PUT** /{index}/_create/{id} | 
[**create_role**](DefaultApi.md#create_role) | **PUT** /_plugins/_security/api/roles/{role} | 
[**create_role_mapping**](DefaultApi.md#create_role_mapping) | **PUT** /_plugins/_security/api/rolesmapping/{role} | 
[**create_tenant**](DefaultApi.md#create_tenant) | **PUT** /_plugins/_security/api/tenants/{tenant} | 
[**create_user**](DefaultApi.md#create_user) | **PUT** /_plugins/_security/api/internalusers/{username} | 
[**dangling_indices_delete_dangling_index**](DefaultApi.md#dangling_indices_delete_dangling_index) | **DELETE** /_dangling/{index_uuid} | 
[**dangling_indices_import_dangling_index**](DefaultApi.md#dangling_indices_import_dangling_index) | **POST** /_dangling/{index_uuid} | 
[**dangling_indices_list_dangling_indices**](DefaultApi.md#dangling_indices_list_dangling_indices) | **GET** /_dangling | 
[**delete**](DefaultApi.md#delete) | **DELETE** /{index}/_doc/{id} | 
[**delete_action_group**](DefaultApi.md#delete_action_group) | **DELETE** /_plugins/_security/api/actiongroups/{action_group} | 
[**delete_all_pits**](DefaultApi.md#delete_all_pits) | **DELETE** /_search/point_in_time/_all | 
[**delete_by_query**](DefaultApi.md#delete_by_query) | **POST** /{index}/_delete_by_query | 
[**delete_by_query_rethrottle**](DefaultApi.md#delete_by_query_rethrottle) | **POST** /_delete_by_query/{task_id}/_rethrottle | 
[**delete_distinguished_names**](DefaultApi.md#delete_distinguished_names) | **DELETE** /_plugins/_security/api/nodesdn/{cluster_name} | 
[**delete_pit**](DefaultApi.md#delete_pit) | **DELETE** /_search/point_in_time | 
[**delete_role**](DefaultApi.md#delete_role) | **DELETE** /_plugins/_security/api/roles/{role} | 
[**delete_role_mapping**](DefaultApi.md#delete_role_mapping) | **DELETE** /_plugins/_security/api/rolesmapping/{role} | 
[**delete_script**](DefaultApi.md#delete_script) | **DELETE** /_scripts/{id} | 
[**delete_tenant**](DefaultApi.md#delete_tenant) | **DELETE** /_plugins/_security/api/tenants/{tenant} | 
[**delete_user**](DefaultApi.md#delete_user) | **DELETE** /_plugins/_security/api/internalusers/{username} | 
[**exists**](DefaultApi.md#exists) | **HEAD** /{index}/_doc/{id} | 
[**exists_source**](DefaultApi.md#exists_source) | **HEAD** /{index}/_source/{id} | 
[**explain_get**](DefaultApi.md#explain_get) | **GET** /{index}/_explain/{id} | 
[**explain_post**](DefaultApi.md#explain_post) | **POST** /{index}/_explain/{id} | 
[**field_caps_get**](DefaultApi.md#field_caps_get) | **GET** /_field_caps | 
[**field_caps_get_with_index**](DefaultApi.md#field_caps_get_with_index) | **GET** /{index}/_field_caps | 
[**field_caps_post**](DefaultApi.md#field_caps_post) | **POST** /_field_caps | 
[**field_caps_post_with_index**](DefaultApi.md#field_caps_post_with_index) | **POST** /{index}/_field_caps | 
[**flush_cache**](DefaultApi.md#flush_cache) | **DELETE** /_plugins/_security/api/cache | 
[**get**](DefaultApi.md#get) | **GET** /{index}/_doc/{id} | 
[**get_account_details**](DefaultApi.md#get_account_details) | **GET** /_plugins/_security/api/account | 
[**get_action_group**](DefaultApi.md#get_action_group) | **GET** /_plugins/_security/api/actiongroups/{action_group} | 
[**get_action_groups**](DefaultApi.md#get_action_groups) | **GET** /_plugins/_security/api/actiongroups/ | 
[**get_all_pits**](DefaultApi.md#get_all_pits) | **GET** /_search/point_in_time/_all | 
[**get_audit_configuration**](DefaultApi.md#get_audit_configuration) | **GET** /_plugins/_security/api/audit | 
[**get_certificates**](DefaultApi.md#get_certificates) | **GET** /_plugins/_security/api/ssl/certs | 
[**get_configuration**](DefaultApi.md#get_configuration) | **GET** /_plugins/_security/api/securityconfig | 
[**get_distinguished_names**](DefaultApi.md#get_distinguished_names) | **GET** /_plugins/_security/api/nodesdn | 
[**get_distinguished_names_with_cluster_name**](DefaultApi.md#get_distinguished_names_with_cluster_name) | **GET** /_plugins/_security/api/nodesdn/{cluster_name} | 
[**get_role**](DefaultApi.md#get_role) | **GET** /_plugins/_security/api/roles/{role} | 
[**get_role_mapping**](DefaultApi.md#get_role_mapping) | **GET** /_plugins/_security/api/rolesmapping/{role} | 
[**get_role_mappings**](DefaultApi.md#get_role_mappings) | **GET** /_plugins/_security/api/rolesmapping | 
[**get_roles**](DefaultApi.md#get_roles) | **GET** /_plugins/_security/api/roles/ | 
[**get_script**](DefaultApi.md#get_script) | **GET** /_scripts/{id} | 
[**get_script_context**](DefaultApi.md#get_script_context) | **GET** /_script_context | 
[**get_script_languages**](DefaultApi.md#get_script_languages) | **GET** /_script_language | 
[**get_source**](DefaultApi.md#get_source) | **GET** /{index}/_source/{id} | 
[**get_tenant**](DefaultApi.md#get_tenant) | **GET** /_plugins/_security/api/tenants/{tenant} | 
[**get_tenants**](DefaultApi.md#get_tenants) | **GET** /_plugins/_security/api/tenants/ | 
[**get_user**](DefaultApi.md#get_user) | **GET** /_plugins/_security/api/internalusers/{username} | 
[**get_users**](DefaultApi.md#get_users) | **GET** /_plugins/_security/api/internalusers | 
[**index_post**](DefaultApi.md#index_post) | **POST** /{index}/_doc | 
[**index_post_with_id**](DefaultApi.md#index_post_with_id) | **POST** /{index}/_doc/{id} | 
[**index_put_with_id**](DefaultApi.md#index_put_with_id) | **PUT** /{index}/_doc/{id} | 
[**indices_add_block**](DefaultApi.md#indices_add_block) | **PUT** /{index}/_block/{block} | 
[**indices_analyze_get**](DefaultApi.md#indices_analyze_get) | **GET** /_analyze | 
[**indices_analyze_get_with_index**](DefaultApi.md#indices_analyze_get_with_index) | **GET** /{index}/_analyze | 
[**indices_analyze_post**](DefaultApi.md#indices_analyze_post) | **POST** /_analyze | 
[**indices_analyze_post_with_index**](DefaultApi.md#indices_analyze_post_with_index) | **POST** /{index}/_analyze | 
[**indices_clear_cache**](DefaultApi.md#indices_clear_cache) | **POST** /_cache/clear | 
[**indices_clear_cache_with_index**](DefaultApi.md#indices_clear_cache_with_index) | **POST** /{index}/_cache/clear | 
[**indices_clone_post**](DefaultApi.md#indices_clone_post) | **POST** /{index}/_clone/{target} | 
[**indices_clone_put**](DefaultApi.md#indices_clone_put) | **PUT** /{index}/_clone/{target} | 
[**indices_close**](DefaultApi.md#indices_close) | **POST** /{index}/_close | 
[**indices_create**](DefaultApi.md#indices_create) | **PUT** /{index} | 
[**indices_create_data_stream**](DefaultApi.md#indices_create_data_stream) | **PUT** /_data_stream/{name} | 
[**indices_data_streams_stats**](DefaultApi.md#indices_data_streams_stats) | **GET** /_data_stream/_stats | 
[**indices_data_streams_stats_with_name**](DefaultApi.md#indices_data_streams_stats_with_name) | **GET** /_data_stream/{name}/_stats | 
[**indices_delete**](DefaultApi.md#indices_delete) | **DELETE** /{index} | 
[**indices_delete_alias**](DefaultApi.md#indices_delete_alias) | **DELETE** /{index}/_alias/{name} | 
[**indices_delete_alias_plural**](DefaultApi.md#indices_delete_alias_plural) | **DELETE** /{index}/_aliases/{name} | 
[**indices_delete_data_stream**](DefaultApi.md#indices_delete_data_stream) | **DELETE** /_data_stream/{name} | 
[**indices_delete_index_template**](DefaultApi.md#indices_delete_index_template) | **DELETE** /_index_template/{name} | 
[**indices_delete_template**](DefaultApi.md#indices_delete_template) | **DELETE** /_template/{name} | 
[**indices_exists**](DefaultApi.md#indices_exists) | **HEAD** /{index} | 
[**indices_exists_alias**](DefaultApi.md#indices_exists_alias) | **HEAD** /_alias/{name} | 
[**indices_exists_alias_with_index**](DefaultApi.md#indices_exists_alias_with_index) | **HEAD** /{index}/_alias/{name} | 
[**indices_exists_index_template**](DefaultApi.md#indices_exists_index_template) | **HEAD** /_index_template/{name} | 
[**indices_exists_template**](DefaultApi.md#indices_exists_template) | **HEAD** /_template/{name} | 
[**indices_flush_get**](DefaultApi.md#indices_flush_get) | **GET** /_flush | 
[**indices_flush_get_with_index**](DefaultApi.md#indices_flush_get_with_index) | **GET** /{index}/_flush | 
[**indices_flush_post**](DefaultApi.md#indices_flush_post) | **POST** /_flush | 
[**indices_flush_post_with_index**](DefaultApi.md#indices_flush_post_with_index) | **POST** /{index}/_flush | 
[**indices_forcemerge**](DefaultApi.md#indices_forcemerge) | **POST** /_forcemerge | 
[**indices_forcemerge_with_index**](DefaultApi.md#indices_forcemerge_with_index) | **POST** /{index}/_forcemerge | 
[**indices_get**](DefaultApi.md#indices_get) | **GET** /{index} | 
[**indices_get_alias**](DefaultApi.md#indices_get_alias) | **GET** /_alias | 
[**indices_get_alias_with_index**](DefaultApi.md#indices_get_alias_with_index) | **GET** /{index}/_alias | 
[**indices_get_alias_with_index_name**](DefaultApi.md#indices_get_alias_with_index_name) | **GET** /{index}/_alias/{name} | 
[**indices_get_alias_with_name**](DefaultApi.md#indices_get_alias_with_name) | **GET** /_alias/{name} | 
[**indices_get_data_stream**](DefaultApi.md#indices_get_data_stream) | **GET** /_data_stream | 
[**indices_get_data_stream_with_name**](DefaultApi.md#indices_get_data_stream_with_name) | **GET** /_data_stream/{name} | 
[**indices_get_field_mapping**](DefaultApi.md#indices_get_field_mapping) | **GET** /_mapping/field/{fields} | 
[**indices_get_field_mapping_with_index**](DefaultApi.md#indices_get_field_mapping_with_index) | **GET** /{index}/_mapping/field/{fields} | 
[**indices_get_index_template**](DefaultApi.md#indices_get_index_template) | **GET** /_index_template | 
[**indices_get_index_template_with_name**](DefaultApi.md#indices_get_index_template_with_name) | **GET** /_index_template/{name} | 
[**indices_get_mapping**](DefaultApi.md#indices_get_mapping) | **GET** /_mapping | 
[**indices_get_mapping_with_index**](DefaultApi.md#indices_get_mapping_with_index) | **GET** /{index}/_mapping | 
[**indices_get_settings**](DefaultApi.md#indices_get_settings) | **GET** /_settings | 
[**indices_get_settings_with_index**](DefaultApi.md#indices_get_settings_with_index) | **GET** /{index}/_settings | 
[**indices_get_settings_with_index_name**](DefaultApi.md#indices_get_settings_with_index_name) | **GET** /{index}/_settings/{name} | 
[**indices_get_settings_with_name**](DefaultApi.md#indices_get_settings_with_name) | **GET** /_settings/{name} | 
[**indices_get_template**](DefaultApi.md#indices_get_template) | **GET** /_template | 
[**indices_get_template_with_name**](DefaultApi.md#indices_get_template_with_name) | **GET** /_template/{name} | 
[**indices_get_upgrade**](DefaultApi.md#indices_get_upgrade) | **GET** /_upgrade | 
[**indices_get_upgrade_with_index**](DefaultApi.md#indices_get_upgrade_with_index) | **GET** /{index}/_upgrade | 
[**indices_open**](DefaultApi.md#indices_open) | **POST** /{index}/_open | 
[**indices_put_alias_post**](DefaultApi.md#indices_put_alias_post) | **POST** /{index}/_alias/{name} | 
[**indices_put_alias_post_plural**](DefaultApi.md#indices_put_alias_post_plural) | **POST** /{index}/_aliases/{name} | 
[**indices_put_alias_put**](DefaultApi.md#indices_put_alias_put) | **PUT** /{index}/_alias/{name} | 
[**indices_put_alias_put_plural**](DefaultApi.md#indices_put_alias_put_plural) | **PUT** /{index}/_aliases/{name} | 
[**indices_put_index_template_post**](DefaultApi.md#indices_put_index_template_post) | **POST** /_index_template/{name} | 
[**indices_put_index_template_put**](DefaultApi.md#indices_put_index_template_put) | **PUT** /_index_template/{name} | 
[**indices_put_mapping_post**](DefaultApi.md#indices_put_mapping_post) | **POST** /{index}/_mapping | 
[**indices_put_mapping_put**](DefaultApi.md#indices_put_mapping_put) | **PUT** /{index}/_mapping | 
[**indices_put_settings**](DefaultApi.md#indices_put_settings) | **PUT** /_settings | 
[**indices_put_settings_with_index**](DefaultApi.md#indices_put_settings_with_index) | **PUT** /{index}/_settings | 
[**indices_put_template_post**](DefaultApi.md#indices_put_template_post) | **POST** /_template/{name} | 
[**indices_put_template_put**](DefaultApi.md#indices_put_template_put) | **PUT** /_template/{name} | 
[**indices_recovery**](DefaultApi.md#indices_recovery) | **GET** /_recovery | 
[**indices_recovery_with_index**](DefaultApi.md#indices_recovery_with_index) | **GET** /{index}/_recovery | 
[**indices_refresh_get**](DefaultApi.md#indices_refresh_get) | **GET** /_refresh | 
[**indices_refresh_get_with_index**](DefaultApi.md#indices_refresh_get_with_index) | **GET** /{index}/_refresh | 
[**indices_refresh_post**](DefaultApi.md#indices_refresh_post) | **POST** /_refresh | 
[**indices_refresh_post_with_index**](DefaultApi.md#indices_refresh_post_with_index) | **POST** /{index}/_refresh | 
[**indices_resolve_index**](DefaultApi.md#indices_resolve_index) | **GET** /_resolve/index/{name} | 
[**indices_rollover**](DefaultApi.md#indices_rollover) | **POST** /{alias}/_rollover | 
[**indices_rollover_with_new_index**](DefaultApi.md#indices_rollover_with_new_index) | **POST** /{alias}/_rollover/{new_index} | 
[**indices_segments**](DefaultApi.md#indices_segments) | **GET** /_segments | 
[**indices_segments_with_index**](DefaultApi.md#indices_segments_with_index) | **GET** /{index}/_segments | 
[**indices_shard_stores**](DefaultApi.md#indices_shard_stores) | **GET** /_shard_stores | 
[**indices_shard_stores_with_index**](DefaultApi.md#indices_shard_stores_with_index) | **GET** /{index}/_shard_stores | 
[**indices_shrink_post**](DefaultApi.md#indices_shrink_post) | **POST** /{index}/_shrink/{target} | 
[**indices_shrink_put**](DefaultApi.md#indices_shrink_put) | **PUT** /{index}/_shrink/{target} | 
[**indices_simulate_index_template**](DefaultApi.md#indices_simulate_index_template) | **POST** /_index_template/_simulate_index/{name} | 
[**indices_simulate_template**](DefaultApi.md#indices_simulate_template) | **POST** /_index_template/_simulate | 
[**indices_simulate_template_with_name**](DefaultApi.md#indices_simulate_template_with_name) | **POST** /_index_template/_simulate/{name} | 
[**indices_split_post**](DefaultApi.md#indices_split_post) | **POST** /{index}/_split/{target} | 
[**indices_split_put**](DefaultApi.md#indices_split_put) | **PUT** /{index}/_split/{target} | 
[**indices_stats**](DefaultApi.md#indices_stats) | **GET** /_stats | 
[**indices_stats_with_index**](DefaultApi.md#indices_stats_with_index) | **GET** /{index}/_stats | 
[**indices_stats_with_index_metric**](DefaultApi.md#indices_stats_with_index_metric) | **GET** /{index}/_stats/{metric} | 
[**indices_stats_with_metric**](DefaultApi.md#indices_stats_with_metric) | **GET** /_stats/{metric} | 
[**indices_update_aliases**](DefaultApi.md#indices_update_aliases) | **POST** /_aliases | 
[**indices_upgrade**](DefaultApi.md#indices_upgrade) | **POST** /_upgrade | 
[**indices_upgrade_with_index**](DefaultApi.md#indices_upgrade_with_index) | **POST** /{index}/_upgrade | 
[**indices_validate_query_get**](DefaultApi.md#indices_validate_query_get) | **GET** /_validate/query | 
[**indices_validate_query_get_with_index**](DefaultApi.md#indices_validate_query_get_with_index) | **GET** /{index}/_validate/query | 
[**indices_validate_query_post**](DefaultApi.md#indices_validate_query_post) | **POST** /_validate/query | 
[**indices_validate_query_post_with_index**](DefaultApi.md#indices_validate_query_post_with_index) | **POST** /{index}/_validate/query | 
[**info**](DefaultApi.md#info) | **GET** / | 
[**ingest_delete_pipeline**](DefaultApi.md#ingest_delete_pipeline) | **DELETE** /_ingest/pipeline/{id} | 
[**ingest_get_pipeline**](DefaultApi.md#ingest_get_pipeline) | **GET** /_ingest/pipeline | 
[**ingest_get_pipeline_with_id**](DefaultApi.md#ingest_get_pipeline_with_id) | **GET** /_ingest/pipeline/{id} | 
[**ingest_processor_grok**](DefaultApi.md#ingest_processor_grok) | **GET** /_ingest/processor/grok | 
[**ingest_put_pipeline**](DefaultApi.md#ingest_put_pipeline) | **PUT** /_ingest/pipeline/{id} | 
[**ingest_simulate_get**](DefaultApi.md#ingest_simulate_get) | **GET** /_ingest/pipeline/_simulate | 
[**ingest_simulate_get_with_id**](DefaultApi.md#ingest_simulate_get_with_id) | **GET** /_ingest/pipeline/{id}/_simulate | 
[**ingest_simulate_post**](DefaultApi.md#ingest_simulate_post) | **POST** /_ingest/pipeline/_simulate | 
[**ingest_simulate_post_with_id**](DefaultApi.md#ingest_simulate_post_with_id) | **POST** /_ingest/pipeline/{id}/_simulate | 
[**mget_get**](DefaultApi.md#mget_get) | **GET** /_mget | 
[**mget_get_with_index**](DefaultApi.md#mget_get_with_index) | **GET** /{index}/_mget | 
[**mget_post**](DefaultApi.md#mget_post) | **POST** /_mget | 
[**mget_post_with_index**](DefaultApi.md#mget_post_with_index) | **POST** /{index}/_mget | 
[**msearch_get**](DefaultApi.md#msearch_get) | **GET** /_msearch | 
[**msearch_get_with_index**](DefaultApi.md#msearch_get_with_index) | **GET** /{index}/_msearch | 
[**msearch_post**](DefaultApi.md#msearch_post) | **POST** /_msearch | 
[**msearch_post_with_index**](DefaultApi.md#msearch_post_with_index) | **POST** /{index}/_msearch | 
[**msearch_template_get**](DefaultApi.md#msearch_template_get) | **GET** /_msearch/template | 
[**msearch_template_get_with_index**](DefaultApi.md#msearch_template_get_with_index) | **GET** /{index}/_msearch/template | 
[**msearch_template_post**](DefaultApi.md#msearch_template_post) | **POST** /_msearch/template | 
[**msearch_template_post_with_index**](DefaultApi.md#msearch_template_post_with_index) | **POST** /{index}/_msearch/template | 
[**mtermvectors_get**](DefaultApi.md#mtermvectors_get) | **GET** /_mtermvectors | 
[**mtermvectors_get_with_index**](DefaultApi.md#mtermvectors_get_with_index) | **GET** /{index}/_mtermvectors | 
[**mtermvectors_post**](DefaultApi.md#mtermvectors_post) | **POST** /_mtermvectors | 
[**mtermvectors_post_with_index**](DefaultApi.md#mtermvectors_post_with_index) | **POST** /{index}/_mtermvectors | 
[**nodes_hot_threads**](DefaultApi.md#nodes_hot_threads) | **GET** /_nodes/hot_threads | 
[**nodes_hot_threads_deprecated**](DefaultApi.md#nodes_hot_threads_deprecated) | **GET** /_nodes/hotthreads | 
[**nodes_hot_threads_deprecated_cluster**](DefaultApi.md#nodes_hot_threads_deprecated_cluster) | **GET** /_cluster/nodes/hotthreads | 
[**nodes_hot_threads_deprecated_dash**](DefaultApi.md#nodes_hot_threads_deprecated_dash) | **GET** /_cluster/nodes/hot_threads | 
[**nodes_hot_threads_with_node_id**](DefaultApi.md#nodes_hot_threads_with_node_id) | **GET** /_nodes/{node_id}/hot_threads | 
[**nodes_hot_threads_with_node_id_deprecated**](DefaultApi.md#nodes_hot_threads_with_node_id_deprecated) | **GET** /_nodes/{node_id}/hotthreads | 
[**nodes_hot_threads_with_node_id_deprecated_cluster**](DefaultApi.md#nodes_hot_threads_with_node_id_deprecated_cluster) | **GET** /_cluster/nodes/{node_id}/hotthreads | 
[**nodes_hot_threads_with_node_id_deprecated_dash**](DefaultApi.md#nodes_hot_threads_with_node_id_deprecated_dash) | **GET** /_cluster/nodes/{node_id}/hot_threads | 
[**nodes_info**](DefaultApi.md#nodes_info) | **GET** /_nodes | 
[**nodes_info_with_metric_node_id**](DefaultApi.md#nodes_info_with_metric_node_id) | **GET** /_nodes/{node_id}/{metric} | 
[**nodes_info_with_node_id**](DefaultApi.md#nodes_info_with_node_id) | **GET** /_nodes/{node_id} | 
[**nodes_reload_secure_settings**](DefaultApi.md#nodes_reload_secure_settings) | **POST** /_nodes/reload_secure_settings | 
[**nodes_reload_secure_settings_with_node_id**](DefaultApi.md#nodes_reload_secure_settings_with_node_id) | **POST** /_nodes/{node_id}/reload_secure_settings | 
[**nodes_stats**](DefaultApi.md#nodes_stats) | **GET** /_nodes/stats | 
[**nodes_stats_with_index_metric_metric**](DefaultApi.md#nodes_stats_with_index_metric_metric) | **GET** /_nodes/stats/{metric}/{index_metric} | 
[**nodes_stats_with_index_metric_metric_node_id**](DefaultApi.md#nodes_stats_with_index_metric_metric_node_id) | **GET** /_nodes/{node_id}/stats/{metric}/{index_metric} | 
[**nodes_stats_with_metric**](DefaultApi.md#nodes_stats_with_metric) | **GET** /_nodes/stats/{metric} | 
[**nodes_stats_with_metric_node_id**](DefaultApi.md#nodes_stats_with_metric_node_id) | **GET** /_nodes/{node_id}/stats/{metric} | 
[**nodes_stats_with_node_id**](DefaultApi.md#nodes_stats_with_node_id) | **GET** /_nodes/{node_id}/stats | 
[**nodes_usage**](DefaultApi.md#nodes_usage) | **GET** /_nodes/usage | 
[**nodes_usage_with_metric**](DefaultApi.md#nodes_usage_with_metric) | **GET** /_nodes/usage/{metric} | 
[**nodes_usage_with_metric_node_id**](DefaultApi.md#nodes_usage_with_metric_node_id) | **GET** /_nodes/{node_id}/usage/{metric} | 
[**nodes_usage_with_node_id**](DefaultApi.md#nodes_usage_with_node_id) | **GET** /_nodes/{node_id}/usage | 
[**patch_action_group**](DefaultApi.md#patch_action_group) | **PATCH** /_plugins/_security/api/actiongroups/{action_group} | 
[**patch_action_groups**](DefaultApi.md#patch_action_groups) | **PATCH** /_plugins/_security/api/actiongroups | 
[**patch_audit_configuration**](DefaultApi.md#patch_audit_configuration) | **PATCH** /_plugins/_security/api/audit | 
[**patch_configuration**](DefaultApi.md#patch_configuration) | **PATCH** /_plugins/_security/api/securityconfig | 
[**patch_distinguished_names**](DefaultApi.md#patch_distinguished_names) | **PATCH** /_plugins/_security/api/nodesdn | 
[**patch_role**](DefaultApi.md#patch_role) | **PATCH** /_plugins/_security/api/roles/{role} | 
[**patch_role_mapping**](DefaultApi.md#patch_role_mapping) | **PATCH** /_plugins/_security/api/rolesmapping/{role} | 
[**patch_role_mappings**](DefaultApi.md#patch_role_mappings) | **PATCH** /_plugins/_security/api/rolesmapping | 
[**patch_roles**](DefaultApi.md#patch_roles) | **PATCH** /_plugins/_security/api/roles | 
[**patch_tenant**](DefaultApi.md#patch_tenant) | **PATCH** /_plugins/_security/api/tenants/{tenant} | 
[**patch_tenants**](DefaultApi.md#patch_tenants) | **PATCH** /_plugins/_security/api/tenants/ | 
[**patch_user**](DefaultApi.md#patch_user) | **PATCH** /_plugins/_security/api/internalusers/{username} | 
[**patch_users**](DefaultApi.md#patch_users) | **PATCH** /_plugins/_security/api/internalusers | 
[**ping**](DefaultApi.md#ping) | **HEAD** / | 
[**put_script_post**](DefaultApi.md#put_script_post) | **POST** /_scripts/{id} | 
[**put_script_post_with_context**](DefaultApi.md#put_script_post_with_context) | **POST** /_scripts/{id}/{context} | 
[**put_script_put**](DefaultApi.md#put_script_put) | **PUT** /_scripts/{id} | 
[**put_script_put_with_context**](DefaultApi.md#put_script_put_with_context) | **PUT** /_scripts/{id}/{context} | 
[**rank_eval_get**](DefaultApi.md#rank_eval_get) | **GET** /_rank_eval | 
[**rank_eval_get_with_index**](DefaultApi.md#rank_eval_get_with_index) | **GET** /{index}/_rank_eval | 
[**rank_eval_post**](DefaultApi.md#rank_eval_post) | **POST** /_rank_eval | 
[**rank_eval_post_with_index**](DefaultApi.md#rank_eval_post_with_index) | **POST** /{index}/_rank_eval | 
[**reindex**](DefaultApi.md#reindex) | **POST** /_reindex | 
[**reindex_rethrottle**](DefaultApi.md#reindex_rethrottle) | **POST** /_reindex/{task_id}/_rethrottle | 
[**reload_http_certificates**](DefaultApi.md#reload_http_certificates) | **PUT** /_plugins/_security/api/ssl/http/reloadcerts | 
[**reload_transport_certificates**](DefaultApi.md#reload_transport_certificates) | **PUT** /_plugins/_security/api/ssl/transport/reloadcerts | 
[**remote_store_restore**](DefaultApi.md#remote_store_restore) | **POST** /_remotestore/_restore | 
[**render_search_template_get**](DefaultApi.md#render_search_template_get) | **GET** /_render/template | 
[**render_search_template_get_with_id**](DefaultApi.md#render_search_template_get_with_id) | **GET** /_render/template/{id} | 
[**render_search_template_post**](DefaultApi.md#render_search_template_post) | **POST** /_render/template | 
[**render_search_template_post_with_id**](DefaultApi.md#render_search_template_post_with_id) | **POST** /_render/template/{id} | 
[**scripts_painless_execute_get**](DefaultApi.md#scripts_painless_execute_get) | **GET** /_scripts/painless/_execute | 
[**scripts_painless_execute_post**](DefaultApi.md#scripts_painless_execute_post) | **POST** /_scripts/painless/_execute | 
[**scroll_get**](DefaultApi.md#scroll_get) | **GET** /_search/scroll | 
[**scroll_get_with_scroll_id**](DefaultApi.md#scroll_get_with_scroll_id) | **GET** /_search/scroll/{scroll_id} | 
[**scroll_post**](DefaultApi.md#scroll_post) | **POST** /_search/scroll | 
[**scroll_post_with_scroll_id**](DefaultApi.md#scroll_post_with_scroll_id) | **POST** /_search/scroll/{scroll_id} | 
[**search_get**](DefaultApi.md#search_get) | **GET** /_search | 
[**search_get_with_index**](DefaultApi.md#search_get_with_index) | **GET** /{index}/_search | 
[**search_post**](DefaultApi.md#search_post) | **POST** /_search | 
[**search_post_with_index**](DefaultApi.md#search_post_with_index) | **POST** /{index}/_search | 
[**search_shards_get**](DefaultApi.md#search_shards_get) | **GET** /_search_shards | 
[**search_shards_get_with_index**](DefaultApi.md#search_shards_get_with_index) | **GET** /{index}/_search_shards | 
[**search_shards_post**](DefaultApi.md#search_shards_post) | **POST** /_search_shards | 
[**search_shards_post_with_index**](DefaultApi.md#search_shards_post_with_index) | **POST** /{index}/_search_shards | 
[**search_template_get**](DefaultApi.md#search_template_get) | **GET** /_search/template | 
[**search_template_get_with_index**](DefaultApi.md#search_template_get_with_index) | **GET** /{index}/_search/template | 
[**search_template_post**](DefaultApi.md#search_template_post) | **POST** /_search/template | 
[**search_template_post_with_index**](DefaultApi.md#search_template_post_with_index) | **POST** /{index}/_search/template | 
[**security_health**](DefaultApi.md#security_health) | **GET** /_plugins/_security/health | 
[**snapshot_cleanup_repository**](DefaultApi.md#snapshot_cleanup_repository) | **POST** /_snapshot/{repository}/_cleanup | 
[**snapshot_clone**](DefaultApi.md#snapshot_clone) | **PUT** /_snapshot/{repository}/{snapshot}/_clone/{target_snapshot} | 
[**snapshot_create_post**](DefaultApi.md#snapshot_create_post) | **POST** /_snapshot/{repository}/{snapshot} | 
[**snapshot_create_put**](DefaultApi.md#snapshot_create_put) | **PUT** /_snapshot/{repository}/{snapshot} | 
[**snapshot_create_repository_post**](DefaultApi.md#snapshot_create_repository_post) | **POST** /_snapshot/{repository} | 
[**snapshot_create_repository_put**](DefaultApi.md#snapshot_create_repository_put) | **PUT** /_snapshot/{repository} | 
[**snapshot_delete**](DefaultApi.md#snapshot_delete) | **DELETE** /_snapshot/{repository}/{snapshot} | 
[**snapshot_delete_repository**](DefaultApi.md#snapshot_delete_repository) | **DELETE** /_snapshot/{repository} | 
[**snapshot_get**](DefaultApi.md#snapshot_get) | **GET** /_snapshot/{repository}/{snapshot} | 
[**snapshot_get_repository**](DefaultApi.md#snapshot_get_repository) | **GET** /_snapshot | 
[**snapshot_get_repository_with_repository**](DefaultApi.md#snapshot_get_repository_with_repository) | **GET** /_snapshot/{repository} | 
[**snapshot_restore**](DefaultApi.md#snapshot_restore) | **POST** /_snapshot/{repository}/{snapshot}/_restore | 
[**snapshot_status**](DefaultApi.md#snapshot_status) | **GET** /_snapshot/_status | 
[**snapshot_status_with_repository**](DefaultApi.md#snapshot_status_with_repository) | **GET** /_snapshot/{repository}/_status | 
[**snapshot_status_with_repository_snapshot**](DefaultApi.md#snapshot_status_with_repository_snapshot) | **GET** /_snapshot/{repository}/{snapshot}/_status | 
[**snapshot_verify_repository**](DefaultApi.md#snapshot_verify_repository) | **POST** /_snapshot/{repository}/_verify | 
[**tasks_cancel**](DefaultApi.md#tasks_cancel) | **POST** /_tasks/_cancel | 
[**tasks_cancel_with_task_id**](DefaultApi.md#tasks_cancel_with_task_id) | **POST** /_tasks/{task_id}/_cancel | 
[**tasks_get**](DefaultApi.md#tasks_get) | **GET** /_tasks/{task_id} | 
[**tasks_list**](DefaultApi.md#tasks_list) | **GET** /_tasks | 
[**termvectors_get**](DefaultApi.md#termvectors_get) | **GET** /{index}/_termvectors | 
[**termvectors_get_with_id**](DefaultApi.md#termvectors_get_with_id) | **GET** /{index}/_termvectors/{id} | 
[**termvectors_post**](DefaultApi.md#termvectors_post) | **POST** /{index}/_termvectors | 
[**termvectors_post_with_id**](DefaultApi.md#termvectors_post_with_id) | **POST** /{index}/_termvectors/{id} | 
[**update**](DefaultApi.md#update) | **POST** /{index}/_update/{id} | 
[**update_audit_configuration**](DefaultApi.md#update_audit_configuration) | **PUT** /_plugins/_security/api/audit/config | 
[**update_by_query**](DefaultApi.md#update_by_query) | **POST** /{index}/_update_by_query | 
[**update_by_query_rethrottle**](DefaultApi.md#update_by_query_rethrottle) | **POST** /_update_by_query/{task_id}/_rethrottle | 
[**update_configuration**](DefaultApi.md#update_configuration) | **PUT** /_plugins/_security/api/securityconfig/config | 
[**update_distinguished_names**](DefaultApi.md#update_distinguished_names) | **PUT** /_plugins/_security/api/nodesdn/{cluster_name} | 



## bulk_post

> bulk_post(body, wait_for_active_shards, refresh, routing, timeout, r#type, _source, _source_excludes, _source_includes, pipeline, require_alias)


Allows to perform multiple index/update/delete operations in a single request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |
**wait_for_active_shards** | Option<**String**> | Sets the number of shard copies that must be active before proceeding with the operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1). |  |[default to 1]
**refresh** | Option<[**RefreshEnum**](.md)> | If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes. |  |
**routing** | Option<**String**> | Routing value. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**r#type** | Option<**String**> | Default document type for items which don't provide one. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or default list of fields to return, can be overridden on each sub-request. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | Default list of fields to exclude from the returned _source field, can be overridden on each sub-request. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | Default list of fields to extract and return from the _source field, can be overridden on each sub-request. |  |
**pipeline** | Option<**String**> | The pipeline id to preprocess incoming documents with. |  |
**require_alias** | Option<**bool**> | Sets require_alias for all incoming documents. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_post_with_index

> bulk_post_with_index(index, body, wait_for_active_shards, refresh, routing, timeout, r#type, _source, _source_excludes, _source_includes, pipeline, require_alias)


Allows to perform multiple index/update/delete operations in a single request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Default index for items which don't provide one. | [required] |
**body** | **serde_json::Value** |  | [required] |
**wait_for_active_shards** | Option<**String**> | Sets the number of shard copies that must be active before proceeding with the operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1). |  |[default to 1]
**refresh** | Option<[**RefreshEnum**](.md)> | If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes. |  |
**routing** | Option<**String**> | Routing value. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**r#type** | Option<**String**> | Default document type for items which don't provide one. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or default list of fields to return, can be overridden on each sub-request. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | Default list of fields to exclude from the returned _source field, can be overridden on each sub-request. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | Default list of fields to extract and return from the _source field, can be overridden on each sub-request. |  |
**pipeline** | Option<**String**> | The pipeline id to preprocess incoming documents with. |  |
**require_alias** | Option<**bool**> | Sets require_alias for all incoming documents. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_put

> bulk_put(body, wait_for_active_shards, refresh, routing, timeout, r#type, _source, _source_excludes, _source_includes, pipeline, require_alias)


Allows to perform multiple index/update/delete operations in a single request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |
**wait_for_active_shards** | Option<**String**> | Sets the number of shard copies that must be active before proceeding with the operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1). |  |[default to 1]
**refresh** | Option<[**RefreshEnum**](.md)> | If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes. |  |
**routing** | Option<**String**> | Routing value. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**r#type** | Option<**String**> | Default document type for items which don't provide one. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or default list of fields to return, can be overridden on each sub-request. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | Default list of fields to exclude from the returned _source field, can be overridden on each sub-request. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | Default list of fields to extract and return from the _source field, can be overridden on each sub-request. |  |
**pipeline** | Option<**String**> | The pipeline id to preprocess incoming documents with. |  |
**require_alias** | Option<**bool**> | Sets require_alias for all incoming documents. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_put_with_index

> bulk_put_with_index(index, body, wait_for_active_shards, refresh, routing, timeout, r#type, _source, _source_excludes, _source_includes, pipeline, require_alias)


Allows to perform multiple index/update/delete operations in a single request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Default index for items which don't provide one. | [required] |
**body** | **serde_json::Value** |  | [required] |
**wait_for_active_shards** | Option<**String**> | Sets the number of shard copies that must be active before proceeding with the operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1). |  |[default to 1]
**refresh** | Option<[**RefreshEnum**](.md)> | If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes. |  |
**routing** | Option<**String**> | Routing value. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**r#type** | Option<**String**> | Default document type for items which don't provide one. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or default list of fields to return, can be overridden on each sub-request. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | Default list of fields to exclude from the returned _source field, can be overridden on each sub-request. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | Default list of fields to extract and return from the _source field, can be overridden on each sub-request. |  |
**pipeline** | Option<**String**> | The pipeline id to preprocess incoming documents with. |  |
**require_alias** | Option<**bool**> | Sets require_alias for all incoming documents. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_aliases

> cat_aliases(format, local, h, help, s, v, expand_wildcards)


Shows information about currently configured aliases to indices including filter and routing infos.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_aliases_with_name

> cat_aliases_with_name(name, format, local, h, help, s, v, expand_wildcards)


Shows information about currently configured aliases to indices including filter and routing infos.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Comma-separated list of alias names. | [required] |
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_all_pit_segments

> crate::models::CatAllPitSegmentsResponseContent cat_all_pit_segments()


Lists all active point-in-time segments.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CatAllPitSegmentsResponseContent**](CatAllPitSegmentsResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_allocation

> cat_allocation(format, bytes, local, master_timeout, cluster_manager_timeout, h, help, s, v)


Provides a snapshot of how many shards are allocated to each data node and how much disk space they are using.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**bytes** | Option<[**Bytes**](.md)> | The unit in which to display byte values. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_allocation_with_node_id

> cat_allocation_with_node_id(node_id, format, bytes, local, master_timeout, cluster_manager_timeout, h, help, s, v)


Provides a snapshot of how many shards are allocated to each data node and how much disk space they are using.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** | Comma-separated list of node IDs or names to limit the returned information. | [required] |
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**bytes** | Option<[**Bytes**](.md)> | The unit in which to display byte values. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_cluster_manager

> cat_cluster_manager(format, local, master_timeout, cluster_manager_timeout, h, help, s, v)


Returns information about the cluster-manager node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_count

> cat_count(format, h, help, s, v)


Provides quick access to the document count of the entire cluster, or individual indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_count_with_index

> cat_count_with_index(index, format, h, help, s, v)


Provides quick access to the document count of the entire cluster, or individual indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices to limit the returned information. | [required] |
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_fielddata

> cat_fielddata(format, bytes, h, help, s, v, fields)


Shows how much heap memory is currently being used by fielddata on every data node in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**bytes** | Option<[**Bytes**](.md)> | The unit in which to display byte values. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields to return in the output. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_fielddata_with_fields

> cat_fielddata_with_fields(fields, format, bytes, h, help, s, v, fields2)


Shows how much heap memory is currently being used by fielddata on every data node in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | **String** | Comma-separated list of fields to return the fielddata size. | [required] |
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**bytes** | Option<[**Bytes**](.md)> | The unit in which to display byte values. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]
**fields2** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields to return in the output. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_health

> cat_health(format, h, help, s, time, ts, v)


Returns a concise representation of the cluster health.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**time** | Option<[**Time**](.md)> | The unit in which to display time values. |  |
**ts** | Option<**bool**> | Set to false to disable timestamping. |  |[default to true]
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_help

> cat_help(help, s)


Returns help for the Cat APIs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_indices

> cat_indices(format, bytes, local, master_timeout, cluster_manager_timeout, h, health, help, pri, s, time, v, include_unloaded_segments, expand_wildcards)


Returns information about indices: number of primaries and replicas, document counts, disk size, ...

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**bytes** | Option<[**Bytes**](.md)> | The unit in which to display byte values. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**health** | Option<[**Health**](.md)> | Health status ('green', 'yellow', or 'red') to filter only indices matching the specified health status. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**pri** | Option<**bool**> | Set to true to return stats only for primary shards. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**time** | Option<[**Time**](.md)> | The unit in which to display time values. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]
**include_unloaded_segments** | Option<**bool**> | If set to true segment stats will include stats for segments that are not currently loaded into memory. |  |[default to false]
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_indices_with_index

> cat_indices_with_index(index, format, bytes, local, master_timeout, cluster_manager_timeout, h, health, help, pri, s, time, v, include_unloaded_segments, expand_wildcards)


Returns information about indices: number of primaries and replicas, document counts, disk size, ...

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices to limit the returned information. | [required] |
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**bytes** | Option<[**Bytes**](.md)> | The unit in which to display byte values. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**health** | Option<[**Health**](.md)> | Health status ('green', 'yellow', or 'red') to filter only indices matching the specified health status. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**pri** | Option<**bool**> | Set to true to return stats only for primary shards. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**time** | Option<[**Time**](.md)> | The unit in which to display time values. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]
**include_unloaded_segments** | Option<**bool**> | If set to true segment stats will include stats for segments that are not currently loaded into memory. |  |[default to false]
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_master

> cat_master(format, local, master_timeout, cluster_manager_timeout, h, help, s, v)


Returns information about the cluster-manager node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_nodeattrs

> cat_nodeattrs(format, local, master_timeout, cluster_manager_timeout, h, help, s, v)


Returns information about custom node attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_nodes

> cat_nodes(bytes, format, full_id, local, master_timeout, cluster_manager_timeout, h, help, s, time, v)


Returns basic statistics about performance of cluster nodes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bytes** | Option<[**Bytes**](.md)> | The unit in which to display byte values. |  |
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**full_id** | Option<**bool**> | Return the full node ID instead of the shortened version. |  |[default to false]
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**time** | Option<[**Time**](.md)> | The unit in which to display time values. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_pending_tasks

> cat_pending_tasks(format, local, master_timeout, cluster_manager_timeout, h, help, s, time, v)


Returns a concise representation of the cluster pending tasks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**time** | Option<[**Time**](.md)> | The unit in which to display time values. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_pit_segments

> crate::models::CatPitSegmentsResponseContent cat_pit_segments(cat_pit_segments_body_params)


List segments for one or several PITs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cat_pit_segments_body_params** | Option<[**CatPitSegmentsBodyParams**](CatPitSegmentsBodyParams.md)> |  |  |

### Return type

[**crate::models::CatPitSegmentsResponseContent**](CatPitSegmentsResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_plugins

> cat_plugins(format, local, master_timeout, cluster_manager_timeout, h, help, s, v)


Returns information about installed plugins across nodes node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_recovery

> cat_recovery(format, active_only, bytes, detailed, h, help, index, s, time, v)


Returns information about index shard recoveries, both on-going completed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**active_only** | Option<**bool**> | If `true`, the response only includes ongoing shard recoveries. |  |[default to false]
**bytes** | Option<[**Bytes**](.md)> | The unit in which to display byte values. |  |
**detailed** | Option<**bool**> | If `true`, the response includes detailed information about shard recoveries. |  |[default to false]
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**index** | Option<[**Vec<String>**](String.md)> | Comma-separated list or wildcard expression of index names to limit the returned information. |  |
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**time** | Option<[**Time**](.md)> | The unit in which to display time values. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_recovery_with_index

> cat_recovery_with_index(index, format, active_only, bytes, detailed, h, help, index2, s, time, v)


Returns information about index shard recoveries, both on-going completed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list or wildcard expression of index names to limit the returned information. | [required] |
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**active_only** | Option<**bool**> | If `true`, the response only includes ongoing shard recoveries. |  |[default to false]
**bytes** | Option<[**Bytes**](.md)> | The unit in which to display byte values. |  |
**detailed** | Option<**bool**> | If `true`, the response includes detailed information about shard recoveries. |  |[default to false]
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**index2** | Option<[**Vec<String>**](String.md)> | Comma-separated list or wildcard expression of index names to limit the returned information. |  |
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**time** | Option<[**Time**](.md)> | The unit in which to display time values. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_repositories

> cat_repositories(format, local, master_timeout, cluster_manager_timeout, h, help, s, v)


Returns information about snapshot repositories registered in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_segment_replication

> cat_segment_replication(format, active_only, completed_only, bytes, detailed, shards, h, help, index, s, time, v)


Returns information about both on-going and latest completed Segment Replication events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**active_only** | Option<**bool**> | If `true`, the response only includes ongoing segment replication events. |  |[default to false]
**completed_only** | Option<**bool**> | If `true`, the response only includes latest completed segment replication events. |  |[default to false]
**bytes** | Option<[**Bytes**](.md)> | The unit in which to display byte values. |  |
**detailed** | Option<**bool**> | If `true`, the response includes detailed information about segment replications. |  |[default to false]
**shards** | Option<[**Vec<String>**](String.md)> | Comma-separated list of shards to display. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**index** | Option<[**Vec<String>**](String.md)> | Comma-separated list or wildcard expression of index names to limit the returned information. |  |
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**time** | Option<[**Time**](.md)> | The unit in which to display time values. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_segment_replication_with_index

> cat_segment_replication_with_index(index, format, active_only, completed_only, bytes, detailed, shards, h, help, index2, s, time, v)


Returns information about both on-going and latest completed Segment Replication events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list or wildcard expression of index names to limit the returned information. | [required] |
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**active_only** | Option<**bool**> | If `true`, the response only includes ongoing segment replication events. |  |[default to false]
**completed_only** | Option<**bool**> | If `true`, the response only includes latest completed segment replication events. |  |[default to false]
**bytes** | Option<[**Bytes**](.md)> | The unit in which to display byte values. |  |
**detailed** | Option<**bool**> | If `true`, the response includes detailed information about segment replications. |  |[default to false]
**shards** | Option<[**Vec<String>**](String.md)> | Comma-separated list of shards to display. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**index2** | Option<[**Vec<String>**](String.md)> | Comma-separated list or wildcard expression of index names to limit the returned information. |  |
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**time** | Option<[**Time**](.md)> | The unit in which to display time values. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_segments

> cat_segments(format, bytes, master_timeout, cluster_manager_timeout, h, help, s, v)


Provides low-level information about the segments in the shards of an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**bytes** | Option<[**Bytes**](.md)> | The unit in which to display byte values. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_segments_with_index

> cat_segments_with_index(index, format, bytes, master_timeout, cluster_manager_timeout, h, help, s, v)


Provides low-level information about the segments in the shards of an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices to limit the returned information. | [required] |
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**bytes** | Option<[**Bytes**](.md)> | The unit in which to display byte values. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_shards

> cat_shards(format, bytes, local, master_timeout, cluster_manager_timeout, h, help, s, time, v)


Provides a detailed view of shard allocation on nodes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**bytes** | Option<[**Bytes**](.md)> | The unit in which to display byte values. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**time** | Option<[**Time**](.md)> | The unit in which to display time values. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_shards_with_index

> cat_shards_with_index(index, format, bytes, local, master_timeout, cluster_manager_timeout, h, help, s, time, v)


Provides a detailed view of shard allocation on nodes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices to limit the returned information. | [required] |
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**bytes** | Option<[**Bytes**](.md)> | The unit in which to display byte values. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**time** | Option<[**Time**](.md)> | The unit in which to display time values. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_snapshots

> cat_snapshots(format, ignore_unavailable, master_timeout, cluster_manager_timeout, h, help, s, time, v)


Returns all snapshots in a specific repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**time** | Option<[**Time**](.md)> | The unit in which to display time values. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_snapshots_with_repository

> cat_snapshots_with_repository(repository, format, ignore_unavailable, master_timeout, cluster_manager_timeout, h, help, s, time, v)


Returns all snapshots in a specific repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | Comma-separated list of repository names. | [required] |
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**time** | Option<[**Time**](.md)> | The unit in which to display time values. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_tasks

> cat_tasks(format, nodes, actions, detailed, parent_task_id, h, help, s, time, v)


Returns information about the tasks currently executing on one or more nodes in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**nodes** | Option<[**Vec<String>**](String.md)> | Comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes. |  |
**actions** | Option<[**Vec<String>**](String.md)> | Comma-separated list of actions that should be returned. Leave empty to return all. |  |
**detailed** | Option<**bool**> | Return detailed task information. |  |[default to false]
**parent_task_id** | Option<**String**> | Return tasks with specified parent task id (node_id:task_number). Set to -1 to return all. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**time** | Option<[**Time**](.md)> | The unit in which to display time values. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_templates

> cat_templates(format, local, master_timeout, cluster_manager_timeout, h, help, s, v)


Returns information about existing templates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_templates_with_name

> cat_templates_with_name(name, format, local, master_timeout, cluster_manager_timeout, h, help, s, v)


Returns information about existing templates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the template. | [required] |
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_thread_pool

> cat_thread_pool(format, size, local, master_timeout, cluster_manager_timeout, h, help, s, v)


Returns cluster-wide thread pool statistics per node. By default the active, queue and rejected statistics are returned for all thread pools.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**size** | Option<**i32**> | The multiplier in which to display values. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cat_thread_pool_with_thread_pool_patterns

> cat_thread_pool_with_thread_pool_patterns(thread_pool_patterns, format, size, local, master_timeout, cluster_manager_timeout, h, help, s, v)


Returns cluster-wide thread pool statistics per node. By default the active, queue and rejected statistics are returned for all thread pools.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_pool_patterns** | **String** | Comma-separated list of regular-expressions to filter the thread pools in the output. | [required] |
**format** | Option<**String**> | A short version of the Accept header, e.g. json, yaml. |  |
**size** | Option<**i32**> | The multiplier in which to display values. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**h** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names to display. |  |
**help** | Option<**bool**> | Return help information. |  |[default to false]
**s** | Option<[**Vec<String>**](String.md)> | Comma-separated list of column names or column aliases to sort by. |  |
**v** | Option<**bool**> | Verbose mode. Display column headers. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_password

> crate::models::ChangePasswordResponseContent change_password(change_password_request_content)


Changes the password for the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_password_request_content** | [**ChangePasswordRequestContent**](ChangePasswordRequestContent.md) |  | [required] |

### Return type

[**crate::models::ChangePasswordResponseContent**](ChangePasswordResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_scroll

> clear_scroll(body)


Explicitly clears the search context for a scroll.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_scroll_with_scroll_id

> clear_scroll_with_scroll_id(scroll_id, body)


Explicitly clears the search context for a scroll.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scroll_id** | **String** | Comma-separated list of scroll IDs to clear. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_allocation_explain_get

> cluster_allocation_explain_get(include_yes_decisions, include_disk_info)


Provides explanations for shard allocations in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_yes_decisions** | Option<**bool**> | Return 'YES' decisions in explanation. |  |[default to false]
**include_disk_info** | Option<**bool**> | Return information about disk usage and shard sizes. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_allocation_explain_post

> cluster_allocation_explain_post(include_yes_decisions, include_disk_info, body)


Provides explanations for shard allocations in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_yes_decisions** | Option<**bool**> | Return 'YES' decisions in explanation. |  |[default to false]
**include_disk_info** | Option<**bool**> | Return information about disk usage and shard sizes. |  |[default to false]
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_delete_component_template

> cluster_delete_component_template(name, timeout, master_timeout, cluster_manager_timeout)


Deletes a component template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the template. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_delete_decommission_awareness

> cluster_delete_decommission_awareness()


Delete any existing decommission.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_delete_voting_config_exclusions

> cluster_delete_voting_config_exclusions(wait_for_removal)


Clears cluster voting config exclusions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wait_for_removal** | Option<**bool**> | Specifies whether to wait for all excluded nodes to be removed from the cluster before clearing the voting configuration exclusions list. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_delete_weighted_routing

> cluster_delete_weighted_routing()


Delete weighted shard routing weights.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_exists_component_template

> cluster_exists_component_template(name, master_timeout, local)


Returns information about whether a particular component template exist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the template. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_get_component_template

> cluster_get_component_template(master_timeout, cluster_manager_timeout, local)


Returns one or more component templates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_get_component_template_with_name

> cluster_get_component_template_with_name(name, master_timeout, cluster_manager_timeout, local)


Returns one or more component templates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The Comma-separated names of the component templates. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_get_decommission_awareness

> cluster_get_decommission_awareness(awareness_attribute_name)


Get details and status of decommissioned attribute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**awareness_attribute_name** | **String** | Awareness attribute name. | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_get_settings

> crate::models::ClusterGetSettingsResponseContent cluster_get_settings(flat_settings, master_timeout, cluster_manager_timeout, timeout, include_defaults)


Returns cluster settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**include_defaults** | Option<**bool**> | Whether to return all default clusters setting. |  |[default to false]

### Return type

[**crate::models::ClusterGetSettingsResponseContent**](ClusterGetSettingsResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_get_weighted_routing

> cluster_get_weighted_routing(attribute)


Fetches weighted shard routing weights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute** | **String** | Awareness attribute name. | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_health

> cluster_health(expand_wildcards, level, local, master_timeout, cluster_manager_timeout, timeout, wait_for_active_shards, wait_for_nodes, wait_for_events, wait_for_no_relocating_shards, wait_for_no_initializing_shards, wait_for_status, awareness_attribute, ensure_node_commissioned)


Returns basic information about the health of the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**level** | Option<[**ClusterHealthLevel**](.md)> | Specify the level of detail for returned information. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**wait_for_active_shards** | Option<**String**> | Wait until the specified number of shards is active. |  |
**wait_for_nodes** | Option<**String**> | Wait until the specified number of nodes is available. |  |
**wait_for_events** | Option<[**WaitForEvents**](.md)> | Wait until all currently queued events with the given priority are processed. |  |
**wait_for_no_relocating_shards** | Option<**bool**> | Whether to wait until there are no relocating shards in the cluster. |  |
**wait_for_no_initializing_shards** | Option<**bool**> | Whether to wait until there are no initializing shards in the cluster. |  |
**wait_for_status** | Option<[**WaitForStatus**](.md)> | Wait until cluster is in a specific state. |  |
**awareness_attribute** | Option<**String**> | The awareness attribute for which the health is required. |  |
**ensure_node_commissioned** | Option<**bool**> | Checks whether local node is commissioned or not. If set to true on a local call it will throw exception if node is decommissioned. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_health_with_index

> cluster_health_with_index(index, expand_wildcards, level, local, master_timeout, cluster_manager_timeout, timeout, wait_for_active_shards, wait_for_nodes, wait_for_events, wait_for_no_relocating_shards, wait_for_no_initializing_shards, wait_for_status, awareness_attribute, ensure_node_commissioned)


Returns basic information about the health of the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Limit the information returned to specific indicies. | [required] |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**level** | Option<[**ClusterHealthLevel**](.md)> | Specify the level of detail for returned information. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**wait_for_active_shards** | Option<**String**> | Wait until the specified number of shards is active. |  |
**wait_for_nodes** | Option<**String**> | Wait until the specified number of nodes is available. |  |
**wait_for_events** | Option<[**WaitForEvents**](.md)> | Wait until all currently queued events with the given priority are processed. |  |
**wait_for_no_relocating_shards** | Option<**bool**> | Whether to wait until there are no relocating shards in the cluster. |  |
**wait_for_no_initializing_shards** | Option<**bool**> | Whether to wait until there are no initializing shards in the cluster. |  |
**wait_for_status** | Option<[**WaitForStatus**](.md)> | Wait until cluster is in a specific state. |  |
**awareness_attribute** | Option<**String**> | The awareness attribute for which the health is required. |  |
**ensure_node_commissioned** | Option<**bool**> | Checks whether local node is commissioned or not. If set to true on a local call it will throw exception if node is decommissioned. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_pending_tasks

> cluster_pending_tasks(local, master_timeout, cluster_manager_timeout)


Returns a list of any cluster-level changes (e.g. create index, update mapping, allocate or fail shard) which have not yet been executed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_post_voting_config_exclusions

> cluster_post_voting_config_exclusions(node_ids, node_names, timeout)


Updates the cluster voting config exclusions by node ids or node names.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_ids** | Option<**String**> | Comma-separated list of the persistent ids of the nodes to exclude from the voting configuration. If specified, you may not also specify ?node_names. |  |
**node_names** | Option<**String**> | Comma-separated list of the names of the nodes to exclude from the voting configuration. If specified, you may not also specify ?node_ids. |  |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_put_component_template_post

> cluster_put_component_template_post(name, body, create, timeout, master_timeout, cluster_manager_timeout)


Creates or updates a component template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the template. | [required] |
**body** | **serde_json::Value** |  | [required] |
**create** | Option<**bool**> | Whether the index template should only be added if new or can also replace an existing one. |  |[default to false]
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_put_component_template_put

> cluster_put_component_template_put(name, body, create, timeout, master_timeout, cluster_manager_timeout)


Creates or updates a component template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the template. | [required] |
**body** | **serde_json::Value** |  | [required] |
**create** | Option<**bool**> | Whether the index template should only be added if new or can also replace an existing one. |  |[default to false]
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_put_decommission_awareness

> cluster_put_decommission_awareness(awareness_attribute_name, awareness_attribute_value)


Decommissions an awareness attribute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**awareness_attribute_name** | **String** | Awareness attribute name. | [required] |
**awareness_attribute_value** | **String** | Awareness attribute value. | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_put_settings

> crate::models::ClusterPutSettingsResponseContent cluster_put_settings(cluster_put_settings_body_params, flat_settings, master_timeout, cluster_manager_timeout, timeout)


Updates the cluster settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_put_settings_body_params** | [**ClusterPutSettingsBodyParams**](ClusterPutSettingsBodyParams.md) |  | [required] |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

[**crate::models::ClusterPutSettingsResponseContent**](ClusterPutSettingsResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_put_weighted_routing

> cluster_put_weighted_routing(attribute)


Updates weighted shard routing weights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute** | **String** | Awareness attribute name. | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_remote_info

> cluster_remote_info()


Returns the information about configured remote clusters.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_reroute

> cluster_reroute(dry_run, explain, retry_failed, metric, master_timeout, cluster_manager_timeout, timeout, body)


Allows to manually change the allocation of individual shards in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dry_run** | Option<**bool**> | Simulate the operation only and return the resulting state. |  |
**explain** | Option<**bool**> | Return an explanation of why the commands can or cannot be executed. |  |
**retry_failed** | Option<**bool**> | Retries allocation of shards that are blocked due to too many subsequent allocation failures. |  |
**metric** | Option<[**Vec<crate::models::ClusterRerouteMetricMember>**](crate::models::ClusterRerouteMetricMember.md)> | Limit the information returned to the specified metrics. Defaults to all but metadata. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_state

> cluster_state(local, master_timeout, cluster_manager_timeout, flat_settings, wait_for_metadata_version, wait_for_timeout, ignore_unavailable, allow_no_indices, expand_wildcards)


Returns a comprehensive information about the state of the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**wait_for_metadata_version** | Option<**i32**> | Wait for the metadata version to be equal or greater than the specified metadata version. |  |
**wait_for_timeout** | Option<**String**> | The maximum time to wait for wait_for_metadata_version before timing out. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_state_with_index_metric

> cluster_state_with_index_metric(index, metric, local, master_timeout, cluster_manager_timeout, flat_settings, wait_for_metadata_version, wait_for_timeout, ignore_unavailable, allow_no_indices, expand_wildcards)


Returns a comprehensive information about the state of the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**metric** | **String** | Limit the information returned to the specified metrics. | [required] |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**wait_for_metadata_version** | Option<**i32**> | Wait for the metadata version to be equal or greater than the specified metadata version. |  |
**wait_for_timeout** | Option<**String**> | The maximum time to wait for wait_for_metadata_version before timing out. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_state_with_metric

> cluster_state_with_metric(metric, local, master_timeout, cluster_manager_timeout, flat_settings, wait_for_metadata_version, wait_for_timeout, ignore_unavailable, allow_no_indices, expand_wildcards)


Returns a comprehensive information about the state of the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric** | **String** | Limit the information returned to the specified metrics. | [required] |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**wait_for_metadata_version** | Option<**i32**> | Wait for the metadata version to be equal or greater than the specified metadata version. |  |
**wait_for_timeout** | Option<**String**> | The maximum time to wait for wait_for_metadata_version before timing out. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_stats

> cluster_stats(flat_settings, timeout)


Returns high-level overview of cluster statistics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_stats_with_node_id

> cluster_stats_with_node_id(node_id, flat_settings, timeout)


Returns high-level overview of cluster statistics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** | Comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes. | [required] |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## count_get

> count_get(ignore_unavailable, ignore_throttled, allow_no_indices, expand_wildcards, min_score, preference, routing, q, analyzer, analyze_wildcard, default_operator, df, lenient, terminate_after)


Returns number of documents matching a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**ignore_throttled** | Option<**bool**> | Whether specified concrete, expanded or aliased indices should be ignored when throttled. |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**min_score** | Option<**i32**> | Include only documents with a specific `_score` value in the result. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**routing** | Option<[**Vec<String>**](String.md)> | Comma-separated list of specific routing values. |  |
**q** | Option<**String**> | Query in the Lucene query string syntax. |  |
**analyzer** | Option<**String**> | The analyzer to use for the query string. |  |
**analyze_wildcard** | Option<**bool**> | Specify whether wildcard and prefix queries should be analyzed. |  |[default to false]
**default_operator** | Option<[**DefaultOperator**](.md)> | The default operator for query string query (AND or OR). |  |
**df** | Option<**String**> | The field to use as default where no field prefix is given in the query string. |  |
**lenient** | Option<**bool**> | Specify whether format-based query failures (such as providing text to a numeric field) should be ignored. |  |
**terminate_after** | Option<**i32**> | The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## count_get_with_index

> count_get_with_index(index, ignore_unavailable, ignore_throttled, allow_no_indices, expand_wildcards, min_score, preference, routing, q, analyzer, analyze_wildcard, default_operator, df, lenient, terminate_after)


Returns number of documents matching a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices to restrict the results. | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**ignore_throttled** | Option<**bool**> | Whether specified concrete, expanded or aliased indices should be ignored when throttled. |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**min_score** | Option<**i32**> | Include only documents with a specific `_score` value in the result. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**routing** | Option<[**Vec<String>**](String.md)> | Comma-separated list of specific routing values. |  |
**q** | Option<**String**> | Query in the Lucene query string syntax. |  |
**analyzer** | Option<**String**> | The analyzer to use for the query string. |  |
**analyze_wildcard** | Option<**bool**> | Specify whether wildcard and prefix queries should be analyzed. |  |[default to false]
**default_operator** | Option<[**DefaultOperator**](.md)> | The default operator for query string query (AND or OR). |  |
**df** | Option<**String**> | The field to use as default where no field prefix is given in the query string. |  |
**lenient** | Option<**bool**> | Specify whether format-based query failures (such as providing text to a numeric field) should be ignored. |  |
**terminate_after** | Option<**i32**> | The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## count_post

> count_post(ignore_unavailable, ignore_throttled, allow_no_indices, expand_wildcards, min_score, preference, routing, q, analyzer, analyze_wildcard, default_operator, df, lenient, terminate_after, body)


Returns number of documents matching a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**ignore_throttled** | Option<**bool**> | Whether specified concrete, expanded or aliased indices should be ignored when throttled. |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**min_score** | Option<**i32**> | Include only documents with a specific `_score` value in the result. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**routing** | Option<[**Vec<String>**](String.md)> | Comma-separated list of specific routing values. |  |
**q** | Option<**String**> | Query in the Lucene query string syntax. |  |
**analyzer** | Option<**String**> | The analyzer to use for the query string. |  |
**analyze_wildcard** | Option<**bool**> | Specify whether wildcard and prefix queries should be analyzed. |  |[default to false]
**default_operator** | Option<[**DefaultOperator**](.md)> | The default operator for query string query (AND or OR). |  |
**df** | Option<**String**> | The field to use as default where no field prefix is given in the query string. |  |
**lenient** | Option<**bool**> | Specify whether format-based query failures (such as providing text to a numeric field) should be ignored. |  |
**terminate_after** | Option<**i32**> | The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## count_post_with_index

> count_post_with_index(index, ignore_unavailable, ignore_throttled, allow_no_indices, expand_wildcards, min_score, preference, routing, q, analyzer, analyze_wildcard, default_operator, df, lenient, terminate_after, body)


Returns number of documents matching a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices to restrict the results. | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**ignore_throttled** | Option<**bool**> | Whether specified concrete, expanded or aliased indices should be ignored when throttled. |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**min_score** | Option<**i32**> | Include only documents with a specific `_score` value in the result. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**routing** | Option<[**Vec<String>**](String.md)> | Comma-separated list of specific routing values. |  |
**q** | Option<**String**> | Query in the Lucene query string syntax. |  |
**analyzer** | Option<**String**> | The analyzer to use for the query string. |  |
**analyze_wildcard** | Option<**bool**> | Specify whether wildcard and prefix queries should be analyzed. |  |[default to false]
**default_operator** | Option<[**DefaultOperator**](.md)> | The default operator for query string query (AND or OR). |  |
**df** | Option<**String**> | The field to use as default where no field prefix is given in the query string. |  |
**lenient** | Option<**bool**> | Specify whether format-based query failures (such as providing text to a numeric field) should be ignored. |  |
**terminate_after** | Option<**i32**> | The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_action_group

> crate::models::CreateActionGroupResponseContent create_action_group(action_group, action_group2)


Creates or replaces the specified action group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_group** | **String** | The name of the action group to create or replace | [required] |
**action_group2** | [**ActionGroup**](ActionGroup.md) |  | [required] |

### Return type

[**crate::models::CreateActionGroupResponseContent**](CreateActionGroupResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_pit

> crate::models::CreatePitResponseContent create_pit(index, allow_partial_pit_creation, keep_alive, preference, routing, expand_wildcards)


Creates point in time context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**allow_partial_pit_creation** | Option<**bool**> | Allow if point in time can be created with partial failures. |  |
**keep_alive** | Option<**String**> | Specify the keep alive for point in time. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**routing** | Option<[**Vec<String>**](String.md)> | Comma-separated list of specific routing values. |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

[**crate::models::CreatePitResponseContent**](CreatePitResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_post

> create_post(id, index, body, wait_for_active_shards, refresh, routing, timeout, version, version_type, pipeline)


Creates a new document in the index.  Returns a 409 response when a document with a same ID already exists in the index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Document ID. | [required] |
**index** | **String** | Index name. | [required] |
**body** | **serde_json::Value** |  | [required] |
**wait_for_active_shards** | Option<**String**> | Sets the number of shard copies that must be active before proceeding with the operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1). |  |[default to 1]
**refresh** | Option<[**RefreshEnum**](.md)> | If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes. |  |
**routing** | Option<**String**> | Routing value. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |
**pipeline** | Option<**String**> | The pipeline id to preprocess incoming documents with. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_put

> create_put(id, index, body, wait_for_active_shards, refresh, routing, timeout, version, version_type, pipeline)


Creates a new document in the index.  Returns a 409 response when a document with a same ID already exists in the index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Document ID. | [required] |
**index** | **String** | Index name. | [required] |
**body** | **serde_json::Value** |  | [required] |
**wait_for_active_shards** | Option<**String**> | Sets the number of shard copies that must be active before proceeding with the operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1). |  |[default to 1]
**refresh** | Option<[**RefreshEnum**](.md)> | If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes. |  |
**routing** | Option<**String**> | Routing value. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |
**pipeline** | Option<**String**> | The pipeline id to preprocess incoming documents with. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_role

> crate::models::CreateRoleResponseContent create_role(role, role2)


Creates or replaces the specified role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** |  | [required] |
**role2** | [**Role**](Role.md) |  | [required] |

### Return type

[**crate::models::CreateRoleResponseContent**](CreateRoleResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_role_mapping

> crate::models::CreateRoleMappingResponseContent create_role_mapping(role, role_mapping)


Creates or replaces the specified role mapping.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** |  | [required] |
**role_mapping** | [**RoleMapping**](RoleMapping.md) |  | [required] |

### Return type

[**crate::models::CreateRoleMappingResponseContent**](CreateRoleMappingResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_tenant

> crate::models::CreateTenantResponseContent create_tenant(tenant, create_tenant_params)


Creates or replaces the specified tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**create_tenant_params** | [**CreateTenantParams**](CreateTenantParams.md) |  | [required] |

### Return type

[**crate::models::CreateTenantResponseContent**](CreateTenantResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user

> crate::models::CreateUserResponseContent create_user(username, user)


Creates or replaces the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**user** | [**User**](User.md) |  | [required] |

### Return type

[**crate::models::CreateUserResponseContent**](CreateUserResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dangling_indices_delete_dangling_index

> dangling_indices_delete_dangling_index(index_uuid, accept_data_loss, timeout, master_timeout, cluster_manager_timeout)


Deletes the specified dangling index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_uuid** | **String** | The UUID of the dangling index. | [required] |
**accept_data_loss** | Option<**bool**> | Must be set to true in order to delete the dangling index. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dangling_indices_import_dangling_index

> dangling_indices_import_dangling_index(index_uuid, accept_data_loss, timeout, master_timeout, cluster_manager_timeout)


Imports the specified dangling index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_uuid** | **String** | The UUID of the dangling index. | [required] |
**accept_data_loss** | Option<**bool**> | Must be set to true in order to import the dangling index. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dangling_indices_list_dangling_indices

> dangling_indices_list_dangling_indices()


Returns all dangling indices.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete

> delete(id, index, wait_for_active_shards, refresh, routing, timeout, if_seq_no, if_primary_term, version, version_type)


Removes a document from the index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Document ID. | [required] |
**index** | **String** | Index name. | [required] |
**wait_for_active_shards** | Option<**String**> | Sets the number of shard copies that must be active before proceeding with the operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1). |  |[default to 1]
**refresh** | Option<[**RefreshEnum**](.md)> | If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes. |  |
**routing** | Option<**String**> | Routing value. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**if_seq_no** | Option<**i32**> | only perform the operation if the last operation that has changed the document has the specified sequence number. |  |
**if_primary_term** | Option<**i32**> | only perform the operation if the last operation that has changed the document has the specified primary term. |  |
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_action_group

> crate::models::DeleteActionGroupResponseContent delete_action_group(action_group)


Delete a specified action group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_group** | **String** | Action group to delete. | [required] |

### Return type

[**crate::models::DeleteActionGroupResponseContent**](DeleteActionGroupResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_pits

> crate::models::DeleteAllPitsResponseContent delete_all_pits()


Deletes all active point in time searches.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DeleteAllPitsResponseContent**](DeleteAllPitsResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_by_query

> delete_by_query(index, body, analyzer, analyze_wildcard, default_operator, df, from, ignore_unavailable, allow_no_indices, conflicts, expand_wildcards, lenient, preference, q, routing, scroll, search_type, search_timeout, size, max_docs, sort, _source, _source_excludes, _source_includes, terminate_after, stats, version, request_cache, refresh, timeout, wait_for_active_shards, scroll_size, wait_for_completion, requests_per_second, slices)


Deletes documents matching the provided query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**body** | **serde_json::Value** |  | [required] |
**analyzer** | Option<**String**> | The analyzer to use for the query string. |  |
**analyze_wildcard** | Option<**bool**> | Specify whether wildcard and prefix queries should be analyzed. |  |[default to false]
**default_operator** | Option<[**DefaultOperator**](.md)> | The default operator for query string query (AND or OR). |  |
**df** | Option<**String**> | The field to use as default where no field prefix is given in the query string. |  |
**from** | Option<**i32**> | Starting offset. |  |[default to 0]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**conflicts** | Option<[**Conflicts**](.md)> | What to do when the operation encounters version conflicts?. |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**lenient** | Option<**bool**> | Specify whether format-based query failures (such as providing text to a numeric field) should be ignored. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**q** | Option<**String**> | Query in the Lucene query string syntax. |  |
**routing** | Option<[**Vec<String>**](String.md)> | Comma-separated list of specific routing values. |  |
**scroll** | Option<**String**> | Specify how long a consistent view of the index should be maintained for scrolled search. |  |
**search_type** | Option<[**SearchType**](.md)> | Search operation type. |  |
**search_timeout** | Option<**String**> | Explicit timeout for each search request. Defaults to no timeout. |  |
**size** | Option<**i32**> | Deprecated, please use `max_docs` instead. |  |
**max_docs** | Option<**i32**> | Maximum number of documents to process (default: all documents). |  |
**sort** | Option<[**Vec<String>**](String.md)> | Comma-separated list of <field>:<direction> pairs. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or a list of fields to return. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | List of fields to exclude from the returned _source field. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | List of fields to extract and return from the _source field. |  |
**terminate_after** | Option<**i32**> | The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early. |  |
**stats** | Option<[**Vec<String>**](String.md)> | Specific 'tag' of the request for logging and statistical purposes. |  |
**version** | Option<**bool**> | Whether to return document version as part of a hit. |  |
**request_cache** | Option<**bool**> | Specify if request cache should be used for this request or not, defaults to index level setting. |  |
**refresh** | Option<**bool**> | Refresh the shard containing the document before performing the operation. |  |
**timeout** | Option<**String**> | Time each individual bulk request should wait for shards that are unavailable. |  |[default to 1m]
**wait_for_active_shards** | Option<**String**> | Sets the number of shard copies that must be active before proceeding with the operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1). |  |[default to 1]
**scroll_size** | Option<**i32**> | Size on the scroll request powering the operation. |  |[default to 100]
**wait_for_completion** | Option<**bool**> | Should this request wait until the operation has completed before returning. |  |[default to true]
**requests_per_second** | Option<**i32**> | The throttle for this request in sub-requests per second. -1 means no throttle. |  |[default to 0]
**slices** | Option<**String**> | The number of slices this task should be divided into. Defaults to 1, meaning the task isn't sliced into subtasks. Can be set to `auto`. |  |[default to 1]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_by_query_rethrottle

> delete_by_query_rethrottle(task_id, requests_per_second)


Changes the number of requests per second for a particular Delete By Query operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | The task id to rethrottle. | [required] |
**requests_per_second** | **i32** | The throttle for this request in sub-requests per second. -1 means no throttle. | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_distinguished_names

> crate::models::DeleteDistinguishedNamesResponseContent delete_distinguished_names(cluster_name)


Deletes all distinguished names in the specified cluster’s or node’s allow list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_name** | **String** |  | [required] |

### Return type

[**crate::models::DeleteDistinguishedNamesResponseContent**](DeleteDistinguishedNamesResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pit

> crate::models::DeletePitResponseContent delete_pit(delete_pit_body_params)


Deletes one or more point in time searches based on the IDs passed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_pit_body_params** | Option<[**DeletePitBodyParams**](DeletePitBodyParams.md)> |  |  |

### Return type

[**crate::models::DeletePitResponseContent**](DeletePitResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_role

> crate::models::DeleteRoleResponseContent delete_role(role)


Delete the specified role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** |  | [required] |

### Return type

[**crate::models::DeleteRoleResponseContent**](DeleteRoleResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_role_mapping

> crate::models::DeleteRoleMappingResponseContent delete_role_mapping(role)


Deletes the specified role mapping.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** |  | [required] |

### Return type

[**crate::models::DeleteRoleMappingResponseContent**](DeleteRoleMappingResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_script

> delete_script(id, timeout, master_timeout, cluster_manager_timeout)


Deletes a script.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Script ID. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tenant

> crate::models::DeleteTenantResponseContent delete_tenant(tenant)


Delete the specified tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |

### Return type

[**crate::models::DeleteTenantResponseContent**](DeleteTenantResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> crate::models::DeleteUserResponseContent delete_user(username)


Delete the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

[**crate::models::DeleteUserResponseContent**](DeleteUserResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exists

> exists(id, index, stored_fields, preference, realtime, refresh, routing, _source, _source_excludes, _source_includes, version, version_type)


Returns information about whether a document exists in an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Document ID. | [required] |
**index** | **String** | Index name. | [required] |
**stored_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of stored fields to return. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**realtime** | Option<**bool**> | Specify whether to perform the operation in realtime or search mode. |  |
**refresh** | Option<**bool**> | Refresh the shard containing the document before performing the operation. |  |
**routing** | Option<**String**> | Routing value. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or a list of fields to return. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | List of fields to exclude from the returned _source field. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | List of fields to extract and return from the _source field. |  |
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exists_source

> exists_source(id, index, preference, realtime, refresh, routing, _source, _source_excludes, _source_includes, version, version_type)


Returns information about whether a document source exists in an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Document ID. | [required] |
**index** | **String** | Index name. | [required] |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**realtime** | Option<**bool**> | Specify whether to perform the operation in realtime or search mode. |  |
**refresh** | Option<**bool**> | Refresh the shard containing the document before performing the operation. |  |
**routing** | Option<**String**> | Routing value. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or a list of fields to return. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | List of fields to exclude from the returned _source field. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | List of fields to extract and return from the _source field. |  |
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## explain_get

> explain_get(id, index, analyze_wildcard, analyzer, default_operator, df, stored_fields, lenient, preference, q, routing, _source, _source_excludes, _source_includes)


Returns information about why a specific matches (or doesn't match) a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Document ID. | [required] |
**index** | **String** | Index name. | [required] |
**analyze_wildcard** | Option<**bool**> | Specify whether wildcards and prefix queries in the query string query should be analyzed. |  |[default to false]
**analyzer** | Option<**String**> | The analyzer to use for the query string. |  |
**default_operator** | Option<[**DefaultOperator**](.md)> | The default operator for query string query (AND or OR). |  |
**df** | Option<**String**> | The default field for query string query. |  |[default to _all]
**stored_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of stored fields to return. |  |
**lenient** | Option<**bool**> | Specify whether format-based query failures (such as providing text to a numeric field) should be ignored. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**q** | Option<**String**> | Query in the Lucene query string syntax. |  |
**routing** | Option<**String**> | Routing value. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or a list of fields to return. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | List of fields to exclude from the returned _source field. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | List of fields to extract and return from the _source field. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## explain_post

> explain_post(id, index, analyze_wildcard, analyzer, default_operator, df, stored_fields, lenient, preference, q, routing, _source, _source_excludes, _source_includes, body)


Returns information about why a specific matches (or doesn't match) a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Document ID. | [required] |
**index** | **String** | Index name. | [required] |
**analyze_wildcard** | Option<**bool**> | Specify whether wildcards and prefix queries in the query string query should be analyzed. |  |[default to false]
**analyzer** | Option<**String**> | The analyzer to use for the query string. |  |
**default_operator** | Option<[**DefaultOperator**](.md)> | The default operator for query string query (AND or OR). |  |
**df** | Option<**String**> | The default field for query string query. |  |[default to _all]
**stored_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of stored fields to return. |  |
**lenient** | Option<**bool**> | Specify whether format-based query failures (such as providing text to a numeric field) should be ignored. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**q** | Option<**String**> | Query in the Lucene query string syntax. |  |
**routing** | Option<**String**> | Routing value. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or a list of fields to return. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | List of fields to exclude from the returned _source field. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | List of fields to extract and return from the _source field. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## field_caps_get

> field_caps_get(fields, ignore_unavailable, allow_no_indices, expand_wildcards, include_unmapped)


Returns the information about the capabilities of fields among multiple indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of field names. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**include_unmapped** | Option<**bool**> | Indicates whether unmapped fields should be included in the response. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## field_caps_get_with_index

> field_caps_get_with_index(index, fields, ignore_unavailable, allow_no_indices, expand_wildcards, include_unmapped)


Returns the information about the capabilities of fields among multiple indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of field names. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**include_unmapped** | Option<**bool**> | Indicates whether unmapped fields should be included in the response. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## field_caps_post

> field_caps_post(fields, ignore_unavailable, allow_no_indices, expand_wildcards, include_unmapped, body)


Returns the information about the capabilities of fields among multiple indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of field names. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**include_unmapped** | Option<**bool**> | Indicates whether unmapped fields should be included in the response. |  |[default to false]
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## field_caps_post_with_index

> field_caps_post_with_index(index, fields, ignore_unavailable, allow_no_indices, expand_wildcards, include_unmapped, body)


Returns the information about the capabilities of fields among multiple indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of field names. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**include_unmapped** | Option<**bool**> | Indicates whether unmapped fields should be included in the response. |  |[default to false]
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flush_cache

> crate::models::FlushCacheResponseContent flush_cache()


Flushes the Security plugin user, authentication, and authorization cache.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::FlushCacheResponseContent**](FlushCacheResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get

> crate::models::GetResponseContent get(id, index, stored_fields, preference, realtime, refresh, routing, _source, _source_excludes, _source_includes, version, version_type)


Returns a document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Document ID. | [required] |
**index** | **String** | Index name. | [required] |
**stored_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of stored fields to return. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**realtime** | Option<**bool**> | Specify whether to perform the operation in realtime or search mode. |  |
**refresh** | Option<**bool**> | Refresh the shard containing the document before performing the operation. |  |
**routing** | Option<**String**> | Routing value. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or a list of fields to return. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | List of fields to exclude from the returned _source field. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | List of fields to extract and return from the _source field. |  |
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |

### Return type

[**crate::models::GetResponseContent**](GetResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_details

> crate::models::AccountDetails get_account_details()


Returns account details for the current user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AccountDetails**](AccountDetails.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_action_group

> ::std::collections::HashMap<String, crate::models::ActionGroup> get_action_group(action_group)


Retrieves one action group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_group** | **String** | Action group to retrieve. | [required] |

### Return type

[**::std::collections::HashMap<String, crate::models::ActionGroup>**](Action_Group.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_action_groups

> ::std::collections::HashMap<String, crate::models::ActionGroup> get_action_groups()


Retrieves all action groups.

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, crate::models::ActionGroup>**](Action_Group.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_pits

> crate::models::GetAllPitsResponseContent get_all_pits()


Lists all active point in time searches.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAllPitsResponseContent**](GetAllPitsResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audit_configuration

> crate::models::AuditConfigWithReadOnly get_audit_configuration()


Retrieves the audit configuration.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AuditConfigWithReadOnly**](AuditConfigWithReadOnly.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_certificates

> crate::models::GetCertificatesResponseContent get_certificates()


Retrieves the cluster’s security certificates.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetCertificatesResponseContent**](GetCertificatesResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_configuration

> crate::models::DynamicConfig get_configuration()


Returns the current Security plugin configuration in JSON format.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DynamicConfig**](DynamicConfig.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_distinguished_names

> ::std::collections::HashMap<String, crate::models::DistinguishedNames> get_distinguished_names()


Retrieves all distinguished names in the allow list.

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, crate::models::DistinguishedNames>**](DistinguishedNames.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_distinguished_names_with_cluster_name

> ::std::collections::HashMap<String, crate::models::DistinguishedNames> get_distinguished_names_with_cluster_name(cluster_name)


Retrieve distinguished names of a specified cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_name** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, crate::models::DistinguishedNames>**](DistinguishedNames.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role

> ::std::collections::HashMap<String, crate::models::Role> get_role(role)


Retrieves one role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, crate::models::Role>**](Role.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_mapping

> ::std::collections::HashMap<String, crate::models::RoleMapping> get_role_mapping(role)


Retrieves one role mapping.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, crate::models::RoleMapping>**](RoleMapping.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_mappings

> ::std::collections::HashMap<String, crate::models::RoleMapping> get_role_mappings()


Retrieves all role mappings.

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, crate::models::RoleMapping>**](RoleMapping.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_roles

> ::std::collections::HashMap<String, crate::models::Role> get_roles()


Retrieves all roles.

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, crate::models::Role>**](Role.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_script

> get_script(id, master_timeout, cluster_manager_timeout)


Returns a script.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Script ID. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_script_context

> get_script_context()


Returns all script contexts.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_script_languages

> get_script_languages()


Returns available script types, languages and contexts.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_source

> get_source(id, index, preference, realtime, refresh, routing, _source, _source_excludes, _source_includes, version, version_type)


Returns the source of a document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Document ID. | [required] |
**index** | **String** | Index name. | [required] |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**realtime** | Option<**bool**> | Specify whether to perform the operation in realtime or search mode. |  |
**refresh** | Option<**bool**> | Refresh the shard containing the document before performing the operation. |  |
**routing** | Option<**String**> | Routing value. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or a list of fields to return. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | List of fields to exclude from the returned _source field. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | List of fields to extract and return from the _source field. |  |
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant

> ::std::collections::HashMap<String, crate::models::Tenant> get_tenant(tenant)


Retrieves one tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, crate::models::Tenant>**](Tenant.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenants

> ::std::collections::HashMap<String, crate::models::Tenant> get_tenants()


Retrieves all tenants.

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, crate::models::Tenant>**](Tenant.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> ::std::collections::HashMap<String, crate::models::User> get_user(username)


Retrieve one internal user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, crate::models::User>**](User.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users

> ::std::collections::HashMap<String, crate::models::User> get_users()


Retrieve all internal users.

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, crate::models::User>**](User.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_post

> index_post(index, body, wait_for_active_shards, op_type, refresh, routing, timeout, version, version_type, if_seq_no, if_primary_term, pipeline, require_alias)


Creates or updates a document in an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Index name. | [required] |
**body** | **serde_json::Value** |  | [required] |
**wait_for_active_shards** | Option<**String**> | Sets the number of shard copies that must be active before proceeding with the operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1). |  |[default to 1]
**op_type** | Option<[**OpType**](.md)> | Explicit operation type. Defaults to `index` for requests with an explicit document ID, and to `create`for requests without an explicit document ID. |  |
**refresh** | Option<[**RefreshEnum**](.md)> | If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes. |  |
**routing** | Option<**String**> | Routing value. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |
**if_seq_no** | Option<**i32**> | only perform the operation if the last operation that has changed the document has the specified sequence number. |  |
**if_primary_term** | Option<**i32**> | only perform the operation if the last operation that has changed the document has the specified primary term. |  |
**pipeline** | Option<**String**> | The pipeline id to preprocess incoming documents with. |  |
**require_alias** | Option<**bool**> | When true, requires destination to be an alias. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_post_with_id

> index_post_with_id(id, index, body, wait_for_active_shards, op_type, refresh, routing, timeout, version, version_type, if_seq_no, if_primary_term, pipeline, require_alias)


Creates or updates a document in an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Document ID. | [required] |
**index** | **String** | Index name. | [required] |
**body** | **serde_json::Value** |  | [required] |
**wait_for_active_shards** | Option<**String**> | Sets the number of shard copies that must be active before proceeding with the operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1). |  |[default to 1]
**op_type** | Option<[**OpType**](.md)> | Explicit operation type. Defaults to `index` for requests with an explicit document ID, and to `create`for requests without an explicit document ID. |  |
**refresh** | Option<[**RefreshEnum**](.md)> | If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes. |  |
**routing** | Option<**String**> | Routing value. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |
**if_seq_no** | Option<**i32**> | only perform the operation if the last operation that has changed the document has the specified sequence number. |  |
**if_primary_term** | Option<**i32**> | only perform the operation if the last operation that has changed the document has the specified primary term. |  |
**pipeline** | Option<**String**> | The pipeline id to preprocess incoming documents with. |  |
**require_alias** | Option<**bool**> | When true, requires destination to be an alias. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_put_with_id

> index_put_with_id(id, index, body, wait_for_active_shards, op_type, refresh, routing, timeout, version, version_type, if_seq_no, if_primary_term, pipeline, require_alias)


Creates or updates a document in an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Document ID. | [required] |
**index** | **String** | Index name. | [required] |
**body** | **serde_json::Value** |  | [required] |
**wait_for_active_shards** | Option<**String**> | Sets the number of shard copies that must be active before proceeding with the operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1). |  |[default to 1]
**op_type** | Option<[**OpType**](.md)> | Explicit operation type. Defaults to `index` for requests with an explicit document ID, and to `create`for requests without an explicit document ID. |  |
**refresh** | Option<[**RefreshEnum**](.md)> | If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes. |  |
**routing** | Option<**String**> | Routing value. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |
**if_seq_no** | Option<**i32**> | only perform the operation if the last operation that has changed the document has the specified sequence number. |  |
**if_primary_term** | Option<**i32**> | only perform the operation if the last operation that has changed the document has the specified primary term. |  |
**pipeline** | Option<**String**> | The pipeline id to preprocess incoming documents with. |  |
**require_alias** | Option<**bool**> | When true, requires destination to be an alias. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_add_block

> indices_add_block(index, block, timeout, master_timeout, cluster_manager_timeout, ignore_unavailable, allow_no_indices, expand_wildcards)


Adds a block to an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices to add a block to. | [required] |
**block** | **String** | The block to add (one of read, write, read_only or metadata). | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_analyze_get

> indices_analyze_get(index)


Performs the analysis process on a text and return the tokens breakdown of the text.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | Option<**String**> | The name of the index to scope the operation. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_analyze_get_with_index

> indices_analyze_get_with_index(index, index2)


Performs the analysis process on a text and return the tokens breakdown of the text.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | The name of the index to scope the operation. | [required] |
**index2** | Option<**String**> | The name of the index to scope the operation. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_analyze_post

> indices_analyze_post(index, body)


Performs the analysis process on a text and return the tokens breakdown of the text.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | Option<**String**> | The name of the index to scope the operation. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_analyze_post_with_index

> indices_analyze_post_with_index(index, index2, body)


Performs the analysis process on a text and return the tokens breakdown of the text.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | The name of the index to scope the operation. | [required] |
**index2** | Option<**String**> | The name of the index to scope the operation. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_clear_cache

> indices_clear_cache(fielddata, fields, query, ignore_unavailable, allow_no_indices, expand_wildcards, index, request)


Clears all or specific caches for one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fielddata** | Option<**bool**> | Clear field data. |  |
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields to clear when using the `fielddata` parameter (default: all). |  |
**query** | Option<**bool**> | Clear query caches. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**index** | Option<[**Vec<String>**](String.md)> | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. |  |
**request** | Option<**bool**> | Clear request cache. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_clear_cache_with_index

> indices_clear_cache_with_index(index, fielddata, fields, query, ignore_unavailable, allow_no_indices, expand_wildcards, index2, request)


Clears all or specific caches for one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**fielddata** | Option<**bool**> | Clear field data. |  |
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields to clear when using the `fielddata` parameter (default: all). |  |
**query** | Option<**bool**> | Clear query caches. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**index2** | Option<[**Vec<String>**](String.md)> | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. |  |
**request** | Option<**bool**> | Clear request cache. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_clone_post

> indices_clone_post(index, target, timeout, master_timeout, cluster_manager_timeout, wait_for_active_shards, body)


Clones an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | The name of the source index to clone. | [required] |
**target** | **String** | The name of the target index. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**wait_for_active_shards** | Option<**String**> | Set the number of active shards to wait for on the cloned index before the operation returns. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_clone_put

> indices_clone_put(index, target, timeout, master_timeout, cluster_manager_timeout, wait_for_active_shards, body)


Clones an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | The name of the source index to clone. | [required] |
**target** | **String** | The name of the target index. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**wait_for_active_shards** | Option<**String**> | Set the number of active shards to wait for on the cloned index before the operation returns. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_close

> indices_close(index, timeout, master_timeout, cluster_manager_timeout, ignore_unavailable, allow_no_indices, expand_wildcards, wait_for_active_shards)


Closes an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices to close. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**wait_for_active_shards** | Option<**String**> | Sets the number of active shards to wait for before the operation returns. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_create

> crate::models::IndicesCreateResponseContent indices_create(index, wait_for_active_shards, timeout, master_timeout, cluster_manager_timeout, indices_create_body_params)


Creates an index with optional settings and mappings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Index name. | [required] |
**wait_for_active_shards** | Option<**String**> | Set the number of active shards to wait for before the operation returns. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**indices_create_body_params** | Option<[**IndicesCreateBodyParams**](IndicesCreateBodyParams.md)> |  |  |

### Return type

[**crate::models::IndicesCreateResponseContent**](IndicesCreateResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_create_data_stream

> crate::models::IndicesCreateDataStreamResponseContent indices_create_data_stream(name, body)


Creates or updates a data stream.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the data stream. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**crate::models::IndicesCreateDataStreamResponseContent**](IndicesCreateDataStreamResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_data_streams_stats

> indices_data_streams_stats()


Provides statistics on operations happening in a data stream.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_data_streams_stats_with_name

> indices_data_streams_stats_with_name(name)


Provides statistics on operations happening in a data stream.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Comma-separated list of data streams; use `_all` or empty string to perform the operation on all data streams. | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_delete

> crate::models::IndicesDeleteResponseContent indices_delete(index, timeout, master_timeout, ignore_unavailable, allow_no_indices, expand_wildcards)


Deletes an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices to delete; use `_all` or `*` string to delete all indices. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |[default to false]
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |[default to false]
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

[**crate::models::IndicesDeleteResponseContent**](IndicesDeleteResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_delete_alias

> indices_delete_alias(index, name, timeout, master_timeout, cluster_manager_timeout)


Deletes an alias.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**name** | **String** | Comma-separated list of aliases to delete (supports wildcards); use `_all` to delete all aliases for the specified indices. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_delete_alias_plural

> indices_delete_alias_plural(index, name, timeout, master_timeout, cluster_manager_timeout)


Deletes an alias.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**name** | **String** | Comma-separated list of aliases to delete (supports wildcards); use `_all` to delete all aliases for the specified indices. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_delete_data_stream

> crate::models::IndicesDeleteDataStreamResponseContent indices_delete_data_stream(name)


Deletes a data stream.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Comma-separated list of data streams; use `_all` or empty string to perform the operation on all data streams. | [required] |

### Return type

[**crate::models::IndicesDeleteDataStreamResponseContent**](IndicesDeleteDataStreamResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_delete_index_template

> indices_delete_index_template(name, timeout, master_timeout, cluster_manager_timeout)


Deletes an index template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the template. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_delete_template

> indices_delete_template(name, timeout, master_timeout, cluster_manager_timeout)


Deletes an index template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the template. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_exists

> indices_exists(index, local, ignore_unavailable, allow_no_indices, expand_wildcards, flat_settings, include_defaults)


Returns information about whether a particular index exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices. | [required] |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |[default to false]
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |[default to false]
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**include_defaults** | Option<**bool**> | Whether to return all default setting for each of the indices. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_exists_alias

> indices_exists_alias(name, ignore_unavailable, allow_no_indices, expand_wildcards, local)


Returns information about whether a particular alias exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Comma-separated list of alias names. | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_exists_alias_with_index

> indices_exists_alias_with_index(index, name, ignore_unavailable, allow_no_indices, expand_wildcards, local)


Returns information about whether a particular alias exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices to filter aliases. | [required] |
**name** | **String** | Comma-separated list of alias names. | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_exists_index_template

> indices_exists_index_template(name, flat_settings, master_timeout, local)


Returns information about whether a particular index template exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the template. | [required] |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_exists_template

> indices_exists_template(name, flat_settings, master_timeout, local)


Returns information about whether a particular index template exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Comma-separated names of the index templates. | [required] |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_flush_get

> indices_flush_get(force, wait_if_ongoing, ignore_unavailable, allow_no_indices, expand_wildcards)


Performs the flush operation on one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force** | Option<**bool**> | Whether a flush should be forced even if it is not necessarily needed ie. if no changes will be committed to the index. This is useful if transaction log IDs should be incremented even if no uncommitted changes are present. (This setting can be considered as internal). |  |
**wait_if_ongoing** | Option<**bool**> | If set to true the flush operation will block until the flush can be executed if another flush operation is already executing. If set to false the flush will be skipped iff if another flush operation is already running. |  |[default to true]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_flush_get_with_index

> indices_flush_get_with_index(index, force, wait_if_ongoing, ignore_unavailable, allow_no_indices, expand_wildcards)


Performs the flush operation on one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**force** | Option<**bool**> | Whether a flush should be forced even if it is not necessarily needed ie. if no changes will be committed to the index. This is useful if transaction log IDs should be incremented even if no uncommitted changes are present. (This setting can be considered as internal). |  |
**wait_if_ongoing** | Option<**bool**> | If set to true the flush operation will block until the flush can be executed if another flush operation is already executing. If set to false the flush will be skipped iff if another flush operation is already running. |  |[default to true]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_flush_post

> indices_flush_post(force, wait_if_ongoing, ignore_unavailable, allow_no_indices, expand_wildcards)


Performs the flush operation on one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force** | Option<**bool**> | Whether a flush should be forced even if it is not necessarily needed ie. if no changes will be committed to the index. This is useful if transaction log IDs should be incremented even if no uncommitted changes are present. (This setting can be considered as internal). |  |
**wait_if_ongoing** | Option<**bool**> | If set to true the flush operation will block until the flush can be executed if another flush operation is already executing. If set to false the flush will be skipped iff if another flush operation is already running. |  |[default to true]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_flush_post_with_index

> indices_flush_post_with_index(index, force, wait_if_ongoing, ignore_unavailable, allow_no_indices, expand_wildcards)


Performs the flush operation on one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**force** | Option<**bool**> | Whether a flush should be forced even if it is not necessarily needed ie. if no changes will be committed to the index. This is useful if transaction log IDs should be incremented even if no uncommitted changes are present. (This setting can be considered as internal). |  |
**wait_if_ongoing** | Option<**bool**> | If set to true the flush operation will block until the flush can be executed if another flush operation is already executing. If set to false the flush will be skipped iff if another flush operation is already running. |  |[default to true]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_forcemerge

> indices_forcemerge(flush, ignore_unavailable, allow_no_indices, expand_wildcards, max_num_segments, only_expunge_deletes)


Performs the force merge operation on one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flush** | Option<**bool**> | Specify whether the index should be flushed after performing the operation. |  |[default to true]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**max_num_segments** | Option<**i32**> | The number of segments the index should be merged into (default: dynamic). |  |
**only_expunge_deletes** | Option<**bool**> | Specify whether the operation should only expunge deleted documents. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_forcemerge_with_index

> indices_forcemerge_with_index(index, flush, ignore_unavailable, allow_no_indices, expand_wildcards, max_num_segments, only_expunge_deletes)


Performs the force merge operation on one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**flush** | Option<**bool**> | Specify whether the index should be flushed after performing the operation. |  |[default to true]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**max_num_segments** | Option<**i32**> | The number of segments the index should be merged into (default: dynamic). |  |
**only_expunge_deletes** | Option<**bool**> | Specify whether the operation should only expunge deleted documents. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get

> indices_get(index, local, ignore_unavailable, allow_no_indices, expand_wildcards, flat_settings, include_defaults, master_timeout, cluster_manager_timeout)


Returns information about one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices. | [required] |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |[default to false]
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |[default to false]
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**include_defaults** | Option<**bool**> | Whether to return all default setting for each of the indices. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_alias

> indices_get_alias(ignore_unavailable, allow_no_indices, expand_wildcards, local)


Returns an alias.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_alias_with_index

> indices_get_alias_with_index(index, ignore_unavailable, allow_no_indices, expand_wildcards, local)


Returns an alias.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices to filter aliases. | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_alias_with_index_name

> indices_get_alias_with_index_name(index, name, ignore_unavailable, allow_no_indices, expand_wildcards, local)


Returns an alias.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices to filter aliases. | [required] |
**name** | **String** | Comma-separated list of alias names. | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_alias_with_name

> indices_get_alias_with_name(name, ignore_unavailable, allow_no_indices, expand_wildcards, local)


Returns an alias.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Comma-separated list of alias names. | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_data_stream

> crate::models::IndicesGetDataStreamResponseContent indices_get_data_stream()


Returns data streams.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IndicesGetDataStreamResponseContent**](IndicesGetDataStreamResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_data_stream_with_name

> crate::models::IndicesGetDataStreamWithNameResponseContent indices_get_data_stream_with_name(name)


Returns data streams.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Comma-separated list of data streams; use `_all` or empty string to perform the operation on all data streams. | [required] |

### Return type

[**crate::models::IndicesGetDataStreamWithNameResponseContent**](IndicesGetDataStream_WithNameResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_field_mapping

> indices_get_field_mapping(fields, include_defaults, ignore_unavailable, allow_no_indices, expand_wildcards, local)


Returns mapping for one or more fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | **String** | Comma-separated list of fields. | [required] |
**include_defaults** | Option<**bool**> | Whether the default mapping values should be returned as well. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_field_mapping_with_index

> indices_get_field_mapping_with_index(index, fields, include_defaults, ignore_unavailable, allow_no_indices, expand_wildcards, local)


Returns mapping for one or more fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices. | [required] |
**fields** | **String** | Comma-separated list of fields. | [required] |
**include_defaults** | Option<**bool**> | Whether the default mapping values should be returned as well. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_index_template

> indices_get_index_template(flat_settings, master_timeout, cluster_manager_timeout, local)


Returns an index template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_index_template_with_name

> indices_get_index_template_with_name(name, flat_settings, master_timeout, cluster_manager_timeout, local)


Returns an index template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Comma-separated names of the index templates. | [required] |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_mapping

> indices_get_mapping(ignore_unavailable, allow_no_indices, expand_wildcards, master_timeout, cluster_manager_timeout, local)


Returns mappings for one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_mapping_with_index

> indices_get_mapping_with_index(index, ignore_unavailable, allow_no_indices, expand_wildcards, master_timeout, cluster_manager_timeout, local)


Returns mappings for one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices. | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_settings

> indices_get_settings(master_timeout, cluster_manager_timeout, ignore_unavailable, allow_no_indices, expand_wildcards, flat_settings, local, include_defaults)


Returns settings for one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**include_defaults** | Option<**bool**> | Whether to return all default setting for each of the indices. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_settings_with_index

> indices_get_settings_with_index(index, master_timeout, cluster_manager_timeout, ignore_unavailable, allow_no_indices, expand_wildcards, flat_settings, local, include_defaults)


Returns settings for one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**include_defaults** | Option<**bool**> | Whether to return all default setting for each of the indices. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_settings_with_index_name

> indices_get_settings_with_index_name(index, name, master_timeout, cluster_manager_timeout, ignore_unavailable, allow_no_indices, expand_wildcards, flat_settings, local, include_defaults)


Returns settings for one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**name** | **String** | Comma-separated list of settings. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**include_defaults** | Option<**bool**> | Whether to return all default setting for each of the indices. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_settings_with_name

> indices_get_settings_with_name(name, master_timeout, cluster_manager_timeout, ignore_unavailable, allow_no_indices, expand_wildcards, flat_settings, local, include_defaults)


Returns settings for one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Comma-separated list of settings. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**include_defaults** | Option<**bool**> | Whether to return all default setting for each of the indices. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_template

> indices_get_template(flat_settings, master_timeout, cluster_manager_timeout, local)


Returns an index template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_template_with_name

> indices_get_template_with_name(name, flat_settings, master_timeout, cluster_manager_timeout, local)


Returns an index template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Comma-separated names of the index templates. | [required] |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_upgrade

> indices_get_upgrade(ignore_unavailable, allow_no_indices, expand_wildcards)


The _upgrade API is no longer useful and will be removed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_get_upgrade_with_index

> indices_get_upgrade_with_index(index, ignore_unavailable, allow_no_indices, expand_wildcards)


The _upgrade API is no longer useful and will be removed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_open

> indices_open(index, timeout, master_timeout, ignore_unavailable, allow_no_indices, expand_wildcards, wait_for_active_shards)


Opens an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices to open. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**wait_for_active_shards** | Option<**String**> | Sets the number of active shards to wait for before the operation returns. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_put_alias_post

> indices_put_alias_post(index, name, timeout, master_timeout, cluster_manager_timeout, body)


Creates or updates an alias.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**name** | **String** | The name of the alias to be created or updated. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_put_alias_post_plural

> indices_put_alias_post_plural(index, name, timeout, master_timeout, cluster_manager_timeout, body)


Creates or updates an alias.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**name** | **String** | The name of the alias to be created or updated. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_put_alias_put

> indices_put_alias_put(index, name, timeout, master_timeout, cluster_manager_timeout, body)


Creates or updates an alias.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**name** | **String** | The name of the alias to be created or updated. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_put_alias_put_plural

> indices_put_alias_put_plural(index, name, timeout, master_timeout, cluster_manager_timeout, body)


Creates or updates an alias.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**name** | **String** | The name of the alias to be created or updated. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_put_index_template_post

> indices_put_index_template_post(name, body, create, cause, master_timeout, cluster_manager_timeout)


Creates or updates an index template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the template. | [required] |
**body** | **serde_json::Value** |  | [required] |
**create** | Option<**bool**> | Whether the index template should only be added if new or can also replace an existing one. |  |[default to false]
**cause** | Option<**String**> | User defined reason for creating/updating the index template. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_put_index_template_put

> indices_put_index_template_put(name, body, create, cause, master_timeout, cluster_manager_timeout)


Creates or updates an index template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the template. | [required] |
**body** | **serde_json::Value** |  | [required] |
**create** | Option<**bool**> | Whether the index template should only be added if new or can also replace an existing one. |  |[default to false]
**cause** | Option<**String**> | User defined reason for creating/updating the index template. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_put_mapping_post

> crate::models::IndicesPutMappingPostResponseContent indices_put_mapping_post(index, body, timeout, master_timeout, cluster_manager_timeout, ignore_unavailable, allow_no_indices, expand_wildcards, write_index_only)


Updates the index mappings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**body** | **serde_json::Value** |  | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**write_index_only** | Option<**bool**> | When true, applies mappings only to the write index of an alias or data stream. |  |[default to false]

### Return type

[**crate::models::IndicesPutMappingPostResponseContent**](IndicesPutMapping_PostResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_put_mapping_put

> crate::models::IndicesPutMappingPutResponseContent indices_put_mapping_put(index, body, timeout, master_timeout, cluster_manager_timeout, ignore_unavailable, allow_no_indices, expand_wildcards, write_index_only)


Updates the index mappings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**body** | **serde_json::Value** |  | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**write_index_only** | Option<**bool**> | When true, applies mappings only to the write index of an alias or data stream. |  |[default to false]

### Return type

[**crate::models::IndicesPutMappingPutResponseContent**](IndicesPutMapping_PutResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_put_settings

> indices_put_settings(body, master_timeout, cluster_manager_timeout, timeout, preserve_existing, ignore_unavailable, allow_no_indices, expand_wildcards, flat_settings)


Updates the index settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**preserve_existing** | Option<**bool**> | Whether to update existing settings. If set to `true` existing settings on an index remain unchanged. |  |[default to false]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_put_settings_with_index

> indices_put_settings_with_index(index, body, master_timeout, cluster_manager_timeout, timeout, preserve_existing, ignore_unavailable, allow_no_indices, expand_wildcards, flat_settings)


Updates the index settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**body** | **serde_json::Value** |  | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**preserve_existing** | Option<**bool**> | Whether to update existing settings. If set to `true` existing settings on an index remain unchanged. |  |[default to false]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_put_template_post

> indices_put_template_post(name, body, order, create, master_timeout, cluster_manager_timeout)


Creates or updates an index template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the template. | [required] |
**body** | **serde_json::Value** |  | [required] |
**order** | Option<**i32**> | The order for this template when merging multiple matching ones (higher numbers are merged later, overriding the lower numbers). |  |
**create** | Option<**bool**> | Whether the index template should only be added if new or can also replace an existing one. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_put_template_put

> indices_put_template_put(name, body, order, create, master_timeout, cluster_manager_timeout)


Creates or updates an index template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the template. | [required] |
**body** | **serde_json::Value** |  | [required] |
**order** | Option<**i32**> | The order for this template when merging multiple matching ones (higher numbers are merged later, overriding the lower numbers). |  |
**create** | Option<**bool**> | Whether the index template should only be added if new or can also replace an existing one. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_recovery

> indices_recovery(detailed, active_only)


Returns information about ongoing index shard recoveries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**detailed** | Option<**bool**> | Whether to display detailed information about shard recovery. |  |[default to false]
**active_only** | Option<**bool**> | Display only those recoveries that are currently on-going. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_recovery_with_index

> indices_recovery_with_index(index, detailed, active_only)


Returns information about ongoing index shard recoveries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**detailed** | Option<**bool**> | Whether to display detailed information about shard recovery. |  |[default to false]
**active_only** | Option<**bool**> | Display only those recoveries that are currently on-going. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_refresh_get

> indices_refresh_get(ignore_unavailable, allow_no_indices, expand_wildcards)


Performs the refresh operation in one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_refresh_get_with_index

> indices_refresh_get_with_index(index, ignore_unavailable, allow_no_indices, expand_wildcards)


Performs the refresh operation in one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_refresh_post

> indices_refresh_post(ignore_unavailable, allow_no_indices, expand_wildcards)


Performs the refresh operation in one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_refresh_post_with_index

> indices_refresh_post_with_index(index, ignore_unavailable, allow_no_indices, expand_wildcards)


Performs the refresh operation in one or more indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_resolve_index

> indices_resolve_index(name, expand_wildcards)


Returns information about any matching indices, aliases, and data streams.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Comma-separated list of names or wildcard expressions. | [required] |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_rollover

> indices_rollover(alias, timeout, dry_run, master_timeout, cluster_manager_timeout, wait_for_active_shards, body)


Updates an alias to point to a new index when the existing index is considered to be too large or too old.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias** | **String** | The name of the alias to rollover. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**dry_run** | Option<**bool**> | If set to true the rollover action will only be validated but not actually performed even if a condition matches. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**wait_for_active_shards** | Option<**String**> | Set the number of active shards to wait for on the newly created rollover index before the operation returns. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_rollover_with_new_index

> indices_rollover_with_new_index(alias, new_index, timeout, dry_run, master_timeout, cluster_manager_timeout, wait_for_active_shards, body)


Updates an alias to point to a new index when the existing index is considered to be too large or too old.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias** | **String** | The name of the alias to rollover. | [required] |
**new_index** | **String** | The name of the rollover index. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**dry_run** | Option<**bool**> | If set to true the rollover action will only be validated but not actually performed even if a condition matches. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**wait_for_active_shards** | Option<**String**> | Set the number of active shards to wait for on the newly created rollover index before the operation returns. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_segments

> indices_segments(ignore_unavailable, allow_no_indices, expand_wildcards, verbose)


Provides low-level information about segments in a Lucene index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**verbose** | Option<**bool**> | Includes detailed memory usage by Lucene. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_segments_with_index

> indices_segments_with_index(index, ignore_unavailable, allow_no_indices, expand_wildcards, verbose)


Provides low-level information about segments in a Lucene index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**verbose** | Option<**bool**> | Includes detailed memory usage by Lucene. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_shard_stores

> indices_shard_stores(status, ignore_unavailable, allow_no_indices, expand_wildcards)


Provides store information for shard copies of indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<[**Vec<crate::models::StatusMember>**](crate::models::StatusMember.md)> | Comma-separated list of statuses used to filter on shards to get store information for. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_shard_stores_with_index

> indices_shard_stores_with_index(index, status, ignore_unavailable, allow_no_indices, expand_wildcards)


Provides store information for shard copies of indices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**status** | Option<[**Vec<crate::models::StatusMember>**](crate::models::StatusMember.md)> | Comma-separated list of statuses used to filter on shards to get store information for. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_shrink_post

> indices_shrink_post(index, target, copy_settings, timeout, master_timeout, cluster_manager_timeout, wait_for_active_shards, body)


Allow to shrink an existing index into a new index with fewer primary shards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | The name of the source index to shrink. | [required] |
**target** | **String** | The name of the target index. | [required] |
**copy_settings** | Option<**bool**> | whether or not to copy settings from the source index. |  |[default to false]
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**wait_for_active_shards** | Option<**String**> | Set the number of active shards to wait for on the shrunken index before the operation returns. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_shrink_put

> indices_shrink_put(index, target, copy_settings, timeout, master_timeout, cluster_manager_timeout, wait_for_active_shards, body)


Allow to shrink an existing index into a new index with fewer primary shards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | The name of the source index to shrink. | [required] |
**target** | **String** | The name of the target index. | [required] |
**copy_settings** | Option<**bool**> | whether or not to copy settings from the source index. |  |[default to false]
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**wait_for_active_shards** | Option<**String**> | Set the number of active shards to wait for on the shrunken index before the operation returns. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_simulate_index_template

> indices_simulate_index_template(name, create, cause, master_timeout, cluster_manager_timeout, body)


Simulate matching the given index name against the index templates in the system.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the index (it must be a concrete index name). | [required] |
**create** | Option<**bool**> | Whether the index template we optionally defined in the body should only be dry-run added if new or can also replace an existing one. |  |[default to false]
**cause** | Option<**String**> | User defined reason for dry-run creating the new template for simulation purposes. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_simulate_template

> indices_simulate_template(create, cause, master_timeout, cluster_manager_timeout, body)


Simulate resolving the given template name or body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create** | Option<**bool**> | Whether the index template we optionally defined in the body should only be dry-run added if new or can also replace an existing one. |  |[default to false]
**cause** | Option<**String**> | User defined reason for dry-run creating the new template for simulation purposes. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_simulate_template_with_name

> indices_simulate_template_with_name(name, create, cause, master_timeout, cluster_manager_timeout, body)


Simulate resolving the given template name or body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the template. | [required] |
**create** | Option<**bool**> | Whether the index template we optionally defined in the body should only be dry-run added if new or can also replace an existing one. |  |[default to false]
**cause** | Option<**String**> | User defined reason for dry-run creating the new template for simulation purposes. |  |[default to false]
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_split_post

> indices_split_post(index, target, copy_settings, timeout, master_timeout, cluster_manager_timeout, wait_for_active_shards, body)


Allows you to split an existing index into a new index with more primary shards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | The name of the source index to split. | [required] |
**target** | **String** | The name of the target index. | [required] |
**copy_settings** | Option<**bool**> | whether or not to copy settings from the source index. |  |[default to false]
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**wait_for_active_shards** | Option<**String**> | Set the number of active shards to wait for on the shrunken index before the operation returns. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_split_put

> indices_split_put(index, target, copy_settings, timeout, master_timeout, cluster_manager_timeout, wait_for_active_shards, body)


Allows you to split an existing index into a new index with more primary shards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | The name of the source index to split. | [required] |
**target** | **String** | The name of the target index. | [required] |
**copy_settings** | Option<**bool**> | whether or not to copy settings from the source index. |  |[default to false]
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**wait_for_active_shards** | Option<**String**> | Set the number of active shards to wait for on the shrunken index before the operation returns. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_stats

> indices_stats(completion_fields, fielddata_fields, fields, groups, level, include_segment_file_sizes, include_unloaded_segments, expand_wildcards, forbid_closed_indices)


Provides statistics on operations happening in an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**completion_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `suggest` index metric (supports wildcards). |  |
**fielddata_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` index metric (supports wildcards). |  |
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `completion` index metric (supports wildcards). |  |
**groups** | Option<[**Vec<String>**](String.md)> | Comma-separated list of search groups for `search` index metric. |  |
**level** | Option<[**IndiciesStatLevel**](.md)> | Return stats aggregated at cluster, index or shard level. |  |
**include_segment_file_sizes** | Option<**bool**> | Whether to report the aggregated disk usage of each one of the Lucene index files (only applies if segment stats are requested). |  |[default to false]
**include_unloaded_segments** | Option<**bool**> | If set to true segment stats will include stats for segments that are not currently loaded into memory. |  |[default to false]
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**forbid_closed_indices** | Option<**bool**> | If set to false stats will also collected from closed indices if explicitly specified or if expand_wildcards expands to closed indices. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_stats_with_index

> indices_stats_with_index(index, completion_fields, fielddata_fields, fields, groups, level, include_segment_file_sizes, include_unloaded_segments, expand_wildcards, forbid_closed_indices)


Provides statistics on operations happening in an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**completion_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `suggest` index metric (supports wildcards). |  |
**fielddata_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` index metric (supports wildcards). |  |
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `completion` index metric (supports wildcards). |  |
**groups** | Option<[**Vec<String>**](String.md)> | Comma-separated list of search groups for `search` index metric. |  |
**level** | Option<[**IndiciesStatLevel**](.md)> | Return stats aggregated at cluster, index or shard level. |  |
**include_segment_file_sizes** | Option<**bool**> | Whether to report the aggregated disk usage of each one of the Lucene index files (only applies if segment stats are requested). |  |[default to false]
**include_unloaded_segments** | Option<**bool**> | If set to true segment stats will include stats for segments that are not currently loaded into memory. |  |[default to false]
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**forbid_closed_indices** | Option<**bool**> | If set to false stats will also collected from closed indices if explicitly specified or if expand_wildcards expands to closed indices. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_stats_with_index_metric

> indices_stats_with_index_metric(index, metric, completion_fields, fielddata_fields, fields, groups, level, include_segment_file_sizes, include_unloaded_segments, expand_wildcards, forbid_closed_indices)


Provides statistics on operations happening in an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**metric** | **String** | Limit the information returned the specific metrics. | [required] |
**completion_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `suggest` index metric (supports wildcards). |  |
**fielddata_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` index metric (supports wildcards). |  |
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `completion` index metric (supports wildcards). |  |
**groups** | Option<[**Vec<String>**](String.md)> | Comma-separated list of search groups for `search` index metric. |  |
**level** | Option<[**IndiciesStatLevel**](.md)> | Return stats aggregated at cluster, index or shard level. |  |
**include_segment_file_sizes** | Option<**bool**> | Whether to report the aggregated disk usage of each one of the Lucene index files (only applies if segment stats are requested). |  |[default to false]
**include_unloaded_segments** | Option<**bool**> | If set to true segment stats will include stats for segments that are not currently loaded into memory. |  |[default to false]
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**forbid_closed_indices** | Option<**bool**> | If set to false stats will also collected from closed indices if explicitly specified or if expand_wildcards expands to closed indices. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_stats_with_metric

> indices_stats_with_metric(metric, completion_fields, fielddata_fields, fields, groups, level, include_segment_file_sizes, include_unloaded_segments, expand_wildcards, forbid_closed_indices)


Provides statistics on operations happening in an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric** | **String** | Limit the information returned the specific metrics. | [required] |
**completion_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `suggest` index metric (supports wildcards). |  |
**fielddata_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` index metric (supports wildcards). |  |
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `completion` index metric (supports wildcards). |  |
**groups** | Option<[**Vec<String>**](String.md)> | Comma-separated list of search groups for `search` index metric. |  |
**level** | Option<[**IndiciesStatLevel**](.md)> | Return stats aggregated at cluster, index or shard level. |  |
**include_segment_file_sizes** | Option<**bool**> | Whether to report the aggregated disk usage of each one of the Lucene index files (only applies if segment stats are requested). |  |[default to false]
**include_unloaded_segments** | Option<**bool**> | If set to true segment stats will include stats for segments that are not currently loaded into memory. |  |[default to false]
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**forbid_closed_indices** | Option<**bool**> | If set to false stats will also collected from closed indices if explicitly specified or if expand_wildcards expands to closed indices. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_update_aliases

> crate::models::IndicesUpdateAliasesResponseContent indices_update_aliases(indices_update_aliases_body_params, timeout, master_timeout, cluster_manager_timeout)


Updates index aliases.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**indices_update_aliases_body_params** | [**IndicesUpdateAliasesBodyParams**](IndicesUpdateAliasesBodyParams.md) |  | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

[**crate::models::IndicesUpdateAliasesResponseContent**](IndicesUpdateAliasesResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_upgrade

> indices_upgrade(allow_no_indices, expand_wildcards, ignore_unavailable, wait_for_completion, only_ancient_segments)


The _upgrade API is no longer useful and will be removed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**wait_for_completion** | Option<**bool**> | Should this request wait until the operation has completed before returning. |  |[default to false]
**only_ancient_segments** | Option<**bool**> | If true, only ancient (an older Lucene major release) segments will be upgraded. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_upgrade_with_index

> indices_upgrade_with_index(index, allow_no_indices, expand_wildcards, ignore_unavailable, wait_for_completion, only_ancient_segments)


The _upgrade API is no longer useful and will be removed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**wait_for_completion** | Option<**bool**> | Should this request wait until the operation has completed before returning. |  |[default to false]
**only_ancient_segments** | Option<**bool**> | If true, only ancient (an older Lucene major release) segments will be upgraded. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_validate_query_get

> indices_validate_query_get(explain, ignore_unavailable, allow_no_indices, expand_wildcards, q, analyzer, analyze_wildcard, default_operator, df, lenient, rewrite, all_shards)


Allows a user to validate a potentially expensive query without executing it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**explain** | Option<**bool**> | Return detailed information about the error. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**q** | Option<**String**> | Query in the Lucene query string syntax. |  |
**analyzer** | Option<**String**> | The analyzer to use for the query string. |  |
**analyze_wildcard** | Option<**bool**> | Specify whether wildcard and prefix queries should be analyzed. |  |[default to false]
**default_operator** | Option<[**DefaultOperator**](.md)> | The default operator for query string query (AND or OR). |  |
**df** | Option<**String**> | The field to use as default where no field prefix is given in the query string. |  |
**lenient** | Option<**bool**> | Specify whether format-based query failures (such as providing text to a numeric field) should be ignored. |  |
**rewrite** | Option<**bool**> | Provide a more detailed explanation showing the actual Lucene query that will be executed. |  |
**all_shards** | Option<**bool**> | Execute validation on all shards instead of one random shard per index. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_validate_query_get_with_index

> indices_validate_query_get_with_index(index, explain, ignore_unavailable, allow_no_indices, expand_wildcards, q, analyzer, analyze_wildcard, default_operator, df, lenient, rewrite, all_shards)


Allows a user to validate a potentially expensive query without executing it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**explain** | Option<**bool**> | Return detailed information about the error. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**q** | Option<**String**> | Query in the Lucene query string syntax. |  |
**analyzer** | Option<**String**> | The analyzer to use for the query string. |  |
**analyze_wildcard** | Option<**bool**> | Specify whether wildcard and prefix queries should be analyzed. |  |[default to false]
**default_operator** | Option<[**DefaultOperator**](.md)> | The default operator for query string query (AND or OR). |  |
**df** | Option<**String**> | The field to use as default where no field prefix is given in the query string. |  |
**lenient** | Option<**bool**> | Specify whether format-based query failures (such as providing text to a numeric field) should be ignored. |  |
**rewrite** | Option<**bool**> | Provide a more detailed explanation showing the actual Lucene query that will be executed. |  |
**all_shards** | Option<**bool**> | Execute validation on all shards instead of one random shard per index. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_validate_query_post

> indices_validate_query_post(explain, ignore_unavailable, allow_no_indices, expand_wildcards, q, analyzer, analyze_wildcard, default_operator, df, lenient, rewrite, all_shards, body)


Allows a user to validate a potentially expensive query without executing it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**explain** | Option<**bool**> | Return detailed information about the error. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**q** | Option<**String**> | Query in the Lucene query string syntax. |  |
**analyzer** | Option<**String**> | The analyzer to use for the query string. |  |
**analyze_wildcard** | Option<**bool**> | Specify whether wildcard and prefix queries should be analyzed. |  |[default to false]
**default_operator** | Option<[**DefaultOperator**](.md)> | The default operator for query string query (AND or OR). |  |
**df** | Option<**String**> | The field to use as default where no field prefix is given in the query string. |  |
**lenient** | Option<**bool**> | Specify whether format-based query failures (such as providing text to a numeric field) should be ignored. |  |
**rewrite** | Option<**bool**> | Provide a more detailed explanation showing the actual Lucene query that will be executed. |  |
**all_shards** | Option<**bool**> | Execute validation on all shards instead of one random shard per index. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indices_validate_query_post_with_index

> indices_validate_query_post_with_index(index, explain, ignore_unavailable, allow_no_indices, expand_wildcards, q, analyzer, analyze_wildcard, default_operator, df, lenient, rewrite, all_shards, body)


Allows a user to validate a potentially expensive query without executing it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**explain** | Option<**bool**> | Return detailed information about the error. |  |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**q** | Option<**String**> | Query in the Lucene query string syntax. |  |
**analyzer** | Option<**String**> | The analyzer to use for the query string. |  |
**analyze_wildcard** | Option<**bool**> | Specify whether wildcard and prefix queries should be analyzed. |  |[default to false]
**default_operator** | Option<[**DefaultOperator**](.md)> | The default operator for query string query (AND or OR). |  |
**df** | Option<**String**> | The field to use as default where no field prefix is given in the query string. |  |
**lenient** | Option<**bool**> | Specify whether format-based query failures (such as providing text to a numeric field) should be ignored. |  |
**rewrite** | Option<**bool**> | Provide a more detailed explanation showing the actual Lucene query that will be executed. |  |
**all_shards** | Option<**bool**> | Execute validation on all shards instead of one random shard per index. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## info

> crate::models::InfoResponseContent info()


Returns basic information about the cluster.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InfoResponseContent**](InfoResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingest_delete_pipeline

> ingest_delete_pipeline(id, master_timeout, cluster_manager_timeout, timeout)


Deletes a pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Pipeline ID. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingest_get_pipeline

> ingest_get_pipeline(master_timeout, cluster_manager_timeout)


Returns a pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingest_get_pipeline_with_id

> ingest_get_pipeline_with_id(id, master_timeout, cluster_manager_timeout)


Returns a pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Comma-separated list of pipeline ids. Wildcards supported. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingest_processor_grok

> ingest_processor_grok()


Returns a list of the built-in patterns.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingest_put_pipeline

> ingest_put_pipeline(id, body, master_timeout, cluster_manager_timeout, timeout)


Creates or updates a pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Pipeline ID. | [required] |
**body** | **serde_json::Value** |  | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingest_simulate_get

> ingest_simulate_get(verbose)


Allows to simulate a pipeline with example documents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> | Verbose mode. Display data output for each processor in executed pipeline. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingest_simulate_get_with_id

> ingest_simulate_get_with_id(id, verbose)


Allows to simulate a pipeline with example documents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Pipeline ID. | [required] |
**verbose** | Option<**bool**> | Verbose mode. Display data output for each processor in executed pipeline. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingest_simulate_post

> ingest_simulate_post(body, verbose)


Allows to simulate a pipeline with example documents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |
**verbose** | Option<**bool**> | Verbose mode. Display data output for each processor in executed pipeline. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingest_simulate_post_with_id

> ingest_simulate_post_with_id(id, body, verbose)


Allows to simulate a pipeline with example documents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Pipeline ID. | [required] |
**body** | **serde_json::Value** |  | [required] |
**verbose** | Option<**bool**> | Verbose mode. Display data output for each processor in executed pipeline. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mget_get

> mget_get(stored_fields, preference, realtime, refresh, routing, _source, _source_excludes, _source_includes)


Allows to get multiple documents in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stored_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of stored fields to return. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**realtime** | Option<**bool**> | Specify whether to perform the operation in realtime or search mode. |  |
**refresh** | Option<**bool**> | Refresh the shard containing the document before performing the operation. |  |
**routing** | Option<**String**> | Routing value. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or a list of fields to return. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | List of fields to exclude from the returned _source field. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | List of fields to extract and return from the _source field. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mget_get_with_index

> mget_get_with_index(index, stored_fields, preference, realtime, refresh, routing, _source, _source_excludes, _source_includes)


Allows to get multiple documents in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Index name. | [required] |
**stored_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of stored fields to return. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**realtime** | Option<**bool**> | Specify whether to perform the operation in realtime or search mode. |  |
**refresh** | Option<**bool**> | Refresh the shard containing the document before performing the operation. |  |
**routing** | Option<**String**> | Routing value. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or a list of fields to return. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | List of fields to exclude from the returned _source field. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | List of fields to extract and return from the _source field. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mget_post

> mget_post(body, stored_fields, preference, realtime, refresh, routing, _source, _source_excludes, _source_includes)


Allows to get multiple documents in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |
**stored_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of stored fields to return. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**realtime** | Option<**bool**> | Specify whether to perform the operation in realtime or search mode. |  |
**refresh** | Option<**bool**> | Refresh the shard containing the document before performing the operation. |  |
**routing** | Option<**String**> | Routing value. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or a list of fields to return. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | List of fields to exclude from the returned _source field. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | List of fields to extract and return from the _source field. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mget_post_with_index

> mget_post_with_index(index, body, stored_fields, preference, realtime, refresh, routing, _source, _source_excludes, _source_includes)


Allows to get multiple documents in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Index name. | [required] |
**body** | **serde_json::Value** |  | [required] |
**stored_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of stored fields to return. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**realtime** | Option<**bool**> | Specify whether to perform the operation in realtime or search mode. |  |
**refresh** | Option<**bool**> | Refresh the shard containing the document before performing the operation. |  |
**routing** | Option<**String**> | Routing value. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or a list of fields to return. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | List of fields to exclude from the returned _source field. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | List of fields to extract and return from the _source field. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## msearch_get

> msearch_get(search_type, max_concurrent_searches, typed_keys, pre_filter_shard_size, max_concurrent_shard_requests, rest_total_hits_as_int, ccs_minimize_roundtrips)


Allows to execute several search operations in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_type** | Option<[**SearchTypeMulti**](.md)> | Search operation type. |  |
**max_concurrent_searches** | Option<**i32**> | Controls the maximum number of concurrent searches the multi search api will execute. |  |
**typed_keys** | Option<**bool**> | Specify whether aggregation and suggester names should be prefixed by their respective types in the response. |  |
**pre_filter_shard_size** | Option<**i32**> | Threshold that enforces a pre-filter round-trip to prefilter search shards based on query rewriting if the number of shards the search request expands to exceeds the threshold. This filter round-trip can limit the number of shards significantly if for instance a shard can not match any documents based on its rewrite method ie. if date filters are mandatory to match but the shard bounds and the query are disjoint. |  |
**max_concurrent_shard_requests** | Option<**i32**> | The number of concurrent shard requests each sub search executes concurrently per node. This value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests. |  |[default to 5]
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]
**ccs_minimize_roundtrips** | Option<**bool**> | Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## msearch_get_with_index

> msearch_get_with_index(index, search_type, max_concurrent_searches, typed_keys, pre_filter_shard_size, max_concurrent_shard_requests, rest_total_hits_as_int, ccs_minimize_roundtrips)


Allows to execute several search operations in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices to use as default. | [required] |
**search_type** | Option<[**SearchTypeMulti**](.md)> | Search operation type. |  |
**max_concurrent_searches** | Option<**i32**> | Controls the maximum number of concurrent searches the multi search api will execute. |  |
**typed_keys** | Option<**bool**> | Specify whether aggregation and suggester names should be prefixed by their respective types in the response. |  |
**pre_filter_shard_size** | Option<**i32**> | Threshold that enforces a pre-filter round-trip to prefilter search shards based on query rewriting if the number of shards the search request expands to exceeds the threshold. This filter round-trip can limit the number of shards significantly if for instance a shard can not match any documents based on its rewrite method ie. if date filters are mandatory to match but the shard bounds and the query are disjoint. |  |
**max_concurrent_shard_requests** | Option<**i32**> | The number of concurrent shard requests each sub search executes concurrently per node. This value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests. |  |[default to 5]
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]
**ccs_minimize_roundtrips** | Option<**bool**> | Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## msearch_post

> msearch_post(body, search_type, max_concurrent_searches, typed_keys, pre_filter_shard_size, max_concurrent_shard_requests, rest_total_hits_as_int, ccs_minimize_roundtrips)


Allows to execute several search operations in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |
**search_type** | Option<[**SearchTypeMulti**](.md)> | Search operation type. |  |
**max_concurrent_searches** | Option<**i32**> | Controls the maximum number of concurrent searches the multi search api will execute. |  |
**typed_keys** | Option<**bool**> | Specify whether aggregation and suggester names should be prefixed by their respective types in the response. |  |
**pre_filter_shard_size** | Option<**i32**> | Threshold that enforces a pre-filter round-trip to prefilter search shards based on query rewriting if the number of shards the search request expands to exceeds the threshold. This filter round-trip can limit the number of shards significantly if for instance a shard can not match any documents based on its rewrite method ie. if date filters are mandatory to match but the shard bounds and the query are disjoint. |  |
**max_concurrent_shard_requests** | Option<**i32**> | The number of concurrent shard requests each sub search executes concurrently per node. This value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests. |  |[default to 5]
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]
**ccs_minimize_roundtrips** | Option<**bool**> | Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## msearch_post_with_index

> msearch_post_with_index(index, body, search_type, max_concurrent_searches, typed_keys, pre_filter_shard_size, max_concurrent_shard_requests, rest_total_hits_as_int, ccs_minimize_roundtrips)


Allows to execute several search operations in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices to use as default. | [required] |
**body** | **serde_json::Value** |  | [required] |
**search_type** | Option<[**SearchTypeMulti**](.md)> | Search operation type. |  |
**max_concurrent_searches** | Option<**i32**> | Controls the maximum number of concurrent searches the multi search api will execute. |  |
**typed_keys** | Option<**bool**> | Specify whether aggregation and suggester names should be prefixed by their respective types in the response. |  |
**pre_filter_shard_size** | Option<**i32**> | Threshold that enforces a pre-filter round-trip to prefilter search shards based on query rewriting if the number of shards the search request expands to exceeds the threshold. This filter round-trip can limit the number of shards significantly if for instance a shard can not match any documents based on its rewrite method ie. if date filters are mandatory to match but the shard bounds and the query are disjoint. |  |
**max_concurrent_shard_requests** | Option<**i32**> | The number of concurrent shard requests each sub search executes concurrently per node. This value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests. |  |[default to 5]
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]
**ccs_minimize_roundtrips** | Option<**bool**> | Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## msearch_template_get

> msearch_template_get(search_type, typed_keys, max_concurrent_searches, rest_total_hits_as_int, ccs_minimize_roundtrips)


Allows to execute several search template operations in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_type** | Option<[**SearchTypeMulti**](.md)> | Search operation type. |  |
**typed_keys** | Option<**bool**> | Specify whether aggregation and suggester names should be prefixed by their respective types in the response. |  |
**max_concurrent_searches** | Option<**i32**> | Controls the maximum number of concurrent searches the multi search api will execute. |  |
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]
**ccs_minimize_roundtrips** | Option<**bool**> | Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## msearch_template_get_with_index

> msearch_template_get_with_index(index, search_type, typed_keys, max_concurrent_searches, rest_total_hits_as_int, ccs_minimize_roundtrips)


Allows to execute several search template operations in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices to use as default. | [required] |
**search_type** | Option<[**SearchTypeMulti**](.md)> | Search operation type. |  |
**typed_keys** | Option<**bool**> | Specify whether aggregation and suggester names should be prefixed by their respective types in the response. |  |
**max_concurrent_searches** | Option<**i32**> | Controls the maximum number of concurrent searches the multi search api will execute. |  |
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]
**ccs_minimize_roundtrips** | Option<**bool**> | Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## msearch_template_post

> msearch_template_post(body, search_type, typed_keys, max_concurrent_searches, rest_total_hits_as_int, ccs_minimize_roundtrips)


Allows to execute several search template operations in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |
**search_type** | Option<[**SearchTypeMulti**](.md)> | Search operation type. |  |
**typed_keys** | Option<**bool**> | Specify whether aggregation and suggester names should be prefixed by their respective types in the response. |  |
**max_concurrent_searches** | Option<**i32**> | Controls the maximum number of concurrent searches the multi search api will execute. |  |
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]
**ccs_minimize_roundtrips** | Option<**bool**> | Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## msearch_template_post_with_index

> msearch_template_post_with_index(index, body, search_type, typed_keys, max_concurrent_searches, rest_total_hits_as_int, ccs_minimize_roundtrips)


Allows to execute several search template operations in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices to use as default. | [required] |
**body** | **serde_json::Value** |  | [required] |
**search_type** | Option<[**SearchTypeMulti**](.md)> | Search operation type. |  |
**typed_keys** | Option<**bool**> | Specify whether aggregation and suggester names should be prefixed by their respective types in the response. |  |
**max_concurrent_searches** | Option<**i32**> | Controls the maximum number of concurrent searches the multi search api will execute. |  |
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]
**ccs_minimize_roundtrips** | Option<**bool**> | Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mtermvectors_get

> mtermvectors_get(ids, term_statistics, field_statistics, fields, offsets, positions, payloads, preference, routing, realtime, version, version_type)


Returns multiple termvectors in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | Comma-separated list of documents ids. You must define ids as parameter or set 'ids' or 'docs' in the request body. |  |
**term_statistics** | Option<**bool**> | Specifies if total term frequency and document frequency should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to false]
**field_statistics** | Option<**bool**> | Specifies if document count, sum of document frequencies and sum of total term frequencies should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to true]
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields to return. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |
**offsets** | Option<**bool**> | Specifies if term offsets should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to true]
**positions** | Option<**bool**> | Specifies if term positions should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to true]
**payloads** | Option<**bool**> | Specifies if term payloads should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to true]
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to random]
**routing** | Option<**String**> | Routing value. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |
**realtime** | Option<**bool**> | Specifies if requests are real-time as opposed to near-real-time. |  |[default to true]
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mtermvectors_get_with_index

> mtermvectors_get_with_index(index, ids, term_statistics, field_statistics, fields, offsets, positions, payloads, preference, routing, realtime, version, version_type)


Returns multiple termvectors in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | The index in which the document resides. | [required] |
**ids** | Option<[**Vec<String>**](String.md)> | Comma-separated list of documents ids. You must define ids as parameter or set 'ids' or 'docs' in the request body. |  |
**term_statistics** | Option<**bool**> | Specifies if total term frequency and document frequency should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to false]
**field_statistics** | Option<**bool**> | Specifies if document count, sum of document frequencies and sum of total term frequencies should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to true]
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields to return. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |
**offsets** | Option<**bool**> | Specifies if term offsets should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to true]
**positions** | Option<**bool**> | Specifies if term positions should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to true]
**payloads** | Option<**bool**> | Specifies if term payloads should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to true]
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to random]
**routing** | Option<**String**> | Routing value. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |
**realtime** | Option<**bool**> | Specifies if requests are real-time as opposed to near-real-time. |  |[default to true]
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mtermvectors_post

> mtermvectors_post(ids, term_statistics, field_statistics, fields, offsets, positions, payloads, preference, routing, realtime, version, version_type, body)


Returns multiple termvectors in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | Comma-separated list of documents ids. You must define ids as parameter or set 'ids' or 'docs' in the request body. |  |
**term_statistics** | Option<**bool**> | Specifies if total term frequency and document frequency should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to false]
**field_statistics** | Option<**bool**> | Specifies if document count, sum of document frequencies and sum of total term frequencies should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to true]
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields to return. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |
**offsets** | Option<**bool**> | Specifies if term offsets should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to true]
**positions** | Option<**bool**> | Specifies if term positions should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to true]
**payloads** | Option<**bool**> | Specifies if term payloads should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to true]
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to random]
**routing** | Option<**String**> | Routing value. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |
**realtime** | Option<**bool**> | Specifies if requests are real-time as opposed to near-real-time. |  |[default to true]
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mtermvectors_post_with_index

> mtermvectors_post_with_index(index, ids, term_statistics, field_statistics, fields, offsets, positions, payloads, preference, routing, realtime, version, version_type, body)


Returns multiple termvectors in one request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | The index in which the document resides. | [required] |
**ids** | Option<[**Vec<String>**](String.md)> | Comma-separated list of documents ids. You must define ids as parameter or set 'ids' or 'docs' in the request body. |  |
**term_statistics** | Option<**bool**> | Specifies if total term frequency and document frequency should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to false]
**field_statistics** | Option<**bool**> | Specifies if document count, sum of document frequencies and sum of total term frequencies should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to true]
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields to return. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |
**offsets** | Option<**bool**> | Specifies if term offsets should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to true]
**positions** | Option<**bool**> | Specifies if term positions should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to true]
**payloads** | Option<**bool**> | Specifies if term payloads should be returned. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to true]
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |[default to random]
**routing** | Option<**String**> | Routing value. Applies to all returned documents unless otherwise specified in body 'params' or 'docs'. |  |
**realtime** | Option<**bool**> | Specifies if requests are real-time as opposed to near-real-time. |  |[default to true]
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_hot_threads

> nodes_hot_threads(interval, snapshots, threads, ignore_idle_threads, r#type, timeout)


Returns information about hot threads on each node in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interval** | Option<**String**> | The interval for the second sampling of threads. |  |
**snapshots** | Option<**i32**> | Number of samples of thread stacktrace. |  |[default to 10]
**threads** | Option<**i32**> | Specify the number of threads to provide information for. |  |[default to 3]
**ignore_idle_threads** | Option<**bool**> | Don't show threads that are in known-idle places, such as waiting on a socket select or pulling from an empty task queue. |  |[default to true]
**r#type** | Option<[**SampleType**](.md)> | The type to sample. |  |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_hot_threads_deprecated

> nodes_hot_threads_deprecated(interval, snapshots, threads, ignore_idle_threads, r#type, timeout)


Returns information about hot threads on each node in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interval** | Option<**String**> | The interval for the second sampling of threads. |  |
**snapshots** | Option<**i32**> | Number of samples of thread stacktrace. |  |[default to 10]
**threads** | Option<**i32**> | Specify the number of threads to provide information for. |  |[default to 3]
**ignore_idle_threads** | Option<**bool**> | Don't show threads that are in known-idle places, such as waiting on a socket select or pulling from an empty task queue. |  |[default to true]
**r#type** | Option<[**SampleType**](.md)> | The type to sample. |  |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_hot_threads_deprecated_cluster

> nodes_hot_threads_deprecated_cluster(interval, snapshots, threads, ignore_idle_threads, r#type, timeout)


Returns information about hot threads on each node in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interval** | Option<**String**> | The interval for the second sampling of threads. |  |
**snapshots** | Option<**i32**> | Number of samples of thread stacktrace. |  |[default to 10]
**threads** | Option<**i32**> | Specify the number of threads to provide information for. |  |[default to 3]
**ignore_idle_threads** | Option<**bool**> | Don't show threads that are in known-idle places, such as waiting on a socket select or pulling from an empty task queue. |  |[default to true]
**r#type** | Option<[**SampleType**](.md)> | The type to sample. |  |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_hot_threads_deprecated_dash

> nodes_hot_threads_deprecated_dash(interval, snapshots, threads, ignore_idle_threads, r#type, timeout)


Returns information about hot threads on each node in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interval** | Option<**String**> | The interval for the second sampling of threads. |  |
**snapshots** | Option<**i32**> | Number of samples of thread stacktrace. |  |[default to 10]
**threads** | Option<**i32**> | Specify the number of threads to provide information for. |  |[default to 3]
**ignore_idle_threads** | Option<**bool**> | Don't show threads that are in known-idle places, such as waiting on a socket select or pulling from an empty task queue. |  |[default to true]
**r#type** | Option<[**SampleType**](.md)> | The type to sample. |  |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_hot_threads_with_node_id

> nodes_hot_threads_with_node_id(node_id, interval, snapshots, threads, ignore_idle_threads, r#type, timeout)


Returns information about hot threads on each node in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** | Comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes. | [required] |
**interval** | Option<**String**> | The interval for the second sampling of threads. |  |
**snapshots** | Option<**i32**> | Number of samples of thread stacktrace. |  |[default to 10]
**threads** | Option<**i32**> | Specify the number of threads to provide information for. |  |[default to 3]
**ignore_idle_threads** | Option<**bool**> | Don't show threads that are in known-idle places, such as waiting on a socket select or pulling from an empty task queue. |  |[default to true]
**r#type** | Option<[**SampleType**](.md)> | The type to sample. |  |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_hot_threads_with_node_id_deprecated

> nodes_hot_threads_with_node_id_deprecated(node_id, interval, snapshots, threads, ignore_idle_threads, r#type, timeout)


Returns information about hot threads on each node in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** | Comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes. | [required] |
**interval** | Option<**String**> | The interval for the second sampling of threads. |  |
**snapshots** | Option<**i32**> | Number of samples of thread stacktrace. |  |[default to 10]
**threads** | Option<**i32**> | Specify the number of threads to provide information for. |  |[default to 3]
**ignore_idle_threads** | Option<**bool**> | Don't show threads that are in known-idle places, such as waiting on a socket select or pulling from an empty task queue. |  |[default to true]
**r#type** | Option<[**SampleType**](.md)> | The type to sample. |  |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_hot_threads_with_node_id_deprecated_cluster

> nodes_hot_threads_with_node_id_deprecated_cluster(node_id, interval, snapshots, threads, ignore_idle_threads, r#type, timeout)


Returns information about hot threads on each node in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** | Comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes. | [required] |
**interval** | Option<**String**> | The interval for the second sampling of threads. |  |
**snapshots** | Option<**i32**> | Number of samples of thread stacktrace. |  |[default to 10]
**threads** | Option<**i32**> | Specify the number of threads to provide information for. |  |[default to 3]
**ignore_idle_threads** | Option<**bool**> | Don't show threads that are in known-idle places, such as waiting on a socket select or pulling from an empty task queue. |  |[default to true]
**r#type** | Option<[**SampleType**](.md)> | The type to sample. |  |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_hot_threads_with_node_id_deprecated_dash

> nodes_hot_threads_with_node_id_deprecated_dash(node_id, interval, snapshots, threads, ignore_idle_threads, r#type, timeout)


Returns information about hot threads on each node in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** | Comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes. | [required] |
**interval** | Option<**String**> | The interval for the second sampling of threads. |  |
**snapshots** | Option<**i32**> | Number of samples of thread stacktrace. |  |[default to 10]
**threads** | Option<**i32**> | Specify the number of threads to provide information for. |  |[default to 3]
**ignore_idle_threads** | Option<**bool**> | Don't show threads that are in known-idle places, such as waiting on a socket select or pulling from an empty task queue. |  |[default to true]
**r#type** | Option<[**SampleType**](.md)> | The type to sample. |  |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_info

> nodes_info(flat_settings, timeout)


Returns information about nodes in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_info_with_metric_node_id

> nodes_info_with_metric_node_id(node_id, metric, flat_settings, timeout)


Returns information about nodes in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** | Comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes. | [required] |
**metric** | **String** | Comma-separated list of metrics you wish returned. Leave empty to return all. | [required] |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_info_with_node_id

> nodes_info_with_node_id(node_id, flat_settings, timeout)


Returns information about nodes in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** | Comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes. | [required] |
**flat_settings** | Option<**bool**> | Return settings in flat format. |  |[default to false]
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_reload_secure_settings

> nodes_reload_secure_settings(timeout, body)


Reloads secure settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timeout** | Option<**String**> | Operation timeout. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_reload_secure_settings_with_node_id

> nodes_reload_secure_settings_with_node_id(node_id, timeout, body)


Reloads secure settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** | Comma-separated list of node IDs to span the reload/reinit call. Should stay empty because reloading usually involves all cluster nodes. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_stats

> nodes_stats(completion_fields, fielddata_fields, fields, groups, level, types, timeout, include_segment_file_sizes)


Returns statistical information about nodes in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**completion_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `suggest` index metric (supports wildcards). |  |
**fielddata_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` index metric (supports wildcards). |  |
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `completion` index metric (supports wildcards). |  |
**groups** | Option<[**Vec<String>**](String.md)> | Comma-separated list of search groups for `search` index metric. |  |
**level** | Option<[**NodesStatLevel**](.md)> | Return indices stats aggregated at index, node or shard level. |  |
**types** | Option<[**Vec<String>**](String.md)> | Comma-separated list of document types for the `indexing` index metric. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**include_segment_file_sizes** | Option<**bool**> | Whether to report the aggregated disk usage of each one of the Lucene index files (only applies if segment stats are requested). |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_stats_with_index_metric_metric

> nodes_stats_with_index_metric_metric(metric, index_metric, completion_fields, fielddata_fields, fields, groups, level, types, timeout, include_segment_file_sizes)


Returns statistical information about nodes in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric** | **String** | Limit the information returned to the specified metrics. | [required] |
**index_metric** | **String** | Limit the information returned for `indices` metric to the specific index metrics. Isn't used if `indices` (or `all`) metric isn't specified. | [required] |
**completion_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `suggest` index metric (supports wildcards). |  |
**fielddata_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` index metric (supports wildcards). |  |
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `completion` index metric (supports wildcards). |  |
**groups** | Option<[**Vec<String>**](String.md)> | Comma-separated list of search groups for `search` index metric. |  |
**level** | Option<[**NodesStatLevel**](.md)> | Return indices stats aggregated at index, node or shard level. |  |
**types** | Option<[**Vec<String>**](String.md)> | Comma-separated list of document types for the `indexing` index metric. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**include_segment_file_sizes** | Option<**bool**> | Whether to report the aggregated disk usage of each one of the Lucene index files (only applies if segment stats are requested). |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_stats_with_index_metric_metric_node_id

> nodes_stats_with_index_metric_metric_node_id(metric, index_metric, node_id, completion_fields, fielddata_fields, fields, groups, level, types, timeout, include_segment_file_sizes)


Returns statistical information about nodes in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric** | **String** | Limit the information returned to the specified metrics. | [required] |
**index_metric** | **String** | Limit the information returned for `indices` metric to the specific index metrics. Isn't used if `indices` (or `all`) metric isn't specified. | [required] |
**node_id** | **String** | Comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes. | [required] |
**completion_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `suggest` index metric (supports wildcards). |  |
**fielddata_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` index metric (supports wildcards). |  |
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `completion` index metric (supports wildcards). |  |
**groups** | Option<[**Vec<String>**](String.md)> | Comma-separated list of search groups for `search` index metric. |  |
**level** | Option<[**NodesStatLevel**](.md)> | Return indices stats aggregated at index, node or shard level. |  |
**types** | Option<[**Vec<String>**](String.md)> | Comma-separated list of document types for the `indexing` index metric. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**include_segment_file_sizes** | Option<**bool**> | Whether to report the aggregated disk usage of each one of the Lucene index files (only applies if segment stats are requested). |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_stats_with_metric

> nodes_stats_with_metric(metric, completion_fields, fielddata_fields, fields, groups, level, types, timeout, include_segment_file_sizes)


Returns statistical information about nodes in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric** | **String** | Limit the information returned to the specified metrics. | [required] |
**completion_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `suggest` index metric (supports wildcards). |  |
**fielddata_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` index metric (supports wildcards). |  |
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `completion` index metric (supports wildcards). |  |
**groups** | Option<[**Vec<String>**](String.md)> | Comma-separated list of search groups for `search` index metric. |  |
**level** | Option<[**NodesStatLevel**](.md)> | Return indices stats aggregated at index, node or shard level. |  |
**types** | Option<[**Vec<String>**](String.md)> | Comma-separated list of document types for the `indexing` index metric. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**include_segment_file_sizes** | Option<**bool**> | Whether to report the aggregated disk usage of each one of the Lucene index files (only applies if segment stats are requested). |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_stats_with_metric_node_id

> nodes_stats_with_metric_node_id(metric, node_id, completion_fields, fielddata_fields, fields, groups, level, types, timeout, include_segment_file_sizes)


Returns statistical information about nodes in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric** | **String** | Limit the information returned to the specified metrics. | [required] |
**node_id** | **String** | Comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes. | [required] |
**completion_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `suggest` index metric (supports wildcards). |  |
**fielddata_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` index metric (supports wildcards). |  |
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `completion` index metric (supports wildcards). |  |
**groups** | Option<[**Vec<String>**](String.md)> | Comma-separated list of search groups for `search` index metric. |  |
**level** | Option<[**NodesStatLevel**](.md)> | Return indices stats aggregated at index, node or shard level. |  |
**types** | Option<[**Vec<String>**](String.md)> | Comma-separated list of document types for the `indexing` index metric. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**include_segment_file_sizes** | Option<**bool**> | Whether to report the aggregated disk usage of each one of the Lucene index files (only applies if segment stats are requested). |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_stats_with_node_id

> nodes_stats_with_node_id(node_id, completion_fields, fielddata_fields, fields, groups, level, types, timeout, include_segment_file_sizes)


Returns statistical information about nodes in the cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** | Comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes. | [required] |
**completion_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `suggest` index metric (supports wildcards). |  |
**fielddata_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` index metric (supports wildcards). |  |
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields for `fielddata` and `completion` index metric (supports wildcards). |  |
**groups** | Option<[**Vec<String>**](String.md)> | Comma-separated list of search groups for `search` index metric. |  |
**level** | Option<[**NodesStatLevel**](.md)> | Return indices stats aggregated at index, node or shard level. |  |
**types** | Option<[**Vec<String>**](String.md)> | Comma-separated list of document types for the `indexing` index metric. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**include_segment_file_sizes** | Option<**bool**> | Whether to report the aggregated disk usage of each one of the Lucene index files (only applies if segment stats are requested). |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_usage

> nodes_usage(timeout)


Returns low-level information about REST actions usage on nodes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_usage_with_metric

> nodes_usage_with_metric(metric, timeout)


Returns low-level information about REST actions usage on nodes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric** | **String** | Limit the information returned to the specified metrics. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_usage_with_metric_node_id

> nodes_usage_with_metric_node_id(metric, node_id, timeout)


Returns low-level information about REST actions usage on nodes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric** | **String** | Limit the information returned to the specified metrics. | [required] |
**node_id** | **String** | Comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## nodes_usage_with_node_id

> nodes_usage_with_node_id(node_id, timeout)


Returns low-level information about REST actions usage on nodes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_id** | **String** | Comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes. | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_action_group

> crate::models::PatchActionGroupResponseContent patch_action_group(action_group, patch_operation)


Updates individual attributes of an action group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_group** | **String** |  | [required] |
**patch_operation** | [**Vec<crate::models::PatchOperation>**](PatchOperation.md) |  | [required] |

### Return type

[**crate::models::PatchActionGroupResponseContent**](PatchActionGroupResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_action_groups

> crate::models::PatchActionGroupsResponseContent patch_action_groups(patch_operation)


Creates, updates, or deletes multiple action groups in a single call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patch_operation** | [**Vec<crate::models::PatchOperation>**](PatchOperation.md) |  | [required] |

### Return type

[**crate::models::PatchActionGroupsResponseContent**](PatchActionGroupsResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_audit_configuration

> patch_audit_configuration(patch_operation)


A PATCH call is used to update specified fields in the audit configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patch_operation** | [**Vec<crate::models::PatchOperation>**](PatchOperation.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_configuration

> crate::models::PatchConfigurationResponseContent patch_configuration(patch_operation)


A PATCH call is used to update the existing configuration using the REST API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patch_operation** | [**Vec<crate::models::PatchOperation>**](PatchOperation.md) |  | [required] |

### Return type

[**crate::models::PatchConfigurationResponseContent**](PatchConfigurationResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_distinguished_names

> crate::models::PatchDistinguishedNamesResponseContent patch_distinguished_names(patch_operation)


Bulk update of distinguished names.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patch_operation** | [**Vec<crate::models::PatchOperation>**](PatchOperation.md) |  | [required] |

### Return type

[**crate::models::PatchDistinguishedNamesResponseContent**](PatchDistinguishedNamesResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_role

> crate::models::PatchRoleResponseContent patch_role(role, patch_operation)


Updates individual attributes of a role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** |  | [required] |
**patch_operation** | [**Vec<crate::models::PatchOperation>**](PatchOperation.md) |  | [required] |

### Return type

[**crate::models::PatchRoleResponseContent**](PatchRoleResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_role_mapping

> crate::models::PatchRoleMappingResponseContent patch_role_mapping(role, patch_operation)


Updates individual attributes of a role mapping.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | **String** |  | [required] |
**patch_operation** | [**Vec<crate::models::PatchOperation>**](PatchOperation.md) |  | [required] |

### Return type

[**crate::models::PatchRoleMappingResponseContent**](PatchRoleMappingResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_role_mappings

> crate::models::PatchRoleMappingsResponseContent patch_role_mappings(patch_operation)


Creates or updates multiple role mappings in a single call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patch_operation** | [**Vec<crate::models::PatchOperation>**](PatchOperation.md) |  | [required] |

### Return type

[**crate::models::PatchRoleMappingsResponseContent**](PatchRoleMappingsResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_roles

> crate::models::PatchRolesResponseContent patch_roles(patch_operation)


Creates, updates, or deletes multiple roles in a single call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patch_operation** | [**Vec<crate::models::PatchOperation>**](PatchOperation.md) |  | [required] |

### Return type

[**crate::models::PatchRolesResponseContent**](PatchRolesResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_tenant

> crate::models::PatchTenantResponseContent patch_tenant(tenant, patch_operation)


Add, delete, or modify a single tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**patch_operation** | [**Vec<crate::models::PatchOperation>**](PatchOperation.md) |  | [required] |

### Return type

[**crate::models::PatchTenantResponseContent**](PatchTenantResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_tenants

> crate::models::PatchTenantsResponseContent patch_tenants(patch_operation)


Add, delete, or modify multiple tenants in a single call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patch_operation** | [**Vec<crate::models::PatchOperation>**](PatchOperation.md) |  | [required] |

### Return type

[**crate::models::PatchTenantsResponseContent**](PatchTenantsResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user

> crate::models::PatchUserResponseContent patch_user(username, patch_operation)


Updates individual attributes of an internal user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**patch_operation** | [**Vec<crate::models::PatchOperation>**](PatchOperation.md) |  | [required] |

### Return type

[**crate::models::PatchUserResponseContent**](PatchUserResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_users

> crate::models::PatchUsersResponseContent patch_users(patch_operation)


Creates, updates, or deletes multiple internal users in a single call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patch_operation** | [**Vec<crate::models::PatchOperation>**](PatchOperation.md) |  | [required] |

### Return type

[**crate::models::PatchUsersResponseContent**](PatchUsersResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ping

> ping()


Returns whether the cluster is running.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_script_post

> put_script_post(id, body, timeout, master_timeout, cluster_manager_timeout)


Creates or updates a script.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Script ID. | [required] |
**body** | **serde_json::Value** |  | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_script_post_with_context

> put_script_post_with_context(id, context, body, timeout, master_timeout, cluster_manager_timeout)


Creates or updates a script.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Script ID. | [required] |
**context** | **String** | Script context. | [required] |
**body** | **serde_json::Value** |  | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_script_put

> put_script_put(id, body, timeout, master_timeout, cluster_manager_timeout)


Creates or updates a script.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Script ID. | [required] |
**body** | **serde_json::Value** |  | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_script_put_with_context

> put_script_put_with_context(id, context, body, timeout, master_timeout, cluster_manager_timeout)


Creates or updates a script.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Script ID. | [required] |
**context** | **String** | Script context. | [required] |
**body** | **serde_json::Value** |  | [required] |
**timeout** | Option<**String**> | Operation timeout. |  |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rank_eval_get

> rank_eval_get(ignore_unavailable, allow_no_indices, expand_wildcards, search_type)


Allows to evaluate the quality of ranked search results over a set of typical search queries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**search_type** | Option<[**SearchType**](.md)> | Search operation type. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rank_eval_get_with_index

> rank_eval_get_with_index(index, ignore_unavailable, allow_no_indices, expand_wildcards, search_type)


Allows to evaluate the quality of ranked search results over a set of typical search queries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**search_type** | Option<[**SearchType**](.md)> | Search operation type. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rank_eval_post

> rank_eval_post(body, ignore_unavailable, allow_no_indices, expand_wildcards, search_type)


Allows to evaluate the quality of ranked search results over a set of typical search queries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**search_type** | Option<[**SearchType**](.md)> | Search operation type. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rank_eval_post_with_index

> rank_eval_post_with_index(index, body, ignore_unavailable, allow_no_indices, expand_wildcards, search_type)


Allows to evaluate the quality of ranked search results over a set of typical search queries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**body** | **serde_json::Value** |  | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**search_type** | Option<[**SearchType**](.md)> | Search operation type. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reindex

> reindex(body, refresh, timeout, wait_for_active_shards, wait_for_completion, requests_per_second, scroll, slices, max_docs)


Allows to copy documents from one index to another, optionally filtering the source documents by a query, changing the destination index settings, or fetching the documents from a remote cluster.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |
**refresh** | Option<**bool**> | Should the affected indexes be refreshed?. |  |
**timeout** | Option<**String**> | Time each individual bulk request should wait for shards that are unavailable. |  |[default to 1m]
**wait_for_active_shards** | Option<**String**> | Sets the number of shard copies that must be active before proceeding with the operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1). |  |[default to 1]
**wait_for_completion** | Option<**bool**> | Should this request wait until the operation has completed before returning. |  |[default to true]
**requests_per_second** | Option<**i32**> | The throttle for this request in sub-requests per second. -1 means no throttle. |  |[default to 0]
**scroll** | Option<**String**> | Specify how long a consistent view of the index should be maintained for scrolled search. |  |
**slices** | Option<**String**> | The number of slices this task should be divided into. Defaults to 1, meaning the task isn't sliced into subtasks. Can be set to `auto`. |  |[default to 1]
**max_docs** | Option<**i32**> | Maximum number of documents to process (default: all documents). |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reindex_rethrottle

> reindex_rethrottle(task_id, requests_per_second)


Changes the number of requests per second for a particular Reindex operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | The task id to rethrottle. | [required] |
**requests_per_second** | **i32** | The throttle for this request in sub-requests per second. -1 means no throttle. | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reload_http_certificates

> crate::models::ReloadHttpCertificatesResponseContent reload_http_certificates()


Reload HTTP layer communication certificates.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ReloadHttpCertificatesResponseContent**](ReloadHttpCertificatesResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reload_transport_certificates

> crate::models::ReloadTransportCertificatesResponseContent reload_transport_certificates()


Reload transport layer communication certificates.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ReloadTransportCertificatesResponseContent**](ReloadTransportCertificatesResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remote_store_restore

> crate::models::RemoteStoreRestoreResponseContent remote_store_restore(remote_store_restore_body_params, cluster_manager_timeout, wait_for_completion)


Restores from remote store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remote_store_restore_body_params** | [**RemoteStoreRestoreBodyParams**](RemoteStoreRestoreBodyParams.md) |  | [required] |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**wait_for_completion** | Option<**bool**> | Should this request wait until the operation has completed before returning. |  |[default to false]

### Return type

[**crate::models::RemoteStoreRestoreResponseContent**](RemoteStoreRestoreResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## render_search_template_get

> render_search_template_get()


Allows to use the Mustache language to pre-render a search definition.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## render_search_template_get_with_id

> render_search_template_get_with_id(id)


Allows to use the Mustache language to pre-render a search definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id of the stored search template. | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## render_search_template_post

> render_search_template_post(body)


Allows to use the Mustache language to pre-render a search definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## render_search_template_post_with_id

> render_search_template_post_with_id(id, body)


Allows to use the Mustache language to pre-render a search definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The id of the stored search template. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scripts_painless_execute_get

> scripts_painless_execute_get()


Allows an arbitrary script to be executed and a result to be returned.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scripts_painless_execute_post

> scripts_painless_execute_post(body)


Allows an arbitrary script to be executed and a result to be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scroll_get

> scroll_get(scroll, scroll_id, rest_total_hits_as_int)


Allows to retrieve a large numbers of results from a single search request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scroll** | Option<**String**> | Specify how long a consistent view of the index should be maintained for scrolled search. |  |
**scroll_id** | Option<**String**> | Scroll ID. |  |
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scroll_get_with_scroll_id

> scroll_get_with_scroll_id(scroll_id, scroll, scroll_id2, rest_total_hits_as_int)


Allows to retrieve a large numbers of results from a single search request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scroll_id** | **String** | Scroll ID. | [required] |
**scroll** | Option<**String**> | Specify how long a consistent view of the index should be maintained for scrolled search. |  |
**scroll_id2** | Option<**String**> | Scroll ID. |  |
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scroll_post

> scroll_post(scroll, scroll_id, rest_total_hits_as_int, body)


Allows to retrieve a large numbers of results from a single search request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scroll** | Option<**String**> | Specify how long a consistent view of the index should be maintained for scrolled search. |  |
**scroll_id** | Option<**String**> | Scroll ID. |  |
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scroll_post_with_scroll_id

> scroll_post_with_scroll_id(scroll_id, scroll, scroll_id2, rest_total_hits_as_int, body)


Allows to retrieve a large numbers of results from a single search request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scroll_id** | **String** | Scroll ID. | [required] |
**scroll** | Option<**String**> | Specify how long a consistent view of the index should be maintained for scrolled search. |  |
**scroll_id2** | Option<**String**> | Scroll ID. |  |
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_get

> crate::models::SearchGetResponseContent search_get(analyzer, analyze_wildcard, ccs_minimize_roundtrips, default_operator, df, explain, stored_fields, docvalue_fields, from, ignore_unavailable, ignore_throttled, allow_no_indices, expand_wildcards, lenient, preference, q, routing, scroll, search_type, size, sort, _source, _source_excludes, _source_includes, terminate_after, stats, suggest_field, suggest_mode, suggest_size, suggest_text, timeout, track_scores, track_total_hits, allow_partial_search_results, typed_keys, version, seq_no_primary_term, request_cache, batched_reduce_size, max_concurrent_shard_requests, pre_filter_shard_size, rest_total_hits_as_int)


Returns results matching a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analyzer** | Option<**String**> | The analyzer to use for the query string. |  |
**analyze_wildcard** | Option<**bool**> | Specify whether wildcard and prefix queries should be analyzed. |  |[default to false]
**ccs_minimize_roundtrips** | Option<**bool**> | Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution. |  |[default to true]
**default_operator** | Option<[**DefaultOperator**](.md)> | The default operator for query string query (AND or OR). |  |
**df** | Option<**String**> | The field to use as default where no field prefix is given in the query string. |  |
**explain** | Option<**bool**> | Specify whether to return detailed information about score computation as part of a hit. |  |
**stored_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of stored fields to return. |  |
**docvalue_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields to return as the docvalue representation of a field for each hit. |  |
**from** | Option<**i32**> | Starting offset. |  |[default to 0]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**ignore_throttled** | Option<**bool**> | Whether specified concrete, expanded or aliased indices should be ignored when throttled. |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**lenient** | Option<**bool**> | Specify whether format-based query failures (such as providing text to a numeric field) should be ignored. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**q** | Option<**String**> | Query in the Lucene query string syntax. |  |
**routing** | Option<[**Vec<String>**](String.md)> | Comma-separated list of specific routing values. |  |
**scroll** | Option<**String**> | Specify how long a consistent view of the index should be maintained for scrolled search. |  |
**search_type** | Option<[**SearchType**](.md)> | Search operation type. |  |
**size** | Option<**i32**> | Number of hits to return. |  |[default to 10]
**sort** | Option<[**Vec<String>**](String.md)> | Comma-separated list of <field>:<direction> pairs. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or a list of fields to return. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | List of fields to exclude from the returned _source field. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | List of fields to extract and return from the _source field. |  |
**terminate_after** | Option<**i32**> | The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early. |  |
**stats** | Option<[**Vec<String>**](String.md)> | Specific 'tag' of the request for logging and statistical purposes. |  |
**suggest_field** | Option<**String**> | Specify which field to use for suggestions. |  |
**suggest_mode** | Option<[**SuggestMode**](.md)> | Specify suggest mode. |  |
**suggest_size** | Option<**i32**> | How many suggestions to return in response. |  |
**suggest_text** | Option<**String**> | The source text for which the suggestions should be returned. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**track_scores** | Option<**bool**> | Whether to calculate and return scores even if they are not used for sorting. |  |
**track_total_hits** | Option<**bool**> | Indicate if the number of documents that match the query should be tracked. |  |
**allow_partial_search_results** | Option<**bool**> | Indicate if an error should be returned if there is a partial search failure or timeout. |  |[default to true]
**typed_keys** | Option<**bool**> | Specify whether aggregation and suggester names should be prefixed by their respective types in the response. |  |
**version** | Option<**bool**> | Whether to return document version as part of a hit. |  |
**seq_no_primary_term** | Option<**bool**> | Specify whether to return sequence number and primary term of the last modification of each hit. |  |
**request_cache** | Option<**bool**> | Specify if request cache should be used for this request or not, defaults to index level setting. |  |
**batched_reduce_size** | Option<**i32**> | The number of shard results that should be reduced at once on the coordinating node. This value should be used as a protection mechanism to reduce the memory overhead per search request if the potential number of shards in the request can be large. |  |[default to 512]
**max_concurrent_shard_requests** | Option<**i32**> | The number of concurrent shard requests per node this search executes concurrently. This value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests. |  |[default to 5]
**pre_filter_shard_size** | Option<**i32**> | Threshold that enforces a pre-filter round-trip to prefilter search shards based on query rewriting if the number of shards the search request expands to exceeds the threshold. This filter round-trip can limit the number of shards significantly if for instance a shard can not match any documents based on its rewrite method ie. if date filters are mandatory to match but the shard bounds and the query are disjoint. |  |
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]

### Return type

[**crate::models::SearchGetResponseContent**](Search_GetResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_get_with_index

> crate::models::SearchGetWithIndexResponseContent search_get_with_index(index, analyzer, analyze_wildcard, ccs_minimize_roundtrips, default_operator, df, explain, stored_fields, docvalue_fields, from, ignore_unavailable, ignore_throttled, allow_no_indices, expand_wildcards, lenient, preference, q, routing, scroll, search_type, size, sort, _source, _source_excludes, _source_includes, terminate_after, stats, suggest_field, suggest_mode, suggest_size, suggest_text, timeout, track_scores, track_total_hits, allow_partial_search_results, typed_keys, version, seq_no_primary_term, request_cache, batched_reduce_size, max_concurrent_shard_requests, pre_filter_shard_size, rest_total_hits_as_int)


Returns results matching a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**analyzer** | Option<**String**> | The analyzer to use for the query string. |  |
**analyze_wildcard** | Option<**bool**> | Specify whether wildcard and prefix queries should be analyzed. |  |[default to false]
**ccs_minimize_roundtrips** | Option<**bool**> | Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution. |  |[default to true]
**default_operator** | Option<[**DefaultOperator**](.md)> | The default operator for query string query (AND or OR). |  |
**df** | Option<**String**> | The field to use as default where no field prefix is given in the query string. |  |
**explain** | Option<**bool**> | Specify whether to return detailed information about score computation as part of a hit. |  |
**stored_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of stored fields to return. |  |
**docvalue_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields to return as the docvalue representation of a field for each hit. |  |
**from** | Option<**i32**> | Starting offset. |  |[default to 0]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**ignore_throttled** | Option<**bool**> | Whether specified concrete, expanded or aliased indices should be ignored when throttled. |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**lenient** | Option<**bool**> | Specify whether format-based query failures (such as providing text to a numeric field) should be ignored. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**q** | Option<**String**> | Query in the Lucene query string syntax. |  |
**routing** | Option<[**Vec<String>**](String.md)> | Comma-separated list of specific routing values. |  |
**scroll** | Option<**String**> | Specify how long a consistent view of the index should be maintained for scrolled search. |  |
**search_type** | Option<[**SearchType**](.md)> | Search operation type. |  |
**size** | Option<**i32**> | Number of hits to return. |  |[default to 10]
**sort** | Option<[**Vec<String>**](String.md)> | Comma-separated list of <field>:<direction> pairs. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or a list of fields to return. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | List of fields to exclude from the returned _source field. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | List of fields to extract and return from the _source field. |  |
**terminate_after** | Option<**i32**> | The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early. |  |
**stats** | Option<[**Vec<String>**](String.md)> | Specific 'tag' of the request for logging and statistical purposes. |  |
**suggest_field** | Option<**String**> | Specify which field to use for suggestions. |  |
**suggest_mode** | Option<[**SuggestMode**](.md)> | Specify suggest mode. |  |
**suggest_size** | Option<**i32**> | How many suggestions to return in response. |  |
**suggest_text** | Option<**String**> | The source text for which the suggestions should be returned. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**track_scores** | Option<**bool**> | Whether to calculate and return scores even if they are not used for sorting. |  |
**track_total_hits** | Option<**bool**> | Indicate if the number of documents that match the query should be tracked. |  |
**allow_partial_search_results** | Option<**bool**> | Indicate if an error should be returned if there is a partial search failure or timeout. |  |[default to true]
**typed_keys** | Option<**bool**> | Specify whether aggregation and suggester names should be prefixed by their respective types in the response. |  |
**version** | Option<**bool**> | Whether to return document version as part of a hit. |  |
**seq_no_primary_term** | Option<**bool**> | Specify whether to return sequence number and primary term of the last modification of each hit. |  |
**request_cache** | Option<**bool**> | Specify if request cache should be used for this request or not, defaults to index level setting. |  |
**batched_reduce_size** | Option<**i32**> | The number of shard results that should be reduced at once on the coordinating node. This value should be used as a protection mechanism to reduce the memory overhead per search request if the potential number of shards in the request can be large. |  |[default to 512]
**max_concurrent_shard_requests** | Option<**i32**> | The number of concurrent shard requests per node this search executes concurrently. This value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests. |  |[default to 5]
**pre_filter_shard_size** | Option<**i32**> | Threshold that enforces a pre-filter round-trip to prefilter search shards based on query rewriting if the number of shards the search request expands to exceeds the threshold. This filter round-trip can limit the number of shards significantly if for instance a shard can not match any documents based on its rewrite method ie. if date filters are mandatory to match but the shard bounds and the query are disjoint. |  |
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]

### Return type

[**crate::models::SearchGetWithIndexResponseContent**](Search_Get_WithIndexResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_post

> crate::models::SearchPostResponseContent search_post(analyzer, analyze_wildcard, ccs_minimize_roundtrips, default_operator, df, explain, stored_fields, docvalue_fields, from, ignore_unavailable, ignore_throttled, allow_no_indices, expand_wildcards, lenient, preference, q, routing, scroll, search_type, size, sort, _source, _source_excludes, _source_includes, terminate_after, stats, suggest_field, suggest_mode, suggest_size, suggest_text, timeout, track_scores, track_total_hits, allow_partial_search_results, typed_keys, version, seq_no_primary_term, request_cache, batched_reduce_size, max_concurrent_shard_requests, pre_filter_shard_size, rest_total_hits_as_int, search_body_params)


Returns results matching a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analyzer** | Option<**String**> | The analyzer to use for the query string. |  |
**analyze_wildcard** | Option<**bool**> | Specify whether wildcard and prefix queries should be analyzed. |  |[default to false]
**ccs_minimize_roundtrips** | Option<**bool**> | Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution. |  |[default to true]
**default_operator** | Option<[**DefaultOperator**](.md)> | The default operator for query string query (AND or OR). |  |
**df** | Option<**String**> | The field to use as default where no field prefix is given in the query string. |  |
**explain** | Option<**bool**> | Specify whether to return detailed information about score computation as part of a hit. |  |
**stored_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of stored fields to return. |  |
**docvalue_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields to return as the docvalue representation of a field for each hit. |  |
**from** | Option<**i32**> | Starting offset. |  |[default to 0]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**ignore_throttled** | Option<**bool**> | Whether specified concrete, expanded or aliased indices should be ignored when throttled. |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**lenient** | Option<**bool**> | Specify whether format-based query failures (such as providing text to a numeric field) should be ignored. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**q** | Option<**String**> | Query in the Lucene query string syntax. |  |
**routing** | Option<[**Vec<String>**](String.md)> | Comma-separated list of specific routing values. |  |
**scroll** | Option<**String**> | Specify how long a consistent view of the index should be maintained for scrolled search. |  |
**search_type** | Option<[**SearchType**](.md)> | Search operation type. |  |
**size** | Option<**i32**> | Number of hits to return. |  |[default to 10]
**sort** | Option<[**Vec<String>**](String.md)> | Comma-separated list of <field>:<direction> pairs. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or a list of fields to return. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | List of fields to exclude from the returned _source field. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | List of fields to extract and return from the _source field. |  |
**terminate_after** | Option<**i32**> | The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early. |  |
**stats** | Option<[**Vec<String>**](String.md)> | Specific 'tag' of the request for logging and statistical purposes. |  |
**suggest_field** | Option<**String**> | Specify which field to use for suggestions. |  |
**suggest_mode** | Option<[**SuggestMode**](.md)> | Specify suggest mode. |  |
**suggest_size** | Option<**i32**> | How many suggestions to return in response. |  |
**suggest_text** | Option<**String**> | The source text for which the suggestions should be returned. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**track_scores** | Option<**bool**> | Whether to calculate and return scores even if they are not used for sorting. |  |
**track_total_hits** | Option<**bool**> | Indicate if the number of documents that match the query should be tracked. |  |
**allow_partial_search_results** | Option<**bool**> | Indicate if an error should be returned if there is a partial search failure or timeout. |  |[default to true]
**typed_keys** | Option<**bool**> | Specify whether aggregation and suggester names should be prefixed by their respective types in the response. |  |
**version** | Option<**bool**> | Whether to return document version as part of a hit. |  |
**seq_no_primary_term** | Option<**bool**> | Specify whether to return sequence number and primary term of the last modification of each hit. |  |
**request_cache** | Option<**bool**> | Specify if request cache should be used for this request or not, defaults to index level setting. |  |
**batched_reduce_size** | Option<**i32**> | The number of shard results that should be reduced at once on the coordinating node. This value should be used as a protection mechanism to reduce the memory overhead per search request if the potential number of shards in the request can be large. |  |[default to 512]
**max_concurrent_shard_requests** | Option<**i32**> | The number of concurrent shard requests per node this search executes concurrently. This value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests. |  |[default to 5]
**pre_filter_shard_size** | Option<**i32**> | Threshold that enforces a pre-filter round-trip to prefilter search shards based on query rewriting if the number of shards the search request expands to exceeds the threshold. This filter round-trip can limit the number of shards significantly if for instance a shard can not match any documents based on its rewrite method ie. if date filters are mandatory to match but the shard bounds and the query are disjoint. |  |
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]
**search_body_params** | Option<[**SearchBodyParams**](SearchBodyParams.md)> |  |  |

### Return type

[**crate::models::SearchPostResponseContent**](Search_PostResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_post_with_index

> crate::models::SearchPostWithIndexResponseContent search_post_with_index(index, analyzer, analyze_wildcard, ccs_minimize_roundtrips, default_operator, df, explain, stored_fields, docvalue_fields, from, ignore_unavailable, ignore_throttled, allow_no_indices, expand_wildcards, lenient, preference, q, routing, scroll, search_type, size, sort, _source, _source_excludes, _source_includes, terminate_after, stats, suggest_field, suggest_mode, suggest_size, suggest_text, timeout, track_scores, track_total_hits, allow_partial_search_results, typed_keys, version, seq_no_primary_term, request_cache, batched_reduce_size, max_concurrent_shard_requests, pre_filter_shard_size, rest_total_hits_as_int, search_body_params)


Returns results matching a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**analyzer** | Option<**String**> | The analyzer to use for the query string. |  |
**analyze_wildcard** | Option<**bool**> | Specify whether wildcard and prefix queries should be analyzed. |  |[default to false]
**ccs_minimize_roundtrips** | Option<**bool**> | Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution. |  |[default to true]
**default_operator** | Option<[**DefaultOperator**](.md)> | The default operator for query string query (AND or OR). |  |
**df** | Option<**String**> | The field to use as default where no field prefix is given in the query string. |  |
**explain** | Option<**bool**> | Specify whether to return detailed information about score computation as part of a hit. |  |
**stored_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of stored fields to return. |  |
**docvalue_fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields to return as the docvalue representation of a field for each hit. |  |
**from** | Option<**i32**> | Starting offset. |  |[default to 0]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**ignore_throttled** | Option<**bool**> | Whether specified concrete, expanded or aliased indices should be ignored when throttled. |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**lenient** | Option<**bool**> | Specify whether format-based query failures (such as providing text to a numeric field) should be ignored. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**q** | Option<**String**> | Query in the Lucene query string syntax. |  |
**routing** | Option<[**Vec<String>**](String.md)> | Comma-separated list of specific routing values. |  |
**scroll** | Option<**String**> | Specify how long a consistent view of the index should be maintained for scrolled search. |  |
**search_type** | Option<[**SearchType**](.md)> | Search operation type. |  |
**size** | Option<**i32**> | Number of hits to return. |  |[default to 10]
**sort** | Option<[**Vec<String>**](String.md)> | Comma-separated list of <field>:<direction> pairs. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or a list of fields to return. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | List of fields to exclude from the returned _source field. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | List of fields to extract and return from the _source field. |  |
**terminate_after** | Option<**i32**> | The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early. |  |
**stats** | Option<[**Vec<String>**](String.md)> | Specific 'tag' of the request for logging and statistical purposes. |  |
**suggest_field** | Option<**String**> | Specify which field to use for suggestions. |  |
**suggest_mode** | Option<[**SuggestMode**](.md)> | Specify suggest mode. |  |
**suggest_size** | Option<**i32**> | How many suggestions to return in response. |  |
**suggest_text** | Option<**String**> | The source text for which the suggestions should be returned. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**track_scores** | Option<**bool**> | Whether to calculate and return scores even if they are not used for sorting. |  |
**track_total_hits** | Option<**bool**> | Indicate if the number of documents that match the query should be tracked. |  |
**allow_partial_search_results** | Option<**bool**> | Indicate if an error should be returned if there is a partial search failure or timeout. |  |[default to true]
**typed_keys** | Option<**bool**> | Specify whether aggregation and suggester names should be prefixed by their respective types in the response. |  |
**version** | Option<**bool**> | Whether to return document version as part of a hit. |  |
**seq_no_primary_term** | Option<**bool**> | Specify whether to return sequence number and primary term of the last modification of each hit. |  |
**request_cache** | Option<**bool**> | Specify if request cache should be used for this request or not, defaults to index level setting. |  |
**batched_reduce_size** | Option<**i32**> | The number of shard results that should be reduced at once on the coordinating node. This value should be used as a protection mechanism to reduce the memory overhead per search request if the potential number of shards in the request can be large. |  |[default to 512]
**max_concurrent_shard_requests** | Option<**i32**> | The number of concurrent shard requests per node this search executes concurrently. This value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests. |  |[default to 5]
**pre_filter_shard_size** | Option<**i32**> | Threshold that enforces a pre-filter round-trip to prefilter search shards based on query rewriting if the number of shards the search request expands to exceeds the threshold. This filter round-trip can limit the number of shards significantly if for instance a shard can not match any documents based on its rewrite method ie. if date filters are mandatory to match but the shard bounds and the query are disjoint. |  |
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]
**search_body_params** | Option<[**SearchBodyParams**](SearchBodyParams.md)> |  |  |

### Return type

[**crate::models::SearchPostWithIndexResponseContent**](Search_Post_WithIndexResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_shards_get

> search_shards_get(preference, routing, local, ignore_unavailable, allow_no_indices, expand_wildcards)


Returns information about the indices and shards that a search request would be executed against.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**routing** | Option<**String**> | Routing value. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_shards_get_with_index

> search_shards_get_with_index(index, preference, routing, local, ignore_unavailable, allow_no_indices, expand_wildcards)


Returns information about the indices and shards that a search request would be executed against.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**routing** | Option<**String**> | Routing value. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_shards_post

> search_shards_post(preference, routing, local, ignore_unavailable, allow_no_indices, expand_wildcards)


Returns information about the indices and shards that a search request would be executed against.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**routing** | Option<**String**> | Routing value. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_shards_post_with_index

> search_shards_post_with_index(index, preference, routing, local, ignore_unavailable, allow_no_indices, expand_wildcards)


Returns information about the indices and shards that a search request would be executed against.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**routing** | Option<**String**> | Routing value. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_template_get

> search_template_get(ignore_unavailable, ignore_throttled, allow_no_indices, expand_wildcards, preference, routing, scroll, search_type, explain, profile, typed_keys, rest_total_hits_as_int, ccs_minimize_roundtrips)


Allows to use the Mustache language to pre-render a search definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**ignore_throttled** | Option<**bool**> | Whether specified concrete, expanded or aliased indices should be ignored when throttled. |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**routing** | Option<[**Vec<String>**](String.md)> | Comma-separated list of specific routing values. |  |
**scroll** | Option<**String**> | Specify how long a consistent view of the index should be maintained for scrolled search. |  |
**search_type** | Option<[**SearchTypeMulti**](.md)> | Search operation type. |  |
**explain** | Option<**bool**> | Specify whether to return detailed information about score computation as part of a hit. |  |
**profile** | Option<**bool**> | Specify whether to profile the query execution. |  |
**typed_keys** | Option<**bool**> | Specify whether aggregation and suggester names should be prefixed by their respective types in the response. |  |
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]
**ccs_minimize_roundtrips** | Option<**bool**> | Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_template_get_with_index

> search_template_get_with_index(index, ignore_unavailable, ignore_throttled, allow_no_indices, expand_wildcards, preference, routing, scroll, search_type, explain, profile, typed_keys, rest_total_hits_as_int, ccs_minimize_roundtrips)


Allows to use the Mustache language to pre-render a search definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**ignore_throttled** | Option<**bool**> | Whether specified concrete, expanded or aliased indices should be ignored when throttled. |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**routing** | Option<[**Vec<String>**](String.md)> | Comma-separated list of specific routing values. |  |
**scroll** | Option<**String**> | Specify how long a consistent view of the index should be maintained for scrolled search. |  |
**search_type** | Option<[**SearchTypeMulti**](.md)> | Search operation type. |  |
**explain** | Option<**bool**> | Specify whether to return detailed information about score computation as part of a hit. |  |
**profile** | Option<**bool**> | Specify whether to profile the query execution. |  |
**typed_keys** | Option<**bool**> | Specify whether aggregation and suggester names should be prefixed by their respective types in the response. |  |
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]
**ccs_minimize_roundtrips** | Option<**bool**> | Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_template_post

> search_template_post(body, ignore_unavailable, ignore_throttled, allow_no_indices, expand_wildcards, preference, routing, scroll, search_type, explain, profile, typed_keys, rest_total_hits_as_int, ccs_minimize_roundtrips)


Allows to use the Mustache language to pre-render a search definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**ignore_throttled** | Option<**bool**> | Whether specified concrete, expanded or aliased indices should be ignored when throttled. |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**routing** | Option<[**Vec<String>**](String.md)> | Comma-separated list of specific routing values. |  |
**scroll** | Option<**String**> | Specify how long a consistent view of the index should be maintained for scrolled search. |  |
**search_type** | Option<[**SearchTypeMulti**](.md)> | Search operation type. |  |
**explain** | Option<**bool**> | Specify whether to return detailed information about score computation as part of a hit. |  |
**profile** | Option<**bool**> | Specify whether to profile the query execution. |  |
**typed_keys** | Option<**bool**> | Specify whether aggregation and suggester names should be prefixed by their respective types in the response. |  |
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]
**ccs_minimize_roundtrips** | Option<**bool**> | Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_template_post_with_index

> search_template_post_with_index(index, body, ignore_unavailable, ignore_throttled, allow_no_indices, expand_wildcards, preference, routing, scroll, search_type, explain, profile, typed_keys, rest_total_hits_as_int, ccs_minimize_roundtrips)


Allows to use the Mustache language to pre-render a search definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**body** | **serde_json::Value** |  | [required] |
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**ignore_throttled** | Option<**bool**> | Whether specified concrete, expanded or aliased indices should be ignored when throttled. |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**routing** | Option<[**Vec<String>**](String.md)> | Comma-separated list of specific routing values. |  |
**scroll** | Option<**String**> | Specify how long a consistent view of the index should be maintained for scrolled search. |  |
**search_type** | Option<[**SearchTypeMulti**](.md)> | Search operation type. |  |
**explain** | Option<**bool**> | Specify whether to return detailed information about score computation as part of a hit. |  |
**profile** | Option<**bool**> | Specify whether to profile the query execution. |  |
**typed_keys** | Option<**bool**> | Specify whether aggregation and suggester names should be prefixed by their respective types in the response. |  |
**rest_total_hits_as_int** | Option<**bool**> | Indicates whether hits.total should be rendered as an integer or an object in the rest search response. |  |[default to false]
**ccs_minimize_roundtrips** | Option<**bool**> | Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_health

> crate::models::SecurityHealthResponseContent security_health()


Checks to see if the Security plugin is up and running.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SecurityHealthResponseContent**](SecurityHealthResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshot_cleanup_repository

> snapshot_cleanup_repository(repository, master_timeout, cluster_manager_timeout, timeout)


Removes stale data from repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | Repository name. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshot_clone

> snapshot_clone(repository, snapshot, target_snapshot, body, master_timeout, cluster_manager_timeout)


Clones indices from one snapshot into another snapshot in the same repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | Repository name. | [required] |
**snapshot** | **String** | Snapshot name. | [required] |
**target_snapshot** | **String** | The name of the cloned snapshot to create. | [required] |
**body** | **serde_json::Value** |  | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshot_create_post

> snapshot_create_post(repository, snapshot, master_timeout, cluster_manager_timeout, wait_for_completion, body)


Creates a snapshot in a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | Repository name. | [required] |
**snapshot** | **String** | Snapshot name. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**wait_for_completion** | Option<**bool**> | Should this request wait until the operation has completed before returning. |  |[default to false]
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshot_create_put

> snapshot_create_put(repository, snapshot, master_timeout, cluster_manager_timeout, wait_for_completion, body)


Creates a snapshot in a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | Repository name. | [required] |
**snapshot** | **String** | Snapshot name. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**wait_for_completion** | Option<**bool**> | Should this request wait until the operation has completed before returning. |  |[default to false]
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshot_create_repository_post

> snapshot_create_repository_post(repository, body, master_timeout, cluster_manager_timeout, timeout, verify)


Creates a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | Repository name. | [required] |
**body** | **serde_json::Value** |  | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**verify** | Option<**bool**> | Whether to verify the repository after creation. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshot_create_repository_put

> snapshot_create_repository_put(repository, body, master_timeout, cluster_manager_timeout, timeout, verify)


Creates a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | Repository name. | [required] |
**body** | **serde_json::Value** |  | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**verify** | Option<**bool**> | Whether to verify the repository after creation. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshot_delete

> snapshot_delete(repository, snapshot, master_timeout, cluster_manager_timeout)


Deletes a snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | Repository name. | [required] |
**snapshot** | **String** | Snapshot name. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshot_delete_repository

> snapshot_delete_repository(repository, master_timeout, cluster_manager_timeout, timeout)


Deletes a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | Name of the snapshot repository to unregister. Wildcard (`*`) patterns are supported. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshot_get

> snapshot_get(repository, snapshot, master_timeout, cluster_manager_timeout, ignore_unavailable, verbose)


Returns information about a snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | Repository name. | [required] |
**snapshot** | **String** | Comma-separated list of snapshot names. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**ignore_unavailable** | Option<**bool**> | Whether to ignore unavailable snapshots, defaults to false which means a SnapshotMissingException is thrown. |  |[default to false]
**verbose** | Option<**bool**> | Whether to show verbose snapshot info or only show the basic info found in the repository index blob. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshot_get_repository

> snapshot_get_repository(master_timeout, cluster_manager_timeout, local)


Returns information about a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshot_get_repository_with_repository

> snapshot_get_repository_with_repository(repository, master_timeout, cluster_manager_timeout, local)


Returns information about a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | Comma-separated list of repository names. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**local** | Option<**bool**> | Return local information, do not retrieve the state from cluster-manager node. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshot_restore

> snapshot_restore(repository, snapshot, master_timeout, cluster_manager_timeout, wait_for_completion, body)


Restores a snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | Repository name. | [required] |
**snapshot** | **String** | Snapshot name. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**wait_for_completion** | Option<**bool**> | Should this request wait until the operation has completed before returning. |  |[default to false]
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshot_status

> snapshot_status(master_timeout, cluster_manager_timeout, ignore_unavailable)


Returns information about the status of a snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**ignore_unavailable** | Option<**bool**> | Whether to ignore unavailable snapshots, defaults to false which means a SnapshotMissingException is thrown. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshot_status_with_repository

> snapshot_status_with_repository(repository, master_timeout, cluster_manager_timeout, ignore_unavailable)


Returns information about the status of a snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | Repository name. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**ignore_unavailable** | Option<**bool**> | Whether to ignore unavailable snapshots, defaults to false which means a SnapshotMissingException is thrown. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshot_status_with_repository_snapshot

> snapshot_status_with_repository_snapshot(repository, snapshot, master_timeout, cluster_manager_timeout, ignore_unavailable)


Returns information about the status of a snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | Repository name. | [required] |
**snapshot** | **String** | Comma-separated list of snapshot names. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**ignore_unavailable** | Option<**bool**> | Whether to ignore unavailable snapshots, defaults to false which means a SnapshotMissingException is thrown. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshot_verify_repository

> snapshot_verify_repository(repository, master_timeout, cluster_manager_timeout, timeout)


Verifies a repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository** | **String** | Repository name. | [required] |
**master_timeout** | Option<**String**> | Operation timeout for connection to master node. |  |
**cluster_manager_timeout** | Option<**String**> | Operation timeout for connection to cluster-manager node. |  |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tasks_cancel

> tasks_cancel(nodes, actions, parent_task_id, wait_for_completion)


Cancels a task, if it can be cancelled through an API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nodes** | Option<[**Vec<String>**](String.md)> | Comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes. |  |
**actions** | Option<[**Vec<String>**](String.md)> | Comma-separated list of actions that should be cancelled. Leave empty to cancel all. |  |
**parent_task_id** | Option<**String**> | Cancel tasks with specified parent task id (node_id:task_number). Set to -1 to cancel all. |  |
**wait_for_completion** | Option<**bool**> | Should this request wait until the operation has completed before returning. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tasks_cancel_with_task_id

> tasks_cancel_with_task_id(task_id, nodes, actions, parent_task_id, wait_for_completion)


Cancels a task, if it can be cancelled through an API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Cancel the task with specified task id (node_id:task_number). | [required] |
**nodes** | Option<[**Vec<String>**](String.md)> | Comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes. |  |
**actions** | Option<[**Vec<String>**](String.md)> | Comma-separated list of actions that should be cancelled. Leave empty to cancel all. |  |
**parent_task_id** | Option<**String**> | Cancel tasks with specified parent task id (node_id:task_number). Set to -1 to cancel all. |  |
**wait_for_completion** | Option<**bool**> | Should this request wait until the operation has completed before returning. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tasks_get

> tasks_get(task_id, wait_for_completion, timeout)


Returns information about a task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | Return the task with specified id (node_id:task_number). | [required] |
**wait_for_completion** | Option<**bool**> | Should this request wait until the operation has completed before returning. |  |[default to false]
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tasks_list

> tasks_list(nodes, actions, detailed, parent_task_id, wait_for_completion, group_by, timeout)


Returns a list of tasks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nodes** | Option<[**Vec<String>**](String.md)> | Comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes. |  |
**actions** | Option<[**Vec<String>**](String.md)> | Comma-separated list of actions that should be returned. Leave empty to return all. |  |
**detailed** | Option<**bool**> | Return detailed task information. |  |[default to false]
**parent_task_id** | Option<**String**> | Return tasks with specified parent task id (node_id:task_number). Set to -1 to return all. |  |
**wait_for_completion** | Option<**bool**> | Should this request wait until the operation has completed before returning. |  |[default to false]
**group_by** | Option<[**GroupBy**](.md)> | Group tasks by nodes or parent/child relationships. |  |
**timeout** | Option<**String**> | Operation timeout. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## termvectors_get

> termvectors_get(index, term_statistics, field_statistics, fields, offsets, positions, payloads, preference, routing, realtime, version, version_type)


Returns information and statistics about terms in the fields of a particular document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | The index in which the document resides. | [required] |
**term_statistics** | Option<**bool**> | Specifies if total term frequency and document frequency should be returned. |  |[default to false]
**field_statistics** | Option<**bool**> | Specifies if document count, sum of document frequencies and sum of total term frequencies should be returned. |  |[default to true]
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields to return. |  |
**offsets** | Option<**bool**> | Specifies if term offsets should be returned. |  |[default to true]
**positions** | Option<**bool**> | Specifies if term positions should be returned. |  |[default to true]
**payloads** | Option<**bool**> | Specifies if term payloads should be returned. |  |[default to true]
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**routing** | Option<**String**> | Routing value. |  |
**realtime** | Option<**bool**> | Specifies if request is real-time as opposed to near-real-time. |  |[default to true]
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## termvectors_get_with_id

> termvectors_get_with_id(index, id, term_statistics, field_statistics, fields, offsets, positions, payloads, preference, routing, realtime, version, version_type)


Returns information and statistics about terms in the fields of a particular document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | The index in which the document resides. | [required] |
**id** | **String** | Document ID. When not specified a doc param should be supplied. | [required] |
**term_statistics** | Option<**bool**> | Specifies if total term frequency and document frequency should be returned. |  |[default to false]
**field_statistics** | Option<**bool**> | Specifies if document count, sum of document frequencies and sum of total term frequencies should be returned. |  |[default to true]
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields to return. |  |
**offsets** | Option<**bool**> | Specifies if term offsets should be returned. |  |[default to true]
**positions** | Option<**bool**> | Specifies if term positions should be returned. |  |[default to true]
**payloads** | Option<**bool**> | Specifies if term payloads should be returned. |  |[default to true]
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**routing** | Option<**String**> | Routing value. |  |
**realtime** | Option<**bool**> | Specifies if request is real-time as opposed to near-real-time. |  |[default to true]
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## termvectors_post

> termvectors_post(index, term_statistics, field_statistics, fields, offsets, positions, payloads, preference, routing, realtime, version, version_type, body)


Returns information and statistics about terms in the fields of a particular document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | The index in which the document resides. | [required] |
**term_statistics** | Option<**bool**> | Specifies if total term frequency and document frequency should be returned. |  |[default to false]
**field_statistics** | Option<**bool**> | Specifies if document count, sum of document frequencies and sum of total term frequencies should be returned. |  |[default to true]
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields to return. |  |
**offsets** | Option<**bool**> | Specifies if term offsets should be returned. |  |[default to true]
**positions** | Option<**bool**> | Specifies if term positions should be returned. |  |[default to true]
**payloads** | Option<**bool**> | Specifies if term payloads should be returned. |  |[default to true]
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**routing** | Option<**String**> | Routing value. |  |
**realtime** | Option<**bool**> | Specifies if request is real-time as opposed to near-real-time. |  |[default to true]
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## termvectors_post_with_id

> termvectors_post_with_id(index, id, term_statistics, field_statistics, fields, offsets, positions, payloads, preference, routing, realtime, version, version_type, body)


Returns information and statistics about terms in the fields of a particular document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | The index in which the document resides. | [required] |
**id** | **String** | Document ID. When not specified a doc param should be supplied. | [required] |
**term_statistics** | Option<**bool**> | Specifies if total term frequency and document frequency should be returned. |  |[default to false]
**field_statistics** | Option<**bool**> | Specifies if document count, sum of document frequencies and sum of total term frequencies should be returned. |  |[default to true]
**fields** | Option<[**Vec<String>**](String.md)> | Comma-separated list of fields to return. |  |
**offsets** | Option<**bool**> | Specifies if term offsets should be returned. |  |[default to true]
**positions** | Option<**bool**> | Specifies if term positions should be returned. |  |[default to true]
**payloads** | Option<**bool**> | Specifies if term payloads should be returned. |  |[default to true]
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**routing** | Option<**String**> | Routing value. |  |
**realtime** | Option<**bool**> | Specifies if request is real-time as opposed to near-real-time. |  |[default to true]
**version** | Option<**i32**> | Explicit version number for concurrency control. |  |
**version_type** | Option<[**VersionType**](.md)> | Specific version type. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update

> update(id, index, body, wait_for_active_shards, _source, _source_excludes, _source_includes, lang, refresh, retry_on_conflict, routing, timeout, if_seq_no, if_primary_term, require_alias)


Updates a document with a script or partial document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Document ID. | [required] |
**index** | **String** | Index name. | [required] |
**body** | **serde_json::Value** |  | [required] |
**wait_for_active_shards** | Option<**String**> | Sets the number of shard copies that must be active before proceeding with the operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1). |  |[default to 1]
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or a list of fields to return. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | List of fields to exclude from the returned _source field. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | List of fields to extract and return from the _source field. |  |
**lang** | Option<**String**> | The script language. |  |[default to painless]
**refresh** | Option<[**RefreshEnum**](.md)> | If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes. |  |
**retry_on_conflict** | Option<**i32**> | Specify how many times should the operation be retried when a conflict occurs. |  |[default to 0]
**routing** | Option<**String**> | Routing value. |  |
**timeout** | Option<**String**> | Operation timeout. |  |
**if_seq_no** | Option<**i32**> | only perform the operation if the last operation that has changed the document has the specified sequence number. |  |
**if_primary_term** | Option<**i32**> | only perform the operation if the last operation that has changed the document has the specified primary term. |  |
**require_alias** | Option<**bool**> | When true, requires destination to be an alias. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_audit_configuration

> crate::models::UpdateAuditConfigurationResponseContent update_audit_configuration(audit_config)


Updates the audit configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**audit_config** | [**AuditConfig**](AuditConfig.md) |  | [required] |

### Return type

[**crate::models::UpdateAuditConfigurationResponseContent**](UpdateAuditConfigurationResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_by_query

> update_by_query(index, analyzer, analyze_wildcard, default_operator, df, from, ignore_unavailable, allow_no_indices, conflicts, expand_wildcards, lenient, pipeline, preference, q, routing, scroll, search_type, search_timeout, size, max_docs, sort, _source, _source_excludes, _source_includes, terminate_after, stats, version, request_cache, refresh, timeout, wait_for_active_shards, scroll_size, wait_for_completion, requests_per_second, slices, body)


Performs an update on every document in the index without changing the source, for example to pick up a mapping change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Comma-separated list of indices; use `_all` or empty string to perform the operation on all indices. | [required] |
**analyzer** | Option<**String**> | The analyzer to use for the query string. |  |
**analyze_wildcard** | Option<**bool**> | Specify whether wildcard and prefix queries should be analyzed. |  |[default to false]
**default_operator** | Option<[**DefaultOperator**](.md)> | The default operator for query string query (AND or OR). |  |
**df** | Option<**String**> | The field to use as default where no field prefix is given in the query string. |  |
**from** | Option<**i32**> | Starting offset. |  |[default to 0]
**ignore_unavailable** | Option<**bool**> | Whether specified concrete indices should be ignored when unavailable (missing or closed). |  |
**allow_no_indices** | Option<**bool**> | Whether to ignore if a wildcard indices expression resolves into no concrete indices. (This includes `_all` string or when no indices have been specified). |  |
**conflicts** | Option<[**Conflicts**](.md)> | What to do when the operation encounters version conflicts?. |  |
**expand_wildcards** | Option<[**ExpandWildcards**](.md)> | Whether to expand wildcard expression to concrete indices that are open, closed or both. |  |
**lenient** | Option<**bool**> | Specify whether format-based query failures (such as providing text to a numeric field) should be ignored. |  |
**pipeline** | Option<**String**> | The pipeline id to preprocess incoming documents with. |  |
**preference** | Option<**String**> | Specify the node or shard the operation should be performed on. |  |[default to random]
**q** | Option<**String**> | Query in the Lucene query string syntax. |  |
**routing** | Option<[**Vec<String>**](String.md)> | Comma-separated list of specific routing values. |  |
**scroll** | Option<**String**> | Specify how long a consistent view of the index should be maintained for scrolled search. |  |
**search_type** | Option<[**SearchType**](.md)> | Search operation type. |  |
**search_timeout** | Option<**String**> | Explicit timeout for each search request. Defaults to no timeout. |  |
**size** | Option<**i32**> | Deprecated, please use `max_docs` instead. |  |
**max_docs** | Option<**i32**> | Maximum number of documents to process (default: all documents). |  |
**sort** | Option<[**Vec<String>**](String.md)> | Comma-separated list of <field>:<direction> pairs. |  |
**_source** | Option<[**Vec<String>**](String.md)> | True or false to return the _source field or not, or a list of fields to return. |  |
**_source_excludes** | Option<[**Vec<String>**](String.md)> | List of fields to exclude from the returned _source field. |  |
**_source_includes** | Option<[**Vec<String>**](String.md)> | List of fields to extract and return from the _source field. |  |
**terminate_after** | Option<**i32**> | The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early. |  |
**stats** | Option<[**Vec<String>**](String.md)> | Specific 'tag' of the request for logging and statistical purposes. |  |
**version** | Option<**bool**> | Whether to return document version as part of a hit. |  |
**request_cache** | Option<**bool**> | Specify if request cache should be used for this request or not, defaults to index level setting. |  |
**refresh** | Option<**bool**> | Should the affected indexes be refreshed?. |  |
**timeout** | Option<**String**> | Time each individual bulk request should wait for shards that are unavailable. |  |[default to 1m]
**wait_for_active_shards** | Option<**String**> | Sets the number of shard copies that must be active before proceeding with the operation. Defaults to 1, meaning the primary shard only. Set to `all` for all shard copies, otherwise set to any non-negative value less than or equal to the total number of copies for the shard (number of replicas + 1). |  |[default to 1]
**scroll_size** | Option<**i32**> | Size on the scroll request powering the operation. |  |[default to 100]
**wait_for_completion** | Option<**bool**> | Should this request wait until the operation has completed before returning. |  |[default to true]
**requests_per_second** | Option<**i32**> | The throttle for this request in sub-requests per second. -1 means no throttle. |  |[default to 0]
**slices** | Option<**String**> | The number of slices this task should be divided into. Defaults to 1, meaning the task isn't sliced into subtasks. Can be set to `auto`. |  |[default to 1]
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_by_query_rethrottle

> update_by_query_rethrottle(task_id, requests_per_second)


Changes the number of requests per second for a particular Update By Query operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** | The task id to rethrottle. | [required] |
**requests_per_second** | **i32** | The throttle for this request in sub-requests per second. -1 means no throttle. | [required] |

### Return type

 (empty response body)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_configuration

> crate::models::UpdateConfigurationResponseContent update_configuration(dynamic_config)


Adds or updates the existing configuration using the REST API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dynamic_config** | [**DynamicConfig**](DynamicConfig.md) |  | [required] |

### Return type

[**crate::models::UpdateConfigurationResponseContent**](UpdateConfigurationResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_distinguished_names

> crate::models::UpdateDistinguishedNamesResponseContent update_distinguished_names(cluster_name, distinguished_names)


Adds or updates the specified distinguished names in the cluster’s or node’s allow list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_name** | **String** |  | [required] |
**distinguished_names** | Option<[**DistinguishedNames**](DistinguishedNames.md)> |  |  |

### Return type

[**crate::models::UpdateDistinguishedNamesResponseContent**](UpdateDistinguishedNamesResponseContent.md)

### Authorization

[smithy.api.httpBasicAuth](../README.md#smithy.api.httpBasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

