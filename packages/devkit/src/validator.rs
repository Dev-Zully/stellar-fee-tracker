//! Basic fee_stats invariant checks.
pub struct FeeStatsLike {
    pub p10: i64,
    pub p50: i64,
    pub p90: i64,
    pub p99: i64,
}

pub fn validate(stats: &FeeStatsLike) -> Result<(), String> {
    if stats.p10 < 0 || stats.p50 < 0 || stats.p90 < 0 || stats.p99 < 0 {
        return Err("fees must be non-negative".into());
    }
    if !(stats.p10 <= stats.p50 && stats.p50 <= stats.p90 && stats.p90 <= stats.p99) {
        return Err("percentiles must be non-decreasing".into());
    }
    Ok(())
}
