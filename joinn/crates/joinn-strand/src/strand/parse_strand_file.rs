//! Read a strand file.

use joinn_frame::Verdict;

use super::{StrandFile, parse_letters};
use crate::refuse::refuse;

/// Line 1 is `codex <n>`; line 2 is the letters. Nothing follows. Which codex
/// is held is the reader's question, not the file's.
pub fn parse_strand_file(text: &str) -> Verdict<StrandFile> {
    let text = text.replace("\r\n", "\n");
    let mut lines = text.lines();
    let first = lines.next().unwrap_or_default();
    let Some(codex) = first
        .strip_prefix("codex ")
        .and_then(|n| n.parse::<u32>().ok())
        .filter(|n| *n > 0)
    else {
        return refuse(format!(
            "strand: line 1 is {first:?}; acceptance is `codex 1`"
        ));
    };
    let strand = match parse_letters(lines.next().unwrap_or_default()) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    if let Some(extra) = lines.find(|l| !l.is_empty()) {
        return refuse(format!(
            "strand: {extra:?} follows the letters; acceptance is two lines, the codex and the letters"
        ));
    }
    Verdict::Ok(StrandFile { codex, strand })
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use super::parse_strand_file;
    use crate::strand::print_strand_file;

    #[test]
    fn a_strand_file_round_trips() {
        let text = "codex 1\nAAT AAA TTT ATT\n";
        let Verdict::Ok(f) = parse_strand_file(text) else {
            panic!("reads");
        };
        assert_eq!(f.codex, 1);
        assert_eq!(f.strand.len(), 12);
        assert_eq!(print_strand_file(&f), text);
    }

    #[test]
    fn another_codex_is_read_and_a_missing_codex_line_is_refused() {
        let Verdict::Ok(f) = parse_strand_file("codex 2\nAAT ATT\n") else {
            panic!("a codex-2 strand file reads");
        };
        assert_eq!(f.codex, 2);
        match parse_strand_file("AAT ATT\n") {
            Verdict::Refused(r) => {
                assert_eq!(
                    r.reason,
                    "strand: line 1 is \"AAT ATT\"; acceptance is `codex 1`"
                )
            }
            Verdict::Ok(_) => panic!("no codex line"),
        }
    }

    #[test]
    fn the_empty_strand_is_two_lines() {
        let Verdict::Ok(f) = parse_strand_file("codex 1\n\n") else {
            panic!("reads");
        };
        assert!(f.strand.is_empty());
        assert_eq!(print_strand_file(&f), "codex 1\n\n");
    }
}
