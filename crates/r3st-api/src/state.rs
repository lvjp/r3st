//! In-memory server state.
//!
//! A real implementation would persist buckets and objects; for now an
//! in-memory map is enough to exercise the conformance suite.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// A bucket and its (currently empty) contents.
#[derive(Debug, Default)]
pub struct BucketEntry {
    pub creation_date: String,
}

/// Shared, cloneable application state.
#[derive(Debug, Clone, Default)]
pub struct AppState {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Debug, Default)]
struct Inner {
    buckets: BTreeMap<String, BucketEntry>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Snapshot the current buckets as `(name, creation_date)` pairs.
    pub fn list_buckets(&self) -> Vec<(String, String)> {
        let inner = self.inner.lock().expect("state mutex poisoned");
        inner
            .buckets
            .iter()
            .map(|(name, entry)| (name.clone(), entry.creation_date.clone()))
            .collect()
    }
}
