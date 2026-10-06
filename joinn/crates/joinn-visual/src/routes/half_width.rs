//! A piece's half-width in its form (§2.5).

use super::{Form, Piece, PieceKind};

/// In sixteenths: a stub 4 in every form; an arrowhead 16 (2 wide); a spine's
/// leg 8; a region's leg or knot 48; a hub's leg 4; a bundle's leg
/// `4 + 2·(legs − 1)`, at most 48; a hub's or bundle's knot disc 16.
pub fn half_width(form: Form, piece: &Piece) -> i64 {
    match (piece.kind, form) {
        (PieceKind::Stub, _) => 4,
        (PieceKind::Arrow, _) => 16,
        (_, Form::Region) => 48,
        (PieceKind::Knot, _) => 16,
        (PieceKind::Leg, Form::Spine) => 8,
        (PieceKind::Leg, Form::Hub) => 4,
        (PieceKind::Leg, Form::Bundle) => (4 + 2 * (i64::from(piece.legs) - 1)).clamp(4, 48),
    }
}

#[cfg(test)]
mod tests {
    use super::half_width;
    use crate::routes::{Form, Piece, PieceKind};

    #[test]
    fn a_bundle_widens_an_eighth_per_sharing_leg_up_to_three() {
        let leg = |legs| Piece {
            a: (0, 0),
            b: (16, 0),
            kind: PieceKind::Leg,
            member: 0,
            legs,
        };
        assert_eq!(half_width(Form::Bundle, &leg(1)), 4);
        assert_eq!(half_width(Form::Bundle, &leg(3)), 8);
        assert_eq!(half_width(Form::Bundle, &leg(23)), 48);
        assert_eq!(half_width(Form::Bundle, &leg(40)), 48);
        assert_eq!(half_width(Form::Hub, &leg(40)), 4);
        assert_eq!(half_width(Form::Region, &leg(1)), 48);
        assert_eq!(half_width(Form::Spine, &leg(1)), 8);
    }
}
