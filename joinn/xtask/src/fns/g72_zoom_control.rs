//! Gate 7.2 item 1 control: the `function` lens can't be laid out.

use joinn_frame::Verdict;
use joinn_visual::layout_universe;

use super::corpus_store::corpus_store;
use super::subject::Subject;

/// True when `layout_universe(subject, "function")` refuses.
pub(crate) fn g72_zoom_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return false;
    };
    let Ok((_, store)) = corpus_store() else {
        return false;
    };
    matches!(layout_universe(u, store, "function"), Verdict::Refused(_))
}
