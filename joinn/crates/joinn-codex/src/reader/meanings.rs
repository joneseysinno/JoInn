//! What each codon of a strand means, for a picture.

use joinn_strand::Strand;

use crate::codex::lookup;
use crate::codon::Codon;
use crate::meaning::Meaning;

/// Each whole codon in frame from letter 0 and its meaning in codex 1, `None`
/// for a reserved codon. Letters past the last whole codon are left out. A
/// view draws this; it never looks a codon up itself (plan 8 §2.10).
pub fn meanings(strand: &Strand) -> Vec<(Codon, Option<Meaning>)> {
    strand
        .letters()
        .chunks_exact(3)
        .map(|c| {
            let codon = Codon::from_letters([c[0], c[1], c[2]]);
            (codon, lookup(codon))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_strand::parse_letters;

    use super::meanings;
    use crate::meaning::{AminoAcid, Meaning};

    #[test]
    fn whole_codons_only_and_reserved_is_none() {
        let Verdict::Ok(s) = parse_letters("AAT AAA TAA AT") else {
            panic!("letters");
        };
        let got: Vec<Option<Meaning>> = meanings(&s).into_iter().map(|(_, m)| m).collect();
        assert_eq!(
            got,
            [
                Some(Meaning::Start),
                Some(Meaning::Acid(AminoAcid::Pair)),
                None
            ]
        );
    }
}
