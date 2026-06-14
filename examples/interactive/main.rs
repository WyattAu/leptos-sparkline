//! Example: Interactive sparkline with localStorage persistence.
//!
//! Run with: `cargo leptos serve --example interactive`

use leptos::prelude::*;
use leptos_sparkline::{Sparkline, SparklineHistory};

#[component]
fn App() -> impl IntoView {
    let history = SparklineHistory::new("btc-price", 30);
    let data = history.signal();

    // Push some initial data
    history.push(42000.0);
    history.push(42500.0);
    history.push(41800.0);
    history.push(43200.0);
    history.push(42800.0);

    let (value, set_value) = signal(String::new());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if let Ok(v) = value.get().parse::<f64>() {
            history.push(v);
            set_value.set(String::new());
        }
    };

    view! {
        <h1>"Sparkline Example"</h1>
        <p>"Data points: " {move || data.get().len()}</p>
        <p>"Last value: " {move || data.get().last().map(|v| format!("{:.2}", v)).unwrap_or_else(|| "--".to_string())}</p>

        <Sparkline data=data color="#00E5FF".to_string() width=200.0 height=40.0/>

        <form on:submit=on_submit>
            <input
                type="number"
                prop:value=value
                on:input=move |ev| set_value.set(event_target_value(&ev))
                placeholder="Enter value..."
            />
            <button type="submit">"Push"</button>
        </form>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
