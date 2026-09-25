//! Builder for constructing storage query parameters.
#[derive(Default, Clone, Debug)]
pub struct QueryParams {
    pub from_ledger: Option<u32>,
    pub to_ledger: Option<u32>,
    pub limit: Option<usize>,
}

#[derive(Default)]
pub struct QueryParamsBuilder(QueryParams);

impl QueryParamsBuilder {
    pub fn ledger_range(mut self, from: u32, to: u32) -> Self {
        self.0.from_ledger = Some(from);
        self.0.to_ledger = Some(to);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.0.limit = Some(limit);
        self
    }

    pub fn build(self) -> QueryParams {
        self.0
    }
}
