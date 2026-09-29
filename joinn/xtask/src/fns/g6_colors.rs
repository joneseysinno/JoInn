//! Probe colors at 1280×720 against the owning row's style.

use joinn_frame::Verdict;
use joinn_visual::{Scene, cpu_pick, fit, shapes_of_tables};

use super::g6_style_rgba::g6_style_rgba;

/// §2.13's fourteen probes, in that order.
const PROBES: [(u32, u32); 14] = [
    (136, 80),
    (360, 220),
    (360, 500),
    (920, 276),
    (192, 220),
    (528, 220),
    (192, 500),
    (528, 500),
    (752, 220),
    (752, 332),
    (1088, 220),
    (640, 220),
    (640, 416),
    (0, 0),
];

/// The color target at each probe equals the row's RGBA8. The error names the
/// event, the pixel, the adapter, and both colors.
pub(crate) fn g6_colors(
    label: &str,
    scene: &Scene,
    color: &[u8],
    adapter: &str,
) -> Result<(), String> {
    let camera = fit(scene.layout(), 1280, 720);
    let image = match cpu_pick(&shapes_of_tables(scene.tables()), &camera) {
        Verdict::Ok(image) => image,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let hex = |bytes: &[u8]| -> String { bytes.iter().map(|b| format!("{b:02X}")).collect() };
    let stride = camera.width as usize;
    for (x, y) in PROBES {
        let i = y as usize * stride + x as usize;
        let Some(pick) = image.pixels.get(i) else {
            return Err(format!(
                "event {label} pixel {x},{y} is outside the {0}x{1} pick; acceptance is a probe inside it",
                camera.width, camera.height
            ));
        };
        let want = g6_style_rgba(scene, pick)?;
        let off = i * 4;
        let Some(got) = color.get(off..off + 4) else {
            return Err(format!(
                "event {label} pixel {x},{y} adapter {adapter}: the color target has no texel there"
            ));
        };
        if got != want {
            return Err(format!(
                "event {label} pixel {x},{y} adapter {adapter}: {} {}",
                hex(got),
                hex(&want)
            ));
        }
    }
    Ok(())
}
