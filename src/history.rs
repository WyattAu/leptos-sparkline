//! Sparkline history persistence.
//!
//! Manages a time series of data points in localStorage,
//! with automatic trimming to a configurable maximum length.

use leptos::prelude::*;
use std::cell::RefCell;

/// Persistent sparkline history for a single metric.
///
/// Stores up to `max_points` values in `localStorage` under the key
/// `spark_{key}`. Provides a reactive `Signal<Vec<f64>>` that updates
/// whenever data is pushed.
pub struct SparklineHistory {
    key: String,
    max_points: usize,
    data: RwSignal<Vec<f64>>,
}

impl SparklineHistory {
    /// Create a new sparkline history.
    ///
    /// - `key`: Unique identifier (e.g., "btc-price", "vix")
    /// - `max_points`: Maximum number of data points to retain (default: 30)
    pub fn new(key: &str, max_points: usize) -> Self {
        let data = Self::load_from_storage(key);
        let data = RwSignal::new(data);
        Self {
            key: key.to_string(),
            max_points,
            data,
        }
    }

    /// Get a read-only signal of the sparkline data.
    pub fn signal(&self) -> Signal<Vec<f64>> {
        self.data.into()
    }

    /// Push a new data point. Trims to `max_points` and persists.
    pub fn push(&self, value: f64) {
        if value.is_nan() || value.is_infinite() {
            return;
        }
        self.data.update(|d| {
            d.push(value);
            let len = d.len();
            if len > self.max_points {
                d.drain(..len - self.max_points);
            }
        });
        self.save_to_storage();
    }

    /// Get the current data as a snapshot.
    pub fn snapshot(&self) -> Vec<f64> {
        self.data.get()
    }

    /// Clear all data.
    pub fn clear(&self) {
        self.data.set(Vec::new());
        self.save_to_storage();
    }

    /// Load data from localStorage.
    fn load_from_storage(key: &str) -> Vec<f64> {
        #[cfg(feature = "hydrate")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let storage_key = format!("spark_{}", key);
                    if let Ok(Some(json)) = storage.get_item(&storage_key) {
                        if let Ok(data) = serde_json::from_str::<Vec<f64>>(&json) {
                            return data;
                        }
                    }
                }
            }
        }
        Vec::new()
    }

    /// Save data to localStorage.
    fn save_to_storage(&self) {
        #[cfg(feature = "hydrate")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let storage_key = format!("spark_{}", self.key);
                    let data = self.data.get();
                    if let Ok(json) = serde_json::to_string(&data) {
                        let _ = storage.set_item(&storage_key, &json);
                    }
                }
            }
        }
    }
}

/// Batch push multiple data points to a history.
pub fn push_batch(history: &SparklineHistory, values: &[f64]) {
    for &v in values {
        history.push(v);
    }
}

/// Compute a color based on the trend of the sparkline data.
///
/// Returns green if trending up, red if trending down, gray if flat.
pub fn trend_color(data: &[f64]) -> &'static str {
    if data.len() < 2 {
        return "rgba(255,255,255,0.3)";
    }
    let first = data.first().unwrap();
    let last = data.last().unwrap();
    let change = (last - first) / first.abs().max(0.01);

    if change > 0.01 {
        "#69f0ae" // green
    } else if change < -0.01 {
        "#ff5252" // red
    } else {
        "rgba(255,255,255,0.3)" // gray
    }
}
