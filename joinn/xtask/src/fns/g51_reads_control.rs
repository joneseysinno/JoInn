//! Gate 5.1 item 8 control: the fixture source is the ignoring control.

use super::subject::Subject;

#[allow(dead_code)]
pub(crate) fn g51_reads_control(subject: &Subject) -> bool {
    let Subject::Text(text) = subject else {
        return true;
    };
    !(text.contains("ignores_its_bytes") && text.contains("_art"))
}
