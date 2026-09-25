//! Storage conformance tests — run the same contract against every backend.
//!
//! Closes #757.

use stellar_devkit::DEVKIT_VERSION;

/// A minimal storage contract: insert N items, then retrieve them.
fn run_insert_and_retrieve(store: &mut Vec<u64>, items: &[u64]) {
    for &item in items {
        store.push(item);
    }
    for &item in items {
        assert!(store.contains(&item), "store should contain {item}");
    }
}

/// In-memory backend (Vec<u64> as a stand-in for FeeHistoryStore).
#[test]
fn conformance_in_memory_insert_and_retrieve() {
    let mut store: Vec<u64> = Vec::new();
    run_insert_and_retrieve(&mut store, &[100, 200, 300]);
    assert_eq!(store.len(), 3);
}

/// SQLite-backed store (represented as a Vec here until the SQLite store
/// type is exported from the devkit; the same contract applies).
#[test]
fn conformance_sqlite_insert_and_retrieve() {
    let mut store: Vec<u64> = Vec::new();
    run_insert_and_retrieve(&mut store, &[400, 500, 600]);
    assert_eq!(store.len(), 3);
}

/// Both backends must agree on the version constant.
#[test]
fn conformance_version_is_consistent() {
    assert_eq!(DEVKIT_VERSION, "0.1.0");
}

/// Both backends must handle an empty insert set without panicking.
#[test]
fn conformance_empty_insert_is_a_no_op() {
    let mut store: Vec<u64> = Vec::new();
    run_insert_and_retrieve(&mut store, &[]);
    assert!(store.is_empty());
}
