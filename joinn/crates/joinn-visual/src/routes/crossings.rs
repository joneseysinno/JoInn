//! V16, checked exactly (plan 7.3 §2.6; V148).

use joinn_link::Link;

use super::{Fold, Piece, PieceKind, Route, SIXTEENTHS};
use crate::camera::ChartId;
use crate::charts::{ChartKind, UniverseLayout};

/// Half-widths in sixteenths, each piece at its widest form: a leg of an
/// unordered link 3 (region), a spine's 1/2, a stub 1/4, a knot's disc and an
/// arrowhead's half-width 1.
const LEG: i64 = 48;
const SPINE: i64 = 8;
const STUB: i64 = 4;
const DISC: i64 = 16;

/// Every way `route` breaks §2.6, one line each, naming the link, the piece
/// and the rectangle: (1) a piece meets a cell rectangle of any body, except
/// a stub's end at its own port; (2) a piece meets a non-member body's
/// surface; (3) a piece enters a folded node, except a leg ending at that
/// node's touch point; (4) a stub does not start exactly at its port's centre.
/// "Meets" is a distance below the piece's half-width, in sixteenths.
pub fn crossings(layout: &UniverseLayout, links: &[Link], route: &Route) -> Vec<String> {
    let s = SIXTEENTHS;
    let Some(link) = links.get(route.link) else {
        return vec![format!("route of link {}: no such link", route.link)];
    };
    let units = |v: i64| {
        let (whole, part) = (v.div_euclid(s), v.rem_euclid(s));
        if part == 0 {
            whole.to_string()
        } else {
            format!("{whole}+{part}/16")
        }
    };
    let piece_text = |p: &Piece| {
        format!(
            "{:?} ({}, {})-({}, {})",
            p.kind,
            units(p.a.0),
            units(p.a.1),
            units(p.b.0),
            units(p.b.1)
        )
        .to_lowercase()
    };
    let head = format!("link {} fold {}", link.id, route.fold.name());
    let half = |p: &Piece| match p.kind {
        PieceKind::Leg if route.ordered => SPINE,
        PieceKind::Leg => LEG,
        PieceKind::Stub => STUB,
        PieceKind::Knot | PieceKind::Arrow => DISC,
    };
    // Squared distance from an axis-aligned piece to a closed rectangle.
    let gap = |a: (i64, i64), b: (i64, i64), r: (i64, i64, i64, i64)| {
        let (x0, x1) = (a.0.min(b.0), a.0.max(b.0));
        let (y0, y1) = (a.1.min(b.1), a.1.max(b.1));
        let dx = (r.0 - x1).max(x0 - (r.0 + r.2)).max(0);
        let dy = (r.1 - y1).max(y0 - (r.1 + r.3)).max(0);
        dx * dx + dy * dy
    };
    let meets = |a, b, r, hw: i64| gap(a, b, r) < hw * hw;
    // The piece with `hw` taken off its `end` end.
    let shorten = |p: &Piece, end: (i64, i64), hw: i64| {
        let (far, near) = if p.a == end { (p.b, p.a) } else { (p.a, p.b) };
        let d = ((far.0 - near.0).signum(), (far.1 - near.1).signum());
        let length = (far.0 - near.0).abs() + (far.1 - near.1).abs();
        let cut = hw.min(length);
        ((near.0 + d.0 * cut, near.1 + d.1 * cut), far)
    };
    let members: Vec<&str> = link.members.iter().map(|m| m.body.as_str()).collect();
    let mut out = Vec::new();
    for (i, chart) in layout.charts.iter().enumerate() {
        let id = ChartId(u32::try_from(i).unwrap_or(u32::MAX));
        let (ox, oy) = (chart.root_origin.0 * s, chart.root_origin.1 * s);
        let folded = matches!(
            (chart.kind, route.fold),
            (ChartKind::System, Fold::Systems) | (ChartKind::Galaxy, Fold::Galaxies)
        );
        if folded {
            let rect = (ox, oy, chart.size.0 * s, chart.size.1 * s);
            let touch: Vec<(i64, i64)> = route
                .touches
                .iter()
                .filter(|t| t.chart == id)
                .map(|t| (t.point.0 * s, t.point.1 * s))
                .collect();
            for p in &route.pieces {
                let hw = half(p);
                if !meets(p.a, p.b, rect, hw) {
                    continue;
                }
                let ends_here = touch
                    .iter()
                    .find(|t| p.kind == PieceKind::Leg && (p.a == **t || p.b == **t));
                let excused = ends_here.is_some_and(|t| {
                    let (a, b) = shorten(p, *t, hw);
                    !meets(a, b, rect, hw)
                });
                if !excused {
                    out.push(format!(
                        "{head}: {} enters folded {} ({}, {}, {}, {})",
                        piece_text(p),
                        chart.name,
                        chart.root_origin.0,
                        chart.root_origin.1,
                        chart.size.0,
                        chart.size.1
                    ));
                }
            }
            continue;
        }
        if chart.kind != ChartKind::Body {
            continue;
        }
        let Some(laid) = chart.body.and_then(|(_, li)| layout.layouts.get(li)) else {
            continue;
        };
        let member_here = members.contains(&chart.name.as_str());
        if !member_here {
            let rect = (ox, oy, laid.surface.w * s, laid.surface.h * s);
            for p in route
                .pieces
                .iter()
                .filter(|p| meets(p.a, p.b, rect, half(p)))
            {
                out.push(format!(
                    "{head}: {} meets the surface of {}, not a member ({}, {}, {}, {})",
                    piece_text(p),
                    chart.name,
                    chart.root_origin.0,
                    chart.root_origin.1,
                    laid.surface.w,
                    laid.surface.h
                ));
            }
        }
        for cell in &laid.cells {
            let rect = (
                ox + cell.rect.x * s,
                oy + cell.rect.y * s,
                cell.rect.w * s,
                cell.rect.h * s,
            );
            for p in &route.pieces {
                let hw = half(p);
                if !meets(p.a, p.b, rect, hw) {
                    continue;
                }
                let own_port = p.kind == PieceKind::Stub
                    && member_here
                    && laid.ports.iter().any(|port| {
                        port.address.instance == cell.instance
                            && (ox + port.x * s, oy + port.y * s) == p.a
                            && link.members.iter().any(|m| {
                                m.body == chart.name
                                    && m.instance == port.address.instance
                                    && m.port == port.address.port
                            })
                    });
                let excused = own_port && {
                    let (a, b) = shorten(p, p.a, hw);
                    !meets(a, b, rect, hw)
                };
                if !excused {
                    out.push(format!(
                        "{head}: {} meets cell {}.{} ({}, {}, {}, {})",
                        piece_text(p),
                        chart.name,
                        cell.instance,
                        chart.root_origin.0 + cell.rect.x,
                        chart.root_origin.1 + cell.rect.y,
                        cell.rect.w,
                        cell.rect.h
                    ));
                }
            }
        }
    }
    // (4) Each member's first stub piece starts at its port's centre.
    for (m, member) in link.members.iter().enumerate() {
        let tag = u32::try_from(m + 1).unwrap_or(0);
        let Some(first) = route
            .pieces
            .iter()
            .find(|p| p.kind == PieceKind::Stub && p.member == tag)
        else {
            continue;
        };
        let centre = layout
            .charts
            .iter()
            .find(|c| c.kind == ChartKind::Body && c.name == member.body)
            .and_then(|c| {
                let laid = c.body.and_then(|(_, li)| layout.layouts.get(li))?;
                let port = laid.ports.iter().find(|p| {
                    p.address.instance == member.instance && p.address.port == member.port
                })?;
                Some((
                    (c.root_origin.0 + port.x) * s,
                    (c.root_origin.1 + port.y) * s,
                ))
            });
        if centre != Some(first.a) {
            out.push(format!(
                "{head}: {} does not start at the centre of {}.{}@{}",
                piece_text(first),
                member.body,
                member.instance,
                member.port
            ));
        }
    }
    out
}
