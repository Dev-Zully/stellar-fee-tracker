//! Demonstrates the crate's intended public export surface.
//! Once #1/#2/#3/#8 land, these local stand-ins should be replaced by
//! `pub use crate::{types::*, error::*, test_helpers::*};` in lib.rs.

#[derive(Debug, Clone)]
pub struct FeeRecordHandle;

#[derive(Debug)]
pub struct DevkitErrorHandle;

#[cfg(test)]
pub mod test_helpers {
    use super::FeeRecordHandle;

    pub fn fixture() -> FeeRecordHandle {
        FeeRecordHandle
    }
}
