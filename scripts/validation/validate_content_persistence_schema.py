#!/usr/bin/env python3
"""Static contract checks for the Content Engine PostgreSQL persistence boundary."""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MIGRATIONS = ROOT / "core" / "eventstore-postgres" / "rust" / "migrations"

REVISIONS = MIGRATIONS / "0022_content_revision_store.sql"
RECEIPTS = MIGRATIONS / "0023_content_publication_receipts.sql"


def require(path: Path, *fragments: str) -> None:
    text = path.read_text(encoding="utf-8")
    missing = [fragment for fragment in fragments if fragment not in text]
    if missing:
        raise SystemExit(f"{path}: missing required fragments: {missing}")


def main() -> None:
    require(
        REVISIONS,
        "CREATE TABLE IF NOT EXISTS content_revisions",
        "tenant_id UUID NOT NULL",
        "revision_id UUID NOT NULL",
        "parent_revision_id UUID NULL",
        "revision_number BIGINT NOT NULL",
        "canonical_payload JSONB NOT NULL",
        "content_hash TEXT NOT NULL",
        "provenance JSONB NOT NULL",
        "UNIQUE (tenant_id, content_id, revision_number)",
        "PRIMARY KEY (tenant_id, revision_id)",
        "content_revisions_self_parent_check",
    )
    require(
        RECEIPTS,
        "CREATE TABLE IF NOT EXISTS content_publication_receipts",
        "tenant_id UUID NOT NULL",
        "receipt_id UUID NOT NULL",
        "content_id UUID NOT NULL",
        "revision_id UUID NOT NULL",
        "destination TEXT NOT NULL",
        "policy_version TEXT NOT NULL",
        "content_hash TEXT NOT NULL",
        "observed_at TIMESTAMPTZ NOT NULL",
        "PRIMARY KEY (tenant_id, receipt_id)",
        "outcome IN ('published', 'rejected', 'failed', 'rolled_back')",
    )

    # Structural ownership invariants: receipts observe outcomes; they do not
    # carry authorization state, and neither table is allowed to overwrite the
    # canonical payload after insertion.
    for path in (REVISIONS, RECEIPTS):
        text = path.read_text(encoding="utf-8")
        if "UPDATE content_revisions" in text or "UPDATE content_publication_receipts" in text:
            raise SystemExit(f"{path}: mutable update path detected")

    print("Content persistence schema checks: PASS")


if __name__ == "__main__":
    main()
