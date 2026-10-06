//! Gate 6 item 2: what the engine computes is a row the picture shows.

use joinn_frame::Verdict;
use joinn_gpu::{OFFSCREEN_FORMAT, Renderer, open};
use joinn_visual::{FILLED, REFUSED, Scene, fit, table_bytes};

use super::g6_adapters::g6_adapters;
use super::g6_colors::g6_colors;
use super::g6_drive::g6_drive;
use super::load_calculator::load_calculator;
use super::once::{memo, say};
use super::pick::port_slot;
use super::regrow::Form;
use std::sync::OnceLock;

static JUDGED: OnceLock<(bool, Vec<String>)> = OnceLock::new();

/// The script's deltas stay inside the bound and match `regrow`. On every
/// adapter the probe colors match the owning row, and a dropped device redraws
/// the same bytes from `regrow`. Judged once per process (gate 7.2 item 1
/// re-checks it).
pub(crate) fn g6_picture() -> bool {
    memo(&JUDGED, || {
        let Some(adapters) = g6_adapters() else {
            return false;
        };
        let Ok((body, cells)) = load_calculator() else {
            say("calculator did not load; acceptance is corpus/phase2/calculator.body");
            return false;
        };
        let mut driven = match g6_drive(Form::Wired, &body, &cells) {
            Ok(driven) => driven,
            Err(reason) => {
                say(&reason);
                return false;
            }
        };
        for failure in &driven.failures {
            say(failure);
        }
        if !driven.failures.is_empty() {
            return false;
        }
        let sum_filled = port_slot(&driven.scene, "body.sum@2")
            .and_then(|slot| driven.scene.tables().port.get(slot as usize))
            .is_some_and(|row| (row.flags & FILLED) != 0);
        if !sum_filled {
            say("after the script, sum@2 is not filled");
            return false;
        }
        if driven
            .scene
            .tables()
            .cell
            .iter()
            .any(|row| (row.flags & REFUSED) != 0)
        {
            say("after the script, a cell is still refused");
            return false;
        }
        let bytes = table_bytes(driven.scene.tables());
        let _narrow = fit(driven.scene.layout(), 640, 360);
        let _wide = fit(driven.scene.layout(), 1920, 1080);
        let rows = driven
            .scene
            .take_pending()
            .map_or(0, |delta| delta.rows.len());
        if rows != 0 || table_bytes(driven.scene.tables()) != bytes {
            say(&format!("a camera change wrote {rows} row(s) (V121)"));
            return false;
        }
        if driven.scene.take_pending().is_some() {
            say("an idle tick has rows to draw (V12)");
            return false;
        }
        for adapter in &adapters {
            let gpu = match open(adapter) {
                Verdict::Ok(gpu) => gpu,
                Verdict::Refused(r) => {
                    say(&r.reason);
                    return false;
                }
            };
            let mut renderer = match Renderer::new(&gpu, OFFSCREEN_FORMAT) {
                Verdict::Ok(renderer) => renderer,
                Verdict::Refused(r) => {
                    say(&r.reason);
                    return false;
                }
            };
            let mut last = None;
            for step in &driven.steps {
                if let Some(delta) = &step.delta {
                    if let Verdict::Refused(r) = renderer.apply(&gpu, step.scene.tables(), delta) {
                        say(&r.reason);
                        return false;
                    }
                } else if let Verdict::Refused(r) = renderer.upload_all(&gpu, step.scene.tables()) {
                    say(&r.reason);
                    return false;
                }
                let camera = fit(step.scene.layout(), 1280, 720);
                let picture = match renderer.picture(&gpu, &camera) {
                    Verdict::Ok(picture) => picture,
                    Verdict::Refused(r) => {
                        say(&r.reason);
                        return false;
                    }
                };
                if let Err(msg) =
                    g6_colors(&step.label, &step.scene, &picture.color, adapter.line())
                {
                    say(&msg);
                    return false;
                }
                last = Some(picture);
            }
            let Some(before) = last else {
                say(&format!("{}: the script drew no picture", adapter.line()));
                return false;
            };
            drop(renderer);
            drop(gpu);
            let gpu = match open(adapter) {
                Verdict::Ok(gpu) => gpu,
                Verdict::Refused(r) => {
                    say(&r.reason);
                    return false;
                }
            };
            let regrown = match Scene::regrow("body", &body, &cells, &driven.state) {
                Verdict::Ok(scene) => scene,
                Verdict::Refused(r) => {
                    say(&r.reason);
                    return false;
                }
            };
            let mut renderer = match Renderer::new(&gpu, OFFSCREEN_FORMAT) {
                Verdict::Ok(renderer) => renderer,
                Verdict::Refused(r) => {
                    say(&r.reason);
                    return false;
                }
            };
            if let Verdict::Refused(r) = renderer.upload_all(&gpu, regrown.tables()) {
                say(&r.reason);
                return false;
            }
            let camera = fit(regrown.layout(), 1280, 720);
            let after = match renderer.picture(&gpu, &camera) {
                Verdict::Ok(picture) => picture,
                Verdict::Refused(r) => {
                    say(&r.reason);
                    return false;
                }
            };
            if before.color != after.color || before.ids != after.ids {
                say(&format!(
                    "{}: regrown color or ID bytes differ from the delta-built picture",
                    adapter.line()
                ));
                return false;
            }
        }
        true
    })
}
