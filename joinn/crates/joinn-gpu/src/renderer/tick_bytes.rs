//! The tick uniform: the camera, and nothing else.

use joinn_frame::Verdict;
use joinn_visual::{Camera, STEPS};

use crate::refuse::refuse;

/// `level step pin_x pin_y ox oy fx fy width height`, then two zeros, as the
/// shader's words. `(ox + fx/2^32, oy + fy/2^32)` is the exact pixel of the
/// anchor's origin. The shader places anchor-relative layout coordinates up to
/// `extent` in magnitude with i32 arithmetic; a camera for which any of that
/// could overflow is refused.
pub(super) fn tick_bytes(camera: &Camera, extent: i64) -> Verdict<[u32; 12]> {
    let (x, y) = camera.pixel_of((0, 0));
    let whole = |p: i128| i32::try_from(p >> 32).ok();
    let frac = |p: i128| (p & 0xFFFF_FFFF) as u32;
    let level = camera.zoom.level;
    let reach = i128::from(extent)
        * i128::from(STEPS + camera.zoom.step)
        * (1i128 << (level - 8).max(0))
        + 2;
    let fits = |o: i32| i128::from(o).abs() + reach < 1i128 << 31;
    let fields = (
        whole(x),
        whole(y),
        i32::try_from(camera.pin.0),
        i32::try_from(camera.pin.1),
    );
    let word = |v: i32| u32::from_le_bytes(v.to_le_bytes());
    match fields {
        (Some(ox), Some(oy), Ok(px), Ok(py)) if fits(ox) && fits(oy) => Verdict::Ok([
            word(level),
            camera.zoom.step,
            word(px),
            word(py),
            word(ox),
            word(oy),
            frac(x),
            frac(y),
            camera.width,
            camera.height,
            0,
            0,
        ]),
        _ => refuse(format!(
            "tick: camera {camera:?} places charts {extent} layout units from the anchor past the shader's 32-bit pixels; acceptance is a camera whose anchor origin and charts lie within 2^31 px"
        )),
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_visual::{Camera, ChartId, FOCUS_UNIT, Zoom};

    use super::tick_bytes;

    fn camera(level: i32, focus: i64) -> Camera {
        Camera {
            zoom: Zoom { level, step: 32 },
            anchor: ChartId(0),
            focus: (focus, 0),
            pin: (640, 360),
            width: 1280,
            height: 720,
        }
    }

    #[test]
    fn the_anchor_origin_splits_into_whole_and_fraction_and_overflow_is_refused() {
        // k = 288/256 at level 0; the origin is 640 - 1.125·3 = 636.625 px.
        let Verdict::Ok(t) = tick_bytes(&camera(0, 3 * FOCUS_UNIT), 5504) else {
            panic!("a near camera must fit");
        };
        assert_eq!(t[..4], [0, 32, 640, 360]);
        assert_eq!((t[4], t[6]), (636, 0xA000_0000));
        assert_eq!((t[5], t[7]), (360, 0));
        assert_eq!(t[8..], [1280, 720, 0, 0]);
        // At level 9 a focus 2^22 layout units out puts the origin past 2^31 px.
        let far = camera(9, (1 << 22) * FOCUS_UNIT);
        assert!(matches!(tick_bytes(&far, 5504), Verdict::Refused(_)));
        // The origin fits, but a chart 2^22 units away does not.
        assert!(matches!(
            tick_bytes(&camera(9, 0), 1 << 22),
            Verdict::Refused(_)
        ));
    }
}
