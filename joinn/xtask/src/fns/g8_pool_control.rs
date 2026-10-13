//! Gate 8 item 3 control: the subject's chromosomes entering a pool.

use joinn_frame::Verdict;
use joinn_tower::{Pool, parse_chromosomes};

use super::subject::Subject;

/// True when every chromosome the subject holds enters a fresh pool, in
/// order: the cycle is admitted. A subject that does not parse is true.
pub(crate) fn g8_pool_control(s: &Subject) -> bool {
    let Subject::Text(text) = s else {
        return true;
    };
    let Verdict::Ok(all) = parse_chromosomes(text) else {
        return true;
    };
    let mut pool = Pool::default();
    all.into_iter()
        .all(|c| matches!(pool.insert(c), Verdict::Ok(_)))
}
