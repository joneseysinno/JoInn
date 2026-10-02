use super::Deflected;
use crate::q::{Q, frac, scale};
use crate::show::fraction;
use crate::verdict::{Verdict, refuse};

/// Refining the complex may change no θ or v at any shared point. Returns
/// how many points were compared.
pub fn refinement_of_deflection(
    id: &str,
    coarse: &[Deflected],
    fine: &[Deflected],
) -> Verdict<usize> {
    let (n_coarse, n_fine) = (coarse.len().saturating_sub(1), fine.len().saturating_sub(1));
    let feet = |q: &Q| fraction(scale(q, &frac!(1, 12)).value());
    for c in coarse {
        let Some(f) = fine.iter().find(|f| f.x == c.x) else {
            return refuse(format!(
                "refinement: {id} point {} ft is missing from the {n_fine}-line complex; acceptance is a refinement that keeps every point",
                feet(&c.x)
            ));
        };
        if f.v != c.v {
            return refuse(format!(
                "refinement: {id} v({} ft) {} in with {n_coarse} lines, {} in with {n_fine} lines; acceptance is a derivation the complex cannot change",
                feet(&c.x),
                fraction(c.v.value()),
                fraction(f.v.value())
            ));
        }
        if f.theta != c.theta {
            return refuse(format!(
                "refinement: {id} θ({} ft) {} with {n_coarse} lines, {} with {n_fine} lines; acceptance is a derivation the complex cannot change",
                feet(&c.x),
                fraction(c.theta.value()),
                fraction(f.theta.value())
            ));
        }
    }
    Verdict::Admitted(coarse.len())
}

#[cfg(test)]
mod tests {
    use super::refinement_of_deflection;
    use crate::beam::{Order, deflect, derive, examples};
    use crate::bridge::{Bridge, Edition};
    use crate::verdict::admitted;

    #[test]
    fn refined_theta_and_v_are_equal_in_all_five() {
        let mut compared = Vec::new();
        for e in examples() {
            let mut bridge = Bridge::new(Some(Edition::aisc_for_tests()));
            let coarse = admitted(derive(&e, false, Order::Faithful, &mut bridge));
            let fine = admitted(derive(&e, true, Order::Faithful, &mut bridge));
            let dc = admitted(deflect(&coarse, e.support(), &mut bridge, "v"));
            let df = admitted(deflect(&fine, e.support(), &mut bridge, "v"));
            compared.push(admitted(refinement_of_deflection(e.id(), &dc, &df)));
        }
        assert_eq!(compared, [3, 3, 3, 2, 4]);
    }
}
