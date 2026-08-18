#!/usr/bin/env python3
"""Deterministically append Coding Standard Part 3 to the canonical document.

The operation is intentionally append-only: the existing canonical prefix is
read as bytes, hashed, and copied byte-for-byte before the continuation pack is
appended. The script refuses to operate if the canonical file already contains
Part 3 or if the continuation pack is missing/malformed.
"""
from __future__ import annotations

import hashlib
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CANONICAL = ROOT / "context/14_CODING_STANDARD.md"
PACK = ROOT / "context/14_CODING_STANDARD_PART3.md"

SECTION_RE = re.compile(r"^## (\d+)\. ", re.MULTILINE)
EXPECTED = list(range(51, 76))


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def main() -> None:
    if not CANONICAL.exists():
        raise SystemExit(f"missing canonical document: {CANONICAL}")
    if not PACK.exists():
        raise SystemExit(f"missing continuation pack: {PACK}")

    canonical = CANONICAL.read_bytes()
    pack = PACK.read_text(encoding="utf-8")

    if "## 51. " in canonical.decode("utf-8") or "# PART 3" in canonical.decode("utf-8"):
        raise SystemExit("canonical document already contains Part 3; refusing to append")

    sections = [int(x) for x in SECTION_RE.findall(pack)]
    if sections != EXPECTED:
        raise SystemExit(f"Part 3 section sequence mismatch: {sections!r}")

    if "**Part 3 Status:** COMPLETE" not in pack:
        raise SystemExit("Part 3 completion contract not found")
    if "CAT-CS-CONST-114" not in pack:
        raise SystemExit("constitutional registry appears incomplete")
    if "CAT-CS-AT-S75-001" not in pack:
        raise SystemExit("acceptance-test registry appears incomplete")
    if "CAT-CS-MEM-S75-001" not in pack:
        raise SystemExit("memory-anchor registry appears incomplete")

    prefix_hash = sha256(canonical)
    separator = "\n\n" if not canonical.endswith(b"\n\n") else ""
    output = canonical + separator.encode() + pack.encode("utf-8")

    # Self-check append-only identity before writing.
    if output[: len(canonical)] != canonical:
        raise SystemExit("append-only prefix identity check failed")

    CANONICAL.write_bytes(output)
    print(f"prefix_bytes={len(canonical)}")
    print(f"prefix_sha256={prefix_hash}")
    print(f"part3_bytes={len(output) - len(canonical)}")
    print(f"final_bytes={len(output)}")
    print(f"final_sha256={sha256(output)}")
    print("sections=51-75")


if __name__ == "__main__":
    main()
