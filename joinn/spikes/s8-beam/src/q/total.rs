use super::Q;
use crate::tag::Tag;
use crate::verdict::{Verdict, refuse};

/// A density added up over a line: combine along the line. The tag is the
/// density's, with length + 1.
pub fn total(a: &Q, line: &Q) -> Verdict<Q> {
    if line.tag != Tag::PLACE_X {
        return refuse(format!(
            "total: over {}; acceptance is a line, {}",
            line.tag,
            Tag::PLACE_X
        ));
    }
    if a.tag.length() >= 0 {
        return refuse(format!(
            "total: {} is not a density; acceptance is a quantity per length",
            a.tag
        ));
    }
    Verdict::Admitted(Q {
        value: &a.value * &line.value,
        tag: Tag::new(a.tag.side(), a.tag.length() + 1, a.tag.axis()),
    })
}

#[cfg(test)]
mod tests {
    use super::super::{Q, frac};
    use super::total;
    use crate::tag::Tag;
    use crate::verdict::{admitted, refused};

    #[test]
    fn a_load_totalled_over_a_line_is_a_force() {
        let q = Q::new(frac!(-1, 10), Tag::DENSITY);
        let line = Q::new(frac!(288, 1), Tag::PLACE_X);
        assert_eq!(
            admitted(total(&q, &line)),
            Q::new(frac!(-144, 5), Tag::FORCE)
        );
    }

    #[test]
    fn curvature_totalled_over_a_line_is_a_rotation() {
        let kappa = Q::new(frac!(1, 1000), Tag::CURVATURE);
        let line = Q::new(frac!(12, 1), Tag::PLACE_X);
        assert_eq!(
            admitted(total(&kappa, &line)),
            Q::new(frac!(3, 250), Tag::ROTATION)
        );
    }

    #[test]
    fn total_needs_a_line_and_a_density() {
        let q = Q::new(frac!(-1, 10), Tag::DENSITY);
        let v = Q::new(frac!(1, 1), Tag::DEFLECTION);
        assert_eq!(
            refused(total(&q, &v)),
            "total: over placement · length 1 · y; acceptance is a line, placement · length 1 · x"
        );
        let f = Q::new(frac!(10, 1), Tag::FORCE);
        let line = Q::new(frac!(12, 1), Tag::PLACE_X);
        assert_eq!(
            refused(total(&f, &line)),
            "total: source · length 0 · y is not a density; acceptance is a quantity per length"
        );
    }
}
