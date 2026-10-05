//! A cursor position as a whole pixel.

/// Physical pixels, floored. `None` off any representable pixel.
pub(super) fn pixel(cursor: Option<(f64, f64)>) -> Option<(i64, i64)> {
    let (x, y) = cursor?;
    let floor = |p: f64| -> Option<i64> {
        let whole = p.floor();
        if !(-1e15..1e15).contains(&whole) {
            return None;
        }
        Some(whole as i64)
    };
    Some((floor(x)?, floor(y)?))
}
