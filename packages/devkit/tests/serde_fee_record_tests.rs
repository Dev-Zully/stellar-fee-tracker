//! Serde round-trip tests against a fixture payload.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct FeeRecord {
    ledger: u32,
    fee_charged: i64,
    max_fee: i64,
}

const FIXTURE: &str = r#"{"ledger": 100, "fee_charged": 100, "max_fee": 500}"#;

#[test]
fn fee_record_round_trips_through_json() {
    let record: FeeRecord = serde_json::from_str(FIXTURE).expect("valid fixture");
    assert_eq!(record.ledger, 100);

    let reserialized = serde_json::to_string(&record).expect("serialize");
    let reparsed: FeeRecord = serde_json::from_str(&reserialized).expect("reparse");
    assert_eq!(record, reparsed);
}
