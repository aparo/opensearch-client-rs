from collections import defaultdict
from pathlib import Path
import os

files = list(Path("opensearch-client/src/").rglob("*.rs"))


def detect_classes():
    classes = defaultdict(list)
    for file in files:
        with open(file, "r") as f:
            current_class = ""
            for line in f:
                if line.startswith("pub struct"):
                    class_name = line.split(" ")[2].split("{")[0].split("(")[0]
                    current_class = class_name
                elif (
                    "^(?!_|template|query|field|point|clear|usage|stats|hot|reload|painless).+$"
                    in line
                ):
                    if current_class.endswith("Index"):
                        if current_class not in classes["IndexName"]:
                            classes["IndexName"].append(current_class)
                        continue
                    elif (
                        current_class.endswith(
                            (
                                "Name",
                                "Alias",
                                "Repository",
                                "Metric",
                                "Context",
                                "Snapshot",
                                "Block",
                                "Target",
                                "Fields",
                                "Patterns",
                                "Attribute",
                                "Value",
                                "Uuid",
                            )
                        )
                        and current_class != "OpenSearchNameValue"
                    ):
                        if current_class not in classes["OpenSearchNameValue"]:
                            classes["OpenSearchNameValue"].append(current_class)
                        continue
                    elif (
                        current_class.endswith("Id") and current_class != "OpenSearchId"
                    ):
                        if current_class not in classes["OpenSearchId"]:
                            classes["OpenSearchId"].append(current_class)
                        continue
                    print(f"Found {current_class}")

    return classes


def process_class():
    detected = detect_classes()
    from pprint import pprint

    pprint(detected)


if __name__ == "__main__":
    process_class()
