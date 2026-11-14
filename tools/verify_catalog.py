#!/usr/bin/env python3
"""Validate the SICP Rust companion's committed navigation and source coverage.

This is a *repository integrity* check, not a claim of textbook example parity.
No external dependencies or network access are required.
"""
from __future__ import annotations

import json
import re
import sys
import tomllib
from collections import Counter
from pathlib import Path
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parent.parent
LINK = re.compile(r"\]\(([^)]+)\)")
errors: list[str] = []


def fail(message: str) -> None:
    errors.append(message)


manifest = json.loads((ROOT / "manifest.json").read_text(encoding="utf-8"))
entries = manifest["entries"]
paths = [item["path"] for item in entries]
duplicates = [path for path, n in Counter(paths).items() if n != 1]
if duplicates:
    fail(f"Duplicate catalog paths: {duplicates}")
for entry in entries:
    path = Path(entry["path"])
    # All navigation paths are relative to the repository root.
    if path.is_absolute() or ".." in path.parts:
        fail(f"Unsafe catalog path: {path}")
    elif not (ROOT / path).is_file():
        fail(f"Missing catalog file: {path}")

examples = [entry for entry in entries if entry["kind"] == "example"]
sources = [entry for entry in entries if entry["kind"] == "source"]
if len(examples) != len(sources):
    fail("Source and README entries must have a 1:1 relationship")
if len({x["bin"] for x in examples}) != len(examples):
    fail("Every Rust example needs a unique bin target")
package = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
targets = {item["name"]: item["path"] for item in package.get("bin", [])}
if len(targets) != len(package.get("bin", [])):
    fail("Duplicate Cargo binary names")
for ex in examples:
    src = ROOT / ex["source"]
    readme = ROOT / ex["path"]
    if ex["source"] != str(Path(ex["path"]).parent / "main.rs"):
        fail(f"Source must be adjacent to README: {ex['path']}")
    if targets.get(ex["bin"]) != ex["source"]:
        fail(f"Cargo target missing or incorrect: {ex['bin']}")
    if src.is_file():
        code = src.read_text(encoding="utf-8")
        if "fn main(" not in code:
            fail(f"Missing runnable main: {src.relative_to(ROOT)}")
        if "#[cfg(test)]" not in code:
            fail(f"Missing unit-test module: {src.relative_to(ROOT)}")
    if readme.is_file():
        explanation = readme.read_text(encoding="utf-8")
        if not explanation.startswith("# "):
            fail(f"README needs a heading: {readme.relative_to(ROOT)}")
if set(targets) != {x["bin"] for x in examples}:
    fail("A Cargo bin exists without a matching manifest example")

# Every checked-in Markdown link must resolve to its own file's directory,
# not to the static index.html browser location.
markdown_files = sorted(ROOT.rglob("*.md"))
for source in markdown_files:
    if ".git" in source.parts or "target" in source.parts:
        continue
    text = source.read_text(encoding="utf-8")
    for raw in LINK.findall(text):
        link = urlsplit(raw.strip().split(" ")[0])
        if link.scheme or link.netloc or not link.path:
            continue
        target = (source.parent / unquote(link.path)).resolve()
        if not target.is_relative_to(ROOT) or not target.exists():
            fail(f"Broken Markdown link: {source.relative_to(ROOT)} -> {raw}")

if errors:
    for error in errors:
        print("ERROR:", error, file=sys.stderr)
    raise SystemExit(1)
print(
    f"Catalog validated: {len(entries)} links, {len(examples)} Rust examples, "
    f"{sum(x['kind']=='subsection' for x in entries)} subsection guides, "
    f"{len(markdown_files)} Markdown files."
)
