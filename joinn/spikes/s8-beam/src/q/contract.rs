use num_bigint::BigInt;
use num_rational::BigRational;

use super::Q;
use crate::tag::{Axis, Side, Tag};
use crate::verdict::{Verdict, refuse};

/// A line taken into a plane quantity: x ⌋ (x ∧ y) = y with value r·a, and
/// y ⌋ (x ∧ y) = −x with value −r·a (order-signed: the line comes first).
/// Placement with placement only. Lengths add.
pub fn contract(r: &Q, a: &Q) -> Verdict<Q> {
    let (axis, sign) = match (r.tag.axis(), a.tag.axis()) {
        (Axis::X, Axis::Plane) => (Axis::Y, 1),
        (Axis::Y, Axis::Plane) => (Axis::X, -1),
        (p, s) => {
            return refuse(format!(
                "contract: {p} ⌋ {s} is not a line into the plane; acceptance is x ⌋ plane or y ⌋ plane"
            ));
        }
    };
    if (r.tag.side(), a.tag.side()) != (Side::Placement, Side::Placement) {
        return refuse(format!(
            "contract: {} ⌋ {} has no side; acceptance is placement with placement",
            r.tag.side(),
            a.tag.side()
        ));
    }
    let Some(length) = r.tag.length().checked_add(a.tag.length()) else {
        return refuse(format!(
            "contract: length {} + {} overflows; acceptance is a length an i32 holds",
            r.tag.length(),
            a.tag.length()
        ));
    };
    Verdict::Admitted(Q {
        value: &r.value * &a.value * BigRational::from_integer(BigInt::from(sign)),
        tag: Tag::new(Side::Placement, length, axis),
    })
}

#[cfg(test)]
mod tests {
    use super::super::{Q, frac};
    use super::contract;
    use crate::tag::Tag;
    use crate::verdict::{admitted, refused};

    #[test]
    fn a_rotation_taken_along_a_line_is_a_deflection() {
        let line = Q::new(frac!(144, 1), Tag::PLACE_X);
        let theta = Q::new(frac!(-1, 100), Tag::ROTATION);
        assert_eq!(
            admitted(contract(&line, &theta)),
            Q::new(frac!(-36, 25), Tag::DEFLECTION)
        );
    }

    #[test]
    fn the_line_comes_first_and_must_be_placement() {
        let line = Q::new(frac!(1, 1), Tag::PLACE_X);
        let theta = Q::new(frac!(1, 1), Tag::ROTATION);
        assert_eq!(
            refused(contract(&theta, &line)),
            "contract: plane ⌋ x is not a line into the plane; acceptance is x ⌋ plane or y ⌋ plane"
        );
        let moment = Q::new(frac!(1, 1), Tag::MOMENT);
        assert_eq!(
            refused(contract(&line, &moment)),
            "contract: placement ⌋ source has no side; acceptance is placement with placement"
        );
    }
}
