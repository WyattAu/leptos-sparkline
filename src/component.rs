//! Sparkline Leptos component.
//!
//! Renders a sparkline canvas that reacts to data changes.

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::canvas::{draw_sparkline, setup_high_dpi};

/// A sparkline mini-chart component.
///
/// Renders a tiny line chart on an HTML5 canvas.
/// Re-renders when the `data` signal changes.
///
/// # Props
///
/// - `data`: Signal<Vec<f64>> — the sparkline data points
/// - `color`: String — CSS color for the line (e.g., "#00E5FF")
/// - `width`: Optional<f64> — canvas width in pixels (default: 60)
/// - `height`: Optional<f64> — canvas height in pixels (default: 20)
#[component]
pub fn Sparkline(
    data: Signal<Vec<f64>>,
    #[prop(optional)] color: String,
    #[prop(optional)] width: Option<f64>,
    #[prop(optional)] height: Option<f64>,
) -> impl IntoView {
    let w = width.unwrap_or(60.0);
    let h = height.unwrap_or(20.0);
    let color = if color.is_empty() {
        "#00E5FF".to_string()
    } else {
        color
    };

    let (canvas_id, _set_canvas_id) = signal(format!("spark-{}", js_sys::Math::random()));

    #[cfg(feature = "hydrate")]
    {
        let color_clone = color.clone();

        // Draw sparkline when data changes
        Effect::new(move |_| {
            let d = data.get();
            let id = canvas_id.get();
            if d.len() < 2 {
                return;
            }

            // Defer to next microtask to ensure canvas is in DOM
            let color = color_clone.clone();
            leptos::task::spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(10).await;
                if let Some(window) = web_sys::window() {
                    if let Some(doc) = window.document() {
                        if let Some(el) = doc.get_element_by_id(&id) {
                            let canvas: web_sys::HtmlCanvasElement = el.unchecked_into();
                            if let Ok(Some(ctx)) = canvas.get_context("2d") {
                                let ctx: web_sys::CanvasRenderingContext2d = ctx.unchecked_into();
                                let (w, h) = setup_high_dpi(&canvas, &ctx);
                                draw_sparkline(&ctx, &d, w, h, &color);
                            }
                        }
                    }
                }
            });
        });
    }

    view! {
        <canvas
            id=move || canvas_id.get()
            class="sparkline-canvas"
            width=w as u32
            height=h as u32
            aria-hidden="true"
            style="display:block"
        ></canvas>
    }
}
