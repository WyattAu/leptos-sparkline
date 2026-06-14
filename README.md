# leptos-sparkline

Sparkline mini-chart component for [Leptos](https://leptos.dev/) — renders tiny line charts on HTML5 canvas with localStorage persistence.

## Features

- **Canvas-based rendering** — high-DPI aware, crisp on Retina displays
- **localStorage persistence** — data survives page reloads
- **Signal-driven reactivity** — re-renders when data changes
- **Trend coloring** — auto-color based on direction (green up, red down)
- **Minimal footprint** — ~2KB WASM, zero dependencies beyond Leptos + web-sys

## Installation

```toml
[dependencies]
leptos-sparkline = "0.1"
```

## Usage

```rust
use leptos::prelude::*;
use leptos_sparkline::{Sparkline, SparklineHistory};

#[component]
fn MetricCard() -> impl IntoView {
    let history = SparklineHistory::new("btc-price", 30);
    let data = history.signal();

    // Push data points (persists to localStorage)
    history.push(42000.0);
    history.push(42500.0);

    view! {
        <div class="metric-card">
            <span class="metric-value">"$42,500"</span>
            <Sparkline data=data color="#00E5FF".to_string()/>
        </div>
    }
}
```

## API

### `SparklineHistory`

Manages a time series with localStorage persistence.

```rust
let history = SparklineHistory::new("my-metric", 30); // max 30 points
history.push(42.0);
let data = history.signal(); // ReadSignal<Vec<f64>>
history.clear();
```

### `Sparkline`

Renders a sparkline canvas.

```rust
view! {
    <Sparkline
        data=signal              // Signal<Vec<f64>>
        color="#00E5FF".into()   // CSS color (optional, default: cyan)
        width=60.0               // pixels (optional, default: 60)
        height=20.0              // pixels (optional, default: 20)
    />
}
```

### Canvas Helpers

```rust
use leptos_sparkline::draw_sparkline;

// Draw directly on a CanvasRenderingContext2d
draw_sparkline(&ctx, &data, width, height, "#00E5FF");
```

## License

MIT
