//! A strand: a row of letters with a head and a tail, read head to tail (D6).
//! Its partner strand is every letter's partner, in reverse order. Packed, it
//! is four letters to a byte, the first letter in the two high bits, with its
//! length in letters beside it. Its identity is JoInn's hash over its length
//! and its packed bytes (D8).

mod identity;
mod new;
mod pack;
mod parse_letters;
mod parse_strand_file;
mod partner;
mod print_letters;
mod print_strand_file;
mod unpack;
mod with_letter;

pub use parse_letters::parse_letters;
pub use parse_strand_file::parse_strand_file;
pub use print_letters::print_letters;
pub use print_strand_file::print_strand_file;

use crate::letter::Letter;

/// The domain tag of the DNA family: a strand's identity and every Phase 8
/// corpus file's hash.
pub const TAG_DNA: &[u8] = b"joinn.dna.v1";

/// Letters from head to tail.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Strand {
    letters: Vec<Letter>,
}

/// A strand packed four letters to a byte, first letter in the high bits.
/// `len` is in letters, so a last partial byte is exact.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Packed {
    bytes: Vec<u8>,
    len: u64,
}

/// A strand file: line 1 `codex <n>` pins the table that reads it, line 2 the
/// letters.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StrandFile {
    /// The codex line's number.
    pub codex: u32,
    /// The letters.
    pub strand: Strand,
}

impl Strand {
    /// The letters, head first.
    pub fn letters(&self) -> &[Letter] {
        &self.letters
    }

    /// Letters in the strand.
    pub fn len(&self) -> usize {
        self.letters.len()
    }

    /// True for the strand of no letters.
    pub fn is_empty(&self) -> bool {
        self.letters.is_empty()
    }
}

impl Packed {
    /// The packed bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Letters packed.
    pub fn len(&self) -> u64 {
        self.len
    }

    /// True when no letter is packed.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[cfg(test)]
mod tests {
    use super::Strand;
    use crate::letter::Letter;

    /// A strand of `n` letters that walks through all four.
    fn strand_of(n: usize) -> Strand {
        Strand::new(
            (0..n)
                .map(|i| Letter::from_bits(((i * 7 + i / 5) % 4) as u8))
                .collect(),
        )
    }

    #[test]
    fn pack_and_unpack_round_trip_for_lengths_0_to_64() {
        for n in 0..=64 {
            let s = strand_of(n);
            let p = s.pack();
            assert_eq!(p.len(), n as u64);
            assert_eq!(p.bytes().len(), n.div_ceil(4), "n {n}");
            assert_eq!(p.unpack(), s, "n {n}");
        }
    }

    #[test]
    fn the_first_letter_is_in_the_high_bits() {
        let s = Strand::new(vec![Letter::A, Letter::T, Letter::G, Letter::C, Letter::C]);
        let p = s.pack();
        assert_eq!(p.bytes(), [0b00_01_10_11, 0b11_00_00_00]);
        assert_eq!(p.len(), 5);
    }

    #[test]
    fn the_partner_of_the_partner_is_the_strand() {
        for n in 0..=64 {
            let s = strand_of(n);
            assert_eq!(s.partner().partner(), s, "n {n}");
        }
        let s = Strand::new(vec![
            Letter::A,
            Letter::A,
            Letter::A,
            Letter::C,
            Letter::A,
            Letter::C,
        ]);
        let want = Strand::new(vec![
            Letter::G,
            Letter::T,
            Letter::G,
            Letter::T,
            Letter::T,
            Letter::T,
        ]);
        assert_eq!(s.partner(), want, "AAA CAC pairs with GTG TTT");
    }

    #[test]
    fn a_strand_and_its_partner_differ_in_identity_unless_they_are_one_strand() {
        for n in 1..=64 {
            let s = strand_of(n);
            let p = s.partner();
            assert_eq!(s == p, s.identity() == p.identity(), "n {n}");
            assert_eq!(s.identity(), s.pack().unpack().identity());
            if n % 2 == 1 {
                assert_ne!(
                    s, p,
                    "no letter is its own partner, so odd n never pairs with itself"
                );
            }
        }
        let own = Strand::new(vec![
            Letter::G,
            Letter::A,
            Letter::A,
            Letter::T,
            Letter::T,
            Letter::C,
        ]);
        assert_eq!(own.partner(), own, "GAATTC is its own partner strand");
        let not = Strand::new(vec![
            Letter::A,
            Letter::A,
            Letter::T,
            Letter::A,
            Letter::A,
            Letter::A,
        ]);
        assert_ne!(not.identity(), not.partner().identity());
    }

    #[test]
    fn length_is_part_of_identity() {
        let three_a = Strand::new(vec![Letter::A; 3]);
        let four_a = Strand::new(vec![Letter::A; 4]);
        assert_eq!(three_a.pack().bytes(), four_a.pack().bytes());
        assert_ne!(three_a.identity(), four_a.identity());
    }

    #[test]
    fn with_letter_changes_one_letter() {
        let s = strand_of(6);
        let Some(t) = s.with_letter(4, Letter::C) else {
            panic!("letter 4 exists");
        };
        assert_eq!(t.letters()[4], Letter::C);
        assert_eq!(&t.letters()[..4], &s.letters()[..4]);
        assert!(s.with_letter(6, Letter::A).is_none());
    }
}
