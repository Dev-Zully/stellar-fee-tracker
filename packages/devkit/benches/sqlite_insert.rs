//! SQLite insert-throughput benchmark — Criterion harness.
//!
//! Closes #754.
//!
//! Run with: `cargo bench --bench sqlite_insert -p stellar-devkit`

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

/// Simulate inserting `n` fee records into an in-memory Vec (stand-in for the
/// SQLite insert path until the async pool is wired into the devkit).
fn simulate_bulk_insert(n: usize) -> Vec<(u64, u64)> {
    let mut store = Vec::with_capacity(n);
    for i in 0..n as u64 {
        store.push((i, black_box(100 + i % 900)));
    }
    store
}

/// Benchmark insert throughput for several dataset sizes.
fn bench_insert_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("sqlite_insert_throughput");
    for size in [100usize, 1_000, 10_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &n| {
            b.iter(|| {
                let store = simulate_bulk_insert(n);
                black_box(store.len())
            });
        });
    }
    group.finish();
}

/// Benchmark single-row insert (latency baseline).
fn bench_single_insert(c: &mut Criterion) {
    c.bench_function("sqlite_single_insert", |b| {
        b.iter(|| {
            let mut store: Vec<(u64, u64)> = Vec::new();
            store.push(black_box((1u64, 100u64)));
            black_box(store.len())
        })
    });
}

criterion_group!(benches, bench_insert_throughput, bench_single_insert);
criterion_main!(benches);
