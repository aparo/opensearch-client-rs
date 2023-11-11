from split_sections import get_comment_size
from pathlib import Path
import re

files = list(Path("opensearch-client/src/").rglob("*.rs"))
DUPPLICATES = {
    "IndexName": [
        #     "IndicesAddBlockIndex",
        #     "IndicesClonePostIndex",
        #     "IndicesClonePutIndex",
        #     "IndicesCloseIndex",
        #     "IndicesDeleteIndex",
        #     "IndicesExistsAliasWithIndexIndex",
        #     "IndicesExistsAliasWithIndexName",
        #     "IndicesDeleteIndexTemplateName",
        #     "IndicesExistsIndex",
        #     "IndicesGetAliasWithIndexIndex",
        #     "IndicesGetAliasWithIndexNameIndex",
        #     "IndicesGetAliasWithIndexNameName",
        #     "IndicesGetFieldMappingWithIndexIndex",
        #     "IndicesGetIndex",
        #     "IndicesGetMappingWithIndexIndex",
        #     "IndicesOpenIndex",
        #     "IndicesResolveIndexName",
        #     "IndicesRolloverWithNewIndexAlias",
        #     "IndicesRolloverWithNewIndexNewIndex",
        #     "IndicesShrinkPostIndex",
        #     "IndicesShrinkPutIndex",
        #     "IndicesSplitPostIndex",
        #     "IndicesSplitPutIndex",
        #     "MtermvectorsGetWithIndexIndex",
        #     "MtermvectorsPostWithIndexIndex",
        #     "CountIndex",
        #     "MsearchGetWithIndexIndex",
        #     "MsearchPostWithIndexIndex",
        #     "MsearchTemplateGetWithIndexIndex",
        #     "MsearchTemplatePostWithIndexIndex",
        #     "TermvectorsGetIndex",
        #     "TermvectorsGetWithIdIndex",
        #     "TermvectorsPostIndex",
        # "TermvectorsPostWithIdIndex",
        # "IndicesPutAliasPostName",
        # "IndicesPutAliasPostPluralName",
        "CatCountWithIndexIndex",
        "CatIndicesWithIndexIndex",
        "CatSegmentsWithIndexIndex",
        "CatShardsWithIndexIndex",
        "ClusterHealthWithIndexIndex",
    ],
    "OpenSearchId": [
        "TasksCancelWithTaskIdTaskId",
        "TasksGetTaskId",
        "ClearScrollWithScrollIdScrollId",
        "DocumentId",
        "DeleteByQueryRethrottleTaskId",
        "DeleteScriptId",
        "GetScriptId",
        "PutScriptPostId",
        "PutScriptPostWithContextId",
        "PutScriptPutId",
        "PutScriptPutWithContextId",
        "ReindexRethrottleTaskId",
        "RenderSearchTemplateGetWithIdId",
        "RenderSearchTemplatePostWithIdId",
        "UpdateByQueryRethrottleTaskId",
        "NodesHotThreadsWithNodeIdDeprecatedClusterNodeId",
        "NodesHotThreadsWithNodeIdDeprecatedDashNodeId",
        "NodesHotThreadsWithNodeIdDeprecatedNodeId",
        "NodesHotThreadsWithNodeIdNodeId",
        "NodesInfoWithMetricNodeIdNodeId",
        "NodesInfoWithNodeIdNodeId",
        "NodesReloadSecureSettingsWithNodeIdNodeId",
        "NodesStatsWithIndexMetricMetricNodeIdNodeId",
        "NodesStatsWithMetricNodeIdNodeId",
        "NodesStatsWithNodeIdNodeId",
        "NodesUsageWithMetricNodeIdNodeId",
        "NodesUsageWithNodeIdNodeId",
        "IngestDeletePipelineId",
        "IngestGetPipelineWithIdId",
        "IngestPutPipelineId",
        "IngestSimulateGetWithIdId",
        "IngestSimulatePostWithIdId",
        "CatAllocationWithNodeIdNodeId",
        "ClusterStatsWithNodeIdNodeId",
    ],
    "OpenSearchNameValue": [
        "DanglingIndicesDeleteDanglingIndexIndexUuid",
        "DanglingIndicesImportDanglingIndexIndexUuid",
        "IndexName",
        "PutScriptPostWithContextContext",
        "PutScriptPutWithContextContext",
        "NodesInfoWithMetricNodeIdMetric",
        "NodesStatsWithIndexMetricMetricIndexMetric",
        "NodesStatsWithIndexMetricMetricMetric",
        "NodesStatsWithIndexMetricMetricNodeIdIndexMetric",
        "NodesStatsWithIndexMetricMetricNodeIdMetric",
        "NodesStatsWithMetricMetric",
        "NodesStatsWithMetricNodeIdMetric",
        "NodesUsageWithMetricMetric",
        "NodesUsageWithMetricNodeIdMetric",
        "SnapshotCleanupRepositoryRepository",
        "SnapshotCloneRepository",
        "SnapshotCloneSnapshot",
        "SnapshotCloneTargetSnapshot",
        "SnapshotCreatePostRepository",
        "SnapshotCreatePostSnapshot",
        "SnapshotCreatePutRepository",
        "SnapshotCreatePutSnapshot",
        "SnapshotCreateRepositoryPostRepository",
        "SnapshotCreateRepositoryPutRepository",
        "SnapshotDeleteRepository",
        "SnapshotDeleteRepositoryRepository",
        "SnapshotDeleteSnapshot",
        "SnapshotGetRepository",
        "SnapshotGetRepositoryWithRepositoryRepository",
        "SnapshotGetSnapshot",
        "SnapshotRestoreRepository",
        "SnapshotRestoreSnapshot",
        "SnapshotStatusWithRepositoryRepository",
        "SnapshotStatusWithRepositorySnapshotRepository",
        "SnapshotStatusWithRepositorySnapshotSnapshot",
        "SnapshotVerifyRepositoryRepository",
        "CatAliasesWithNameName",
        "CatSnapshotsWithRepositoryRepository",
        "CatTemplatesWithNameName",
        "CatThreadPoolWithThreadPoolPatternsThreadPoolPatterns",
        "IndicesAddBlockBlock",
        "IndicesClonePostTarget",
        "IndicesClonePutTarget",
        "IndicesCreateDataStreamName",
        "IndicesDataStreamsStatsWithNameName",
        "IndicesDeleteAliasName",
        "IndicesDeleteAliasPluralName",
        "IndicesDeleteDataStreamName",
        "IndicesDeleteTemplateName",
        "IndicesExistsAliasName",
        "IndicesExistsIndexTemplateName",
        "IndicesExistsTemplateName",
        "IndicesGetAliasWithNameName",
        "IndicesGetDataStreamWithNameName",
        "IndicesGetFieldMappingFields",
        "IndicesGetFieldMappingWithIndexFields",
        "IndicesGetIndexTemplateWithNameName",
        "IndicesGetSettingsWithIndexNameName",
        "IndicesGetSettingsWithNameName",
        "IndicesGetTemplateWithNameName",
        "IndicesPutAliasPostName",
        "IndicesPutAliasPostPluralName",
        "IndicesPutAliasPutName",
        "IndicesPutAliasPutPluralName",
        "IndicesPutIndexTemplatePostName",
        "IndicesPutIndexTemplatePutName",
        "IndicesPutTemplatePostName",
        "IndicesPutTemplatePutName",
        "IndicesRolloverAlias",
        "IndicesShrinkPostTarget",
        "IndicesShrinkPutTarget",
        "IndicesSimulateIndexTemplateName",
        "IndicesSimulateTemplateWithNameName",
        "IndicesSplitPostTarget",
        "IndicesSplitPutTarget",
        "IndicesStatsWithIndexMetricMetric",
        "IndicesStatsWithMetricMetric",
        "ClusterDeleteComponentTemplateName",
        "ClusterExistsComponentTemplateName",
        "ClusterGetComponentTemplateWithNameName",
        "ClusterGetDecommissionAwarenessAwarenessAttributeName",
        "ClusterGetWeightedRoutingAttribute",
        "ClusterPutComponentTemplatePostName",
        "ClusterPutComponentTemplatePutName",
        "ClusterPutDecommissionAwarenessAwarenessAttributeName",
        "ClusterPutDecommissionAwarenessAwarenessAttributeValue",
        "ClusterPutWeightedRoutingAttribute",
        "ClusterStateWithIndexMetricMetric",
        "ClusterStateWithMetricMetric",
    ],
}


