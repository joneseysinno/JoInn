//! Gate 6 item 3: a click names an address; a stale click is refused.

use joinn_frame::Verdict;
use joinn_gpu::{OFFSCREEN_FORMAT, Renderer, open, read_texel};
use joinn_visual::{Scene, fit};

use super::g6_adapters::g6_adapters;
use super::load_calculator::load_calculator;
use super::mutate::{Mutation, mutate};
use super::subject::Subject;

/// §2.13, which item 3 asserts on every adapter through `read_texel`.
const PROBES: [(u32, u32, &str); 14] = [
    (136, 80, "body membrane"),
    (360, 220, "body.cli_a"),
    (360, 500, "body.cli_b"),
    (920, 276, "body.sum"),
    (192, 220, "body.cli_a@0"),
    (528, 220, "body.cli_a@1"),
    (192, 500, "body.cli_b@0"),
    (528, 500, "body.cli_b@1"),
    (752, 220, "body.sum@0"),
    (752, 332, "body.sum@1"),
    (1088, 220, "body.sum@2"),
    (640, 220, "body wire cli_a@1 -> sum@0"),
    (640, 416, "body wire cli_b@1 -> sum@1"),
    (0, 0, "background"),
];

/// Probe texels print §2.13's owners. Replacing with `DropGenome(cli_b)` writes
/// 5 rows, stales that cell's old ID, and leaves `body.cli_a` resolvable.
pub(crate) fn g6_click() -> bool {
    let Some(adapters) = g6_adapters() else {
        return false;
    };
    let Ok((body, cells)) = load_calculator() else {
        println!("calculator did not load; acceptance is corpus/phase2/calculator.body");
        return false;
    };
    let mut scene = match Scene::grow("body", &body, &cells) {
        Verdict::Ok(scene) => scene,
        Verdict::Refused(r) => {
            println!("{}", r.reason);
            return false;
        }
    };
    let camera = fit(scene.layout(), 1280, 720);
    for adapter in &adapters {
        let gpu = match open(adapter) {
            Verdict::Ok(gpu) => gpu,
            Verdict::Refused(r) => {
                println!("{}", r.reason);
                return false;
            }
        };
        let mut renderer = match Renderer::new(&gpu, OFFSCREEN_FORMAT) {
            Verdict::Ok(renderer) => renderer,
            Verdict::Refused(r) => {
                println!("{}", r.reason);
                return false;
            }
        };
        if let Verdict::Refused(r) = renderer.upload_all(&gpu, scene.tables()) {
            println!("{}", r.reason);
            return false;
        }
        if let Verdict::Refused(r) = renderer.frame(&gpu, &camera) {
            println!("{}", r.reason);
            return false;
        }
        let Some(ids) = renderer.id_texture() else {
            println!(
                "{}: the frame left no ID target; acceptance is a drawn frame",
                adapter.line()
            );
            return false;
        };
        for (x, y, want) in PROBES {
            let texel = match read_texel(&gpu, ids, x, y) {
                Verdict::Ok(texel) => texel,
                Verdict::Refused(r) => {
                    println!("{}", r.reason);
                    return false;
                }
            };
            let got = match scene.resolve(texel) {
                Verdict::Ok(owner) => scene.print_owner(&owner),
                Verdict::Refused(r) => r.reason,
            };
            if got != want {
                println!(
                    "{}: probe {x},{y}: {got}; acceptance is {want}",
                    adapter.line()
                );
                return false;
            }
        }
    }
    let (old_cli_b, old_cli_a) = {
        let cell_id = |printed: &str| -> Option<[u32; 4]> {
            scene
                .tables()
                .cell
                .iter()
                .enumerate()
                .find_map(|(slot, row)| {
                    let id = [row.body + 1, slot as u32 + 1, 0, row.generation];
                    match scene.resolve(id) {
                        Verdict::Ok(owner) if scene.print_owner(&owner) == printed => Some(id),
                        _ => None,
                    }
                })
        };
        (cell_id("body.cli_b"), cell_id("body.cli_a"))
    };
    let Some(old_cli_b) = old_cli_b else {
        println!("no cell prints body.cli_b; acceptance is the calculator's cli_b");
        return false;
    };
    let Some(old_cli_a) = old_cli_a else {
        println!("no cell prints body.cli_a; acceptance is the calculator's cli_a");
        return false;
    };
    let mutant = match mutate(&Subject::Body(body), &Mutation::DropGenome("cli_b")) {
        Verdict::Ok(Subject::Body(body)) => body,
        Verdict::Ok(_) => {
            println!("DropGenome(cli_b) did not yield a body");
            return false;
        }
        Verdict::Refused(r) => {
            println!("{}", r.reason);
            return false;
        }
    };
    let delta = match scene.replace(&mutant, &cells) {
        Verdict::Ok(delta) => delta,
        Verdict::Refused(r) => {
            println!("{}", r.reason);
            return false;
        }
    };
    if delta.rows.len() != 5 {
        println!("replace wrote {} row(s); acceptance is 5", delta.rows.len());
        return false;
    }
    let [_, g, _, generation] = old_cli_b;
    let slot = g.wrapping_sub(1);
    if generation != 1 {
        println!("old cli_b generation is {generation}; acceptance is generation 1");
        return false;
    }
    let want = format!(
        "stale pick: cell slot {slot} generation {generation}, now 2; acceptance is a pick taken from the current picture"
    );
    match scene.resolve(old_cli_b) {
        Verdict::Refused(r) if r.reason == want => {}
        Verdict::Refused(r) => {
            println!("{}", r.reason);
            return false;
        }
        Verdict::Ok(owner) => {
            println!(
                "old cli_b resolved to {}; acceptance is a stale pick",
                scene.print_owner(&owner)
            );
            return false;
        }
    }
    match scene.resolve(old_cli_a) {
        Verdict::Ok(owner) if scene.print_owner(&owner) == "body.cli_a" => true,
        Verdict::Ok(owner) => {
            println!(
                "old cli_a resolved to {}; acceptance is body.cli_a",
                scene.print_owner(&owner)
            );
            false
        }
        Verdict::Refused(r) => {
            println!("{}", r.reason);
            false
        }
    }
}
