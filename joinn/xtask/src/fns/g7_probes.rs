//! §2.10's probes and seams on the contact calculator at 1280×720.

use joinn_frame::Verdict;
use joinn_visual::{Pick, Scene, cpu_pick, fit, shapes_of_tables};

use super::pick::owner_name;

/// §2.10's probe table: pixel and printed owner.
const PROBES: [(u32, u32, &str); 8] = [
    (192, 104, "body surface"),
    (448, 264, "body.cli_a"),
    (448, 456, "body.cli_b"),
    (832, 360, "body.sum"),
    (256, 264, "body.cli_a@0"),
    (256, 456, "body.cli_b@0"),
    (1024, 264, "body.sum@2"),
    (0, 0, "background"),
];
/// Column 639 | 640 at these rows is `cli_a` | `sum`.
const SEAM_ROWS: [u32; 3] = [232, 264, 296];
/// Row 359 / 360 at these columns is `cli_a` / `cli_b`.
const SEAM_COLUMNS: [u32; 3] = [320, 448, 576];

/// Every probe names its owner, and each seam crosses from one cell straight
/// into the next: the cells touch. Each break is one failure.
pub(crate) fn g7_probes(scene: &Scene) -> Vec<String> {
    let camera = fit(scene.layout(), 1280, 720);
    let image = match cpu_pick(&shapes_of_tables(scene.tables()), &camera) {
        Verdict::Ok(image) => image,
        Verdict::Refused(r) => return vec![r.reason],
    };
    let name = |x: u32, y: u32| -> String {
        let i = y as usize * camera.width as usize + x as usize;
        match image.pixels.get(i) {
            Some(Pick::Owned(id)) => owner_name(scene, *id),
            Some(Pick::Background) => "background".to_owned(),
            Some(Pick::Edge) => "edge".to_owned(),
            None => "outside the image".to_owned(),
        }
    };
    let mut wants: Vec<(u32, u32, &str)> = PROBES.to_vec();
    for y in SEAM_ROWS {
        wants.push((639, y, "body.cli_a"));
        wants.push((640, y, "body.sum"));
    }
    for x in SEAM_COLUMNS {
        wants.push((x, 359, "body.cli_a"));
        wants.push((x, 360, "body.cli_b"));
    }
    let mut failures = Vec::new();
    for (x, y, want) in wants {
        let got = name(x, y);
        if got != want {
            failures.push(format!("probe {x},{y}: {got}, want {want}"));
        }
    }
    failures
}
