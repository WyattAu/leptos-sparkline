# Pattern adoption

| Pattern | Status | Evidence |
|---|---|---|
| T15 | adopted | src/history.rs:35 (history exposed as a Leptos Signal; updates flow through the glitch-free reactive DAG), src/component.rs:43 (Effect redraws on data change) |
| T13 | partial | src/canvas.rs:11 (pure draw function over a data slice), src/component.rs:43 (canvas Effect is the imperative shell) |
