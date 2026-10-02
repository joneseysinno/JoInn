use num_bigint::BigInt;
use num_rational::BigRational;

use super::Q;
use crate::tag::{Axis, Side, Tag};
use crate::verdict::{Verdict, refuse};

/// The plane product: x ∧ y → plane with value a·b, y ∧ x → plane with value
/// −a·b (order-signed). Lengths add.
pub fn wedge(a: &Q, b: &Q) -> Verdict<Q> {
    let sign = match (a.tag.axis(), b.tag.axis()) {
        (Axis::X, Axis::Y) => 1,
        (Axis::Y, Axis::X) => -1,
        (p, r) => {
            return refuse(format!(
                "wedge: {p} ∧ {r} is not the elevation plane; acceptance is x ∧ y or y ∧ x"
            ));
        }
    };
    let side = match (a.tag.side(), b.tag.side()) {
        (Side::Placement, Side::Source) | (Side::Source, Side::Placement) => Side::Source,
        (Side::Placement, Side::Placement) => Side::Placement,
        (s, r) => {
            return refuse(format!(
                "wedge: {s} ∧ {r} has no side; acceptance is placement with source, or placement with placement"
            ));
        }
    };
    let Some(length) = a.tag.length().checked_add(b.tag.length()) else {
        return refuse(format!(
            "wedge: length {} + {} overflows; acceptance is a length an i32 holds",
            a.tag.length(),
            b.tag.length()
        ));
    };
    Verdict::Admitted(Q {
        value: &a.value * &b.value * BigRational::from_integer(BigInt::from(sign)),
        tag: Tag::new(side, length, Axis::Plane),
    })
}

#[cfg(test)]
mod tests {
    use super::super::{Q, frac, scale};
    use super::wedge;
    use crate::tag::{Axis, Side, Tag};
    use crate::verdict::{admitted, refused};

    #[test]
    fn lever_arm_wedge_force_is_a_moment() {
        let r = Q::new(frac!(144, 1), Tag::PLACE_X);
        let f = Q::new(frac!(72, 5), Tag::FORCE);
        assert_eq!(
            admitted(wedge(&r, &f)),
            Q::new(frac!(10368, 5), Tag::MOMENT)
        );
    }

    #[test]
    fn wedge_is_order_signed() {
        let r = Q::new(frac!(72, 1), Tag::PLACE_X);
        let f = Q::new(frac!(-10, 1), Tag::FORCE);
        let rf = admitted(wedge(&r, &f));
        let fr = admitted(wedge(&f, &r));
        assert_eq!(fr, scale(&rf, &frac!(-1, 1)));
        assert_eq!(rf.tag(), fr.tag());
        assert_eq!(rf.value(), &frac!(-720, 1));
    }

    #[test]
    fn placement_wedge_placement_is_placement_and_lengths_add() {
        let x = Q::new(frac!(3, 1), Tag::PLACE_X);
        let v = Q::new(frac!(2, 1), Tag::DEFLECTION);
        assert_eq!(
            admitted(wedge(&x, &v)),
            Q::new(frac!(6, 1), Tag::new(Side::Placement, 2, Axis::Plane))
        );
    }

    #[test]
    fn source_wedge_source_is_refused() {
        let f = Q::new(frac!(1, 1), Tag::FORCE);
        let g = Q::new(frac!(1, 1), Tag::new(Side::Source, 0, Axis::X));
        assert_eq!(
            refused(wedge(&g, &f)),
            "wedge: source ∧ source has no side; acceptance is placement with source, or placement with placement"
        );
    }

    #[test]
    fn a_shared_axis_is_not_the_plane() {
        let x = Q::new(frac!(1, 1), Tag::PLACE_X);
        assert_eq!(
            refused(wedge(&x, &x)),
            "wedge: x ∧ x is not the elevation plane; acceptance is x ∧ y or y ∧ x"
        );
    }
}
