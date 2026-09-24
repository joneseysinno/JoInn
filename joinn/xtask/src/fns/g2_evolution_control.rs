//! Phase 2 evolution control: a wrong lineage must not be admitted.

use super::{workspace_root};
use joinn_frame::{FrameRegistry, Hash, Verdict};

pub(crate) fn g2_evolution_control(_art: &()) -> bool {
    let Ok(root) = workspace_root() else {
        return true;
    };
    let frames = FrameRegistry::phase1();
    let Ok(parent_src) = std::fs::read_to_string(root.join("corpus").join("phase0").join("sum.cell"))
    else {
        return true;
    };
    let Verdict::Ok(parent) = joinn_dna::parse_cell(&parent_src, &frames) else {
        return true;
    };
    let Ok(src) = std::fs::read_to_string(root.join("corpus").join("phase2").join("sum_turn.cell"))
    else {
        return true;
    };
    let Verdict::Ok(mut proposed) = joinn_dna::parse_cell(&src, &frames) else {
        return true;
    };
    proposed.coding.lineage = Some(Hash::from_bytes([0xab; 32]));
    super::admit_lineage(&proposed, &parent).is_ok()
}
