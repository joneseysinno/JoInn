//! Decode an ID texel against the tables as they stand.

use joinn_frame::Verdict;

use super::Scene;
use crate::pick::{Owner, PORT_TAG, TAG_MASK, WIRE_TAG};
use crate::refuse::refuse;
use crate::tables::LIVE;

impl Scene {
    /// `[body + 1, cell + 1, tag | n, generation]`; a port carries its cell's
    /// generation. An ID whose generation isn't its row's current one is stale.
    pub fn resolve(&self, id: [u32; 4]) -> Verdict<Owner> {
        let [r, g, b, generation] = id;
        if id == [0; 4] {
            return Verdict::Ok(Owner::Background);
        }
        let unknown = || {
            refuse(format!(
                "pick: ID {id:?} names nothing in this scene; acceptance is an ID drawn from its tables"
            ))
        };
        let stale = |table: &str, slot: u32, now: u32| {
            refuse(format!(
                "stale pick: {table} slot {slot} generation {generation}, now {now}; acceptance is a pick taken from the current picture"
            ))
        };
        let Some(body) = r.checked_sub(1) else {
            return unknown();
        };
        let Some(surface) = self.tables.body.get(body as usize) else {
            return unknown();
        };
        let n = b & !TAG_MASK;
        match (g, b & TAG_MASK) {
            (0, 0) if b == 0 => {
                if surface.generation != generation {
                    stale("body", body, surface.generation)
                } else if surface.flags & LIVE == 0 {
                    unknown()
                } else {
                    Verdict::Ok(Owner::Surface)
                }
            }
            (0, WIRE_TAG) => {
                let Some(row) = self.tables.link.get(n as usize) else {
                    return unknown();
                };
                if row.generation != generation {
                    return stale("link", n, row.generation);
                }
                match self.links.iter().find(|(_, s)| **s == n) {
                    Some(((src, dst), _)) if row.flags & LIVE != 0 && row.body == body => {
                        Verdict::Ok(Owner::Wire {
                            src: src.clone(),
                            dst: dst.clone(),
                        })
                    }
                    _ => unknown(),
                }
            }
            (0, _) => unknown(),
            (c, tag) => {
                let slot = c.checked_sub(1).unwrap_or(u32::MAX);
                let Some(row) = self.tables.cell.get(slot as usize) else {
                    return unknown();
                };
                if row.generation != generation {
                    return stale("cell", slot, row.generation);
                }
                let Some((name, _)) = self.cells.iter().find(|(_, s)| **s == slot) else {
                    return unknown();
                };
                if row.flags & LIVE == 0 || row.body != body {
                    return unknown();
                }
                match tag {
                    0 if b == 0 => Verdict::Ok(Owner::Cell(name.clone())),
                    PORT_TAG => match self
                        .ports
                        .keys()
                        .find(|a| a.instance == *name && a.port == n)
                    {
                        Some(a) => Verdict::Ok(Owner::Port(a.clone())),
                        None => unknown(),
                    },
                    _ => unknown(),
                }
            }
        }
    }
}
