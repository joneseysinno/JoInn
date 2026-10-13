//! The strand witnesses of plan 8 §2.9, built from codex 1.

use joinn_codex::{AminoAcid, CODEX, Codon, Meaning, codon_of};
use joinn_strand::{Letter, Strand, StrandFile, print_strand_file};

use super::strand_reading;

/// `(corpus name, text)` for each strand file, its `.proteins.txt` and its
/// `.partner.txt`, in the plan's order.
pub(crate) fn strand_witnesses() -> Result<Vec<(String, String)>, String> {
    let named = |m: Meaning| codon_of(m).ok_or(format!("codex 1 has no codon for {}", m.name()));
    let start = named(Meaning::Start)?;
    let stop = named(Meaning::Stop)?;
    let mut twelve = vec![start];
    for a in AminoAcid::ALL {
        twelve.push(named(Meaning::Acid(a))?);
    }
    twelve.push(stop);
    let strands: [(&str, Vec<Codon>); 4] = [
        ("empty", vec![start, stop]),
        (
            "pair-split",
            vec![
                start,
                named(Meaning::Acid(AminoAcid::Pair))?,
                named(Meaning::Acid(AminoAcid::Split))?,
                stop,
            ],
        ),
        ("twelve", twelve),
        (
            "reserved",
            vec![
                start,
                Codon::from_letters([Letter::T, Letter::A, Letter::A]),
                stop,
            ],
        ),
    ];
    let mut out = Vec::new();
    for (name, codons) in strands {
        let file = StrandFile {
            codex: CODEX,
            strand: Strand::new(codons.iter().flat_map(|c| c.letters()).collect()),
        };
        let (proteins, partner) = strand_reading(&file);
        let at = format!("phase8/strands/{name}");
        out.push((format!("{at}.strand"), print_strand_file(&file)));
        out.push((format!("{at}.proteins.txt"), proteins));
        out.push((format!("{at}.partner.txt"), partner));
    }
    Ok(out)
}
