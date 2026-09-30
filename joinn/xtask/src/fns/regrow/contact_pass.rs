//! The contact calculator on one adapter: the script, Amendment A4's pair on the
//! first adapter, then VH2.

use std::collections::BTreeMap;

use joinn_dna::{Body, Cell, Contact};
use joinn_frame::{Hash, Verdict};
use joinn_gpu::{GpuAdapter, OFFSCREEN_FORMAT, Renderer, open};
use joinn_visual::fit;

use super::drive::drive;
use super::{Form, Input};

/// `body` is the contact's lowered body; `ports` counts each instance's drawn
/// (surface) ports. Event lines print only on the first adapter (`n == 0`), as
/// the wired script's do; the VH2 line prints on every adapter.
pub(super) fn contact_pass(
    adapter: &GpuAdapter,
    n: usize,
    (contact, body, cells): (&Contact, &Body, &BTreeMap<Hash, Cell>),
    ports: &BTreeMap<String, usize>,
    (script, fresh): (&[Input], &[Input]),
    failures: &mut Vec<String>,
) -> Result<(), String> {
    let form = Form::Contact(contact);
    let gpu = match open(adapter) {
        Verdict::Ok(g) => g,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let mut renderer = match Renderer::new(&gpu, OFFSCREEN_FORMAT) {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let mut lines = Vec::new();
    let (scene, state) = drive(
        &gpu,
        &mut renderer,
        (form, body, cells),
        ports,
        script,
        0,
        &mut lines,
        failures,
    )?;
    if n == 0 {
        lines.push("fresh contact calculator (Amendment A4):".into());
        let mut again = match Renderer::new(&gpu, OFFSCREEN_FORMAT) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => return Err(r.reason),
        };
        drive(
            &gpu,
            &mut again,
            (form, body, cells),
            ports,
            fresh,
            script.len(),
            &mut lines,
            failures,
        )?;
        for line in &lines {
            println!("{line}");
        }
    }
    let camera = fit(scene.layout(), 1280, 720);
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
    let regrown = match form.regrow(body, cells, &state) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(format!("regrow contact: {}", r.reason)),
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
        "regrow contact {}: color {}, ids {}",
        adapter.line(),
        word(color),
        word(ids)
    );
    if !(color && ids) {
        failures.push(format!(
            "regrow contact {}: the regrown GPU state drew different bytes (VH2)",
            adapter.line()
        ));
    }
    Ok(())
}
