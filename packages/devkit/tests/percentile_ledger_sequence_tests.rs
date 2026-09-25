//! Unit tests for Percentile/LedgerSequence validation.
struct Percentile(f64);
impl Percentile {
    fn new(v: f64) -> Result<Self, String> {
        if !(0.0..=100.0).contains(&v) {
            return Err(format!("{v} out of range"));
        }
        Ok(Self(v))
    }
}

#[derive(PartialEq, Debug)]
struct LedgerSequence(u32);
impl LedgerSequence {
    fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

#[test]
fn percentile_rejects_out_of_range() {
    assert!(Percentile::new(150.0).is_err());
    assert!(Percentile::new(50.0).is_ok());
}

#[test]
fn ledger_sequence_increments() {
    assert_eq!(LedgerSequence(5).next(), LedgerSequence(6));
}
