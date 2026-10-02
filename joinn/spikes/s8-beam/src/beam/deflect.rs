use num_rational::BigRational;
use num_traits::{One, Zero};

use super::{Deflected, Derived, Support};
use crate::bridge::Bridge;
use crate::q::{Q, add, contract, frac, ratio, scale, total, wedge};
use crate::tag::Tag;
use crate::verdict::{Verdict, admit, refuse};

/// Rotation and deflection: the exact integrals of κ = M / EI along each line,
/// using M and V before the step:
///
/// ```text
/// θ ← θ + (M·ℓ + V·ℓ²/2 + q·ℓ³/6) / EI
/// v ← v + θ_prev·ℓ + (M·ℓ²/2 + V·ℓ³/6 + q·ℓ⁴/24) / EI
/// ```
///
/// Each bracket is a moment (M + V·ℓ/2 + q·ℓ²/6, and M/2 + V·ℓ/6 + q·ℓ²/24)
/// that crosses the bridge, is totalled over ℓ, and for v is taken along ℓ.
/// Simple span: θ at A is the one unknown that makes v = 0 at B.
/// Cantilever: θ = v = 0 at A. `what` names the quantity asked for.
pub fn deflect(
    derived: &Derived,
    support: Support,
    bridge: &mut Bridge,
    what: &str,
) -> Verdict<Vec<Deflected>> {
    let stations = &derived.stations;
    let Some(first) = stations.first() else {
        return refuse(format!(
            "deflection: {what} has no points; acceptance is a complex with both ends"
        ));
    };
    let mut theta = Q::new(BigRational::zero(), Tag::ROTATION);
    let mut v = Q::new(BigRational::zero(), Tag::DEFLECTION);
    let mut out = vec![Deflected {
        x: first.x.clone(),
        theta: theta.clone(),
        v: v.clone(),
    }];
    for (line, pair) in derived.complex.lines.iter().zip(stations.windows(2)) {
        let (s, next) = (&pair[0], &pair[1]);
        let l = &line.length;
        let vl = admit!(wedge(l, &s.v));
        let ql2 = admit!(wedge(l, &admit!(total(&line.q, l))));
        let m_theta = admit!(add(
            &admit!(add(&s.m, &scale(&vl, &frac!(1, 2)))),
            &scale(&ql2, &frac!(1, 6))
        ));
        let m_v = admit!(add(
            &admit!(add(&scale(&s.m, &frac!(1, 2)), &scale(&vl, &frac!(1, 6)))),
            &scale(&ql2, &frac!(1, 24))
        ));
        let d_theta = admit!(total(&admit!(bridge.bridge(&m_theta, what)), l));
        let bent = admit!(total(&admit!(bridge.bridge(&m_v, what)), l));
        let d_v = admit!(contract(l, &admit!(add(&theta, &bent))));
        v = admit!(add(&v, &d_v));
        theta = admit!(add(&theta, &d_theta));
        out.push(Deflected {
            x: next.x.clone(),
            theta: theta.clone(),
            v: v.clone(),
        });
    }
    if support == Support::Cantilever {
        return Verdict::Admitted(out);
    }
    let Some(b) = out.last() else {
        return Verdict::Admitted(out);
    };
    let unit = Q::new(BigRational::one(), Tag::ROTATION);
    let per_unit = admit!(contract(&b.x, &unit));
    let theta_a = scale(&unit, &(-admit!(ratio(&b.v, &per_unit))));
    let mut fixed = Vec::with_capacity(out.len());
    for d in &out {
        fixed.push(Deflected {
            x: d.x.clone(),
            theta: admit!(add(&d.theta, &theta_a)),
            v: admit!(add(&d.v, &admit!(contract(&d.x, &theta_a)))),
        });
    }
    Verdict::Admitted(fixed)
}

#[cfg(test)]
mod tests {
    use num_traits::Zero;

    use super::deflect;
    use crate::beam::{Order, deflected_at, derive, examples};
    use crate::bridge::{Bridge, Edition};
    use crate::q::{Q, frac, scale};
    use crate::tag::Tag;
    use crate::verdict::admitted;

    /// Every v §2.8 prints, in inches, signed: up positive.
    #[test]
    fn every_printed_deflection_is_the_plan_value() {
        let want: [(&str, i64, Q); 6] = [
            ("E1", 12, Q::new(frac!(-93312, 61625), Tag::DEFLECTION)),
            ("E2", 12, Q::new(frac!(-10368, 12325), Tag::DEFLECTION)),
            ("E3", 6, Q::new(frac!(-5832, 12325), Tag::DEFLECTION)),
            ("E4", 10, Q::new(frac!(-240, 493), Tag::DEFLECTION)),
            ("E5", 6, Q::new(frac!(-478224, 308125), Tag::DEFLECTION)),
            ("E5", 12, Q::new(frac!(-128952, 61625), Tag::DEFLECTION)),
        ];
        let mut printed = 0;
        for e in examples() {
            let mut bridge = Bridge::new(Some(Edition::aisc_for_tests()));
            let d = admitted(derive(&e, false, Order::Faithful, &mut bridge));
            let deflected = admitted(deflect(&d, e.support(), &mut bridge, "v"));
            for x in e.deflection_sections() {
                let got = admitted(deflected_at(&deflected, x));
                let feet = scale(x, &frac!(1, 12));
                let hit = want
                    .iter()
                    .find(|(id, at, _)| *id == e.id() && feet.value() == &frac!(*at, 1));
                assert_eq!(hit.map(|(_, _, v)| v), Some(got.v()), "{} v", e.id());
                printed += 1;
            }
        }
        assert_eq!(printed, want.len());
    }

    #[test]
    fn the_supports_hold_v_and_the_wall_holds_theta() {
        for e in examples() {
            let mut bridge = Bridge::new(Some(Edition::aisc_for_tests()));
            let d = admitted(derive(&e, false, Order::Faithful, &mut bridge));
            let deflected = admitted(deflect(&d, e.support(), &mut bridge, "v"));
            let a = &deflected[0];
            assert!(a.v().value().is_zero(), "{} v at A", e.id());
            match e.support() {
                crate::beam::Support::Simple => {
                    let b = &deflected[deflected.len() - 1];
                    assert!(b.v().value().is_zero(), "{} v at B", e.id());
                }
                crate::beam::Support::Cantilever => {
                    assert!(a.theta().value().is_zero(), "{} θ at A", e.id());
                }
            }
        }
    }

    #[test]
    fn balance_and_m_read_no_bridge_and_v_reads_two_per_line() {
        let mut reads = Vec::new();
        for e in examples() {
            let mut bridge = Bridge::new(Some(Edition::aisc_for_tests()));
            let d = admitted(derive(&e, false, Order::Faithful, &mut bridge));
            let before = d.reads_before_bridge();
            admitted(deflect(&d, e.support(), &mut bridge, "v"));
            reads.push((before, bridge.reads()));
        }
        assert_eq!(reads, [(0, 4), (0, 4), (0, 4), (0, 2), (0, 6)]);
    }
}
