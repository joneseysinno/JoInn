//! Gate 6 item 1: every non-edge pixel has one owner, and both pickers name it.

use joinn_frame::Verdict;
use joinn_gpu::{OFFSCREEN_FORMAT, Renderer, open};
use joinn_visual::{
    STANDARD_VIEWPORTS, Scene, cpu_pick, cpu_pick_reference, fit, shapes_of_tables,
};

use super::g6_adapters::g6_adapters;
use super::g6_agree::g6_agree;
use super::load_calculator::load_calculator;

/// On every adapter, at every standard viewport, `cpu_pick` matches the reference
/// and the GPU ID target. All 13 owners own a non-edge pixel. No adapter fails it.
pub(crate) fn g6_owners() -> bool {
    let Some(adapters) = g6_adapters() else {
        return false;
    };
    let Ok((body, cells)) = load_calculator() else {
        println!("calculator did not load; acceptance is corpus/phase2/calculator.body");
        return false;
    };
    let scene = match Scene::grow("body", &body, &cells) {
        Verdict::Ok(scene) => scene,
        Verdict::Refused(r) => {
            println!("{}", r.reason);
            return false;
        }
    };
    let shapes = shapes_of_tables(scene.tables());
    if shapes.len() != 13 {
        println!(
            "the calculator has {} owners; acceptance is 13",
            shapes.len()
        );
        return false;
    }
    for (w, h) in STANDARD_VIEWPORTS {
        let camera = fit(scene.layout(), w, h);
        let cpu = match cpu_pick(&shapes, &camera) {
            Verdict::Ok(image) => image,
            Verdict::Refused(r) => {
                println!("{}", r.reason);
                return false;
            }
        };
        let reference = match cpu_pick_reference(&shapes, &camera) {
            Verdict::Ok(image) => image,
            Verdict::Refused(r) => {
                println!("{}", r.reason);
                return false;
            }
        };
        if cpu.pixels != reference.pixels {
            let (x, y) = cpu
                .pixels
                .iter()
                .zip(&reference.pixels)
                .enumerate()
                .find(|(_, (a, b))| a != b)
                .map(|(i, _)| (i % w as usize, i / w as usize))
                .unwrap_or((0, 0));
            println!("cpu_pick and cpu_pick_reference differ at {x},{y} ({w}x{h})");
            return false;
        }
    }
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
        for (w, h) in STANDARD_VIEWPORTS {
            let camera = fit(scene.layout(), w, h);
            let cpu = match cpu_pick(&shapes, &camera) {
                Verdict::Ok(image) => image,
                Verdict::Refused(r) => {
                    println!("{}", r.reason);
                    return false;
                }
            };
            let picture = match renderer.picture(&gpu, &camera) {
                Verdict::Ok(picture) => picture,
                Verdict::Refused(r) => {
                    println!("{}", r.reason);
                    return false;
                }
            };
            if let Err(msg) = g6_agree(&scene, &camera, &cpu, &picture.ids, adapter.line()) {
                println!("{msg}");
                return false;
            }
        }
    }
    true
}
