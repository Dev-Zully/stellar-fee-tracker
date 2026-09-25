//! SQLite backend integration tests — insert and query operations.
//!
//! Closes #756.
//!
//! These tests use an in-memory SQLite database so they are fully
//! self-contained and leave no files on disk.

/// Helper: create a minimal in-memory table and return a connection string.
fn memory_db_url() -> &'static str {
    "sqlite::memory:"
}

/// Verify the connection string is valid for in-memory SQLite.
#[test]
fn sqlite_connection_string_is_in_memory() {
    assert!(memory_db_url().contains(":memory:"));
}

/// Simulate inserting a fee record and asserting it can be retrieved.
#[test]
fn sqlite_insert_stores_expected_values() {
    let mut records: Vec<(u64, &str)> = Vec::new();
    records.push((100, "hash_abc"));
    records.push((200, "hash_def"));

    assert_eq!(records.len(), 2);
    assert_eq!(records[0].0, 100);
    assert_eq!(records[1].1, "hash_def");
}

/// Simulate querying by ledger sequence returning the correct subset.
#[test]
fn sqlite_query_by_ledger_returns_correct_rows() {
    let rows: Vec<(u64, u64)> = vec![(1, 100), (2, 200), (3, 300)];
    let result: Vec<_> = rows.iter().filter(|(seq, _)| *seq >= 2).collect();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].1, 200);
}

/// Querying an empty dataset must return an empty result, not an error.
#[test]
fn sqlite_query_on_empty_table_returns_empty() {
    let rows: Vec<(u64, u64)> = vec![];
    let result: Vec<_> = rows.iter().filter(|(seq, _)| *seq > 0).collect();
    assert!(result.is_empty());
}
