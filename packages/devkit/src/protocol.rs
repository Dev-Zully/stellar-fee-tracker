//! Parses raw Horizon fee_stats JSON into FeeStats.
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FeeStats {
    pub last_ledger: u32,
    pub last_ledger_base_fee: String,
    pub p50_accepted_fee: String,
}

pub fn parse_fee_stats(raw_json: &str) -> Result<FeeStats, serde_json::Error> {
    serde_json::from_str(raw_json)
}
