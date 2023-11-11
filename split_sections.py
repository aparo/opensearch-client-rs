from collections import defaultdict
from pathlib import Path
import os

# files = list(Path("opensearch-client/src/").rglob("*.rs"))
files = [
    Path("opensearch-client/src/types/mod.rs"),
    Path("opensearch-client/src/builder.rs"),
]

SECTIONS = [
    "Cat",
    "Indices",
    "Cluster",
    "Snapshot",
    "Nodes",
    "Remote",
    "Tasks",
    "Mtermvectors",
]


def detect_classes():
    classes = []
    for file in files:
        with open(file, "r") as f:
            for line in f:
                if line.startswith("pub struct"):
                    class_name = line.split(" ")[2].split("{")[0].split("(")[0]
                    for section in SECTIONS:
                        if class_name.startswith(section):
                            classes.append((section, class_name))
    return classes


def get_in_category(name, classes):
    for section, class_name in classes:
        if name == class_name:
            return section
    return None


def get_comment_size(lines, valid_space=False):
    reverse_lines = lines[::-1]
    for i, line in enumerate(reverse_lines):
        line = line.strip()
        if line.startswith("///") or line.startswith("#"):
            continue
        if valid_space and not line:
            continue
        return i


def split_class(classes):
    for file in files:
        changed = False
        lines = []
        extracted = defaultdict(list)
        category = None
        name = None
        skip = False
        # print(f"Processing {file}")
        with open(file, "r") as f:
            for line in f:
                if (
                    line.startswith("pub struct")
                    and get_in_category(
                        line.split(" ")[2].split("{")[0].split("(")[0], classes
                    )
                    is not None
                ):
                    name = line.split(" ")[2].split("{")[0].split("(")[0]
                    category = get_in_category(name, classes).lower()
                    changed = True
                    cmt_size = get_comment_size(lines)
                    if cmt_size:
                        for l in lines[-cmt_size:]:
                            extracted[category].append(l)
                        lines = lines[:-cmt_size]
                    extracted[category].append(line)
                    skip = True
                elif (
                    line.startswith("impl<'a> ")
                    and get_in_category(
                        line.split(" ")[1].split("{")[0].split("(")[0], classes
                    )
                    is not None
                ):
                    name = line.split(" ")[1].split("{")[0].split("(")[0]
                    category = get_in_category(name, classes).lower()
                    changed = True
                    cmt_size = get_comment_size(lines)
                    if cmt_size:
                        for l in lines[-cmt_size:]:
                            extracted[category].append(l)
                        lines = lines[:-cmt_size]
                    extracted[category].append(line)
                    skip = True

                elif not skip and (
                    f"impl {name}" in line
                    or f"for {name}" in line
                    or f"From<{name}" in line
                    or f"From<&{name}" in line
                ):
                    changed = True
                    cmt_size = get_comment_size(lines)
                    if cmt_size:
                        for l in lines[-cmt_size:]:
                            extracted[category].append(l)
                        lines = lines[:-cmt_size]
                    extracted[category].append(line)
                    skip = True
                elif skip and line.startswith("}"):
                    skip = False
                    extracted[category].append(line)
                elif not skip:
                    lines.append(line)
                else:
                    extracted[category].append(line)
                    changed = True
        if changed:
            original_file = file
            with open(original_file, "w") as f:
                f.write("".join(lines))
            print(f"Updated {original_file}")

            for category, lines in extracted.items():
                headers = []
                os.makedirs(f"opensearch-client/src/{category.lower()}", exist_ok=True)
                if original_file.name == "mod.rs":
                    file = f"opensearch-client/src/{category.lower()}/types.rs"
                    headers = [
                        """#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::types::UserDefinedValueMap;
"""
                    ]
                elif original_file.name == "builder.rs":
                    file = f"opensearch-client/src/{category.lower()}/builder.rs"
                    headers = [
                        """use crate::types::*;
use super::types;
#[allow(unused_imports)]
use crate::{
  encode_path, encode_path_option_vec_string, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt,
  ReqwestResponse, ResponseValue,
};

"""
                    ]
                if not os.path.exists(file):
                    with open(file, "w") as f:
                        f.write("".join(headers + lines))
                    print(f"Created {file}")
                else:
                    with open(file, "r") as f:
                        existing = f.readlines()
                    with open(file, "w") as f:
                        f.write("".join(existing + lines))
                    print(f"Updated {file}")


def process_class():
    detected = detect_classes()
    if len(detected) == 0:
        print("No classes found")
        return
    split_class(detected)


def get_category_from_method(method):
    for section in SECTIONS:
        if method.startswith(section.lower() + "_"):
            return section
    return None


def split_lib():
    file = Path("opensearch-client/src/lib.rs")
    changed = False
    lines = []
    extracted = defaultdict(list)
    category = None
    name = None
    skip = False
    # print(f"Processing {file}")
    with open(file, "r") as f:
        for line in f:
            if (
                line.startswith("  pub fn ")
                and get_category_from_method(
                    line.strip().split(" ")[2].split("{")[0].split("(")[0]
                )
                is not None
            ):
                name = line.strip().split(" ")[2].split("{")[0].split("(")[0]
                category = get_category_from_method(name).lower()
                changed = True
                cmt_size = get_comment_size(lines)
                for l in lines[-cmt_size:]:
                    extracted[category].append(l)
                lines = lines[:-cmt_size]
                extracted[category].append(line)
                skip = True
            elif skip and line.strip().startswith("}"):
                skip = False
                extracted[category].append(line)
            elif not skip:
                lines.append(line)
            else:
                extracted[category].append(line)
                changed = True
    if changed:
        original_file = file
        with open(original_file, "w") as f:
            f.write("".join(lines))
        print(f"Updated {original_file}")

        for category, lines in extracted.items():
            os.makedirs(f"opensearch-client/src/{category.lower()}", exist_ok=True)
            file = f"opensearch-client/src/{category.lower()}/mod.rs"
            if not os.path.exists(file):
                starts = f"""use crate::OsClient;
mod builder;
mod types;
pub struct {category}<'a> {{
  os_client: &'a OsClient,
}}

impl<'a> {category}<'a> {{
  pub fn new(os_client: &'a OsClient) -> Self {{
    Self {{ os_client }}
  }}
"""
                with open(file, "w") as f:
                    f.write(starts + "".join(lines) + "\n}\n")
                print(f"Created {file}")
            else:
                with open(file, "r") as f:
                    existing = f.readlines()
                with open(file, "w") as f:
                    f.write("".join(existing + lines))
                print(f"Updated {file}")


if __name__ == "__main__":
    process_class()
    split_lib()
