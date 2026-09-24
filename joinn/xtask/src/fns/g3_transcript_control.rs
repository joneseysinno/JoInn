//! Gate 3 item 4 control: the no-indent transcript is not the golden.

use super::{workspace_root};

pub(crate) fn g3_transcript_control(_art: &()) -> bool {
    let Ok(root) = workspace_root() else {
        return true;
    };
    let Ok(want) = std::fs::read_to_string(
        root.join("corpus")
            .join("transcripts")
            .join("calculator.txt"),
    ) else {
        return true;
    };
    let Ok(planted) = std::fs::read_to_string(
        root.join("corpus")
            .join("phase3")
            .join("controls")
            .join("no_indent_transcript.txt"),
    ) else {
        return true;
    };
    planted.replace("\r\n", "\n") == want.replace("\r\n", "\n")
}
