//! Gate 3 item 9: the live lock's scores are passes.

use super::{parse_lock_scores, workspace_root};

pub(crate) fn g3_lock() -> bool {
    let Ok(root) = workspace_root() else {
        return false;
    };
    let Ok(live) = std::fs::read_to_string(root.join("gates.lock")) else {
        return false;
    };
    match parse_lock_scores(&live) {
        Ok(rows) => {
            rows.iter().all(|row| row.n == row.total)
                && live.contains("phase 0")
                && live.contains("phase 2.2")
        }
        Err(msg) => {
            println!("{msg}");
            false
        }
    }
}
