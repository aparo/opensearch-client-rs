from pathlib import Path
import os

files = list(Path("opensearch-client/src/").rglob("*.rs"))


def detect_classes(base_class):
    classes = []
    for file in files:
        with open(file, "r") as f:
            for line in f:
                if line.startswith("pub struct"):
                    class_name = line.split(" ")[2].split("{")[0].split("(")[0]
                    if base_class in class_name and class_name != base_class:
                        classes.append(class_name)
    return classes


def remove_class(name, replace):
    implement = f"for {name}"
    from_implement = f"From<{name}>"

    for file in files:
        changed = False
        lines = []
        skip = False
        removed = 0
        # print(f"Processing {file}")
        with open(file, "r") as f:
            for line in f:
                if (
                    line.startswith("pub struct")
                    and name == line.split(" ")[2].split("{")[0].split("(")[0]
                ):
                    changed = True
                    removed += 1
                    lines = lines[:-2]

                elif not skip and implement in line or from_implement in line:
                    skip = True
                    changed = True
                    removed += 1
                elif skip and line.startswith("}"):
                    removed += 1
                    skip = False
                elif not skip:
                    if name in line:
                        lines.append(line.replace(name, replace))
                        changed = True
                    else:
                        lines.append(line)
                else:
                    removed += 1
        if changed:
            with open(file, "w") as f:
                f.write("".join(lines))
            print(f"Removed {removed} lines from {file}")


def process_class(name):
    detected = detect_classes(name)
    if len(detected) == 0:
        print("No classes found")
        return
    with open(f"{name}.txt", "w") as f:
        f.write("\n".join(detected))

    for n in detected:
        print(f"Removing {n}")
        remove_class(n, name)


if __name__ == "__main__":
    # process_class("ClusterManagerTimeout")
    # process_class("MasterTimeout")
    process_class("Timeout")
