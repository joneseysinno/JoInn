//! Port list for a floor member that can appear as `prim:` in a genome.

use joinn_dna::{Direction, PortDecl};
use joinn_frame::FrameRef;

/// Port list for a floor member that can appear as `prim:` in a genome.
pub fn prim_ports(name: &str) -> Option<Vec<PortDecl>> {
    let z = FrameRef::int();
    let t = FrameRef::text();
    let port = |position, direction, frame, required| PortDecl {
        position,
        direction,
        frame,
        required,
    };
    let ins_outs = |ins: &[(u32, FrameRef, bool)], outs: &[(u32, FrameRef)]| {
        let mut v = Vec::new();
        for (p, f, req) in ins {
            v.push(port(*p, Direction::In, *f, *req));
        }
        for (p, f) in outs {
            v.push(port(*p, Direction::Out, *f, true));
        }
        v
    };
    Some(match name {
        "eq" => ins_outs(&[(0, z, true), (1, z, true)], &[(0, z)]),
        "choose" => ins_outs(&[(0, z, true), (1, z, false), (2, z, false)], &[(0, z)]),
        "pair" => ins_outs(&[(0, z, true), (1, z, true)], &[(0, z)]),
        "split" => ins_outs(&[(0, z, true)], &[(0, z), (1, z)]),
        "build" => ins_outs(
            &[(0, z, true), (1, z, true), (2, z, false), (3, z, false)],
            &[(0, z)],
        ),
        "case" => ins_outs(&[(0, z, true)], &[(0, z), (1, z), (2, z)]),
        "hash" => ins_outs(&[(0, z, true)], &[(0, t)]),
        "resolve" => ins_outs(&[(0, t, true)], &[(0, z)]),
        _ => return None,
    })
}
