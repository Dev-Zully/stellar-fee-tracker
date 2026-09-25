//! Parses ledger sequence and close-time from a Horizon ledger response.
use serde_json::Value;

pub struct LedgerInfo {
    pub sequence: u32,
    pub closed_at_rfc3339: String,
}

pub fn parse_ledger_response(raw_json: &str) -> Option<LedgerInfo> {
    let value: Value = serde_json::from_str(raw_json).ok()?;
    Some(LedgerInfo {
        sequence: value.get("sequence")?.as_u64()? as u32,
        closed_at_rfc3339: value.get("closed_at")?.as_str()?.to_string(),
    })
}
