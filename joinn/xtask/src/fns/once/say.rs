//! Print one line, or keep it for the innermost open capture.

use super::SINK;

/// `println!` when no memoized work is running; otherwise the line is kept and
/// printed by `memo` on every call.
pub(crate) fn say(line: &str) {
    let kept = SINK.with(|sink| match sink.borrow_mut().last_mut() {
        Some(open) => {
            open.push(line.to_owned());
            true
        }
        None => false,
    });
    if !kept {
        println!("{line}");
    }
}
