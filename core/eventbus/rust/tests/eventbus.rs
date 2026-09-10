use std::sync::{Arc, Mutex};

use cat_eventbus::{CatEvent, EventBus, EventEnvelope, EventKind, PublishOutcome};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CampaignStarted { campaign_id: String }

impl CatEvent for CampaignStarted {
    const TYPE: &'static str = "affiliate.campaign.started";
    const VERSION: u16 = 1;
}

#[test]
fn envelope_contains_stable_contract_metadata() {
    let envelope = CampaignStarted { campaign_id: "cmp-001".into() }.into_envelope("affiliate-engine").unwrap().with_kind(EventKind::Integration);
    assert_eq!(envelope.event_type, "affiliate.campaign.started");
    assert_eq!(envelope.version, 1);
    assert_eq!(envelope.kind, EventKind::Integration);
    assert_eq!(envelope.producer, "affiliate-engine");
    assert_eq!(envelope.event_id.get_version_num(), 7);
}

#[test]
fn handlers_run_in_registration_order() {
    let bus = EventBus::new(); let observed = Arc::new(Mutex::new(Vec::new()));
    for marker in [1_u8, 2_u8, 3_u8] { let observed = Arc::clone(&observed); bus.subscribe(CampaignStarted::TYPE, Arc::new(move |_| { observed.lock().unwrap().push(marker); Ok(()) })).unwrap(); }
    let envelope = CampaignStarted { campaign_id: "cmp-002".into() }.into_envelope("affiliate-engine").unwrap();
    assert_eq!(bus.publish(envelope).unwrap(), PublishOutcome::Published { handlers_called: 3 }); assert_eq!(*observed.lock().unwrap(), vec![1, 2, 3]);
}

#[test]
fn duplicate_event_ids_are_suppressed() {
    let bus = EventBus::new(); let calls = Arc::new(Mutex::new(0_u32)); let calls_ref = Arc::clone(&calls);
    bus.subscribe(CampaignStarted::TYPE, Arc::new(move |_| { *calls_ref.lock().unwrap() += 1; Ok(()) })).unwrap();
    let envelope = CampaignStarted { campaign_id: "cmp-003".into() }.into_envelope("affiliate-engine").unwrap();
    assert_eq!(bus.publish(envelope.clone()).unwrap(), PublishOutcome::Published { handlers_called: 1 }); assert_eq!(bus.publish(envelope).unwrap(), PublishOutcome::DuplicateSuppressed); assert_eq!(*calls.lock().unwrap(), 1);
}

#[test]
fn unsubscribe_removes_handler() {
    let bus = EventBus::new(); let calls = Arc::new(Mutex::new(0_u32)); let calls_ref = Arc::clone(&calls);
    let subscription = bus.subscribe(CampaignStarted::TYPE, Arc::new(move |_| { *calls_ref.lock().unwrap() += 1; Ok(()) })).unwrap();
    assert!(bus.unsubscribe(subscription).unwrap());
    let envelope = CampaignStarted { campaign_id: "cmp-004".into() }.into_envelope("affiliate-engine").unwrap();
    assert_eq!(bus.publish(envelope).unwrap(), PublishOutcome::Published { handlers_called: 0 }); assert_eq!(*calls.lock().unwrap(), 0);
}

#[test]
fn envelope_serialization_round_trip_preserves_contract() {
    let envelope = CampaignStarted { campaign_id: "cmp-005".into() }.into_envelope("affiliate-engine").unwrap();
    let encoded = serde_json::to_string(&envelope).unwrap(); let decoded: EventEnvelope = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded.event_id, envelope.event_id); assert_eq!(decoded.event_type, envelope.event_type); assert_eq!(decoded.version, envelope.version); assert_eq!(decoded.payload, envelope.payload);
}
