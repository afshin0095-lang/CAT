#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn idempotency_claims_once() {
        let mut store = InMemoryIdempotencyStore::default();
        let id = Uuid::now_v7();
        assert!(store.claim(id).unwrap());
        assert!(!store.claim(id).unwrap());
        store.complete(id).unwrap();
        assert_eq!(store.state(id), Some(DeliveryState::Completed));
    }

    #[test]
    fn inbox_deduplicates_delivery() {
        let mut store = InMemoryInboxStore::default();
        let id = Uuid::now_v7();
        assert!(store.accept(id).unwrap());
        assert!(!store.accept(id).unwrap());
        store.mark_succeeded(id).unwrap();
    }
}
