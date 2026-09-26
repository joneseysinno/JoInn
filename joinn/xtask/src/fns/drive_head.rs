//! Bind, assemble, and run, then say whether the head body was reached.

use super::head_role::{HeadRole, head_role};
use super::{int_val, load_phase5_bodies, text_val};
use joinn_dna::Direction;
use joinn_frame::{FrameId, Value, Verdict};
use joinn_host::describe;
use joinn_link::{
    Address, Mark, Universe, UniverseReport, UniverseState, assemble_universe, bind, membrane,
};

/// What a drive of the head body showed. An error is a binding, assembly, or run failure.
pub(crate) struct HeadDrive {
    /// Head body read from link marks.
    pub role: HeadRole,
    /// A value landed on a head port, or the head body fired.
    pub value_reached: bool,
    /// The head body fired.
    pub fired: bool,
}

pub(crate) fn drive_head(universe: &Universe) -> Result<HeadDrive, String> {
    let role = head_role(universe);
    let store = load_phase5_bodies()?;
    let bound = match bind(universe, &store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Err(r.reason),
    };
    match assemble_universe(universe, &bound) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Err(r.reason),
    }
    let mut state = match UniverseState::new(universe, &bound, joinn_prim::sealed_natives()) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let HeadRole::One(head) = &role else {
        match state.run() {
            Verdict::Ok(_) => {
                return Ok(HeadDrive {
                    role,
                    value_reached: false,
                    fired: false,
                });
            }
            Verdict::Refused(r) => return Err(r.reason),
        }
    };
    let head = head.clone();
    let mut epoch = 0u64;
    let mut text_i = 0usize;
    let texts = ["2", "3"];
    let mut factor_fed = false;
    for link in &universe.coding.links {
        for member in &link.members {
            if member.mark != Mark::Tail {
                continue;
            }
            let Some((body, cells)) = bound.get(&member.body) else {
                return Err(format!(
                    "tail {} has no bound body; acceptance is a body for every alias",
                    member.body
                ));
            };
            let ports = match membrane(body, cells) {
                Verdict::Ok(p) => p,
                Verdict::Refused(r) => return Err(r.reason),
            };
            for port in ports {
                if port.direction != Direction::In {
                    continue;
                }
                let value = match port.frame.id {
                    FrameId::Text => {
                        let Some(literal) = texts.get(text_i) else {
                            return Err(
                                "more text in-ports than the crossing feed; acceptance is two"
                                    .into(),
                            );
                        };
                        text_i = text_i.saturating_add(1);
                        text_val(literal)?
                    }
                    FrameId::Int | FrameId::Rat => {
                        return Err(format!(
                            "tail {}.{}@{} is not text; acceptance is a text in-port",
                            member.body, port.address.instance, port.address.port
                        ));
                    }
                };
                let addr = Address {
                    instance: port.address.instance.clone(),
                    port: port.address.port,
                };
                match state.inject(&member.body, &addr, value, epoch) {
                    Verdict::Ok(()) => {}
                    Verdict::Refused(r) => return Err(r.reason),
                }
                epoch = epoch.saturating_add(1);
            }
        }
    }
    let Some((body, cells)) = bound.get(&head) else {
        return Err(format!(
            "head {head} has no bound body; acceptance is a body for every alias"
        ));
    };
    let ports = match membrane(body, cells) {
        Verdict::Ok(p) => p,
        Verdict::Refused(r) => return Err(r.reason),
    };
    for port in ports {
        if port.direction != Direction::In {
            continue;
        }
        let is_head = universe.coding.links.iter().any(|link| {
            link.members.iter().any(|m| {
                m.mark == Mark::Head
                    && m.body == head
                    && m.instance == port.address.instance
                    && m.port == port.address.port
            })
        });
        if is_head {
            continue;
        }
        let value: Value = match port.frame.id {
            FrameId::Int => {
                if factor_fed {
                    return Err(
                        "more than one free int in-port on the head; acceptance is the factor"
                            .into(),
                    );
                }
                factor_fed = true;
                int_val(12)?
            }
            FrameId::Text | FrameId::Rat => {
                return Err(format!(
                    "head {head}.{}@{} is not an int factor; acceptance is an int in-port",
                    port.address.instance, port.address.port
                ));
            }
        };
        let addr = Address {
            instance: port.address.instance.clone(),
            port: port.address.port,
        };
        match state.inject(&head, &addr, value, epoch) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Err(r.reason),
        }
        epoch = epoch.saturating_add(1);
    }
    let reports = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let fired = reports
        .iter()
        .any(|r| matches!(r, UniverseReport::Fired { body, .. } if body == &head));
    let mut value_reached = fired;
    if let Some(live) = state.body(&head) {
        for link in &universe.coding.links {
            for member in &link.members {
                if member.mark != Mark::Head || member.body != head {
                    continue;
                }
                match describe(live, &member.instance) {
                    Verdict::Ok(d) => {
                        if d.ports
                            .iter()
                            .any(|p| p.position == member.port && p.value.is_some())
                        {
                            value_reached = true;
                        }
                    }
                    Verdict::Refused(r) => return Err(r.reason),
                }
            }
        }
    }
    Ok(HeadDrive {
        role,
        value_reached,
        fired,
    })
}
