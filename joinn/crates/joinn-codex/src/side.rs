//! A strand's side, read from the parity of its head codon: START (`AAT`,
//! `00 00 01`) reads odd, so an odd head is `sense`; STOP (`ATT`) reads even,
//! and so does START read from the other strand's tail (`TTA`), so an even
//! head is `partner`.

mod of_strand;
mod side_name;

pub use of_strand::side;

/// Which way a strand reads.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Side {
    /// The head codon is odd.
    Sense,
    /// The head codon is even.
    Partner,
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_strand::{Strand, parse_letters};

    use super::{Side, side};

    fn strand(text: &str) -> Strand {
        match parse_letters(text) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn start_at_the_head_reads_sense_and_from_the_other_tail_reads_partner() {
        let s = strand("AAT AAA TTT ATT");
        assert_eq!(side(&s), Some(Side::Sense));
        let tail_first = Strand::new(s.partner().letters().iter().rev().copied().collect());
        assert_eq!(side(&tail_first), Some(Side::Partner), "TTA TTT AAA TAA");
        assert_eq!(side(&strand("AA")), None);
    }
}
