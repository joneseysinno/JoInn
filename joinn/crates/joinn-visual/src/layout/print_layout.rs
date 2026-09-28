//! The canonical printed form of a layout.

use joinn_dna::Direction;
use std::fmt::Write;

use super::{CELL_RADIUS, Layout, MEMBRANE_RADIUS, PORT_RADIUS, WIRE_HALF_WIDTH_QUARTERS};

/// `layout body`, the membrane, cells, ports, then wires. A rectangle prints
/// `x y width height`.
pub fn print_layout(layout: &Layout) -> String {
    let mut out = String::from("layout body\n");
    let m = layout.membrane;
    let _ = writeln!(
        out,
        "membrane {} {} {} {} r {MEMBRANE_RADIUS}",
        m.x, m.y, m.w, m.h
    );
    for c in &layout.cells {
        let r = c.rect;
        let _ = writeln!(
            out,
            "cell {} {} {} {} {} r {CELL_RADIUS}",
            c.instance, r.x, r.y, r.w, r.h
        );
    }
    for p in &layout.ports {
        let direction = match p.direction {
            Direction::In => "in",
            Direction::Out => "out",
        };
        let _ = writeln!(
            out,
            "port {} {direction} {} {} r {PORT_RADIUS}",
            p.address.printed(),
            p.x,
            p.y
        );
    }
    for w in &layout.wires {
        let _ = writeln!(
            out,
            "wire {} -> {} {} {} {} {} w {WIRE_HALF_WIDTH_QUARTERS}/4",
            w.src.printed(),
            w.dst.printed(),
            w.from.0,
            w.from.1,
            w.to.0,
            w.to.1
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use super::print_layout;
    use crate::fixtures::calculator;
    use crate::layout::layout;

    #[test]
    fn the_calculator_prints_the_plan_block() {
        let (body, cells) = calculator();
        let got = match layout(&body, &cells) {
            Verdict::Ok(l) => print_layout(&l),
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let want = "\
layout body
membrane 0 0 40 24 r 3
cell cli_a 4 4 12 6 r 2
cell cli_b 4 14 12 6 r 2
cell sum 24 4 12 10 r 2
port cli_a@0 in 4 7 r 1
port cli_a@1 out 16 7 r 1
port cli_b@0 in 4 17 r 1
port cli_b@1 out 16 17 r 1
port sum@0 in 24 7 r 1
port sum@1 in 24 11 r 1
port sum@2 out 36 7 r 1
wire cli_a@1 -> sum@0 16 7 24 7 w 1/4
wire cli_b@1 -> sum@1 16 17 24 11 w 1/4
";
        assert_eq!(got, want);
    }
}
