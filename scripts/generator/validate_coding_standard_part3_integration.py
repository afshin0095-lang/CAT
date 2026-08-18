#!/usr/bin/env python3
"""Validate the Coding Standard Part 3 continuation before canonical append."""
from __future__ import annotations

import hashlib
import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CANONICAL = ROOT / "context/14_CODING_STANDARD.md"
PACK = ROOT / "context/14_CODING_STANDARD_PART3.md"
EXPECTED = list(range(51, 76))
SECTION_RE = re.compile(r"^## (\d+)\. (.+)$", re.MULTILINE)
RULE_RE = re.compile(r"CAT-CS-CONST-(\d+)")
TEST_RE = re.compile(r"CAT-CS-AT-S(\d+)-(\d+)")
MEM_RE = re.compile(r"CAT-CS-MEM-S(\d+)-001")


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def check(name: str, condition: bool, details: str = "") -> None:
    if not condition:
        raise AssertionError(f"FAIL: {name} {details}")
    print(f"PASS: {name}")


def main() -> None:
    check("canonical exists", CANONICAL.exists())
    check("continuation pack exists", PACK.exists())
    canonical = CANONICAL.read_bytes()
    pack = PACK.read_text(encoding="utf-8")

    check("canonical UTF-8", canonical.decode("utf-8") is not None)
    check("canonical LF-only", b"\r" not in canonical)
    check("pack LF-only", "\r" not in pack)
    check("pack sections", [int(n) for n, _ in SECTION_RE.findall(pack)] == EXPECTED)
    check("no section 76 in pack", "## 76. " not in pack)
    check("completion contract", "## 75. Part 3 Completion Contract" in pack)
    check("rules contiguous", sorted(set(map(int, RULE_RE.findall(pack)))) == list(range(101, 115)))
    check("acceptance tests present", len(TEST_RE.findall(pack)) >= 10)
    check("memory anchors present", len(MEM_RE.findall(pack)) >= 10)
    check("json registry", '"record_type": "coding_standard_part3_completion"' in pack)
    check("yaml registry", "cat_coding_standard_part3_completion_v1" in pack)
    check("append-only declaration", "append_only: true" in pack or "append-only" in pack.lower())

    # Validate every fenced JSON block in the pack.
    json_blocks = re.findall(r"```json\n(.*?)\n```", pack, re.DOTALL)
    check("JSON blocks exist", len(json_blocks) >= 25)
    for i, block in enumerate(json_blocks, 1):
        json.loads(block)
    print(f"PASS: JSON parse {len(json_blocks)}/{len(json_blocks)}")

    # Validate YAML blocks through PyYAML when available.
    try:
        import yaml  # type: ignore
    except ImportError:
        raise SystemExit("PyYAML is required for this validator")
    yaml_blocks = re.findall(r"```yaml\n(.*?)\n```", pack, re.DOTALL)
    check("YAML blocks exist", len(yaml_blocks) >= 25)
    for block in yaml_blocks:
        yaml.safe_load(block)
    print(f"PASS: YAML parse {len(yaml_blocks)}/{len(yaml_blocks)}")

    diagrams = re.findall(r"```mermaid\n(.*?)\n```", pack, re.DOTALL)
    check("Mermaid blocks", len(diagrams) >= 25)
    check("diagram metadata fields", all("flowchart" in d or "sequenceDiagram" in d for d in diagrams))

    digest = subprocess.check_output(
        ["bash", "-lc", "grep -v '^`[0-9a-f]\\{64\\}`$' context/14_CODING_STANDARD_PART3.md | sha256sum"],
        cwd=ROOT,
        text=True,
    ).split()[0]
    print(f"PASS: published-pack-digest={digest}")

    print("ALL CODING STANDARD PART 3 PRE-APPEND CHECKS PASSED")
    print(f"canonical_prefix_sha256={sha256(canonical)}")
    print(f"canonical_bytes={len(canonical)}")


if __name__ == "__main__":
    main()
