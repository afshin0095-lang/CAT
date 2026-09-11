use cat_memory::{
    Classification, Consent, InMemoryMemoryStore, LifecycleState, MemoryId, MemoryKind,
    MemoryObject, MemoryPolicy, MemoryStore, Provenance, Retention,
};

fn object(state: LifecycleState, consent: bool) -> MemoryObject {
    MemoryObject {
        id: MemoryId::new(),
        kind: MemoryKind::Semantic,
        namespace: "test".into(),
        subject: Some("subject-1".into()),
        classification: Classification::Internal,
        state,
        version: 1,
        content: serde_json::json!({"value":"example"}),
        provenance: Provenance {
            source: "test".into(),
            source_version: Some("1".into()),
            captured_at_ms: 100,
            captured_by: "test-agent".into(),
        },
        consent: Consent {
            required: true,
            granted: consent,
            scope: "test".into(),
            policy_version: "1".into(),
        },
        retention: Retention {
            expires_at_ms: Some(10_000),
            legal_hold: false,
            policy_version: "1".into(),
        },
        created_at_ms: 100,
        updated_at_ms: 100,
    }
}

#[test]
fn rejects_missing_required_consent() {
    let mut store = InMemoryMemoryStore::new(MemoryPolicy::default(), 100);
    let result = store.insert(object(LifecycleState::Active, false));
    assert!(result.is_err());
}

#[test]
fn stores_and_lists_by_namespace() {
    let mut store = InMemoryMemoryStore::new(MemoryPolicy::default(), 100);
    let value = object(LifecycleState::Active, true);
    let id = value.id;
    store.insert(value).unwrap();
    assert_eq!(store.len(), 1);
    assert_eq!(store.get(id).unwrap().namespace, "test");
    assert_eq!(store.list_namespace("test").len(), 1);
}

#[test]
fn duplicate_identity_is_rejected() {
    let mut store = InMemoryMemoryStore::new(MemoryPolicy::default(), 100);
    let value = object(LifecycleState::Active, true);
    store.insert(value.clone()).unwrap();
    assert!(store.insert(value).is_err());
}
