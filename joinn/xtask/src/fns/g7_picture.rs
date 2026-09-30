//! Gate 7 item 3: the body is drawn as cells in contact.

use joinn_frame::Verdict;
use joinn_gpu::{OFFSCREEN_FORMAT, Renderer, open};
use joinn_visual::{Scene, cpu_pick_reference, fit, shapes_of_tables};

use super::g6_adapters::g6_adapters;
use super::g6_drive::g6_drive;
use super::g7_colors::g7_colors;
use super::g7_load::g7_load;
use super::g7_probes::g7_probes;
use super::pick::{contact_lines, contact_subjects};
use super::regrow::Form;

/// On every adapter and standard viewport the GPU agrees with `cpu_pick`, which
/// agrees with `cpu_pick_reference`; 7/7 owners and no link. The probes and
/// seams hold, the script keeps `regrow_contact` equality within the bound,
/// §2.10's colors hold after each event, and a dropped device redraws the same
/// bytes.
pub(crate) fn g7_picture() -> bool {
    let Some(adapters) = g6_adapters() else {
        return false;
    };
    let (contact, lowered, cells) = match g7_load() {
        Ok(loaded) => loaded,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let subjects = match contact_subjects() {
        Ok(all) => all
            .into_iter()
            .filter(|(label, _, _)| label == "calculator.contact")
            .collect::<Vec<_>>(),
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let mut failures = Vec::new();
    if subjects.is_empty() {
        failures.push("calculator.contact has no picture to judge".to_owned());
    }
    for (label, scene, picks) in &subjects {
        let shapes = shapes_of_tables(scene.tables());
        for (camera, cpu) in picks {
            match cpu_pick_reference(&shapes, camera) {
                Verdict::Ok(reference) if reference == *cpu => {}
                Verdict::Ok(_) => failures.push(format!(
                    "{label} {}x{}: cpu_pick differs from cpu_pick_reference",
                    camera.width, camera.height
                )),
                Verdict::Refused(r) => failures.push(r.reason),
            }
        }
        failures.extend(g7_probes(scene));
    }
    let driven = match g6_drive(Form::Contact(&contact), &lowered, &cells) {
        Ok(driven) => driven,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    failures.extend(driven.failures.iter().cloned());
    for adapter in &adapters {
        let gpu = match open(adapter) {
            Verdict::Ok(gpu) => gpu,
            Verdict::Refused(r) => {
                println!("{}", r.reason);
                return false;
            }
        };
        if let Err(e) = contact_lines(&gpu, adapter.line(), &subjects, &mut failures) {
            println!("{e}");
            return false;
        }
        let mut renderer = match Renderer::new(&gpu, OFFSCREEN_FORMAT) {
            Verdict::Ok(renderer) => renderer,
            Verdict::Refused(r) => {
                println!("{}", r.reason);
                return false;
            }
        };
        let mut last = None;
        for (n, step) in driven.steps.iter().enumerate() {
            let written = match &step.delta {
                Some(delta) => renderer.apply(&gpu, step.scene.tables(), delta),
                None => renderer.upload_all(&gpu, step.scene.tables()),
            };
            if let Verdict::Refused(r) = written {
                println!("{}", r.reason);
                return false;
            }
            let camera = fit(step.scene.layout(), 1280, 720);
            let picture = match renderer.picture(&gpu, &camera) {
                Verdict::Ok(picture) => picture,
                Verdict::Refused(r) => {
                    println!("{}", r.reason);
                    return false;
                }
            };
            if let Err(msg) = g7_colors(n, &step.label, &picture.color, 1280, adapter.line()) {
                failures.push(msg);
            }
            last = Some(picture);
        }
        let Some(before) = last else {
            println!("{}: the script drew no picture", adapter.line());
            return false;
        };
        drop(renderer);
        drop(gpu);
        let gpu = match open(adapter) {
            Verdict::Ok(gpu) => gpu,
            Verdict::Refused(r) => {
                println!("{}", r.reason);
                return false;
            }
        };
        let regrown = match Scene::regrow_contact("body", &contact, &cells, &driven.state) {
            Verdict::Ok(scene) => scene,
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
        if let Verdict::Refused(r) = renderer.upload_all(&gpu, regrown.tables()) {
            println!("{}", r.reason);
            return false;
        }
        let after = match renderer.picture(&gpu, &fit(regrown.layout(), 1280, 720)) {
            Verdict::Ok(picture) => picture,
            Verdict::Refused(r) => {
                println!("{}", r.reason);
                return false;
            }
        };
        if before.color != after.color || before.ids != after.ids {
            failures.push(format!(
                "{}: regrown color or ID bytes differ from the delta-built picture (VH2)",
                adapter.line()
            ));
        }
    }
    for f in &failures {
        println!("{f}");
    }
    failures.is_empty()
}
