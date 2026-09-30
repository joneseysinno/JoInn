//! §2.10's colors at the contact calculator's probes, after each event.

/// The probes, in §2.10's column order, then the surface and the background.
const PIXELS: [(u32, u32, &str); 8] = [
    (448, 264, "cli_a"),
    (448, 456, "cli_b"),
    (832, 360, "sum"),
    (256, 264, "cli_a@0"),
    (256, 456, "cli_b@0"),
    (1024, 264, "sum@2"),
    (192, 104, "surface"),
    (0, 0, "background"),
];
/// §2.10's table with every `.` filled in: grow, then events 1, 2 and 3.
/// Alpha is `FF` everywhere.
const RGB: [[u32; 8]; 4] = [
    [
        0x2F5D8A, 0x2F5D8A, 0x39414D, 0xC9CED6, 0xC9CED6, 0xC9CED6, 0x22262E, 0x15171C,
    ],
    [
        0xB03A2E, 0x2F5D8A, 0x39414D, 0xF2B134, 0xC9CED6, 0xC9CED6, 0x22262E, 0x15171C,
    ],
    [
        0x2F5D8A, 0x2F5D8A, 0x39414D, 0xF2B134, 0xC9CED6, 0xC9CED6, 0x22262E, 0x15171C,
    ],
    [
        0x2F5D8A, 0x2F5D8A, 0x2F5D8A, 0xF2B134, 0xF2B134, 0xF2B134, 0x22262E, 0x15171C,
    ],
];

/// Step `step` (0 is grow) of a 1280-wide RGBA8 color target against the table.
/// The error names the step, the pixel, the adapter, the probe, and both colors.
pub(crate) fn g7_colors(
    step: usize,
    label: &str,
    color: &[u8],
    width: u32,
    adapter: &str,
) -> Result<(), String> {
    let Some(row) = RGB.get(step) else {
        return Err(format!(
            "{label}: step {step} has no row in §2.10's table; acceptance is grow and three events"
        ));
    };
    let hex = |bytes: &[u8]| -> String { bytes.iter().map(|b| format!("{b:02X}")).collect() };
    for ((x, y, name), rgb) in PIXELS.iter().zip(row) {
        let want = [(rgb >> 16) as u8, (rgb >> 8) as u8, *rgb as u8, 0xFF];
        let off = (*y as usize * width as usize + *x as usize) * 4;
        let Some(got) = color.get(off..off + 4) else {
            return Err(format!(
                "{label} pixel {x},{y} adapter {adapter}: the color target has no texel there"
            ));
        };
        if got != want {
            return Err(format!(
                "{label} pixel {x},{y} adapter {adapter}: {name} {}, want {}",
                hex(got),
                hex(&want)
            ));
        }
    }
    Ok(())
}
