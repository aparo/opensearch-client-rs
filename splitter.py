
from pathlib import Path
from collections import defaultdict
from re import sub

BASE_PATH=Path(__file__).parent / "opensearch-client"/"src"

TYPE_HEADER=[
    """
#[allow(unused_imports)]
use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
"""
]

CATEGORIES=["Indices", "Cluster", "Nodes", "Snapshot", "Tasks", "Ingest", "Cat", "User", "Patch", "PutScript", "Delete", "Dangling", "Bulk"]
def snake_case(s):
  return '_'.join(
    sub('([A-Z][a-z]+)', r' \1',
    sub('([A-Z]+)', r' \1',
    s.replace('-', ' '))).split()).lower()
  
def extract_category(name:str) -> str:
    for cat in CATEGORIES:
        if name.startswith(cat):
            return cat
    return "Common"

def split_types() -> None:
    (BASE_PATH/ "types").mkdir(exist_ok=True, parents=True)
    sections=defaultdict(list)
    name="Common"
    block=[]
    for line in (BASE_PATH / "types.rs").read_text().splitlines():
        if line.startswith("///"):
            if len(block) > 4:
                # print("\n".join(block))
                sections[extract_category(name)].extend(block)
                block=[]
                name=""
            else:
                block.append(line)
        elif line.startswith("pub struct "):
            block.append(line)
            name=line.split(" ")[2].split("{")[0].split("(")[0].strip()
            # print(f"export struct {name}")
        else:
            block.append(line)
    if len(block) > 1:
        # print("\n".join(block))
        sections[extract_category(name)].extend(block)
        block=[]
        name=""
            
    mod=["pub mod "+snake_case(name.lower())+";" for name in sections.keys()] + \
    ["pub use "+snake_case(name.lower())+"::*;" for name in sections.keys()]+ \
        ["pub mod builder;"]
    for name, blocks in sections.items():
        filename=snake_case(name.lower()+".rs")
        dest=BASE_PATH/ "types" / filename
        print(f"export namespace {dest}")
        dest.write_text("\n".join(TYPE_HEADER)+"\n"+"\n".join(blocks))
    dest=BASE_PATH/ "types" / "mod.rs"
    print(f"export namespace {dest}")
    dest.write_text("\n".join(mod))

if __name__ == "__main__":
    split_types()