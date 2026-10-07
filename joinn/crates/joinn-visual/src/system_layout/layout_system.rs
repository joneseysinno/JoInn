//! Lay out a system: the grown body in rows, the response, the lasso.

use joinn_dna::{Cell, CellEntry, Contact, ContactCoding, Direction, System};
use joinn_frame::{Hash, Verdict};
use joinn_link::{Address, Grown};
use std::collections::BTreeMap;

use super::response_shape::response_shape;
use super::{
    ARROW, LASSO_GAP, LassoStroke, NECK, ROW_CELLS, SYSTEM_PAD, SystemLayout, lasso_outline,
};
use crate::layout::{CellBox, GAP_X, GAP_Y, MARGIN, PORT_INSET, PortDot, Rect, layout_contact};
use crate::refuse::refuse;

/// `grown`'s cells, then `waiting` boxes, sit in rows of `ROW_CELLS` with
/// Phase 6's gaps inside the body's surface (Phase 6's margins); each has the
/// input cell's size from `layout_contact`. The response sits right of the
/// body, vertically centred on it. The lasso is `LASSO_GAP` outside the
/// surface; its neck leaves the right side at the body's centre and its
/// arrowhead's tip touches the response's in-port side.
pub fn layout_system(
    system: &System,
    contacts: &BTreeMap<Hash, Contact>,
    cells: &BTreeMap<Hash, Cell>,
    grown: &Grown,
    waiting: u32,
) -> Verdict<SystemLayout> {
    if grown.system() != system {
        return refuse(
            "layout: the grown state is of another system; acceptance is its own system",
        );
    }
    let body = system
        .coding
        .bodies
        .first()
        .and_then(|b| contacts.get(&b.contact));
    let (Some(body), Some(force)) = (body, system.coding.forces.first()) else {
        return refuse("layout: a system needs a supplied body and a force; acceptance is both");
    };
    let Some(grows) = body.coding.grows.as_ref() else {
        return refuse("layout: the system's body does not grow; acceptance is a growing body");
    };
    let one = Contact {
        coding: ContactCoding {
            codex: body.coding.codex,
            genome: vec![CellEntry {
                cell: grows.cell,
                instances: vec![grown.instance(0)],
            }],
            grants: BTreeMap::from([("stdin".to_owned(), vec![grown.instance(0)])]),
            reads: body.coding.reads.clone(),
            forces: Vec::new(),
            grows: None,
            budget_steps: body.coding.budget_steps,
            lineage: None,
        },
        regulatory: body.regulatory.clone(),
    };
    let input = match layout_contact(&one, cells) {
        Verdict::Ok(l) => l,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let (Some(size), Some(in_dot)) = (
        input.cells.first().map(|c| (c.rect.w, c.rect.h)),
        input.ports.iter().find(|p| p.direction == Direction::In),
    ) else {
        return refuse("layout: the input cell has no box or no in-port; acceptance is both");
    };
    let (w, h) = size;
    let dot_dy = in_dot.y - input.cells.first().map_or(0, |c| c.rect.y);
    let filled = grown.inputs().len();
    let total = i64::try_from(filled + waiting as usize).unwrap_or(i64::MAX);
    let cols = total.clamp(1, ROW_CELLS);
    let rows = ((total + ROW_CELLS - 1) / ROW_CELLS).max(1);
    let surface = Rect {
        x: SYSTEM_PAD,
        y: SYSTEM_PAD,
        w: 2 * MARGIN + cols * w + (cols - 1) * GAP_X,
        h: 2 * MARGIN + rows * h + (rows - 1) * GAP_Y,
    };
    let mut boxes = Vec::new();
    let mut waiting_boxes = Vec::new();
    let mut ports = Vec::new();
    for k in 0..total {
        let rect = Rect {
            x: surface.x + MARGIN + (k % ROW_CELLS) * (w + GAP_X),
            y: surface.y + MARGIN + (k / ROW_CELLS) * (h + GAP_Y),
            w,
            h,
        };
        let n = usize::try_from(k).unwrap_or(usize::MAX);
        let instance = grown.instance(n);
        ports.push(PortDot {
            address: Address {
                instance: instance.clone(),
                port: in_dot.address.port,
            },
            direction: Direction::In,
            x: rect.x,
            y: rect.y + dot_dy,
        });
        let cell = CellBox {
            instance,
            rect,
            response: false,
        };
        if n < filled {
            boxes.push(cell);
        } else {
            waiting_boxes.push(cell);
        }
    }
    let Some((response_h, out_port)) = response_shape(force, cells) else {
        return refuse(format!(
            "layout: force {}'s response cell:{} was not supplied or has no out-port; acceptance is that cell",
            force.name, force.response
        ));
    };
    let right = surface.x + surface.w + LASSO_GAP;
    let cy = surface.y + surface.h / 2;
    let response = Rect {
        x: right + NECK,
        y: cy - response_h / 2,
        w: crate::layout::CELL_W,
        h: response_h,
    };
    ports.push(PortDot {
        address: Address {
            instance: force.name.clone(),
            port: out_port,
        },
        direction: Direction::Out,
        x: response.x + response.w,
        y: response.y + PORT_INSET,
    });
    let tip = (response.x, cy);
    let mut lasso = lasso_outline(&surface, LASSO_GAP);
    lasso.push(LassoStroke {
        from: (right, cy),
        to: tip,
    });
    for dy in [-ARROW, ARROW] {
        lasso.push(LassoStroke {
            from: tip,
            to: (tip.0 - ARROW, tip.1 + dy),
        });
    }
    Verdict::Ok(SystemLayout {
        surface,
        cells: boxes,
        waiting: waiting_boxes,
        ports,
        response: CellBox {
            instance: force.name.clone(),
            rect: response,
            response: true,
        },
        lasso,
        tip,
    })
}
