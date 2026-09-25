//! Core Horizon fee_stats response types.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeRecord {
    pub ledger: u32,
    pub fee_charged: i64,
    pub max_fee: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeStats {
    pub last_ledger: u32,
    pub last_ledger_base_fee: i64,
    pub min_accepted_fee: i64,
    pub mode_accepted_fee: i64,
    pub p10_accepted_fee: i64,
    pub p50_accepted_fee: i64,
    pub p90_accepted_fee: i64,
    pub p99_accepted_fee: i64,
}
