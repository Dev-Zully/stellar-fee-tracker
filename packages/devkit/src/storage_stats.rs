//! Reports record counts and timestamp bounds for a Storage backend.
pub struct StorageStats {
    pub record_count: usize,
    pub min_ledger: Option<u32>,
    pub max_ledger: Option<u32>,
}

pub struct StorageStatsReporter;

impl StorageStatsReporter {
    pub fn report(ledgers: &[u32]) -> StorageStats {
        StorageStats {
            record_count: ledgers.len(),
            min_ledger: ledgers.iter().min().copied(),
            max_ledger: ledgers.iter().max().copied(),
        }
    }
}
