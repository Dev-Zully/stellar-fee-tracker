//! Validated percentile newtype.
use crate::error::DevkitError;
use crate::result::Result;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentile(f64);

impl Percentile {
    pub fn new(value: f64) -> Result<Self> {
        if !(0.0..=100.0).contains(&value) {
            return Err(DevkitError::Validation(format!(
                "percentile {value} out of range 0-100"
            )));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}
