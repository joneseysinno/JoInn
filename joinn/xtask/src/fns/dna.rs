//! `cargo xtask strand | codex`: Phase 8's letters, strands and codex 1,
//! printed from code (plan 8 §3).

mod codex_cmd;
mod codex_text;
mod read_strand_file;
mod strand_cmd;
mod strand_text;

pub(crate) use codex_cmd::codex;
pub(crate) use codex_text::codex_text;
pub(crate) use read_strand_file::read_strand_file;
pub(crate) use strand_cmd::strand;
pub(crate) use strand_text::strand_text;

#[cfg(test)]
mod tests {
    use super::{codex_text, strand_text};

    #[test]
    fn codex_prints_the_table_and_its_last_line() {
        let text = match codex_text() {
            Ok(t) => t,
            Err(e) => panic!("{e}"),
        };
        assert!(
            text.contains("AAT 00 00 01 odd   START  | ATT 00 01 01 even  STOP\n"),
            "{text}"
        );
        assert!(text.contains("\nreader 8.0\ncodex 1\n"), "{text}");
        assert!(
            text.ends_with("codex 1: 14 named, 50 reserved; every partner names the opposite\n"),
            "{text}"
        );
    }

    #[test]
    fn strand_prints_both_sides_and_refuses_a_bad_letter() {
        let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("target")
            .join("dna_strand");
        assert!(std::fs::create_dir_all(&dir).is_ok());
        let good = dir.join("pair-split.strand");
        let bad = dir.join("bad.strand");
        assert!(std::fs::write(&good, "codex 1\nAAT AAA TTT ATT\n").is_ok());
        assert!(std::fs::write(&bad, "codex 1\nAAT AXT\n").is_ok());
        let text = match strand_text(&good.display().to_string()) {
            Ok(t) => t,
            Err(e) => panic!("{e}"),
        };
        assert!(
            text.contains(
                "partner from its tail TTA TTT AAA TAA: side partner (head TTA 01 01 00 even)\n"
            ),
            "{text}"
        );
        assert!(
            text.ends_with("strand: 12 letters, 4 codons, side sense\n"),
            "{text}"
        );
        match strand_text(&bad.display().to_string()) {
            Err(e) => assert!(
                e.ends_with("strand: 'X' at letter 4 is not a letter; acceptance is A, T, G or C"),
                "{e}"
            ),
            Ok(t) => panic!("X read as a letter:\n{t}"),
        }
    }
}
