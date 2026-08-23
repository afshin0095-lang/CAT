use cat_memory::{filter, is_terminal, validate_transition, Classification, LifecycleState, MemoryKind, MemoryObject, MemoryId, Provenance, Consent, Retention, MemoryQuery};

fn object(id: MemoryId, kind: MemoryKind, state: LifecycleState, classification: Classification, namespace: &str) -> MemoryObject {
    MemoryObject {
        id,
        kind,
        namespace: namespace.into(),
        subject: None,
        classification,
        state,
        version: 1,
        content: serde_json::json!({"v": 1}),
        provenance: Provenance { source: "test".into(), source_version: None, captured_at_ms: 1, captured_by: "test".into() },
        consent: Consent { required: false, granted: true, scope: "test".into(), policy_version: "1".into() },
        retention: Retention { expires_at_ms: None, legal_hold: false, policy_version: "1".into() },
        created_at_ms: 1,
        updated_at_ms: 1,
    }
}

#[test]
fn lifecycle_allows_governed_forward_progression() {
    assert!(validate_transition(LifecycleState::Proposed, LifecycleState::Validated).is_ok());
    assert!(validate_transition(LifecycleState::Validated, LifecycleState::Active).is_ok());
    assert!(validate_transition(LifecycleState::Active, LifecycleState::Superseded).is_ok());
    assert!(validate_transition(LifecycleState::Active, LifecycleState::Revoked).is_ok());
}

#[test]
fn lifecycle_rejects_reactivation_and_terminal_escape() {
    assert!(validate_transition(LifecycleState::Revoked, LifecycleState::Active).is_err());
    assert!(validate_transition(LifecycleState::Expired, LifecycleState::Active).is_err());
    assert!(is_terminal(LifecycleState::Revoked));
    assert!(is_terminal(LifecycleState::Expired));
    assert!(!is_terminal(LifecycleState::Active));
}

#[test]
fn query_filters_memory_objects_without_mutation() {
    let a = object(MemoryId::new(), MemoryKind::Semantic, LifecycleState::Active, Classification::Internal, "affiliate");
    let b = object(MemoryId::new(), MemoryKind::Preference, LifecycleState::Retained, Classification::Confidential, "affiliate");
    let c = object(MemoryId::new(), MemoryKind::Semantic, LifecycleState::Active, Classification::Public, "content");
    let objects = vec![a, b, c];
    let matches = filter(objects.iter(), MemoryQuery::namespace("affiliate").with_kind(MemoryKind::Semantic).with_state(LifecycleState::Active));
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].namespace, "affiliate");
}

#[test]
fn query_respects_classification_floor() {
    let public = object(MemoryId::new(), MemoryKind::Semantic, LifecycleState::Active, Classification::Public, "x");
    let confidential = object(MemoryId::new(), MemoryKind::Semantic, LifecycleState::Active, Classification::Confidential, "x");
    let objects = vec![public, confidential];
    let matches = filter(objects.iter(), MemoryQuery::all().with_minimum_classification(Classification::Confidential));
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].classification, Classification::Confidential);
}
