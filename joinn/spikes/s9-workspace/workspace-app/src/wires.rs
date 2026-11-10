//! Relationship lines and hand-drawn links.
//! A line to a cell inside a folded group ends on that group's edge, once.

use std::collections::BTreeSet;

use workspace_model::{Answer, CellId, Worksheet};

use crate::layout::{self, find, Laid};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Stroke {
    /// A formula. Arrow at the result.
    Formula,
    /// A hand-drawn link. No arrow.
    Link,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Wire {
    pub from: CellId,
    pub to: CellId,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub stroke: Stroke,
}

/// `uses` maps each Result to the cells it reads. Links are the hand-drawn ones.
pub fn wires(
    sheet: &Worksheet,
    laid: &Laid,
    uses: &[(CellId, Vec<CellId>)],
    links: &[(CellId, CellId)],
) -> Vec<Wire> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for (result, cells) in uses {
        for cell in cells {
            let Some((source, target)) = visible_pair(sheet, laid, *cell, *result) else {
                continue;
            };
            if !seen.insert((Stroke::Formula, source, target)) {
                continue;
            }
            if let Some(wire) = connect(laid, source, target, Stroke::Formula) {
                out.push(wire);
            }
        }
    }
    for (from, to) in links {
        let Some((source, target)) = visible_pair(sheet, laid, *from, *to) else {
            continue;
        };
        if source == target || !seen.insert((Stroke::Link, source, target)) {
            continue;
        }
        if let Some(wire) = connect(laid, source, target, Stroke::Link) {
            out.push(wire);
        }
    }
    out
}

pub fn result_uses(sheet: &Worksheet, answers: &std::collections::BTreeMap<CellId, Answer>) -> Vec<(CellId, Vec<CellId>)> {
    let mut uses = Vec::new();
    for cell in sheet.cells() {
        if let Some(answer) = answers.get(&cell.id()) {
            if !answer.uses.is_empty() {
                uses.push((cell.id(), answer.uses.clone()));
            }
        }
    }
    uses
}

fn visible_pair(sheet: &Worksheet, laid: &Laid, from: CellId, to: CellId) -> Option<(CellId, CellId)> {
    Some((visible(sheet, laid, from)?, visible(sheet, laid, to)?))
}

fn visible(sheet: &Worksheet, laid: &Laid, id: CellId) -> Option<CellId> {
    let mut cursor = Some(id);
    while let Some(current) = cursor {
        if find(laid, current).is_some() {
            return Some(current);
        }
        cursor = sheet.parent_of(current);
    }
    None
}

fn connect(laid: &Laid, from: CellId, to: CellId, stroke: Stroke) -> Option<Wire> {
    let source = find(laid, from)?;
    let target = find(laid, to)?;
    let (sx, sy) = (source.x + source.w / 2.0, source.y + source.h / 2.0);
    let (tx, ty) = (target.x + target.w / 2.0, target.y + target.h / 2.0);
    let (x1, y1) = border(source, tx, ty);
    let (x2, y2) = border(target, sx, sy);
    Some(Wire { from, to, x1, y1, x2, y2, stroke })
}

fn border(node: &Laid, toward_x: f64, toward_y: f64) -> (f64, f64) {
    let cx = node.x + node.w / 2.0;
    let cy = node.y + node.h / 2.0;
    let dx = toward_x - cx;
    let dy = toward_y - cy;
    if dx.abs() < 1e-6 && dy.abs() < 1e-6 {
        return (cx, cy);
    }
    let scale = if dx.abs() * node.h > dy.abs() * node.w {
        (node.w / 2.0) / dx.abs()
    } else {
        (node.h / 2.0) / dy.abs()
    };
    (cx + dx * scale, cy + dy * scale)
}

/// Arrowhead at the result end, pointing along the line. Returns three points.
pub fn arrow(wire: &Wire) -> [(f64, f64); 3] {
    let dx = wire.x2 - wire.x1;
    let dy = wire.y2 - wire.y1;
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    let ux = dx / len;
    let uy = dy / len;
    let size = 10.0;
    let px = -uy;
    let py = ux;
    [
        (wire.x2, wire.y2),
        (wire.x2 - ux * size + px * size * 0.45, wire.y2 - uy * size + py * size * 0.45),
        (wire.x2 - ux * size - px * size * 0.45, wire.y2 - uy * size - py * size * 0.45),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_model::simple_beam;

    #[test]
    fn folding_inputs_leaves_one_line_on_its_edge() {
        let sheet = simple_beam().now().expect("beam");
        let laid = layout::layout(&sheet, 0.12, 2).expect("layout");
        let inputs = sheet.cells_named("Inputs")[0];
        let results = sheet.cells_named("Results")[0];
        assert!(find(&laid, inputs).is_some());
        assert!(find(&laid, sheet.cells_named("L")[0]).is_none(), "L is folded away");
        let moment = sheet.cells_named("M")[0];
        let span = sheet.cells_named("L")[0];
        let load = sheet.cells_named("w")[0];
        let lines = wires(
            &sheet,
            &laid,
            &[(moment, vec![load, span])],
            &[],
        );
        let onto_inputs: Vec<_> = lines
            .iter()
            .filter(|wire| wire.from == inputs || wire.to == inputs)
            .collect();
        assert_eq!(onto_inputs.len(), 1, "both names inside Inputs share one end");
        assert!(onto_inputs[0].from == results || onto_inputs[0].to == results || onto_inputs[0].from == moment || onto_inputs[0].to == moment);
    }
}
