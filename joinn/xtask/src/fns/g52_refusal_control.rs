//! Gate 5.2 item 2 control: true when the CLI prints a line the transcript lacks.

use super::run_body_bin::run_body_bin;
use super::subject::Subject;

pub(crate) fn g52_refusal_control(subject: &Subject) -> bool {
    let Subject::Transcript(lines) = subject else {
        return false;
    };
    let Ok(cli) = run_body_bin("universe", b"two\n2\n3\n12\n") else {
        return false;
    };
    cli.lines()
        .any(|line| !lines.iter().any(|have| have == line))
}
