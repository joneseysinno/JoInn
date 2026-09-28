//! Put a shape through the camera, in sixteenths of a pixel.

use super::{Geom, Geom16, Shape};
use crate::camera::Camera;

/// A layout point `u` is pixel `o + k·u`, so sixteenth `16·(o + k·u)`. A length
/// `n` is `16·k·n`; a quarter-unit half-width `q` is `4·k·q`. `None` on overflow.
pub(crate) fn geom16(shape: &Shape, camera: &Camera) -> Option<Geom16> {
    let k = i128::from(camera.k);
    let at = |o: i64, u: i64| -> Option<i128> {
        k.checked_mul(i128::from(u))?
            .checked_add(i128::from(o))?
            .checked_mul(16)
    };
    let len = |n: i64| -> Option<i128> { k.checked_mul(i128::from(n))?.checked_mul(16) };
    Some(match shape.geom {
        Geom::RoundRect { x, y, w, h, r } => {
            let hx = k.checked_mul(i128::from(w))?.checked_mul(8)?;
            let hy = k.checked_mul(i128::from(h))?.checked_mul(8)?;
            Geom16::RoundRect {
                c: (
                    at(camera.ox, x)?.checked_add(hx)?,
                    at(camera.oy, y)?.checked_add(hy)?,
                ),
                h: (hx, hy),
                r: len(r)?,
            }
        }
        Geom::Circle { x, y, r } => Geom16::Circle {
            c: (at(camera.ox, x)?, at(camera.oy, y)?),
            r: len(r)?,
        },
        Geom::Capsule {
            a,
            b,
            half_quarters,
        } => Geom16::Capsule {
            a: (at(camera.ox, a.0)?, at(camera.oy, a.1)?),
            b: (at(camera.ox, b.0)?, at(camera.oy, b.1)?),
            w: k.checked_mul(i128::from(half_quarters))?.checked_mul(4)?,
        },
    })
}
