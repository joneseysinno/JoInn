use super::Q;
use crate::tag::{Axis, Side, Tag};
use crate::verdict::{Verdict, refuse};

/// The along product: x · x or y · y → axis none, value a·b (order-blind).
/// Source with placement is energy. Lengths add.
pub fn dot(a: &Q, b: &Q) -> Verdict<Q> {
    match (a.tag.axis(), b.tag.axis()) {
        (Axis::X, Axis::X) | (Axis::Y, Axis::Y) => {}
        (p, r) => {
            return refuse(format!(
                "dot: {p} · {r} share no axis; acceptance is x · x or y · y"
            ));
        }
    }
    let side = match (a.tag.side(), b.tag.side()) {
        (Side::Placement, Side::Source) | (Side::Source, Side::Placement) => Side::Energy,
        (Side::Placement, Side::Placement) => Side::Placement,
        (s, r) => {
            return refuse(format!(
                "dot: {s} · {r} has no side; acceptance is source with placement, or placement with placement"
            ));
        }
    };
    let Some(length) = a.tag.length().checked_add(b.tag.length()) else {
        return refuse(format!(
            "dot: length {} + {} overflows; acceptance is a length an i32 holds",
            a.tag.length(),
            b.tag.length()
        ));
    };
    Verdict::Admitted(Q {
        value: &a.value * &b.value,
        tag: Tag::new(side, length, Axis::None),
    })
}

#[cfg(test)]
mod tests {
    use super::super::{Q, frac};
    use super::dot;
    use crate::tag::{Axis, Side, Tag};
    use crate::verdict::{admitted, refused};

    #[test]
    fn force_dot_displacement_is_work() {
        let p = Q::new(frac!(-10, 1), Tag::FORCE);
        let v = Q::new(frac!(-10368, 12325), Tag::DEFLECTION);
        assert_eq!(
            admitted(dot(&p, &v)),
            Q::new(frac!(103680, 12325), Tag::WORK)
        );
    }

    #[test]
    fn dot_is_order_blind() {
        let p = Q::new(frac!(-10, 1), Tag::FORCE);
        let v = Q::new(frac!(3, 7), Tag::DEFLECTION);
        assert_eq!(admitted(dot(&p, &v)), admitted(dot(&v, &p)));
        let x = Q::new(frac!(5, 2), Tag::PLACE_X);
        let y = Q::new(frac!(-4, 3), Tag::PLACE_X);
        let xy = admitted(dot(&x, &y));
        assert_eq!(xy, admitted(dot(&y, &x)));
        assert_eq!(
            xy,
            Q::new(frac!(-10, 3), Tag::new(Side::Placement, 2, Axis::None))
        );
    }

    #[test]
    fn different_axes_share_nothing() {
        let x = Q::new(frac!(1, 1), Tag::PLACE_X);
        let f = Q::new(frac!(1, 1), Tag::FORCE);
        assert_eq!(
            refused(dot(&x, &f)),
            "dot: x · y share no axis; acceptance is x · x or y · y"
        );
    }

    #[test]
    fn source_dot_source_is_refused() {
        let f = Q::new(frac!(1, 1), Tag::FORCE);
        assert_eq!(
            refused(dot(&f, &f)),
            "dot: source · source has no side; acceptance is source with placement, or placement with placement"
        );
    }
}
