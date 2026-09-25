//! Fixture builder for constructing FeeRecord-like test data.
pub struct FeeRecordBuilder {
    ledger: u32,
    fee_charged: i64,
    max_fee: i64,
}

impl Default for FeeRecordBuilder {
    fn default() -> Self {
        Self { ledger: 1, fee_charged: 100, max_fee: 500 }
    }
}

impl FeeRecordBuilder {
    pub fn ledger(mut self, ledger: u32) -> Self {
        self.ledger = ledger;
        self
    }

    pub fn fee_charged(mut self, fee: i64) -> Self {
        self.fee_charged = fee;
        self
    }

    pub fn build(self) -> (u32, i64, i64) {
        (self.ledger, self.fee_charged, self.max_fee)
    }
}
