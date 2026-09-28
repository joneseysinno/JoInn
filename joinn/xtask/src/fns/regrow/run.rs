//! `cargo xtask regrow`: §2.12's script, Amendment A4's event, the camera and
//! idle checks, then VH2 on every adapter, then the plant.

use std::collections::BTreeMap;

use joinn_frame::Verdict;
use joinn_gpu::{OFFSCREEN_FORMAT, Renderer, adapters, open};
use joinn_link::instance_ports;
use joinn_live::BodyState;
use joinn_visual::{FILLED, REFUSED, Scene, fit, table_bytes};

use super::Input;
use super::drive::drive;
use crate::fns::corpus_bodies;
use crate::fns::pick::port_slot;

/// §2.12: refused at the membrane, then `2`, then `3` (`sum` fires).
const SCRIPT: [Input; 3] = [("cli_a", "two"), ("cli_a", "2"), ("cli_b", "3")];
/// Amendment A4, on a fresh calculator: the second run clears `cli_b`'s
/// refused bit without touching `cli_b`.
const FRESH: [Input; 2] = [("cli_b", "x"), ("cli_a", "2")];

/// Any table inequality, a broken bound, rows pending when idle, a GPU image
/// that moves when regrown, or an unrefused plant fails.
pub(crate) fn regrow() -> Result<(), String> {
    let rel = "phase2/calculator.body";
    let (body, cells) = match corpus_bodies()?.into_iter().find(|(r, _)| r == rel) {
        Some((_, Verdict::Ok(pair))) => pair,
        Some((_, Verdict::Refused(r))) => return Err(format!("regrow: {rel}: {}", r.reason)),
        None => return Err(format!("regrow: {rel} is not in the corpus")),
    };
    let ports: BTreeMap<String, usize> = match instance_ports(&body, &cells) {
        Verdict::Ok(p) => p.into_iter().map(|(i, ps)| (i, ps.len())).collect(),
        Verdict::Refused(r) => return Err(format!("regrow: {}", r.reason)),
    };
    let all = match adapters() {
        Verdict::Ok(a) => a,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let mut failures: Vec<String> = Vec::new();
    let mut kept: Option<(Scene, BodyState)> = None;
    for (n, adapter) in all.iter().enumerate() {
        let gpu = match open(adapter) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let mut renderer = match Renderer::new(&gpu, OFFSCREEN_FORMAT) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let mut lines = Vec::new();
        let (mut scene, state) = drive(
            &gpu,
            &mut renderer,
            (&body, &cells),
            &ports,
            &SCRIPT,
            0,
            &mut lines,
            &mut failures,
        )?;
        let camera = fit(scene.layout(), 1280, 720);
        if n == 0 {
            let t = scene.tables();
            let sum_out = port_slot(&scene, "body.sum@2")
                .and_then(|s| t.port.get(s as usize))
                .is_some_and(|p| p.flags & FILLED != 0);
            if !sum_out {
                failures.push("after the script, sum@2 is not filled".into());
            }
            if t.cell.iter().any(|c| c.flags & REFUSED != 0) {
                failures.push("after the script, a cell is still refused".into());
            }
            lines.push("fresh calculator (Amendment A4):".into());
            let mut fresh = match Renderer::new(&gpu, OFFSCREEN_FORMAT) {
                Verdict::Ok(r) => r,
                Verdict::Refused(r) => return Err(r.reason),
            };
            drive(
                &gpu,
                &mut fresh,
                (&body, &cells),
                &ports,
                &FRESH,
                SCRIPT.len(),
                &mut lines,
                &mut failures,
            )?;
            for line in &lines {
                println!("{line}");
            }
            let bytes = table_bytes(scene.tables());
            for (w, h) in [(640, 360), (1920, 1080)] {
                if let Verdict::Refused(r) = renderer.frame(&gpu, &fit(scene.layout(), w, h)) {
                    return Err(r.reason);
                }
            }
            let rows = scene.take_pending().map_or(0, |d| d.rows.len());
            if rows != 0 || table_bytes(scene.tables()) != bytes {
                failures.push(format!("a camera change wrote {rows} row(s) (V121)"));
            }
            println!("camera change: rows {rows}");
            match scene.take_pending() {
                None => println!("idle tick: nothing to draw"),
                Some(d) => failures.push(format!(
                    "an idle tick has {} row(s) to draw (V12)",
                    d.rows.len()
                )),
            }
        }
        let before = match renderer.picture(&gpu, &camera) {
            Verdict::Ok(p) => p,
            Verdict::Refused(r) => return Err(r.reason),
        };
        drop(renderer);
        drop(gpu);
        let gpu = match open(adapter) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let regrown = match Scene::regrow("body", &body, &cells, &state) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => return Err(format!("regrow: {}", r.reason)),
        };
        let mut renderer = match Renderer::new(&gpu, OFFSCREEN_FORMAT) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => return Err(r.reason),
        };
        if let Verdict::Refused(r) = renderer.upload_all(&gpu, regrown.tables()) {
            return Err(r.reason);
        }
        let after = match renderer.picture(&gpu, &camera) {
            Verdict::Ok(p) => p,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let word = |same: bool| if same { "identical" } else { "differ" };
        let (color, ids) = (before.color == after.color, before.ids == after.ids);
        println!(
            "regrow {}: color {}, ids {}",
            adapter.line(),
            word(color),
            word(ids)
        );
        if !(color && ids) {
            failures.push(format!(
                "regrow {}: the regrown GPU state drew different bytes (VH2)",
                adapter.line()
            ));
        }
        if n == 0 {
            kept = Some((scene, state));
        }
    }

    let Some((scene, state)) = kept else {
        return Err("regrow: no adapter ran the script".into());
    };
    let regrown = match Scene::regrow("body", &body, &cells, &state) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(format!("regrow: {}", r.reason)),
    };
    let slot = port_slot(&regrown, "body.cli_a@0")
        .ok_or("regrow plant: no port body.cli_a@0 in the regrown tables")?;
    let mut flipped = regrown.tables().clone();
    let row = flipped
        .port
        .get_mut(slot as usize)
        .ok_or("regrow plant: cli_a@0's slot is past the table")?;
    row.flags ^= FILLED;
    if table_bytes(&flipped) == table_bytes(scene.tables()) {
        failures.push("planted: flipping cli_a@0's filled bit left the tables equal; the plant was not refused".into());
    } else {
        println!(
            "planted: regrown tables with cli_a@0's filled bit flipped differ from the delta-built tables; refused (ok)"
        );
    }
    if !failures.is_empty() {
        for f in &failures {
            println!("{f}");
        }
        return Err(format!("regrow: {} failure(s)", failures.len()));
    }
    println!(
        "regrow: {} adapter(s); planted difference: refused (ok)",
        all.len()
    );
    Ok(())
}
