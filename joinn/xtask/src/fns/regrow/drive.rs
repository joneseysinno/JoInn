//! Drive a script on a fresh calculator, deltas to the GPU, regrow after each.

use std::collections::BTreeMap;

use joinn_dna::{Body, Cell};
use joinn_frame::{Hash, Verdict};
use joinn_gpu::{Gpu, Renderer};
use joinn_live::BodyState;
use joinn_visual::{Scene, fit, table_bytes};

use super::Input;
use super::cleared_elsewhere::cleared_elsewhere;
use super::event::event;

/// Grows the scene and uploads it whole, then for each input: runs it, takes
/// the pending delta, applies it to the renderer and draws, regrows from the live
/// state, and compares. One line per event goes to `lines`, each broken
/// promise to `failures`. Returns the delta-built scene and the state.
pub(super) fn drive(
    gpu: &Gpu,
    renderer: &mut Renderer,
    (body, cells): (&Body, &BTreeMap<Hash, Cell>),
    ports: &BTreeMap<String, usize>,
    inputs: &[Input],
    first: usize,
    lines: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Result<(Scene, BodyState), String> {
    let mut scene = match Scene::grow("body", body, cells) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(format!("regrow: {}", r.reason)),
    };
    let mut state =
        match BodyState::new(body.clone(), cells.clone(), joinn_prim::sealed_natives(), 1) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => return Err(format!("regrow: {}", r.reason)),
        };
    if let Verdict::Refused(r) = renderer.upload_all(gpu, scene.tables()) {
        return Err(r.reason);
    }
    for (epoch, (instance, text)) in (0u64..).zip(inputs) {
        let n = first + epoch as usize + 1;
        let before = scene.tables().clone();
        let (delta, touched) = event(
            &mut scene,
            &mut state,
            (body, cells),
            (instance, text),
            epoch,
        )?;
        let extra = cleared_elsewhere(&scene, &before, &touched);
        let base: usize = touched
            .iter()
            .map(|i| 1 + ports.get(i).copied().unwrap_or(0))
            .sum();
        let bound = base + extra;
        let pending = scene.take_pending().unwrap_or_default();
        if pending != delta {
            failures.push(format!(
                "event {n}: the pending delta {:?} is not the run's delta {:?}",
                pending.rows, delta.rows
            ));
        }
        let up = match renderer.apply(gpu, scene.tables(), &pending) {
            Verdict::Ok(u) => u,
            Verdict::Refused(r) => return Err(r.reason),
        };
        if let Verdict::Refused(r) = renderer.frame(gpu, &fit(scene.layout(), 1280, 720)) {
            return Err(r.reason);
        }
        if up.rows != delta.rows.len() {
            failures.push(format!(
                "event {n}: the renderer wrote {} row(s) for a delta of {}",
                up.rows,
                delta.rows.len()
            ));
        }
        let regrown = match Scene::regrow("body", body, cells, &state) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => return Err(format!("regrow: {}", r.reason)),
        };
        let equal = table_bytes(scene.tables()) == table_bytes(regrown.tables());
        let bound_text = if extra > 0 {
            format!("{base} + {extra}")
        } else {
            base.to_string()
        };
        lines.push(format!(
            "event {n} {instance}@0 {text:?}: touched {}, rows {} (bound {bound_text}), bytes {}, {}",
            touched.join(", "),
            delta.rows.len(),
            up.bytes,
            if equal {
                "tables equal regrow"
            } else {
                "TABLES DIFFER FROM REGROW"
            }
        ));
        if !equal {
            failures.push(format!(
                "event {n}: delta-built tables differ from regrow (V122)"
            ));
        }
        if delta.rows.len() > bound {
            failures.push(format!(
                "event {n}: {} row(s) break the V121 bound of {bound}",
                delta.rows.len()
            ));
        }
        if scene.take_pending().is_some() {
            failures.push(format!(
                "event {n}: rows still pending after the draw (V12)"
            ));
        }
    }
    Ok((scene, state))
}
