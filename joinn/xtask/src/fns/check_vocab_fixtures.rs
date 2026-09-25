//! Vocab fixtures: JoInn idents are banned; std calls are not.

use super::{line_has, mask_std_calls, workspace_root};
use std::fs;

/// Fixture pair must report a JoInn turn-ident and accept a std method call.
pub(crate) fn check_vocab_fixtures() -> Result<(), String> {
    let root = workspace_root()?.join("xtask").join("vocab_fixtures");
    let refuse = fs::read_to_string(root.join("refuse_sub_total.rs")).map_err(|e| e.to_string())?; // allow(vocab): fixture path names the planted turn-ident file
    let accept = fs::read_to_string(root.join("accept_std_call.rs")).map_err(|e| e.to_string())?;

    let banned = concat!("su", "b"); // allow(vocab): turn-ident under test
    let mut refuse_hit = false;
    for line in refuse.lines() {
        let masked = mask_std_calls(line);
        if line_has(&masked, banned) {
            refuse_hit = true;
        }
    }
    if !refuse_hit {
        return Err(
            "vocab: blind refuse — refuse fixture must report the planted turn-ident".into(),
        );
    }

    let words = [banned, "subtract", "minus"]; // allow(vocab): turn-ident list under test
    for (i, line) in accept.lines().enumerate() {
        let masked = mask_std_calls(line);
        for word in words {
            if line_has(&masked, word) {
                return Err(format!(
                    "vocab: blind accept — std-call fixture:{} still hits a turn-ident after mask",
                    i + 1
                ));
            }
        }
    }
    Ok(())
}
