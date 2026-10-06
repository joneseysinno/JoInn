//! The grown system's response, as the engine gives it.

use joinn_dna::{Cell, Direction};
use joinn_frame::{Frame, FrameRegistry, Hash, Term, TextFrame, Value, Verdict};
use joinn_live::BodyState;
use std::collections::BTreeMap;

use super::Grown;
use super::identity::identity;
use super::lower_grown::lower_grown;

/// With no input, the response's identity. Otherwise the engine runs
/// `lower_grown`: each input enters its grown instance as text, in growth
/// order, and the response is the last stage's out-port (with one input, the
/// grown instance's own out-port).
pub fn respond(
    grown: &Grown,
    cells: &BTreeMap<Hash, Cell>,
    frames: &FrameRegistry,
) -> Verdict<Value> {
    let Some(force) = grown.force() else {
        return crate::refuse("grow: the system has no force; acceptance is a force");
    };
    let Some(response) = cells.get(&force.response) else {
        return crate::refuse(format!(
            "force {}'s response cell:{} was not supplied; acceptance is that cell in the cell map",
            force.name, force.response
        ));
    };
    let n = grown.inputs.len();
    if n == 0 {
        return identity(response, frames);
    }
    let body = match lower_grown(grown, cells, frames) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let mut state = match BodyState::new(body, cells.clone(), joinn_prim::engine_natives(), 1) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    for (epoch, (i, input)) in (0u64..).zip(grown.inputs.iter().enumerate()) {
        let line = match TextFrame::new().canonicalize(Term::text(input.print_term())) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        if let Verdict::Refused(r) = state.inject(&grown.instance(i), 0, line, epoch) {
            return Verdict::Refused(r);
        }
    }
    if let Verdict::Refused(r) = state.run() {
        return Verdict::Refused(r);
    }
    let (instance, cell) = if n == 1 {
        let grows = grown.contact.coding.grows.as_ref().map(|g| g.cell);
        (grown.instance(0), grows.and_then(|h| cells.get(&h)))
    } else {
        (force.name.clone(), Some(response))
    };
    let out = cell.and_then(|c| {
        c.coding
            .contract
            .ports
            .iter()
            .find(|p| p.direction == Direction::Out)
            .map(|p| p.position)
    });
    let value = out.and_then(|o| state.last_ports(&instance).and_then(|p| p.get(&o)));
    match value {
        Some(v) => Verdict::Ok(v.clone()),
        None => crate::refuse(format!(
            "grow: {instance} gave no response after {n} input(s); acceptance is the engine's out-port"
        )),
    }
}
