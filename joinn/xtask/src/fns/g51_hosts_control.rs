//! Gate 5.1 control: true when the transcript does not start with calculator.txt.

use super::subject::Subject;
use super::workspace_root;
use std::fs;

pub(crate) fn g51_hosts_control(subject: &Subject) -> bool {
    let Subject::Transcript(lines) = subject else {
        return true;
    };
    let Ok(root) = workspace_root() else {
        return true;
    };
    let Ok(calc) = fs::read_to_string(
        root.join("corpus")
            .join("transcripts")
            .join("calculator.txt"),
    ) else {
        return true;
    };
    let calc_lines: Vec<&str> = calc.lines().collect();
    if lines.len() < calc_lines.len() {
        return true;
    }
    !lines
        .iter()
        .zip(calc_lines.iter())
        .all(|(a, b)| a.as_str() == *b)
}
