//! The zoom script on the grove: tables after every step, then each rebase
//! on every adapter.

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use joinn_frame::Verdict;
use joinn_gpu::{GpuAdapter, OFFSCREEN_FORMAT, Renderer, open};
use joinn_visual::{Camera, ChartId, Delta, Table, TableBytes, Tables, UniverseScene, table_bytes};

use crate::fns::grove::grove_layout;
use crate::fns::zoom::zoom_script;

/// One rebase: where it went, the camera and tables before and after, and the
/// delta that moved them.
struct Rebase {
    from: String,
    to: String,
    before: (Camera, Tables),
    after: (Camera, Tables),
    delta: Delta,
}

/// Per step `grove step <i> <action>: rows <r>, <anchor>, tables equal
/// regrow`; then per rebase and adapter `rebase <from> -> <to> <adapter>:
/// color identical, ids identical`, the delta applied to the tables drawn
/// before it. Tables that differ from a fresh grow at the step's anchor, a
/// rebase that writes anything but chart rows, or a picture that moves fails.
pub(crate) fn grove_pass(all: &[GpuAdapter], failures: &mut Vec<String>) -> Result<(), String> {
    let (_, layout) = grove_layout()?;
    let mut scene = match UniverseScene::grow(layout.clone(), ChartId(0)) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(format!("regrow: grove: {}", r.reason)),
    };
    let mut fresh: BTreeMap<ChartId, TableBytes> = BTreeMap::new();
    let mut rebases: Vec<Rebase> = Vec::new();
    let mut i = 0usize;
    let name = |c: ChartId| {
        layout
            .chart(c)
            .map_or_else(|| "?".to_owned(), |c| c.name.clone())
    };
    zoom_script(|action, camera| {
        let tables = scene.tables().clone();
        let from = scene.anchor();
        let (next, delta) = match scene.rebase(camera) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => return Err(r.reason),
        };
        scene.take_pending();
        let to = scene.anchor();
        let want = match fresh.entry(to) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => match UniverseScene::grow(layout.clone(), to) {
                Verdict::Ok(s) => e.insert(table_bytes(s.tables())),
                Verdict::Refused(r) => return Err(r.reason),
            },
        };
        let equal = *want == table_bytes(scene.tables());
        println!(
            "grove step {i} {action}: rows {}, {}, tables {} regrow",
            delta.rows.len(),
            name(to),
            if equal { "equal" } else { "differ from" }
        );
        if !equal {
            failures.push(format!(
                "grove step {i} {action}: the tables differ from a fresh grow at {} (rule 58)",
                name(to)
            ));
        }
        if delta.rows.iter().any(|w| w.table != Table::Chart) {
            failures.push(format!(
                "grove step {i} {action}: the rebase wrote a row that is not a chart row (V142)"
            ));
        }
        if from != to {
            rebases.push(Rebase {
                from: name(from),
                to: name(to),
                before: (camera, tables),
                after: (next, scene.tables().clone()),
                delta,
            });
        }
        i += 1;
        Ok(next)
    })?;
    for adapter in all {
        let gpu = match open(adapter) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => return Err(r.reason),
        };
        for r in &rebases {
            let mut renderer = match Renderer::new(&gpu, OFFSCREEN_FORMAT) {
                Verdict::Ok(r) => r,
                Verdict::Refused(r) => return Err(r.reason),
            };
            if let Verdict::Refused(e) = renderer.upload_all(&gpu, &r.before.1) {
                return Err(e.reason);
            }
            let before = match renderer.picture_at(&gpu, &r.before.0) {
                Verdict::Ok(p) => p,
                Verdict::Refused(e) => return Err(e.reason),
            };
            if let Verdict::Refused(e) = renderer.apply(&gpu, &r.after.1, &r.delta) {
                return Err(e.reason);
            }
            let after = match renderer.picture_at(&gpu, &r.after.0) {
                Verdict::Ok(p) => p,
                Verdict::Refused(e) => return Err(e.reason),
            };
            let word = |same: bool| if same { "identical" } else { "differ" };
            let (color, ids) = (before.color == after.color, before.ids == after.ids);
            let line = format!("rebase {} -> {} {}", r.from, r.to, adapter.line());
            println!("{line}: color {}, ids {}", word(color), word(ids));
            if !(color && ids) {
                failures.push(format!("{line}: a rebase moved a pixel (V143)"));
            }
        }
    }
    Ok(())
}
