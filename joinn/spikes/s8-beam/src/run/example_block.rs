use num_traits::{Signed, Zero};

use super::Block;
use crate::beam::{
    Example, Order, Support, deflect, deflected_at, derive, refinement, refinement_of_deflection,
    station_at,
};
use crate::bridge::{Bridge, Edition};
use crate::q::{Q, frac, scale};
use crate::show::{decimal, fraction};
use crate::verdict::Verdict;
use crate::witness::{Kind, compare, stated};

/// One example's lines: header, complex, balance, every printed M and v, the
/// witnesses, and refinement. Up is positive and sagging is positive inside;
/// the words are printed here.
pub fn example_block(example: &Example, edition: &Edition) -> Block {
    let mut block = Block {
        text: String::new(),
        agree: 0,
        disagree: 0,
        refined: false,
        clean: true,
    };
    let feet = |q: &Q| fraction(scale(q, &frac!(1, 12)).value());
    let in_feet = |q: &Q| scale(q, &frac!(1, 12)).value().abs();
    let force_word = |q: &Q| match q.value() {
        v if v.is_negative() => " down",
        v if v.is_positive() => " up",
        _ => "",
    };
    let moment_word = |q: &Q| match q.value() {
        v if v.is_negative() => " hogging",
        v if v.is_positive() => " sagging",
        _ => "",
    };
    let shown = |r: &num_rational::BigRational| match decimal(r) {
        Some(d) => format!(" ({d})"),
        None => String::new(),
    };

    let mut loads = Vec::new();
    let mut senses = Vec::new();
    if let Some(q) = example.uniform() {
        loads.push(format!(
            "w {} kip/ft",
            fraction(&(q.value().abs() * num_rational::BigRational::from_integer(12.into())))
        ));
        senses.push(force_word(q));
    }
    for load in example.point_loads() {
        loads.push(format!("P {} kip", fraction(&load.force().value().abs())));
        senses.push(force_word(load.force()));
    }
    let sense = match senses.as_slice() {
        [one] => *one,
        _ => "",
    };
    block.text.push_str(&format!(
        "{} {} · L {} ft · {}{sense}\n",
        example.id(),
        example.title(),
        feet(example.span()),
        loads.join(" · ")
    ));

    let mut bridge = Bridge::new(Some(edition.clone()));
    let derived = match derive(example, false, Order::Faithful, &mut bridge) {
        Verdict::Admitted(d) => d,
        Verdict::Refused(r) => {
            block.text.push_str(&format!("  refused: {}\n", r.reason()));
            block.clean = false;
            return block;
        }
    };
    let (n_points, n_lines) = (
        derived.complex().points().len(),
        derived.complex().lines().len(),
    );
    block.text.push_str(&format!(
        "  complex: {n_points} point{}, {n_lines} line{} (elevation plane)\n",
        if n_points == 1 { "" } else { "s" },
        if n_lines == 1 { "" } else { "s" }
    ));

    let balance = derived.balance();
    let r_a = balance.r_a();
    let second = match (example.support(), balance.r_b(), balance.couple()) {
        (Support::Simple, Some(r_b), _) => format!(
            "R_B {} kip{}",
            fraction(&r_b.value().abs()),
            force_word(r_b)
        ),
        (Support::Cantilever, _, Some(couple)) => {
            let m_a = scale(couple, &frac!(-1, 1));
            format!(
                "M_A {} kip·ft{}",
                fraction(&in_feet(&m_a)),
                moment_word(&m_a)
            )
        }
        _ => {
            block.clean = false;
            "no second reaction".to_string()
        }
    };
    let reads = match derived.reads_before_bridge() {
        0 => "no bridge read".to_string(),
        n => {
            block.clean = false;
            format!("{n} bridge read(s)")
        }
    };
    if !balance.sum_f().value().is_zero() || !balance.sum_m().value().is_zero() {
        block.clean = false;
    }
    block.text.push_str(&format!(
        "  balance: R_A {} kip{}, {second} · ΣF {} · ΣM {} · M at end {} · {reads}\n",
        fraction(&r_a.value().abs()),
        force_word(r_a),
        fraction(balance.sum_f().value()),
        feet(balance.sum_m()),
        feet(derived.end())
    ));

    for x in example.moment_sections() {
        match station_at(derived.stations(), x) {
            Verdict::Admitted(s) => {
                let m = in_feet(s.m());
                block.text.push_str(&format!(
                    "  M({} ft) {} kip·ft{}{}\n",
                    feet(x),
                    fraction(&m),
                    moment_word(s.m()),
                    shown(&m)
                ));
            }
            Verdict::Refused(r) => {
                block
                    .text
                    .push_str(&format!("  M: refused: {}\n", r.reason()));
                block.clean = false;
            }
        }
    }

    let what = match example.deflection_sections().first() {
        Some(x) => format!("v({} ft)", feet(x)),
        None => "v".to_string(),
    };
    let deflected = match deflect(&derived, example.support(), &mut bridge, &what) {
        Verdict::Admitted(d) => d,
        Verdict::Refused(r) => {
            block
                .text
                .push_str(&format!("  v: refused: {}\n", r.reason()));
            block.clean = false;
            Vec::new()
        }
    };
    for x in example.deflection_sections() {
        if let Verdict::Admitted(d) = deflected_at(&deflected, x) {
            let v = d.v().value().abs();
            block.text.push_str(&format!(
                "  v({} ft) {} in{}{}\n",
                feet(x),
                fraction(&v),
                force_word(d.v()),
                shown(&v)
            ));
        }
    }

    for witness in stated(example, edition) {
        let (derived_value, stated_value, unit) = match witness.kind() {
            Kind::Moment => (
                match station_at(derived.stations(), witness.at()) {
                    Verdict::Admitted(s) => Verdict::Admitted(s.m().clone()),
                    Verdict::Refused(r) => Verdict::Refused(r),
                },
                witness.value() / num_rational::BigRational::from_integer(12.into()),
                "kip·ft",
            ),
            Kind::Deflection => (
                match deflected_at(&deflected, witness.at()) {
                    Verdict::Admitted(d) => Verdict::Admitted(d.v().clone()),
                    Verdict::Refused(r) => Verdict::Refused(r),
                },
                witness.value().clone(),
                "in",
            ),
        };
        let verdict = match derived_value {
            Verdict::Admitted(value) => compare(example.id(), &witness, &value),
            Verdict::Refused(r) => Verdict::Refused(r),
        };
        let head = format!(
            "  witness {} = {} {unit}",
            witness.name(),
            fraction(&stated_value)
        );
        match verdict {
            Verdict::Admitted(()) => {
                block.agree += 1;
                block.text.push_str(&format!("{head}: agree\n"));
            }
            Verdict::Refused(r) => {
                block.disagree += 1;
                block.clean = false;
                block
                    .text
                    .push_str(&format!("{head}: disagree: {}\n", r.reason()));
            }
        }
    }

    let fine = match derive(example, true, Order::Faithful, &mut bridge) {
        Verdict::Admitted(fine) => fine,
        Verdict::Refused(r) => {
            block
                .text
                .push_str(&format!("  refined: refused: {}\n", r.reason()));
            block.clean = false;
            return block;
        }
    };
    let n_fine = fine.complex().lines().len();
    let fine_deflected = deflect(&fine, example.support(), &mut bridge, &what);
    let checked = match (
        refinement(example.id(), derived.stations(), fine.stations()),
        fine_deflected,
    ) {
        (Verdict::Admitted(n), Verdict::Admitted(fine_d)) => {
            match refinement_of_deflection(example.id(), &deflected, &fine_d) {
                Verdict::Admitted(_) => Verdict::Admitted(n),
                Verdict::Refused(r) => Verdict::Refused(r),
            }
        }
        (Verdict::Refused(r), _) | (_, Verdict::Refused(r)) => Verdict::Refused(r),
    };
    match checked {
        Verdict::Admitted(n) => {
            block.refined = true;
            block.text.push_str(&format!(
                "  refined to {n_fine} lines: equal at every point of the {n}\n"
            ));
        }
        Verdict::Refused(r) => {
            block.clean = false;
            block
                .text
                .push_str(&format!("  refined to {n_fine} lines: {}\n", r.reason()));
        }
    }
    block
}
