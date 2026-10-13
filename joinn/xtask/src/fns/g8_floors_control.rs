//! Gate 8 item 2 control: the derived floors against the artifact.

use joinn_frame::Verdict;
use joinn_tower::parse_chromosomes;

use super::dna::floors_text;
use super::subject::Subject;
use super::{corpus_artifact, workspace_root};

/// True when the floors derived from `twelve.chromosome` differ from the
/// subject's text, or the chromosome cannot be read.
pub(crate) fn g8_floors_control(s: &Subject) -> bool {
    let Subject::Text(text) = s else {
        return true;
    };
    let Ok(root) = workspace_root() else {
        return true;
    };
    let Ok(held) =
        std::fs::read_to_string(corpus_artifact(&root, "phase8/tower/twelve.chromosome"))
    else {
        return true;
    };
    match parse_chromosomes(&held) {
        Verdict::Ok(all) => match all.first() {
            Some(c) => floors_text(c).trim() != text,
            None => true,
        },
        Verdict::Refused(_) => true,
    }
}
