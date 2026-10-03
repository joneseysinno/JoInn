//! `cargo xtask layout --universe <path|grove> [--lens NAME]`: one line per
//! chart, then the totals.

use joinn_frame::Verdict;
use joinn_visual::{ChartKind, layout_universe};
use std::fmt::Write;

use crate::fns::corpus_store::corpus_store;
use crate::fns::grove::{kind_name, load_universe_arg};

/// `galaxy g0 64 64 1296 704`, `system g0s00 16 32 304 152`,
/// `body b0000 calc 8 16 40 24` (each in its parent chart), then
/// `layout universe: <g> galaxies, <s> systems, <b> bodies, 5504x1600`. The
/// lens defaults to the first in canonical order. A refusal is an error.
pub(crate) fn universe_layout_text(args: Vec<String>) -> Result<String, String> {
    let mut it = args.into_iter();
    let Some(subject) = it.next() else {
        return Err("usage: cargo xtask layout --universe <path|grove> [--lens NAME]".into());
    };
    let lens = match (it.next().as_deref(), it.next()) {
        (Some("--lens"), Some(name)) => Some(name),
        (None, _) => None,
        _ => return Err("usage: cargo xtask layout --universe <path|grove> [--lens NAME]".into()),
    };
    let universe = load_universe_arg(&subject)?;
    let lens = match lens {
        Some(l) => l,
        None => universe
            .coding
            .lenses
            .iter()
            .map(|l| l.name.clone())
            .min()
            .ok_or_else(|| {
                format!("layout: {subject} declares no lens; acceptance is a universe with a lens")
            })?,
    };
    let (_, store) = corpus_store()?;
    let placed = match layout_universe(&universe, store, &lens) {
        Verdict::Ok(l) => l,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let mut out = String::new();
    let mut counts = [0usize; 3];
    let mut root = (0, 0);
    for chart in &placed.charts {
        let (x, y) = chart.origin;
        let (w, h) = chart.size;
        match (chart.kind, &chart.body) {
            (ChartKind::Universe, _) => root = chart.size,
            (ChartKind::Galaxy, _) => {
                counts[0] += 1;
                let _ = writeln!(out, "galaxy {} {x} {y} {w} {h}", chart.name);
            }
            (ChartKind::System, _) => {
                counts[1] += 1;
                let _ = writeln!(out, "system {} {x} {y} {w} {h}", chart.name);
            }
            (ChartKind::Body, body) => {
                counts[2] += 1;
                let kind = body
                    .as_ref()
                    .map_or_else(String::new, |(h, _)| kind_name(h));
                let _ = writeln!(out, "body {} {kind} {x} {y} {w} {h}", chart.name);
            }
        }
    }
    let _ = writeln!(
        out,
        "layout universe: {} galaxies, {} systems, {} bodies, {}x{}",
        counts[0], counts[1], counts[2], root.0, root.1
    );
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::universe_layout_text;

    #[test]
    fn the_grove_prints_its_charts_and_the_plan_s_last_line() {
        let text = match universe_layout_text(vec!["grove".into()]) {
            Ok(t) => t,
            Err(e) => panic!("{e}"),
        };
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 8 + 128 + 3072 + 1);
        assert_eq!(
            &lines[..3],
            [
                "galaxy g0 64 64 1296 704",
                "system g0s00 16 32 304 152",
                "body b0000 calc 8 16 40 24"
            ]
        );
        assert_eq!(
            lines.last().copied(),
            Some("layout universe: 8 galaxies, 128 systems, 3072 bodies, 5504x1600")
        );
    }

    #[test]
    fn a_lens_the_universe_does_not_declare_is_refused() {
        let got = universe_layout_text(vec!["grove".into(), "--lens".into(), "deployment".into()]);
        assert_eq!(
            got,
            Err(
                "layout: lens deployment not found; acceptance is a lens the universe declares"
                    .into()
            )
        );
    }

    #[test]
    fn the_s_centre_is_in_b0000_and_the_frame_centre_is_in_the_universe_only() {
        let u = match crate::fns::grove::grow_grove(7) {
            Ok(u) => u,
            Err(e) => panic!("{e}"),
        };
        let store = match crate::fns::corpus_store::corpus_store() {
            Ok((_, s)) => s,
            Err(e) => panic!("{e}"),
        };
        let l = match joinn_visual::layout_universe(&u, store, "function") {
            joinn_frame::Verdict::Ok(l) => l,
            joinn_frame::Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let s = l.chart_at((907, 938), 8);
        assert_eq!(l.chart(s).map(|c| c.name.as_str()), Some("b0000"));
        assert_eq!(l.chart(s).map(|c| c.root_origin), Some((88, 112)));
        assert_eq!(l.chart_at((2752, 800), 1), joinn_visual::ChartId(0));
    }

    #[test]
    fn the_phase_5_universe_prints_its_function_lens() {
        let text = match universe_layout_text(vec![
            "phase5/universe.universe".into(),
            "--lens".into(),
            "function".into(),
        ]) {
            Ok(t) => t,
            Err(e) => panic!("{e}"),
        };
        assert_eq!(
            text,
            "\
galaxy app 64 64 1296 704
system calculation 16 32 304 152
body calc calc 8 16 40 24
system measurement 336 32 304 152
body units units 8 16 20 18
layout universe: 1 galaxies, 2 systems, 2 bodies, 5504x1600
"
        );
    }
}
