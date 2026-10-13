//! Gate 8 item 1: the reader holds the one table.

use joinn_codex::{CODEX_1, Codon, check_table, codon_of, lookup};
use joinn_frame::Verdict;

use super::dna::strand_witnesses;
use super::{corpus_artifact, workspace_root};

/// Codex 1 passes the codon algebra with 14 named codons; every named codon
/// is the one codon its meaning has; and every strand witness (the strand,
/// its proteins, its partner's reading) is what the reader prints from that
/// table today, byte for byte.
pub(crate) fn g8_reader() -> bool {
    let named = match check_table(&CODEX_1) {
        Verdict::Ok(n) => n,
        Verdict::Refused(r) => {
            println!("reader: {}", r.reason);
            return false;
        }
    };
    let mut failures = Vec::new();
    if named != 14 {
        failures.push(format!(
            "reader: codex 1 names {named} codons; acceptance is 14"
        ));
    }
    for c in Codon::all() {
        if let Some(m) = lookup(c)
            && codon_of(m) != Some(c)
        {
            failures.push(format!(
                "reader: {} means {} but {} is written {:?}; acceptance is one codon per meaning",
                c.text(),
                m.name(),
                m.name(),
                codon_of(m).map(Codon::text)
            ));
        }
    }
    let root = match workspace_root() {
        Ok(r) => r,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let witnesses = match strand_witnesses() {
        Ok(w) => w,
        Err(e) => {
            println!("reader: {e}");
            return false;
        }
    };
    for (name, text) in &witnesses {
        match std::fs::read_to_string(corpus_artifact(&root, name)) {
            Ok(on_disk) if on_disk.replace("\r\n", "\n") == *text => {}
            Ok(_) => failures.push(format!(
                "reader: {name} differs from the reader's output; acceptance is the witness byte for byte"
            )),
            Err(e) => failures.push(format!("reader: {name}: {e}")),
        }
    }
    for f in &failures {
        println!("{f}");
    }
    if failures.is_empty() {
        println!(
            "reader: codex 1 names {named} codons, one per meaning; {} strand witnesses read back byte for byte",
            witnesses.len()
        );
    }
    failures.is_empty()
}
