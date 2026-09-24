//! Gate 5.1 item 2 control: the transcript's prefix is the calculator golden.

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
    let calc_lines: Vec<String> = calc.lines().map(str::to_owned).collect();
    if lines.len() < calc_lines.len() {
        return true;
    }
    lines[..calc_lines.len()] != calc_lines[..]
}
