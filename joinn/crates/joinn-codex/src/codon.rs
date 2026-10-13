//! A codon: three letters, six bits, 64 in all (D9). All three letters carry
//! meaning (D13). Its parity is its count of 1 bits, mod 2; its partner is the
//! three letters' partners, reversed, and always has the opposite parity.

mod all;
mod bit_text;
mod from_letters;
mod letters;
mod parity;
mod parity_name;
mod partner;
mod text;

/// Six bits, the first letter in the two high bits.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Codon(u8);

/// A codon's count of 1 bits, mod 2.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Parity {
    /// An even count.
    Even,
    /// An odd count.
    Odd,
}

impl Codon {
    /// The six bits, `0 ..= 63`.
    pub fn index(self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use joinn_strand::Letter;

    use super::{Codon, Parity};

    #[test]
    fn there_are_64_codons_and_letters_round_trip() {
        let all = Codon::all();
        assert_eq!(all.len(), 64);
        for (i, c) in all.iter().enumerate() {
            assert_eq!(usize::from(c.index()), i);
            assert_eq!(Codon::from_letters(c.letters()), *c);
        }
    }

    #[test]
    fn parity_flips_for_all_64_and_no_codon_is_its_own_partner() {
        for c in Codon::all() {
            let p = c.partner();
            assert_ne!(c.parity(), p.parity(), "{}", c.text());
            assert_ne!(c, p, "{}", c.text());
            assert_eq!(p.partner(), c, "{}", c.text());
        }
    }

    #[test]
    fn start_reads_odd_and_stop_reads_even() {
        let start = Codon::from_letters([Letter::A, Letter::A, Letter::T]);
        let stop = Codon::from_letters([Letter::A, Letter::T, Letter::T]);
        assert_eq!(start.bit_text(), "00 00 01");
        assert_eq!(stop.bit_text(), "00 01 01");
        assert_eq!(start.parity(), Parity::Odd);
        assert_eq!(stop.parity(), Parity::Even);
        assert_eq!(start.partner(), stop);
        println!(
            "AAT {} {}, ATT {} {}",
            start.bit_text(),
            start.parity().name(),
            stop.bit_text(),
            stop.parity().name()
        );
    }
}
