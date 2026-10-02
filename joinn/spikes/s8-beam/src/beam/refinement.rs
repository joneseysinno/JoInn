use super::Station;
use crate::q::{Q, frac, scale};
use crate::show::fraction;
use crate::verdict::{Verdict, refuse};

/// Refining the complex may change no derived value at any shared point.
/// Compares M and V at every point of `coarse` with the same point of `fine`;
/// returns how many points were compared.
pub fn refinement(id: &str, coarse: &[Station], fine: &[Station]) -> Verdict<usize> {
    let (n_coarse, n_fine) = (coarse.len().saturating_sub(1), fine.len().saturating_sub(1));
    let feet = |q: &Q| fraction(scale(q, &frac!(1, 12)).value());
    for c in coarse {
        let Some(f) = fine.iter().find(|f| f.x == c.x) else {
            return refuse(format!(
                "refinement: {id} point {} ft is missing from the {n_fine}-line complex; acceptance is a refinement that keeps every point",
                feet(&c.x)
            ));
        };
        if f.m != c.m {
            return refuse(format!(
                "refinement: {id} M({} ft) {} kip·ft with {n_coarse} lines, {} kip·ft with {n_fine} lines; acceptance is a derivation the complex cannot change",
                feet(&c.x),
                feet(&c.m),
                feet(&f.m)
            ));
        }
        if f.v != c.v {
            return refuse(format!(
                "refinement: {id} V({} ft) {} kip with {n_coarse} lines, {} kip with {n_fine} lines; acceptance is a derivation the complex cannot change",
                feet(&c.x),
                fraction(c.v.value()),
                fraction(f.v.value())
            ));
        }
    }
    Verdict::Admitted(coarse.len())
}

#[cfg(test)]
mod tests {
    use super::refinement;
    use crate::beam::{Order, derive, examples};
    use crate::bridge::Bridge;
    use crate::verdict::admitted;

    #[test]
    fn refinement_is_equal_in_all_five() {
        let mut compared = Vec::new();
        for e in examples() {
            let coarse = admitted(derive(&e, false, Order::Faithful, &mut Bridge::new(None)));
            let fine = admitted(derive(&e, true, Order::Faithful, &mut Bridge::new(None)));
            compared.push((
                admitted(refinement(e.id(), coarse.stations(), fine.stations())),
                fine.complex().lines().len(),
            ));
        }
        assert_eq!(compared, [(3, 24), (3, 24), (3, 24), (2, 10), (4, 24)]);
    }
}
