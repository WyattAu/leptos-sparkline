//! # leptos-sparkline
//!
//! Sparkline mini-chart component for Leptos.
//!
//! Renders tiny line charts on HTML5 canvas with localStorage persistence.
//! Designed for metric cards, dashboards, and data-dense UIs.
//!
//! ## Features
//!
//! - Canvas-based sparkline rendering (high DPI aware)
//! - localStorage persistence with configurable max points
//! - Signal-driven reactivity (re-renders on data change)
//! - Color customization
//! - Minimal footprint (~2KB WASM)
//!
//! ## Usage
//!
//! ```rust,no_run
//! use leptos::prelude::*;
//! use leptos_sparkline::{Sparkline, SparklineHistory};
//!
//! #[component]
//! fn MetricCard() -> impl IntoView {
//!     let history = SparklineHistory::new("btc-price", 30);
//!     let data = history.signal();
//!
//!     // Push new data points
//!     history.push(42000.0);
//!     history.push(42500.0);
//!
//!     view! {
//!         <Sparkline data=data color="#00E5FF".to_string()/>
//!     }
//! }
//! ```

#![deny(missing_docs)]

mod canvas;
mod component;
mod history;

pub use canvas::draw_sparkline;
pub use component::Sparkline;
pub use history::{push_batch, trend_color, SparklineHistory};
