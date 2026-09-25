//! TTL-based in-memory cache for fee_stats-style lookups.
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct TtlCache<V: Clone> {
    ttl: Duration,
    entries: HashMap<String, (V, Instant)>,
}

impl<V: Clone> TtlCache<V> {
    pub fn new(ttl: Duration) -> Self {
        Self { ttl, entries: HashMap::new() }
    }

    pub fn get(&self, key: &str) -> Option<V> {
        self.entries
            .get(key)
            .filter(|(_, at)| at.elapsed() < self.ttl)
            .map(|(v, _)| v.clone())
    }

    pub fn put(&mut self, key: String, value: V) {
        self.entries.insert(key, (value, Instant::now()));
    }
}
