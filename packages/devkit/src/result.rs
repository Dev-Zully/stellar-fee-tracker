//! Crate-wide Result alias.
use crate::error::DevkitError;

pub type Result<T> = std::result::Result<T, DevkitError>;
