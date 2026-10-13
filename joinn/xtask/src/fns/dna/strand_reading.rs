//! A strand file's reading, as its witnesses hold it.

use joinn_codex::{print_proteins, read_file, translate};
use joinn_frame::Verdict;
use joinn_strand::{StrandFile, print_letters};

/// The `.proteins.txt` text (the protein list, or the refusal line) and the
/// `.partner.txt` text (`partner <letters>`, then the partner's reading).
pub(crate) fn strand_reading(file: &StrandFile) -> (String, String) {
    let proteins = match read_file(file) {
        Verdict::Ok(p) => print_proteins(&p),
        Verdict::Refused(r) => format!("{}\n", r.reason),
    };
    let partner = file.strand.partner();
    let partner_proteins = match translate(&partner) {
        Verdict::Ok(p) => print_proteins(&p),
        Verdict::Refused(r) => format!("{}\n", r.reason),
    };
    (
        proteins,
        format!("partner {}\n{partner_proteins}", print_letters(&partner)),
    )
}
