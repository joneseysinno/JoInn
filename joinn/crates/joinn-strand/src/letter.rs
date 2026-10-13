//! A letter is two bits (rule 82): A = 00, T = 01, G = 10, C = 11. The high
//! bit is its strength, the low bit its shape; its partner is the same letter
//! with the low bit flipped.

mod bits;
mod from_bits;
mod from_char;
mod partner;
mod shape;
mod strength;
mod to_char;

/// One of the four letters. It carries which of four it is and nothing else.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Letter {
    /// `00`: weak, big.
    A,
    /// `01`: weak, small.
    T,
    /// `10`: strong, big.
    G,
    /// `11`: strong, small.
    C,
}

/// The high bit. Pairing never changes it.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Strength {
    /// High bit 0: A, T.
    Weak,
    /// High bit 1: G, C.
    Strong,
}

/// The low bit. Pairing always swaps it.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Shape {
    /// Low bit 0 (purine): A, G.
    Big,
    /// Low bit 1 (pyrimidine): T, C.
    Small,
}

impl Letter {
    /// The four letters in bit order.
    pub const ALL: [Letter; 4] = [Letter::A, Letter::T, Letter::G, Letter::C];
}

#[cfg(test)]
mod tests {
    use super::{Letter, Shape, Strength};

    #[test]
    fn the_bits_are_the_plans_table() {
        let bits: Vec<(char, u8)> = Letter::ALL
            .iter()
            .map(|l| (l.to_char(), l.bits()))
            .collect();
        assert_eq!(bits, [('A', 0b00), ('T', 0b01), ('G', 0b10), ('C', 0b11)]);
        for l in Letter::ALL {
            assert_eq!(Letter::from_bits(l.bits()), l);
            assert_eq!(Letter::from_char(l.to_char()), Some(l));
        }
    }

    #[test]
    fn all_four_partners() {
        let pairs: Vec<(Letter, Letter)> = Letter::ALL.iter().map(|l| (*l, l.partner())).collect();
        assert_eq!(
            pairs,
            [
                (Letter::A, Letter::T),
                (Letter::T, Letter::A),
                (Letter::G, Letter::C),
                (Letter::C, Letter::G),
            ]
        );
    }

    #[test]
    fn partner_is_an_involution_with_no_fixed_letter() {
        for l in Letter::ALL {
            assert_eq!(l.partner().partner(), l);
            assert_ne!(l.partner(), l, "{l:?} is its own partner");
            assert_eq!(l.partner().bits(), l.bits() ^ 1, "partner is the low bit");
        }
    }

    #[test]
    fn pairing_keeps_strength_and_swaps_shape() {
        for l in Letter::ALL {
            assert_eq!(l.partner().strength(), l.strength());
            assert_ne!(l.partner().shape(), l.shape());
        }
        assert_eq!(Letter::A.strength(), Strength::Weak);
        assert_eq!(Letter::T.strength(), Strength::Weak);
        assert_eq!(Letter::G.strength(), Strength::Strong);
        assert_eq!(Letter::C.strength(), Strength::Strong);
        assert_eq!(Letter::A.shape(), Shape::Big);
        assert_eq!(Letter::G.shape(), Shape::Big);
        assert_eq!(Letter::T.shape(), Shape::Small);
        assert_eq!(Letter::C.shape(), Shape::Small);
    }

    #[test]
    fn no_other_character_is_a_letter() {
        let letters: Vec<char> = (0u8..=127)
            .map(char::from)
            .filter(|c| Letter::from_char(*c).is_some())
            .collect();
        assert_eq!(letters, ['A', 'C', 'G', 'T']);
    }
}
