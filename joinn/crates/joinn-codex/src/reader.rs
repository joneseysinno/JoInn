//! The reader: codex 1 applied to a strand in frame from letter 0. Each START
//! opens a protein and the next STOP closes it; a protein is the amino acids
//! between them (D10, D20). Anything else is a refusal, never a guess.

mod opposite;
mod print_proteins;
mod read_file;
mod translate;

pub use print_proteins::print_proteins;
pub use read_file::read_file;
pub use translate::translate;

use crate::meaning::AminoAcid;

/// A chain of amino acids and the letter its START sits at.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Protein {
    /// The START codon's first letter.
    pub start: usize,
    /// The amino acids, head first. Empty is the empty protein (D25).
    pub acids: Vec<AminoAcid>,
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_strand::{Strand, StrandFile, parse_letters};

    use super::{Protein, print_proteins, read_file, translate};
    use crate::codex::codon_of;
    use crate::meaning::{AminoAcid, Meaning};

    fn strand(text: &str) -> Strand {
        match parse_letters(text) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    fn twelve() -> Strand {
        let mut meanings = vec![Meaning::Start];
        meanings.extend(AminoAcid::ALL.iter().map(|a| Meaning::Acid(*a)));
        meanings.push(Meaning::Stop);
        let letters = meanings
            .into_iter()
            .filter_map(codon_of)
            .flat_map(|c| c.letters())
            .collect();
        Strand::new(letters)
    }

    fn refusal(s: &Strand) -> String {
        match translate(s) {
            Verdict::Refused(r) => r.reason,
            Verdict::Ok(p) => panic!("read: {p:?}"),
        }
    }

    #[test]
    fn twelve_reads_as_one_protein_and_its_partner_reads_the_opposites() {
        let s = twelve();
        let Verdict::Ok(proteins) = translate(&s) else {
            panic!("twelve reads");
        };
        assert_eq!(
            proteins,
            [Protein {
                start: 0,
                acids: AminoAcid::ALL.to_vec()
            }]
        );
        let Verdict::Ok(partner) = translate(&s.partner()) else {
            panic!("its partner reads");
        };
        let want: Vec<Vec<AminoAcid>> = proteins.iter().rev().map(Protein::opposite).collect();
        let got: Vec<Vec<AminoAcid>> = partner.iter().map(|p| p.acids.clone()).collect();
        assert_eq!(got, want);
        assert_eq!(
            print_proteins(&partner),
            "protein at letter 0: grant revoke keep recall sense show refer return build case pair split\n"
        );
    }

    #[test]
    fn the_empty_protein_and_the_empty_strand() {
        let Verdict::Ok(p) = translate(&strand("AAT ATT")) else {
            panic!("AAT ATT reads");
        };
        assert_eq!(print_proteins(&p), "protein at letter 0: nothing\n");
        let Verdict::Ok(none) = translate(&strand("")) else {
            panic!("the empty strand reads");
        };
        assert_eq!(print_proteins(&none), "no protein\n");
    }

    #[test]
    fn two_proteins_read_in_frame_order() {
        let Verdict::Ok(p) = translate(&strand("AAT AAA ATT AAT TTT ATT")) else {
            panic!("reads");
        };
        assert_eq!(
            print_proteins(&p),
            "protein at letter 0: pair\nprotein at letter 9: split\n"
        );
    }

    #[test]
    fn each_refusal_says_what_where_and_what_is_accepted() {
        assert_eq!(
            refusal(&strand("AAT AAA AAA AAA TAA ATT")),
            "reader: codon TAA at letter 12 names nothing in codex 1; acceptance is a codon codex 1 names"
        );
        assert_eq!(
            refusal(&strand("AAT AAA")),
            "reader: start at letter 0 has no stop; acceptance is a stop in frame"
        );
        assert_eq!(
            refusal(&strand("AAT ATT ATT")),
            "reader: stop at letter 6 closes no start; acceptance is a start before it"
        );
        assert_eq!(
            refusal(&strand("AAA AAT ATT")),
            "reader: pair at letter 0 is outside a protein; acceptance is an amino acid between a start and a stop"
        );
        assert_eq!(
            refusal(&strand("AAT AAT ATT")),
            "reader: start at letter 3 opens inside the protein at letter 0; acceptance is a stop first"
        );
        assert_eq!(
            refusal(&strand("AAT ATT AA")),
            "reader: 2 letters after the last codon at letter 6; acceptance is whole codons"
        );
    }

    #[test]
    fn another_codex_is_refused() {
        let file = StrandFile {
            codex: 2,
            strand: strand("AAT ATT"),
        };
        match read_file(&file) {
            Verdict::Refused(r) => {
                assert_eq!(
                    r.reason,
                    "reader: strand names codex 2; this reader holds codex 1"
                )
            }
            Verdict::Ok(p) => panic!("codex 2 read: {p:?}"),
        }
    }
}
