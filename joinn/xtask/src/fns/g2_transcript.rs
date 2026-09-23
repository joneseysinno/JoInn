//! Phase 2 item: the calculator transcript, printed.

use super::{run_calculator_bin, workspace_root};

pub(crate) fn g2_transcript() -> bool {
    let (Ok(got), Ok(root)) = (run_calculator_bin(), workspace_root()) else {
        return false;
    };
    let path = root
        .join("corpus")
        .join("transcripts")
        .join("calculator.txt");
    let Ok(want) = std::fs::read_to_string(&path) else {
        return false;
    };
    let want = want.replace("\r\n", "\n");
    let got = got.replace("\r\n", "\n");
    if got != want {
        return false;
    }
    print!("{got}");
    true
}
