//! The Hilbert map.

use super::Point;

/// Node `d` of the order-`order` curve. Two bits of `d` per level, lowest
/// level first; each level may reflect and transpose what is below it.
pub fn point(order: u32, d: u64) -> Point {
    let (mut x, mut y) = (0u64, 0u64);
    let mut t = d;
    let mut s = 1u64;
    for _ in 0..order {
        let rx = 1 & (t / 2);
        let ry = 1 & (t ^ rx);
        if ry == 0 {
            if rx == 1 {
                x = s - 1 - x;
                y = s - 1 - y;
            }
            std::mem::swap(&mut x, &mut y);
        }
        x += s * rx;
        y += s * ry;
        t /= 4;
        s *= 2;
    }
    Point { x, y }
}
