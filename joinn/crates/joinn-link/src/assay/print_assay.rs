//! Canonical text of an assay report.

use std::fmt::Write as _;

use super::AssayReport;

/// One block of text. A body assayed alone starts `assay body`.
pub fn print_assay(report: &AssayReport) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "{}", report.headline);
    for region in &report.regions {
        if region.pieces.len() > 1 {
            let groups = region
                .pieces
                .iter()
                .map(|piece| format!("{{{}}}", piece.join(", ")))
                .collect::<Vec<_>>()
                .join(" ");
            let _ = writeln!(
                out,
                "regions: {} {} {groups}",
                region.alias,
                region.pieces.len()
            );
        } else {
            let _ = writeln!(out, "regions: {} {}", region.alias, region.pieces.len());
        }
    }
    let _ = writeln!(out, "islands: {}", report.islands);
    let _ = writeln!(out, "loops: {}", report.loops);
    for line in &report.filled {
        let _ = writeln!(out, "filled: {line}");
    }
    for line in &report.open {
        let _ = writeln!(out, "open: {line}");
    }
    let _ = writeln!(out, "not measured: {}", report.not_measured);
    let _ = writeln!(out, "H₀: {}", report.b0);
    let _ = writeln!(out, "H₁: {}", report.b1);
    let _ = writeln!(out, "H₂: {}", report.b2);
    let left = i64::from(report.v) - i64::from(report.e) + i64::from(report.f);
    let _ = writeln!(
        out,
        "euler: V {} − E {} + F {} = {left} = {} − {} + {}",
        report.v, report.e, report.f, report.b0, report.b1, report.b2
    );
    out
}
