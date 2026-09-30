//! Place a contact body's cells touching, with its surface ports only.

use joinn_dna::{Cell, Contact, Direction, Force};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_link::{Address, check_contact, contact_surface};
use std::collections::{BTreeMap, BTreeSet};

use super::{CELL_W, CellBox, Layout, MARGIN, PITCH, PORT_INSET, PortDot, Rect};
use crate::refuse::refuse;

/// An admitted contact's layout, from its coding region and its cells'
/// contracts alone. Cells touch: a genome cell is column 0 and stacks in name
/// order; a response is one column right of its members and spans them. Only
/// surface ports are placed, and there are no wires.
pub fn layout_contact(contact: &Contact, cells: &BTreeMap<Hash, Cell>) -> Verdict<Layout> {
    if let Verdict::Refused(r) = check_contact(contact, cells, &FrameRegistry::phase1()) {
        return Verdict::Refused(r);
    }
    let surface = match contact_surface(contact, cells) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let mut sides: BTreeMap<&str, (Vec<u32>, Vec<u32>)> = BTreeMap::new();
    for p in &surface {
        let side = sides.entry(p.address.instance.as_str()).or_default();
        match p.direction {
            Direction::In => side.0.push(p.address.port),
            Direction::Out => side.1.push(p.address.port),
        }
    }
    let height = |name: &str| {
        let rows = sides.get(name).map_or(0, |(i, o)| i.len().max(o.len()));
        let rows = i64::try_from(rows).unwrap_or(i64::MAX).max(1);
        PORT_INSET + PITCH * (rows - 1) + PORT_INSET
    };

    let genome: BTreeSet<&str> = contact
        .coding
        .genome
        .iter()
        .flat_map(|g| g.instances.iter().map(String::as_str))
        .collect();
    let mut column: BTreeMap<&str, i64> = genome.iter().map(|n| (*n, 0)).collect();
    let mut forces: Vec<&Force> = contact.coding.forces.iter().collect();
    forces.sort_by(|a, b| a.name.cmp(&b.name));
    let mut unplaced = forces.clone();
    while !unplaced.is_empty() {
        let before = unplaced.len();
        unplaced.retain(|f| {
            let cols: Option<Vec<i64>> = f
                .members
                .iter()
                .map(|m| column.get(m.instance.as_str()).copied())
                .collect();
            match cols.and_then(|c| c.into_iter().max()) {
                Some(most) => {
                    column.insert(f.name.as_str(), most + 1);
                    false
                }
                None => true,
            }
        });
        if unplaced.len() == before {
            let names: Vec<&str> = unplaced.iter().map(|f| f.name.as_str()).collect();
            return refuse(format!(
                "layout: forces {} have no column; acceptance is forces that combine only cells that exist before them (R84)",
                names.join(", ")
            ));
        }
    }

    let mut rects: BTreeMap<&str, Rect> = BTreeMap::new();
    let mut top = MARGIN;
    for name in &genome {
        let h = height(name);
        rects.insert(
            name,
            Rect {
                x: MARGIN,
                y: top,
                w: CELL_W,
                h,
            },
        );
        top += h;
    }
    forces.sort_by(|a, b| {
        let col = |f: &Force| column.get(f.name.as_str()).copied().unwrap_or(0);
        col(a).cmp(&col(b)).then(a.name.cmp(&b.name))
    });
    for force in &forces {
        let members: BTreeSet<&str> = force.members.iter().map(|m| m.instance.as_str()).collect();
        let mut spans: Vec<Rect> = members
            .iter()
            .filter_map(|m| rects.get(m).copied())
            .collect();
        let columns: BTreeSet<i64> = members
            .iter()
            .filter_map(|m| column.get(m).copied())
            .collect();
        spans.sort_by_key(|r| r.y);
        let touching = spans.windows(2).all(|w| w[1].y == w[0].y + w[0].h);
        let (Some(first), Some(last)) = (spans.first(), spans.last()) else {
            return refuse(format!(
                "layout: force {}'s members do not touch in one column; acceptance is members stacked together (R84)",
                force.name
            ));
        };
        if columns.len() != 1 || !touching || spans.len() != members.len() {
            return refuse(format!(
                "layout: force {}'s members do not touch in one column; acceptance is members stacked together (R84)",
                force.name
            ));
        }
        let span = last.y + last.h - first.y;
        let need = height(&force.name);
        if need > span {
            return refuse(format!(
                "layout: response {} needs height {need}, its members span {span}",
                force.name
            ));
        }
        let col = column.get(force.name.as_str()).copied().unwrap_or(0);
        rects.insert(
            force.name.as_str(),
            Rect {
                x: MARGIN + col * CELL_W,
                y: first.y,
                w: CELL_W,
                h: span,
            },
        );
    }

    let responses: BTreeSet<&str> = contact
        .coding
        .forces
        .iter()
        .map(|f| f.name.as_str())
        .collect();
    let mut boxes = Vec::new();
    let mut dots = Vec::new();
    for (name, rect) in &rects {
        boxes.push(CellBox {
            instance: (*name).to_owned(),
            rect: *rect,
            response: responses.contains(name),
        });
        let (ins, outs) = sides.get(name).cloned().unwrap_or_default();
        let mut placed: Vec<(u32, Direction, i64, i64)> = Vec::new();
        for (x, direction, ports) in [
            (rect.x, Direction::In, ins),
            (rect.x + CELL_W, Direction::Out, outs),
        ] {
            for (k, port) in (0i64..).zip(ports) {
                placed.push((port, direction, x, rect.y + PORT_INSET + PITCH * k));
            }
        }
        placed.sort_by_key(|p| p.0);
        for (port, direction, x, y) in placed {
            dots.push(PortDot {
                address: Address {
                    instance: (*name).to_owned(),
                    port,
                },
                direction,
                x,
                y,
            });
        }
    }
    let columns = column.values().max().map_or(0, |c| c + 1);
    let lowest = rects.values().map(|r| r.y + r.h).max().unwrap_or(MARGIN);
    Verdict::Ok(Layout {
        surface: Rect {
            x: 0,
            y: 0,
            w: 2 * MARGIN + columns * CELL_W,
            h: MARGIN + lowest,
        },
        cells: boxes,
        ports: dots,
        wires: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use joinn_dna::{Cell, Contact, hash, parse_cell, parse_contact};
    use joinn_frame::{FrameRegistry, Hash, Verdict};
    use std::collections::BTreeMap;

    use super::layout_contact;
    use crate::layout::print_layout;

    const SUM: &str = "6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39";
    const CLI: &str = "c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e";

    fn cells() -> BTreeMap<Hash, Cell> {
        let mut cells = BTreeMap::new();
        for src in [
            include_str!("../../../../corpus/phase0/sum.cell"),
            include_str!("../../../../corpus/phase0/cli_input.cell"),
        ] {
            match parse_cell(src, &FrameRegistry::phase1()) {
                Verdict::Ok(c) => {
                    cells.insert(hash(&c.coding), c);
                }
                Verdict::Refused(r) => panic!("{}", r.reason),
            }
        }
        cells
    }

    fn contact(src: &str) -> Contact {
        match parse_contact(src, &FrameRegistry::phase1()) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn the_calculator_contact_prints_the_plan_block() {
        let c = contact(include_str!("../../../../corpus/phase7/calculator.contact"));
        let got = match layout_contact(&c, &cells()) {
            Verdict::Ok(l) => print_layout(&l),
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(
            got,
            "\
layout body
surface 0 0 32 20 r 3
cell cli_a 4 4 12 6 r 2
cell cli_b 4 10 12 6 r 2
cell sum 16 4 12 12 r 2 response
port cli_a@0 in 4 7 r 1
port cli_b@0 in 4 13 r 1
port sum@2 out 28 7 r 1
"
        );
    }

    #[test]
    fn members_that_do_not_touch_in_one_column_are_refused() {
        let c = contact(&format!(
            "contact {{ codex 1 genome {{ cell:{CLI} as a, b, c }} grants {{ stdin: a, b, c }} forces {{ combine ℤ 1 cell:{SUM} as s from a@1, c@1 }} budget {{ steps 100 }} lineage none }}"
        ));
        let Verdict::Refused(r) = layout_contact(&c, &cells()) else {
            panic!("a and c do not touch");
        };
        assert_eq!(
            r.reason,
            "layout: force s's members do not touch in one column; acceptance is members stacked together (R84)"
        );
        let chained = contact(&format!(
            "contact {{ codex 1 genome {{ cell:{CLI} as a, b, c }} grants {{ stdin: a, b, c }} forces {{ combine ℤ 1 cell:{SUM} as s1 from a@1, b@1 combine ℤ 1 cell:{SUM} as s2 from c@1, s1@2 }} budget {{ steps 100 }} lineage none }}"
        ));
        let Verdict::Refused(r) = layout_contact(&chained, &cells()) else {
            panic!("c and s1 are in different columns");
        };
        assert_eq!(
            r.reason,
            "layout: force s2's members do not touch in one column; acceptance is members stacked together (R84)"
        );
    }

    #[test]
    fn a_refused_contact_has_no_layout() {
        let c = contact(include_str!("../../../../corpus/phase7/calculator.contact"));
        let Verdict::Refused(r) = layout_contact(&c, &BTreeMap::new()) else {
            panic!("no cells: refused");
        };
        assert_eq!(r.reason, format!("unknown cell hash {CLI}"));
    }
}
