//! `cargo xtask pick`: every adapter, every standard viewport, every corpus
//! contact, every measured corpus body, then the plant.

use joinn_frame::Verdict;
use joinn_gpu::{OFFSCREEN_FORMAT, Renderer, adapters, open};
use joinn_visual::{Camera, PickImage, STANDARD_VIEWPORTS, Scene, cpu_pick, fit, shapes_of_tables};

use super::compare::compare;
use super::contact_lines::contact_lines;
use super::contact_subjects::contact_subjects;
use super::frame_median::frame_median;
use super::measured::measured;
use super::owner_name::owner_name;
use super::plant::plant;
use super::{Subject, Tally};

/// The calculator at the four standard viewports, then each corpus body at
/// 1280×720, on every adapter. Any disagreement, an owner that owns no pixel,
/// a count that doesn't sum to the image, or an unrefused plant fails.
pub(crate) fn pick() -> Result<(), String> {
    let calculator_rel = "phase2/calculator.body";
    let subjects = measured()?;
    let calc = match subjects.iter().find(|(rel, _)| rel == calculator_rel) {
        Some((_, Verdict::Ok(scene))) => scene.clone(),
        Some((_, Verdict::Refused(r))) => {
            return Err(format!("pick: {calculator_rel}: {}", r.reason));
        }
        None => return Err(format!("pick: {calculator_rel} is not in the corpus")),
    };
    let cpu_of = |scene: &Scene, camera: &Camera| -> Result<PickImage, String> {
        match cpu_pick(&shapes_of_tables(scene.tables()), camera) {
            Verdict::Ok(p) => Ok(p),
            Verdict::Refused(r) => Err(format!("pick: {}", r.reason)),
        }
    };
    let owners_total = shapes_of_tables(calc.tables()).len();
    let mut calc_cpu: Vec<(Camera, PickImage)> = Vec::new();
    for (w, h) in STANDARD_VIEWPORTS {
        let camera = fit(calc.layout(), w, h);
        calc_cpu.push((camera, cpu_of(&calc, &camera)?));
    }
    let mut bodies: Vec<Subject> = Vec::new();
    for (rel, scene) in subjects {
        let entry = match scene {
            Verdict::Ok(scene) => {
                let camera = fit(scene.layout(), 1280, 720);
                let cpu = cpu_of(&scene, &camera)?;
                Ok((scene, camera, cpu))
            }
            Verdict::Refused(r) => Err(r.reason),
        };
        bodies.push((rel, entry));
    }
    let contacts = contact_subjects()?;
    let all = match adapters() {
        Verdict::Ok(a) => a,
        Verdict::Refused(r) => return Err(r.reason),
    };

    let mut failures: Vec<String> = Vec::new();
    let judge = |label: &str, scene: &Scene, camera: &Camera, t: &Tally, line: &str| {
        let mut out = Vec::new();
        let pixels = camera.width as usize * camera.height as usize;
        if t.agree + t.edge + t.disagree != pixels {
            out.push(format!(
                "{line}: {label}: agree + edge + disagree = {}, not {pixels}",
                t.agree + t.edge + t.disagree
            ));
        }
        for (i, cpu, gpu) in &t.first {
            let (x, y) = (i % camera.width as usize, i / camera.width as usize);
            out.push(format!(
                "{line}: {label}: TRUTH VIOLATION at {x},{y}: cpu {}, gpu {}",
                owner_name(scene, *cpu),
                owner_name(scene, *gpu)
            ));
        }
        out
    };
    let mut plant_ids: Option<(String, Vec<[u32; 4]>)> = None;
    let mut measured_count = 0usize;
    for (n, adapter) in all.iter().enumerate() {
        println!("adapter {}", adapter.line());
        let gpu = match open(adapter) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let mut renderer = match Renderer::new(&gpu, OFFSCREEN_FORMAT) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => return Err(r.reason),
        };
        if let Verdict::Refused(r) = renderer.upload_all(&gpu, calc.tables()) {
            return Err(r.reason);
        }
        for (camera, cpu) in &calc_cpu {
            let picture = match renderer.picture(&gpu, camera) {
                Verdict::Ok(p) => p,
                Verdict::Refused(r) => return Err(r.reason),
            };
            let t = compare(cpu, &picture.ids);
            let label = format!("calculator {}x{}", camera.width, camera.height);
            println!(
                "{label}: agree {}, edge {}, disagree {}, owners {}/{owners_total}",
                t.agree,
                t.edge,
                t.disagree,
                t.owners.len()
            );
            failures.extend(judge(&label, &calc, camera, &t, adapter.line()));
            if t.owners.len() != owners_total {
                failures.push(format!(
                    "{}: {label}: {} of {owners_total} owners own a non-edge pixel",
                    adapter.line(),
                    t.owners.len()
                ));
            }
            if n == 0 && (camera.width, camera.height) == (1280, 720) {
                plant_ids = Some((adapter.line().to_owned(), picture.ids));
            }
        }
        contact_lines(&gpu, adapter.line(), &contacts, &mut failures)?;
        if let Some((camera, _)) = calc_cpu
            .iter()
            .find(|(c, _)| (c.width, c.height) == (1280, 720))
        {
            let median = frame_median(&mut renderer, &gpu, camera)?;
            println!("frame median {median} (calculator 1280x720; information, not a check)");
        }
        let mut agreed = 0usize;
        for (rel, entry) in &bodies {
            let (scene, camera, cpu) = match entry {
                Ok(e) => e,
                Err(reason) => {
                    println!("{rel}: not measured: {reason}");
                    continue;
                }
            };
            let mut r = match Renderer::new(&gpu, OFFSCREEN_FORMAT) {
                Verdict::Ok(r) => r,
                Verdict::Refused(r) => return Err(r.reason),
            };
            if let Verdict::Refused(r) = r.upload_all(&gpu, scene.tables()) {
                return Err(r.reason);
            }
            let picture = match r.picture(&gpu, camera) {
                Verdict::Ok(p) => p,
                Verdict::Refused(r) => return Err(r.reason),
            };
            let t = compare(cpu, &picture.ids);
            println!(
                "{rel}: agree {}, edge {}, disagree {}",
                t.agree, t.edge, t.disagree
            );
            failures.extend(judge(rel, scene, camera, &t, adapter.line()));
            if t.disagree == 0 {
                agreed += 1;
            }
        }
        measured_count = if n == 0 {
            agreed
        } else {
            measured_count.min(agreed)
        };
    }

    let Some((plant_on, ids)) = plant_ids else {
        return Err(
            "pick: no 1280x720 calculator picture on the first adapter for the plant".into(),
        );
    };
    let Some((camera, _)) = calc_cpu
        .iter()
        .find(|(c, _)| (c.width, c.height) == (1280, 720))
    else {
        return Err("pick: no 1280x720 camera for the plant".into());
    };
    let planted = plant(&calc, camera, &ids)?;
    if planted == 0 {
        failures.push(format!(
            "planted: port sum@1 moved one unit on {plant_on}: disagree 0; the plant was not refused"
        ));
    } else {
        println!(
            "planted: port sum@1 moved one unit, cpu_pick against the GPU on {plant_on}: disagree {planted}; refused as truth violation (ok)"
        );
    }
    if !failures.is_empty() {
        for f in &failures {
            println!("{f}");
        }
        return Err(format!("pick: {} failure(s)", failures.len()));
    }
    println!(
        "pick: {} adapter(s), {measured_count} subject(s) agree; planted disagreement: refused as truth violation (ok)",
        all.len()
    );
    Ok(())
}
