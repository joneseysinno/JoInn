use crate::beam::{Example, Order, deflect, deflected_at, derive, station_at};
use crate::bridge::{Bridge, Edition};
use crate::q::{Q, add, dot};
use crate::verdict::{Verdict, admit, refuse};

/// add(M(12 ft) of E1, dot(P, v(12 ft)) of E2): a moment and a work, both
/// kip·in. `add` must refuse them.
pub fn moment_plus_work(e1: &Example, e2: &Example, edition: &Edition) -> Verdict<Q> {
    let (Some(x1), Some(x2), Some(load)) = (
        e1.moment_sections().first(),
        e2.deflection_sections().first(),
        e2.point_loads().first(),
    ) else {
        return refuse(
            "moment + work: E1 has no moment section or E2 no deflection section or load; acceptance is E1 and E2 as the plan states them".to_string(),
        );
    };
    let d1 = admit!(derive(e1, false, Order::Faithful, &mut Bridge::new(None)));
    let m = admit!(station_at(d1.stations(), x1)).m().clone();
    let mut bridge = Bridge::new(Some(edition.clone()));
    let d2 = admit!(derive(e2, false, Order::Faithful, &mut bridge));
    let deflected = admit!(deflect(&d2, e2.support(), &mut bridge, "v"));
    let v = admit!(deflected_at(&deflected, x2)).v().clone();
    let work = admit!(dot(load.force(), &v));
    add(&m, &work)
}

#[cfg(test)]
mod tests {
    use super::moment_plus_work;
    use crate::beam::examples;
    use crate::bridge::Edition;
    use crate::plants::line;

    #[test]
    fn moment_plus_work_is_refused_with_the_plan_line() {
        let all = examples();
        let (text, ok) = line(
            "moment + work",
            moment_plus_work(&all[0], &all[1], &Edition::aisc_for_tests()),
        );
        assert_eq!(
            text,
            "plant moment + work: refused (ok): add: source · length 1 · plane and energy · length 1 · none differ, though both are kip·in; acceptance is two quantities on one piece, side and pair"
        );
        assert!(ok);
    }
}
