//! Gate 7.3 item 3: a folded system is touched once.

use std::collections::BTreeSet;

use joinn_frame::Verdict;
use joinn_gpu::open;
use joinn_visual::{
    Camera, ChartId, ChartKind, FOCUS_UNIT, LEVEL_MIN, LINK_TAG, Pick, Piece, PieceKind,
    SIXTEENTHS, TAG_MASK, UniverseScene, Zoom, cpu_pick_sample, fold_at,
};

use super::g6_adapters::g6_adapters;
use super::g73_pictures::g73_pictures;
use super::grove::grove_layout;
use super::zoom::{VIEWPORT, zoom_views};

/// In sixteenths: past a region leg's half-width (48) and a pixel's slack.
const CLEAR: i64 = 64;

/// The grove's routes at the `s`, level −4 step 0: 264 touches and 136 legs,
/// each touch a folded system, no system touched twice by one link, and every
/// member stood for. At the frame: 1182 stubs. Then a camera at level 3 step
/// 0 centred on a point of a leg that serves one member, in the first open
/// route that has one: the longest such leg's midpoint, quarter or eighth
/// point clear of every other open piece by more than the widest half-width:
/// `cpu_pick` names that leg's link and member there, and on every adapter
/// the GPU's owner of the pixel is the same.
pub(crate) fn g73_folded() -> bool {
    let Some(adapters) = g6_adapters() else {
        return false;
    };
    let (grove, layout) = match grove_layout() {
        Ok(g) => g,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let mut failures = Vec::new();
    let views = zoom_views(&layout);
    let low = views
        .iter()
        .find(|(l, c)| l == "at s" && c.zoom.level == LEVEL_MIN && c.zoom.step == 0);
    let framed = views.iter().find(|(l, _)| l == "frame");
    let (Some((_, low)), Some((_, framed))) = (low, framed) else {
        println!("the grove's views have no frame or no level −4 view at the s");
        return false;
    };
    let routes = &layout.routes;
    let (folded, unfolded) = (fold_at(low.zoom), fold_at(framed.zoom));
    let (mut touches, mut legs) = (0usize, 0usize);
    for r in routes.routes.iter().filter(|r| r.fold == folded) {
        touches += r.touches.len();
        legs += r.legs;
        let id = routes.ids.get(r.link).map_or("?", String::as_str);
        let charts: BTreeSet<ChartId> = r.touches.iter().map(|t| t.chart).collect();
        if charts.len() != r.touches.len() {
            failures.push(format!(
                "folded grove: link {id} touches a node more than once"
            ));
        }
        let kinds = r
            .touches
            .iter()
            .filter(|t| layout.chart(t.chart).map(|c| c.kind) != Some(ChartKind::System))
            .count();
        if kinds != 0 {
            failures.push(format!(
                "folded grove: link {id} touches {kinds} node(s) that are not folded systems"
            ));
        }
        let members = grove
            .coding
            .links
            .get(r.link)
            .map_or(0, |l| l.members.len());
        let stood: BTreeSet<usize> = r
            .touches
            .iter()
            .flat_map(|t| t.members.iter().copied())
            .collect();
        if stood != (0..members).collect() {
            failures.push(format!(
                "folded grove: link {id} stands for members {stood:?} of {members}"
            ));
        }
    }
    let stubs: usize = routes
        .routes
        .iter()
        .filter(|r| r.fold == unfolded)
        .map(|r| r.stubs)
        .sum();
    println!(
        "folded grove: {} {touches} touches, {legs} legs; {} {stubs} stubs",
        folded.name(),
        unfolded.name()
    );
    for (what, got, want) in [
        ("touches at level −4", touches, 264),
        ("legs at level −4", legs, 136),
        ("stubs at the frame", stubs, 1182),
    ] {
        if got != want {
            failures.push(format!("folded grove: {what} {got}; acceptance is {want}"));
        }
    }
    let length = |a: (i64, i64), b: (i64, i64)| (b.0 - a.0).abs() + (b.1 - a.1).abs();
    let Some(route) = routes
        .routes
        .iter()
        .filter(|r| r.fold == unfolded)
        .find(|r| {
            r.pieces
                .iter()
                .any(|p| p.kind == PieceKind::Leg && p.member > 0)
        })
    else {
        println!("folded grove: no open route has a leg that serves one member");
        return false;
    };
    let mut legs: Vec<(usize, &Piece)> = route
        .pieces
        .iter()
        .enumerate()
        .filter(|(_, p)| p.kind == PieceKind::Leg && p.member > 0)
        .collect();
    legs.sort_by_key(|(_, p)| std::cmp::Reverse(length(p.a, p.b)));
    let clear = |q: (i64, i64), own: usize| {
        routes
            .routes
            .iter()
            .filter(|r| r.fold == unfolded)
            .flat_map(|r| {
                r.pieces
                    .iter()
                    .enumerate()
                    .filter(move |(i, _)| !(std::ptr::eq(r, route) && *i == own))
            })
            .all(|(_, p)| {
                let dx = (p.a.0.min(p.b.0) - q.0).max(q.0 - p.a.0.max(p.b.0));
                let dy = (p.a.1.min(p.b.1) - q.1).max(q.1 - p.a.1.max(p.b.1));
                dx.max(dy) > CLEAR
            })
    };
    let spot = legs.iter().find_map(|(i, p)| {
        [(1, 2), (1, 4), (3, 4), (1, 8), (3, 8), (5, 8), (7, 8)]
            .iter()
            .map(|(n, d)| {
                (
                    p.a.0 + (p.b.0 - p.a.0) * n / d,
                    p.a.1 + (p.b.1 - p.a.1) * n / d,
                )
            })
            .find(|q| clear(*q, *i))
            .map(|q| (**p, q))
    });
    let Some((leg, mid)) = spot else {
        println!(
            "folded grove: no point of a member's leg is clear of every other piece; acceptance is one"
        );
        return false;
    };
    let unit = FOCUS_UNIT / SIXTEENTHS;
    let (w, h) = VIEWPORT;
    let camera = layout.rebase(Camera {
        zoom: Zoom { level: 3, step: 0 },
        anchor: ChartId(0),
        focus: (mid.0 * unit, mid.1 * unit),
        pin: (i64::from(w / 2), i64::from(h / 2)),
        width: w,
        height: h,
    });
    let origin = layout
        .chart(camera.anchor)
        .map_or((0, 0), |c| c.root_origin);
    let at = |m: i64, o: i64| (m - SIXTEENTHS * o) * unit;
    let (px, py) = camera.pixel_of((at(mid.0, origin.0), at(mid.1, origin.1)));
    let (Ok(x), Ok(y)) = (u32::try_from(px >> 32), u32::try_from(py >> 32)) else {
        println!("folded grove: the leg's midpoint is off the viewport");
        return false;
    };
    let scene = match UniverseScene::grow(layout.clone(), camera.anchor) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => {
            println!("{}", r.reason);
            return false;
        }
    };
    let picked = scene
        .shapes_at(&camera)
        .and_then(|shapes| cpu_pick_sample(&shapes, &camera, &[(x, y)]));
    let cpu = match picked {
        Verdict::Ok(p) => p.first().copied(),
        Verdict::Refused(r) => {
            println!("{}", r.reason);
            return false;
        }
    };
    let want_slot = u32::try_from(route.link + 1).unwrap_or(0);
    let Some(Pick::Owned(id)) = cpu else {
        println!("folded grove: pick {x},{y}: cpu {cpu:?}; acceptance is the leg's link");
        return false;
    };
    let name = match scene.print_id(id) {
        Verdict::Ok(n) => n,
        Verdict::Refused(r) => r.reason,
    };
    println!(
        "folded pick {x},{y} at level 3 step 0: {name} (cpu; leg of member index {})",
        leg.member - 1
    );
    if id[2] & TAG_MASK != LINK_TAG || id[0] != want_slot || id[1] != leg.member {
        failures.push(format!(
            "folded pick {x},{y}: cpu names {name}; acceptance is the leg's link, member index {}",
            leg.member - 1
        ));
    }
    let scenes = [scene];
    let i = y as usize * w as usize + x as usize;
    for adapter in &adapters {
        let gpu = match open(adapter) {
            Verdict::Ok(g) => g,
            Verdict::Refused(r) => {
                println!("{}", r.reason);
                return false;
            }
        };
        let images = match g73_pictures(&gpu, &scenes, &[(0, camera)]) {
            Ok(i) => i,
            Err(e) => {
                println!("{e}");
                return false;
            }
        };
        let got = images.first().and_then(|image| image.get(i)).copied();
        if got == Some(id) {
            println!(
                "folded pick {} {x},{y}: {name} (gpu) · agree",
                adapter.line()
            );
        } else {
            failures.push(format!(
                "folded pick {} {x},{y}: gpu {got:?}, cpu {id:?}; acceptance is the cpu's owner",
                adapter.line()
            ));
        }
    }
    for f in &failures {
        println!("{f}");
    }
    failures.is_empty()
}
