//! `cargo xtask strand | codex | read`: Phase 8's letters, strands, codex 1
//! and the reader, printed from code (plan 8 §3).

mod codex_cmd;
mod codex_text;
mod dna_witness;
mod read_cmd;
mod read_strand_file;
mod read_text;
mod strand_cmd;
mod strand_reading;
mod strand_text;
mod strand_witnesses;

pub(crate) use codex_cmd::codex;
pub(crate) use codex_text::codex_text;
pub(crate) use dna_witness::dna_witness;
pub(crate) use read_cmd::read;
pub(crate) use read_strand_file::read_strand_file;
pub(crate) use read_text::read_text;
pub(crate) use strand_cmd::strand;
pub(crate) use strand_reading::strand_reading;
pub(crate) use strand_text::strand_text;
pub(crate) use strand_witnesses::strand_witnesses;

#[cfg(test)]
mod tests {
    use super::super::{corpus_artifact, workspace_root};
    use super::{
        codex_text, read_strand_file, read_text, strand_reading, strand_text, strand_witnesses,
    };

    /// The bytes of a corpus file, line ends as committed.
    fn corpus_text(name: &str) -> String {
        let root = match workspace_root() {
            Ok(r) => r,
            Err(e) => panic!("{e}"),
        };
        match std::fs::read_to_string(corpus_artifact(&root, name)) {
            Ok(t) => t.replace("\r\n", "\n"),
            Err(e) => panic!("{name}: {e}"),
        }
    }

    #[test]
    fn every_strand_witness_reads_byte_identically() {
        let witnesses = match strand_witnesses() {
            Ok(w) => w,
            Err(e) => panic!("{e}"),
        };
        let mut read = 0;
        for (name, _) in &witnesses {
            let Some(stem) = name.strip_suffix(".strand") else {
                continue;
            };
            let root = match workspace_root() {
                Ok(r) => r,
                Err(e) => panic!("{e}"),
            };
            let path = corpus_artifact(&root, name).display().to_string();
            let file = match read_strand_file(&path) {
                Ok(f) => f,
                Err(e) => panic!("{e}"),
            };
            let (proteins, partner) = strand_reading(&file);
            assert_eq!(
                proteins,
                corpus_text(&format!("{stem}.proteins.txt")),
                "{name}"
            );
            assert_eq!(
                partner,
                corpus_text(&format!("{stem}.partner.txt")),
                "{name}"
            );
            for text in [&proteins, &partner] {
                assert!(
                    !text.contains(joinn_codex::READER_VERSION),
                    "{name}: {text}"
                );
            }
            read += 1;
        }
        assert_eq!(read, 4);
    }

    #[test]
    fn the_witnesses_on_disk_are_the_ones_codex_1_builds() {
        let witnesses = match strand_witnesses() {
            Ok(w) => w,
            Err(e) => panic!("{e}"),
        };
        for (name, text) in witnesses {
            assert_eq!(corpus_text(&name), text, "{name}");
        }
    }

    #[test]
    fn read_prints_the_partner_reading_and_refuses_a_reserved_codon() {
        let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("target")
            .join("dna_read");
        assert!(std::fs::create_dir_all(&dir).is_ok());
        let twelve = dir.join("twelve.strand");
        let reserved = dir.join("reserved.strand");
        assert!(
            std::fs::write(
                &twelve,
                "codex 1\nAAT AAA TTT CAC GTG AAC GTT CAA TTG GAA TTC AAG CTT ATT\n"
            )
            .is_ok()
        );
        assert!(std::fs::write(&reserved, "codex 1\nAAT TAA ATT\n").is_ok());
        let text = match read_text(&twelve.display().to_string()) {
            Ok(t) => t,
            Err(e) => panic!("{e}"),
        };
        assert!(
            text.contains("\nprotein at letter 0: pair split build case refer return sense show keep recall grant revoke\n"),
            "{text}"
        );
        assert!(
            text.contains("\npartner protein at letter 0: grant revoke keep recall sense show refer return build case pair split\n"),
            "{text}"
        );
        assert!(
            text.ends_with("read: 1 proteins; partner reads the opposites\n"),
            "{text}"
        );
        match read_text(&reserved.display().to_string()) {
            Err(e) => assert!(
                e.ends_with("reader: codon TAA at letter 3 names nothing in codex 1; acceptance is a codon codex 1 names"),
                "{e}"
            ),
            Ok(t) => panic!("a reserved codon read:\n{t}"),
        }
    }

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
