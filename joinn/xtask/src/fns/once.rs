//! Pure work once per xtask process (plan 7.3 §2.11 c). A memoized function
//! keeps the lines it printed, and every caller prints them again, so a gate
//! prints exactly what it printed when the work ran each time. Nothing is kept
//! across processes or on disk.

mod memo;
mod say;

use std::cell::RefCell;

pub(crate) use memo::memo;
pub(crate) use say::say;

thread_local! {
    /// Open captures, innermost last. `say` prints when none is open.
    static SINK: RefCell<Vec<Vec<String>>> = const { RefCell::new(Vec::new()) };
}
