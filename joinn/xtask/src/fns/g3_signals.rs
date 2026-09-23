//! Gate 3 item 6: columns_reader is admitted only when columns is declared.

use super::workspace_root;
use joinn_frame::Verdict;
use joinn_host::{Signals, check_signals};
use std::collections::BTreeSet;

pub(crate) fn g3_signals() -> bool {
    let Ok(root) = workspace_root() else {
        return false;
    };
    let frames = joinn_frame::FrameRegistry::phase1();
    let Ok(src) = std::fs::read_to_string(
        root.join("corpus")
            .join("phase3")
            .join("columns_reader.body"),
    ) else {
        return false;
    };
    let Verdict::Ok(body) = joinn_dna::parse_body(&src, &frames) else {
        return false;
    };
    let none = Signals::new(BTreeSet::new());
    let cols = Signals::new(BTreeSet::from(["columns".into()]));
    matches!(check_signals(&body, &cols), Verdict::Ok(()))
        && matches!(
            check_signals(&body, &none),
            Verdict::Refused(r) if r.reason.contains("columns")
        )
}
