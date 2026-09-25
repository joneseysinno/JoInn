//! Gate 5.2 item 2 control: true when the refusal line is gone.

use super::subject::Subject;

pub(crate) fn g52_refusal_control(subject: &Subject) -> bool {
    let Subject::Transcript(lines) = subject else {
        return false;
    };
    !lines
        .iter()
        .any(|line| line.contains("refused") && line.contains("two"))
}
