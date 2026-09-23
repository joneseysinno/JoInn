//! Gate 3 item 4: the calculator binary still matches the transcript.

use super::{run_calculator_bin, workspace_root};

pub(crate) fn g3_transcript() -> bool {
    let Ok(root) = workspace_root() else {
        return false;
    };
    let Ok(want) = std::fs::read_to_string(
        root.join("corpus")
            .join("transcripts")
            .join("calculator.txt"),
    ) else {
        return false;
    };
    let Ok(got) = run_calculator_bin() else {
        return false;
    };
    got.replace("\r\n", "\n") == want.replace("\r\n", "\n")
}
