//! Place a body's cells, ports and wires on the whole-number grid.

use joinn_dna::{Body, Cell, Direction};
use joinn_frame::{Hash, Verdict};
use joinn_link::{Address, instance_ports};
use std::collections::{BTreeMap, BTreeSet};

use super::columns::columns;
use super::{
    CELL_W, CellBox, GAP_X, GAP_Y, Layout, MARGIN, PITCH, PORT_INSET, PortDot, Rect, WireSeg,
};
use crate::refuse::refuse;

/// A body's layout, from its coding region and its cells' contracts alone.
/// Labels, names, prompts, alleles and lenses cannot reach it (V119).
pub fn layout(body: &Body, cells: &BTreeMap<Hash, Cell>) -> Verdict<Layout> {
    let ports = match instance_ports(body, cells) {
        Verdict::Ok(ports) => ports,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let mut wires = body.coding.wires.clone();
    wires.sort();
    for w in &wires {
        let has = |instance: &str, position: u32, direction: Direction| {
            ports.get(instance).is_some_and(|decls| {
                decls
                    .iter()
                    .any(|p| p.position == position && p.direction == direction)
            })
        };
        let joined = has(&w.src_instance, w.src_port, Direction::Out)
            && has(&w.dst_instance, w.dst_port, Direction::In);
        if !joined {
            return refuse(format!(
                "layout: wire {}@{} -> {}@{} does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances",
                w.src_instance, w.src_port, w.dst_instance, w.dst_port
            ));
        }
    }
    let names: BTreeSet<String> = ports.keys().cloned().collect();
    let column = match columns(&names, &wires) {
        Verdict::Ok(column) => column,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };

    let mut next_top: BTreeMap<i64, i64> = BTreeMap::new();
    let mut boxes = Vec::new();
    let mut dots = Vec::new();
    let mut centre: BTreeMap<(&str, u32, bool), (i64, i64)> = BTreeMap::new();
    for (name, decls) in &ports {
        let col = column.get(name).copied().unwrap_or(0);
        let left = MARGIN + col * (CELL_W + GAP_X);
        let ins = decls
            .iter()
            .filter(|p| p.direction == Direction::In)
            .count() as i64;
        let outs = decls.len() as i64 - ins;
        let rows = ins.max(outs).max(1);
        let height = PORT_INSET + PITCH * (rows - 1) + PORT_INSET;
        let top = next_top.get(&col).copied().unwrap_or(MARGIN);
        next_top.insert(col, top + height + GAP_Y);
        boxes.push(CellBox {
            instance: name.clone(),
            rect: Rect {
                x: left,
                y: top,
                w: CELL_W,
                h: height,
            },
        });
        let (mut k_in, mut k_out) = (0, 0);
        for p in decls {
            let (x, k) = match p.direction {
                Direction::In => (left, &mut k_in),
                Direction::Out => (left + CELL_W, &mut k_out),
            };
            let y = top + PORT_INSET + PITCH * *k;
            *k += 1;
            let out = p.direction == Direction::Out;
            centre.insert((name.as_str(), p.position, out), (x, y));
            dots.push(PortDot {
                address: Address {
                    instance: name.clone(),
                    port: p.position,
                },
                direction: p.direction,
                x,
                y,
            });
        }
    }

    let count = column.values().max().map_or(0, |c| c + 1);
    let tallest = next_top
        .values()
        .map(|next| next - GAP_Y - MARGIN)
        .max()
        .unwrap_or(0);
    let width = if count == 0 {
        2 * MARGIN
    } else {
        2 * MARGIN + count * CELL_W + (count - 1) * GAP_X
    };
    let surface = Rect {
        x: 0,
        y: 0,
        w: width,
        h: 2 * MARGIN + tallest,
    };

    let mut segs = Vec::new();
    for w in &wires {
        let from = centre.get(&(w.src_instance.as_str(), w.src_port, true));
        let to = centre.get(&(w.dst_instance.as_str(), w.dst_port, false));
        let (Some(from), Some(to)) = (from, to) else {
            return refuse(format!(
                "layout: wire {}@{} -> {}@{} has no placed port; acceptance is a wire between placed ports",
                w.src_instance, w.src_port, w.dst_instance, w.dst_port
            ));
        };
        segs.push(WireSeg {
            src: Address {
                instance: w.src_instance.clone(),
                port: w.src_port,
            },
            dst: Address {
                instance: w.dst_instance.clone(),
                port: w.dst_port,
            },
            from: *from,
            to: *to,
        });
    }
    Verdict::Ok(Layout {
        surface,
        cells: boxes,
        ports: dots,
        wires: segs,
    })
}

#[cfg(test)]
mod tests {
    use joinn_dna::Wire;
    use joinn_frame::Verdict;

    use super::layout;
    use crate::fixtures::calculator;

    #[test]
    fn a_wire_cycle_is_refused_naming_its_instances() {
        let (mut body, cells) = calculator();
        body.coding.wires.push(Wire {
            src_instance: "sum".into(),
            src_port: 2,
            dst_instance: "cli_a".into(),
            dst_port: 0,
        });
        match layout(&body, &cells) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "layout: wire cycle through cli_a, sum; acceptance is a body whose wires form no cycle"
            ),
            Verdict::Ok(_) => panic!("a wire cycle must be refused"),
        }
    }

    #[test]
    fn a_self_wire_is_a_cycle_through_one_instance() {
        let (mut body, cells) = calculator();
        body.coding.wires.push(Wire {
            src_instance: "sum".into(),
            src_port: 2,
            dst_instance: "sum".into(),
            dst_port: 0,
        });
        match layout(&body, &cells) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "layout: wire cycle through sum; acceptance is a body whose wires form no cycle"
            ),
            Verdict::Ok(_) => panic!("a self-wire must be refused"),
        }
    }

    #[test]
    fn a_wire_joins_the_out_port_and_in_port_that_share_a_position() {
        let src = "body {\n  codex 1\n  genome {\n    prim:eq as a, b\n  }\n  grants { }\n  wires {\n    a@0 -> b@0\n  }\n  budget { steps 1 }\n  lineage none\n}\n";
        let body = match joinn_dna::parse_body(src, &joinn_frame::FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let l = match layout(&body, &std::collections::BTreeMap::new()) {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let printed = crate::layout::print_layout(&l);
        assert!(printed.contains("port a@0 in 4 7 r 1\n"), "{printed}");
        assert!(printed.contains("port a@0 out 16 7 r 1\n"), "{printed}");
        assert!(
            printed.contains("wire a@0 -> b@0 16 7 24 7 w 1/4\n"),
            "{printed}"
        );
    }
}
