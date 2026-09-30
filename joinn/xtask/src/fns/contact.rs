//! `cargo xtask contact`: every `.contact` in the corpus is admitted, lowers to
//! its wired twin, runs and describes itself as that twin does, keeps its sum
//! under every pairing, and answers the §2.14 catalogue as predicted.

mod corpus_files;
mod desc_lines;
mod lower_paired;
mod lowered_line;
mod mutant_lines;
mod pairings;
mod permutations;
mod run;
mod script_events;
mod surface_line;
mod transcript_line;

pub(crate) use run::contact;

/// The §2.12 script, as (instance, port, line).
const SCRIPT: [(&str, u32, &str); 3] = [("cli_a", 0, "two"), ("cli_a", 0, "2"), ("cli_b", 0, "3")];
