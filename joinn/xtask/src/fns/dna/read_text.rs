//! What `cargo xtask read <file>` prints.

use joinn_codex::{
    AminoAcid, CODEX, Protein, READER_VERSION, print_proteins, read_file, translate,
};
use joinn_frame::Verdict;
use joinn_strand::print_letters;

use super::read_strand_file;

/// The proteins, then the partner strand's proteins, which must be the
/// opposites in reverse order. A refusal is the error, path first.
pub(crate) fn read_text(path: &str) -> Result<String, String> {
    let file = read_strand_file(path)?;
    let proteins = match read_file(&file) {
        Verdict::Ok(p) => p,
        Verdict::Refused(r) => return Err(format!("{path}: {}", r.reason)),
    };
    let partner = file.strand.partner();
    let partner_proteins = match translate(&partner) {
        Verdict::Ok(p) => p,
        Verdict::Refused(r) => return Err(format!("{path}: partner: {}", r.reason)),
    };
    let want: Vec<Vec<AminoAcid>> = proteins.iter().rev().map(Protein::opposite).collect();
    let got: Vec<Vec<AminoAcid>> = partner_proteins.iter().map(|p| p.acids.clone()).collect();
    if got != want {
        return Err(format!(
            "{path}: the partner reads {}; acceptance is the opposites in reverse order",
            print_proteins(&partner_proteins).trim_end()
        ));
    }
    let mut out = format!("file      {path}\ncodex {CODEX} · {READER_VERSION}\n");
    out.push_str(&format!("strand    {}\n", print_letters(&file.strand)));
    out.push_str(&print_proteins(&proteins));
    out.push_str(&format!("partner   {}\n", print_letters(&partner)));
    for line in print_proteins(&partner_proteins).lines() {
        out.push_str(&format!("partner {line}\n"));
    }
    out.push_str(&format!(
        "read: {} proteins; partner reads the opposites\n",
        proteins.len()
    ));
    Ok(out)
}
