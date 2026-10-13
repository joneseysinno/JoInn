//! Gate 8 item 1 control: the reader's output against the artifact.

use joinn_codex::{print_proteins, read_file};
use joinn_frame::Verdict;
use joinn_strand::parse_strand_file;

use super::subject::Subject;
use super::{corpus_artifact, workspace_root};

/// True when the reader's output for `twelve.strand` differs from the
/// subject's text, or the strand cannot be read.
pub(crate) fn g8_reader_control(s: &Subject) -> bool {
    let Subject::Text(text) = s else {
        return true;
    };
    let Ok(root) = workspace_root() else {
        return true;
    };
    let Ok(strand) =
        std::fs::read_to_string(corpus_artifact(&root, "phase8/strands/twelve.strand"))
    else {
        return true;
    };
    let Verdict::Ok(file) = parse_strand_file(&strand) else {
        return true;
    };
    match read_file(&file) {
        Verdict::Ok(proteins) => print_proteins(&proteins).trim() != text,
        Verdict::Refused(_) => true,
    }
}
