//! Timestamp newtype used consistently across the crate.
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn from_utc(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }

    pub fn inner(&self) -> DateTime<Utc> {
        self.0
    }
}
