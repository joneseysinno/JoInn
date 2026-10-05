//! Put a shape through the camera, in 2^-32 px.

use super::{Geom, GeomPx, Shape};
use crate::camera::{Camera, FOCUS_UNIT};

/// Every coordinate goes to focus units (2^-16 layout units) and then through
/// `pin + k·(p − focus)`, `k·n = (256 + step)·n·2^(level + 8)` in 2^-32 px,
/// exact for every level from −4 up. A dot's radius is 2 px. `None` on
/// overflow.
pub(crate) fn geom_px(shape: &Shape, camera: &Camera) -> Option<GeomPx> {
    let base = 256 + i128::from(camera.zoom.step);
    let scale = base.checked_mul(1i128.checked_shl(u32::try_from(camera.zoom.level + 8).ok()?)?)?;
    let len = |n: i128| n.checked_mul(scale);
    let at = |pin: i64, focus: i64, u: i128| -> Option<i128> {
        let pin = i128::from(pin).checked_mul(1 << 32)?;
        pin.checked_add(len(u.checked_sub(i128::from(focus))?)?)
    };
    let point = |u: i128, v: i128| -> Option<(i128, i128)> {
        Some((
            at(camera.pin.0, camera.focus.0, u)?,
            at(camera.pin.1, camera.focus.1, v)?,
        ))
    };
    let unit = i128::from(FOCUS_UNIT);
    let (sixteenth, quarter) = (unit / 16, unit / 4);
    let units = |n: i64| i128::from(n).checked_mul(unit);
    Some(match shape.geom {
        Geom::RoundRect { x, y, w, h, r } => {
            let (hw, hh) = (units(w)? / 2, units(h)? / 2);
            GeomPx::RoundRect {
                c: point(units(x)?.checked_add(hw)?, units(y)?.checked_add(hh)?)?,
                h: (len(hw)?, len(hh)?),
                r: len(units(r)?)?,
            }
        }
        Geom::Circle { x, y, r } => GeomPx::Circle {
            c: point(units(x)?, units(y)?)?,
            r: len(units(r)?)?,
        },
        Geom::Capsule {
            a,
            b,
            half_quarters,
        } => GeomPx::Capsule {
            a: point(units(a.0)?, units(a.1)?)?,
            b: point(units(b.0)?, units(b.1)?)?,
            w: len(i128::from(half_quarters).checked_mul(quarter)?)?,
        },
        Geom::Dot { x, y } => GeomPx::Circle {
            c: point(
                i128::from(x).checked_mul(sixteenth)?,
                i128::from(y).checked_mul(sixteenth)?,
            )?,
            r: 2 << 32,
        },
        Geom::Stroke { a, b, half } => {
            let s = |n: i64| i128::from(n).checked_mul(sixteenth);
            GeomPx::Capsule {
                a: point(s(a.0)?, s(a.1)?)?,
                b: point(s(b.0)?, s(b.1)?)?,
                w: len(s(half)?)?,
            }
        }
    })
}