def remove_class(name: str, items: list[str]):
    for file in files:
        changed = False
        skip = False
        removed = 0
        original_lines = open(file, "r").readlines()
        # print(f"Processing {file}")
        for item in items:
            implement = f"for {item} "
            from_implement = f"From<{item}>"
            from_implement2 = f"From<&{item}>"
            lines: list[str] = []
            for line in original_lines:
                if (
                    line.startswith("pub struct")
                    and item == line.split(" ")[2].split("{")[0].split("(")[0]
                ):
                    changed = True
                    removed += 1
                    cmt_size = get_comment_size(lines)
                    if cmt_size:
                        lines = lines[:-cmt_size]
                elif not skip and (
                    implement in line
                    or from_implement in line
                    or from_implement2 in line
                ):
                    skip = True
                    changed = True
                    removed += 1
                elif skip and line.startswith("}"):
                    removed += 1
                    skip = False
                elif not skip:
                    if item in line:
                        if f"this::{item}," in line:
                            lines.append(line.replace(f"this::{item},", name + ","))
                        else:
                            lines.append(re.sub(r"\b" + item + r"\b", name, line))
                        changed = True
                    else:
                        lines.append(line)
                else:
                    removed += 1
            original_lines = lines
        if changed:
            with open(file, "w") as f:
                f.write("".join(lines))
            print(f"Removed {removed} lines from {file}")


def process_dupplicates():
    for name, items in DUPPLICATES.items():
        print(f"Processing {name}")
        remove_class(name, items)


if __name__ == "__main__":
    process_dupplicates()
