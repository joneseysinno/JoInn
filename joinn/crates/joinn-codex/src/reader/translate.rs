//! Translate a strand into proteins.

use joinn_frame::Verdict;
use joinn_strand::Strand;

use super::Protein;
use crate::codex::lookup;
use crate::codon::Codon;
use crate::meaning::Meaning;
use crate::refuse::refuse;

/// Codons in frame from letter 0, in order. A reserved codon, a START with no
/// STOP, a STOP or amino acid outside a protein, a START inside one, or
/// letters past the last whole codon each refuse the strand at that letter.
pub fn translate(strand: &Strand) -> Verdict<Vec<Protein>> {
    let letters = strand.letters();
    let mut proteins = Vec::new();
    let mut open: Option<Protein> = None;
    for (k, chunk) in letters.chunks(3).enumerate() {
        let at = k * 3;
        let [a, b, c] = chunk else {
            return refuse(format!(
                "reader: {} letters after the last codon at letter {at}; acceptance is whole codons",
                chunk.len()
            ));
        };
        let codon = Codon::from_letters([*a, *b, *c]);
        let Some(meaning) = lookup(codon) else {
            return refuse(format!(
                "reader: codon {} at letter {at} names nothing in codex 1; acceptance is a codon codex 1 names",
                codon.text()
            ));
        };
        match (meaning, open.take()) {
            (Meaning::Start, None) => {
                open = Some(Protein {
                    start: at,
                    acids: Vec::new(),
                })
            }
            (Meaning::Start, Some(p)) => {
                return refuse(format!(
                    "reader: start at letter {at} opens inside the protein at letter {}; acceptance is a stop first",
                    p.start
                ));
            }
            (Meaning::Stop, Some(p)) => proteins.push(p),
            (Meaning::Stop, None) => {
                return refuse(format!(
                    "reader: stop at letter {at} closes no start; acceptance is a start before it"
                ));
            }
            (Meaning::Acid(acid), Some(mut p)) => {
                p.acids.push(acid);
                open = Some(p);
            }
            (Meaning::Acid(acid), None) => {
                return refuse(format!(
                    "reader: {} at letter {at} is outside a protein; acceptance is an amino acid between a start and a stop",
                    acid.name()
                ));
            }
        }
    }
    if let Some(p) = open {
        return refuse(format!(
            "reader: start at letter {} has no stop; acceptance is a stop in frame",
            p.start
        ));
    }
    Verdict::Ok(proteins)
}
