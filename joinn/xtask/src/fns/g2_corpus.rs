//! Phase 2 item: the corpus still verifies.

use super::corpus_verify;

pub(crate) fn g2_corpus() -> bool {
    corpus_verify().is_ok()
}
