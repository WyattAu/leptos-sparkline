//! Canvas sparkline rendering.
//!
//! Draws a tiny line chart on a `CanvasRenderingContext2d`.
//! High-DPI aware — scales by `devicePixelRatio`.

use web_sys::CanvasRenderingContext2d;

/// Draw a sparkline on a canvas context.
///
/// `data` must have at least 2 points. Values are auto-scaled to fit the canvas.
pub fn draw_sparkline(
    ctx: &CanvasRenderingContext2d,
    data: &[f64],
    width: f64,
    height: f64,
    color: &str,
) {
    if data.len() < 2 {
        return;
    }

    let min_val = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = (max_val - min_val).max(0.01);

    // Clear canvas
    ctx.clear_rect(0.0, 0.0, width, height);

    // Draw line
    ctx.set_stroke_style_str(color);
    ctx.set_line_width(1.0);
    ctx.begin_path();
    for (i, val) in data.iter().enumerate() {
        let x = width / (data.len() - 1) as f64 * i as f64;
        let y = height - (val - min_val) / range * height;
        if i == 0 {
            ctx.move_to(x, y);
        } else {
            ctx.line_to(x, y);
        }
    }
    ctx.stroke();

    // Draw fill gradient under line
    let last_x = width;
    ctx.line_to(last_x, height);
    ctx.line_to(0.0, height);
    ctx.close_path();

    // Convert hex color to rgba for fill
    let fill_color = hex_to_rgba(color, 0.1);
    ctx.set_fill_style_str(&fill_color);
    ctx.fill();
}

/// Convert hex color string to `rgba(r,g,b,a)`.
fn hex_to_rgba(hex: &str, alpha: f64) -> String {
    let hex = hex.trim_start_matches('#');
    let (r, g, b) = match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[1..2], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[2..3], 16).unwrap_or(0);
            (r * 17, g * 17, b * 17)
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            (r, g, b)
        }
        _ => (0, 229, 255), // default cyan
    };
    format!("rgba({},{},{},{})", r, g, b, alpha)
}

/// Setup a canvas for high-DPI rendering.
///
/// Sets the canvas width/height attributes to `rect.width * dpr` and
/// `rect.height * dpr`, then scales the context by `dpr`.
/// Returns `(logical_width, logical_height)`.
pub fn setup_high_dpi(
    canvas: &web_sys::HtmlCanvasElement,
    ctx: &CanvasRenderingContext2d,
) -> (f64, f64) {
    let dpr = web_sys::window()
        .map(|w| w.device_pixel_ratio())
        .unwrap_or(1.0);
    let rect = canvas.get_bounding_client_rect();
    let w = rect.width();
    let h = rect.height();

    canvas.set_width((w * dpr) as u32);
    canvas.set_height((h * dpr) as u32);
    ctx.scale(dpr, dpr).ok();

    (w, h)
}
