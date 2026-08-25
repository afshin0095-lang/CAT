use cat_content::{ContentId, ContentProvenance, ProvenanceLink};
use uuid::Uuid;

#[test]
fn provenance_records_source_version_and_actor() {
    let provenance = ContentProvenance::new(
        "merchant-feed-42",
        Some(7),
        "2026-08-25T12:00:00Z",
        "content-ingest",
    );

    assert_eq!(provenance.source_id, "merchant-feed-42");
    assert_eq!(provenance.source_version, Some(7));
    assert_eq!(provenance.actor, "content-ingest");
    assert_ne!(provenance.evidence_id, Uuid::nil());
}

#[test]
fn provenance_link_has_explicit_derivation_relation() {
    let content_id = ContentId(Uuid::now_v7());
    let evidence_id = Uuid::now_v7();
    let link = ProvenanceLink::derives_from(content_id, evidence_id);

    assert_eq!(link.content_id, content_id);
    assert_eq!(link.evidence_id, evidence_id);
    assert_eq!(link.relation, "derives_from");
}
