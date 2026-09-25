//! Storage trait: persistence abstraction for fee records.
pub struct FeeRecordLike {
    pub ledger: u32,
    pub fee_charged: i64,
}

#[derive(Default, Clone)]
pub struct QueryParams {
    pub from_ledger: Option<u32>,
    pub to_ledger: Option<u32>,
    pub limit: Option<usize>,
}

pub trait Storage {
    fn insert(&mut self, record: FeeRecordLike) -> Result<(), String>;
    fn query(&self, params: QueryParams) -> Vec<FeeRecordLike>;
}
