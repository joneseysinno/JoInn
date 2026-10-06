//! A route's form at a zoom (§2.5).

use super::{FORM_THRESHOLDS, Form, Route};
use crate::camera::Zoom;

/// A spine when the link's order is declared, at every zoom; otherwise the
/// owner rule over `s = k·size`: bundle when `10·s ≥ 11·1920`, hub when
/// `10·s ≥ 11·240`, else region. With it, `Some(T)` when `T ≤ s < 1.2·T`
/// (the form crossfade window), in integers as `owner_band` tests.
pub fn form_of(zoom: Zoom, route: &Route) -> (Form, Option<i64>) {
    if route.ordered {
        return (Form::Spine, None);
    }
    let a = ((256 + i128::from(zoom.step)) * i128::from(route.size)) << zoom.level.max(0);
    let b = 256i128 << (-zoom.level).max(0);
    let past = |t: i64| 10 * a >= 11 * i128::from(t) * b;
    let [hub, bundle] = FORM_THRESHOLDS;
    let form = if past(bundle) {
        Form::Bundle
    } else if past(hub) {
        Form::Hub
    } else {
        Form::Region
    };
    let fading = FORM_THRESHOLDS.into_iter().find(|t| {
        let t = i128::from(*t);
        5 * a >= 5 * t * b && 5 * a < 6 * t * b
    });
    (form, fading)
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_link::{Link, Mark, Member, Order};

    use super::form_of;
    use crate::camera::{LEVEL_MAX, LEVEL_MIN, Zoom};
    use crate::fixtures::phase5_universe;
    use crate::routes::{Fold, Form, PieceKind, Route, routes};

    fn sized(size: i64, ordered: bool) -> Route {
        Route {
            link: 0,
            fold: Fold::Open,
            ordered,
            touches: Vec::new(),
            knot: None,
            legs: 0,
            stubs: 0,
            size,
            pieces: Vec::new(),
        }
    }

    #[test]
    fn at_10s_equal_to_11t_the_higher_form_owns_and_one_step_below_the_lower() {
        let z = |level, step| Zoom { level, step };
        let r = sized(33, false);
        // s = 33·8 = 264 = 1.1·240; one step below, s = 33·511/64 < 264.
        assert_eq!(form_of(z(3, 0), &r), (Form::Hub, Some(240)));
        assert_eq!(form_of(z(2, 255), &r), (Form::Region, Some(240)));
        // s = 33·64 = 2112 = 1.1·1920; one step below, s = 33·511/8 < 2112.
        assert_eq!(form_of(z(6, 0), &r), (Form::Bundle, Some(1920)));
        assert_eq!(form_of(z(5, 255), &r), (Form::Hub, Some(1920)));
        // s = 30·8 = 240 opens the window; s = 36·8 = 288 = 1.2·240 closes it.
        assert_eq!(
            form_of(z(3, 0), &sized(30, false)),
            (Form::Region, Some(240))
        );
        assert_eq!(form_of(z(3, 0), &sized(36, false)), (Form::Hub, None));
    }

    #[test]
    fn an_ordered_link_is_a_spine_at_every_zoom_and_an_unordered_one_never() {
        let layout = phase5_universe();
        let u = match joinn_link::parse_universe(include_str!(
            "../../../../corpus/phase5/universe.universe"
        )) {
            Verdict::Ok(u) => u,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let port_of = |alias: &str, n: usize| {
            let laid = layout
                .charts
                .iter()
                .find(|c| c.name == alias)
                .and_then(|c| c.body)
                .and_then(|(_, li)| layout.layouts.get(li));
            let p = laid
                .and_then(|l| l.ports.get(n))
                .unwrap_or_else(|| panic!("{alias} port {n}"));
            Member {
                body: alias.to_owned(),
                instance: p.address.instance.clone(),
                port: p.address.port,
                mark: Mark::None,
            }
        };
        let mut links = u.coding.links.clone();
        links.push(Link {
            id: "trio".to_owned(),
            order: Order::None,
            members: vec![port_of("calc", 0), port_of("calc", 1), port_of("units", 0)],
        });
        let all = match routes(&layout, &links) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        for route in &all.routes {
            let declared = route.ordered;
            assert_eq!(declared, route.link == 0);
            if !declared {
                assert!(
                    !route
                        .pieces
                        .iter()
                        .any(|p| p.kind == PieceKind::Arrow && p.member == 0),
                    "unordered link has a mid-path arrowhead"
                );
            }
            for level in LEVEL_MIN..=LEVEL_MAX {
                for step in [0, 128, 255] {
                    let (form, _) = form_of(Zoom { level, step }, route);
                    assert_eq!(form == Form::Spine, declared, "level {level} step {step}");
                }
            }
        }
    }
}
