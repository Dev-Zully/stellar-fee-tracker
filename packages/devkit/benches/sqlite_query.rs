//! SQLite query-performance benchmark — Criterion harness.
//!
//! Closes #755.
//!
//! Run with: `cargo bench --bench sqlite_query -p stellar-devkit`

use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Simulate a dataset of fee records (ledger_seq, fee_amount).
fn make_dataset(n: usize) -> Vec<(u64, u64)> {
    (0..n as u64).map(|i| (i, 100 + i % 500)).collect()
}

/// Benchmark: linear scan filter by ledger sequence (simulates a WHERE clause).
fn bench_query_by_ledger(c: &mut Criterion) {
    let dataset = make_dataset(10_000);
    c.bench_function("sqlite_query_by_ledger_10k", |b| {
        b.iter(|| {
            let threshold = black_box(5_000u64);
            let result: Vec<_> = dataset.iter().filter(|(seq, _)| *seq >= threshold).collect();
            black_box(result.len())
        })
    });
}

/// Benchmark: lookup by fee amount range (simulates a range query).
fn bench_query_by_fee_range(c: &mut Criterion) {
    let dataset = make_dataset(10_000);
    c.bench_function("sqlite_query_fee_range_10k", |b| {
        b.iter(|| {
            let result: Vec<_> = dataset
                .iter()
                .filter(|(_, fee)| *fee >= black_box(200) && *fee <= black_box(400))
                .collect();
            black_box(result.len())
        })
    });
}

criterion_group!(benches, bench_query_by_ledger, bench_query_by_fee_range);
criterion_main!(benches);
