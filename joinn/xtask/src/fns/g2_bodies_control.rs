//! Phase 2 bodies control: calculator and calculator_b must not diverge.

use super::{artifact_loads, workspace_root};
use joinn_dna::{parse_body, print_body};
use joinn_frame::{FrameRegistry, Verdict};

pub(crate) fn g2_bodies_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    let Ok(root) = workspace_root() else {
        return true;
    };
    let frames = FrameRegistry::phase1();
    let dir = root.join("corpus").join("phase2");
    let Ok(a) = std::fs::read_to_string(dir.join("calculator.body")) else {
        return true;
    };
    let Ok(b) = std::fs::read_to_string(dir.join("variants").join("calculator_b.body")) else {
        return true;
    };
    let Verdict::Ok(ba) = parse_body(&a, &frames) else {
        return true;
    };
    let Verdict::Ok(bb) = parse_body(&b, &frames) else {
        return true;
    };
    print_body(&ba.coding) != print_body(&bb.coding)
}
