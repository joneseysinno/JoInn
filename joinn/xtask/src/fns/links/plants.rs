//! The standing plants (rule 93): three faults no admitted universe can make,
//! planted in the grove's routes on every run of `links`.

use joinn_link::{Link, Order};
use joinn_visual::{
    ChartKind, Fold, Piece, PieceKind, Route, SIXTEENTHS, SYSTEM_GUTTER_X, UniverseLayout,
};

use super::fold_line::fold_line;

/// Plant (a) a system gutter line moved 4 units into its bodies, (b) a second
/// touch on a folded node and (c) a mid-path arrowhead on an unordered link,
/// each in copies of the grove's routes, and check each through `fold_line`,
/// the check every real route goes through. Returns one line per plant and
/// the name of every plant that was not refused.
pub(crate) fn plants(layout: &UniverseLayout, links: &[Link]) -> (Vec<String>, Vec<&'static str>) {
    let s = SIXTEENTHS;
    let all = &layout.routes.routes;
    type Case = Result<(String, Fold, Vec<Route>), &'static str>;
    let mut cases: Vec<(&'static str, Case)> = Vec::new();

    // (a) The vertical system gutter line under the first open vertical leg.
    let (x0, dx, n) = SYSTEM_GUTTER_X;
    let line = all
        .iter()
        .filter(|r| r.fold == Fold::Open)
        .flat_map(|r| &r.pieces)
        .filter(|p| p.kind == PieceKind::Leg && p.a.0 == p.b.0)
        .find_map(|p| {
            layout.charts.iter().find_map(|c| {
                let local = p.a.0 - c.root_origin.0 * s;
                let (top, bottom) = (c.root_origin.1 * s, (c.root_origin.1 + c.size.1) * s);
                let col = (local - x0 * s).div_euclid(dx * s);
                let on_line = c.kind == ChartKind::System
                    && (local - x0 * s).rem_euclid(dx * s) == 0
                    && (0..n).contains(&col)
                    && p.a.1.min(p.b.1) >= top - 8 * s
                    && p.a.1.max(p.b.1) <= bottom + 8 * s;
                on_line.then_some((c, p.a.0, col, top, bottom))
            })
        });
    match line {
        Some((chart, x, col, top, bottom)) => {
            let shift = if col == n - 1 { -4 * s } else { 4 * s };
            let moved: Vec<Route> = all
                .iter()
                .filter(|r| r.fold == Fold::Open)
                .filter_map(|r| {
                    let mut r = r.clone();
                    let mut hit = false;
                    for p in &mut r.pieces {
                        let on = p.kind == PieceKind::Leg
                            && p.a.0 == x
                            && p.b.0 == x
                            && p.a.1.min(p.b.1) >= top - 8 * s
                            && p.a.1.max(p.b.1) <= bottom + 8 * s;
                        if on {
                            p.a.0 += shift;
                            p.b.0 += shift;
                            hit = true;
                        }
                    }
                    hit.then_some(r)
                })
                .collect();
            let what = format!(
                "line x = {} of {} moved {} units, {} route(s)",
                x / s - chart.root_origin.0,
                chart.name,
                shift / s,
                moved.len()
            );
            cases.push(("crossing", Ok((what, Fold::Open, moved))));
        }
        None => cases.push(("crossing", Err("no open leg on a system gutter line"))),
    }

    // (b) The first systems-folded route touches its first node again.
    match all
        .iter()
        .find(|r| r.fold == Fold::Systems && !r.touches.is_empty())
    {
        Some(r) => {
            let mut twice = r.clone();
            twice.touches.extend(r.touches.first().cloned());
            let id = links.get(r.link).map_or("?", |l| l.id.as_str());
            let what = format!("link {id} touches its first node twice");
            cases.push(("double touch", Ok((what, Fold::Systems, vec![twice]))));
        }
        None => cases.push(("double touch", Err("no folded route"))),
    }

    // (c) The first unordered open route gains an arrowhead on its first leg.
    let unordered = all.iter().find(|r| {
        r.fold == Fold::Open
            && links.get(r.link).is_some_and(|l| l.order != Order::Ordered)
            && r.pieces.iter().any(|p| p.kind == PieceKind::Leg)
    });
    match unordered {
        Some(r) => {
            let mut arrowed = r.clone();
            if let Some(leg) = r.pieces.iter().find(|p| p.kind == PieceKind::Leg) {
                let mid = ((leg.a.0 + leg.b.0) / 2, (leg.a.1 + leg.b.1) / 2);
                arrowed.pieces.push(Piece {
                    a: leg.a,
                    b: mid,
                    kind: PieceKind::Arrow,
                    member: 0,
                    legs: 1,
                });
            }
            let id = links.get(r.link).map_or("?", |l| l.id.as_str());
            let what = format!("link {id} gains a mid-path arrowhead");
            cases.push(("false arrow", Ok((what, Fold::Open, vec![arrowed]))));
        }
        None => cases.push(("false arrow", Err("no unordered open route"))),
    }

    let mut lines = Vec::new();
    let mut missed = Vec::new();
    for (name, case) in cases {
        let found = match &case {
            Ok((_, fold, routes)) => {
                let refs: Vec<&Route> = routes.iter().collect();
                fold_line("grove", *fold, layout, links, &refs).1.len()
            }
            Err(_) => 0,
        };
        let what = match &case {
            Ok((what, _, _)) => what.as_str(),
            Err(why) => why,
        };
        if found == 0 {
            lines.push(format!("planted {name}: {what}: not refused"));
            missed.push(name);
        } else {
            lines.push(format!("planted {name}: {what}: {found} fault(s), refused"));
        }
    }
    (lines, missed)
}
