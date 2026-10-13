//! A click on a strand or a fold: the CPU pick, waiting for the GPU to agree.

use joinn_frame::Verdict;
use joinn_visual::{CODON_TAG, Pick, TAG_MASK, cpu_pick_sample};

use super::{Confirm, Dna};

impl Dna {
    /// `pick x,y: <hash>.codon<k> (cpu)` on a codon, in either picture. A ring
    /// in the fold is selected for Enter (`selected node <d>`); any other
    /// owner clears the selection. An edge pixel is named and not confirmed.
    pub fn click(&mut self, x: u32, y: u32) -> Vec<String> {
        self.confirm = None;
        self.selected = None;
        if x >= self.camera.width || y >= self.camera.height {
            return vec![format!("pick {x},{y}: background (cpu)")];
        }
        let shapes = if self.folded {
            self.fold.shapes()
        } else {
            self.strand.shapes()
        };
        let picks = match cpu_pick_sample(&shapes, &self.camera, &[(x, y)]) {
            Verdict::Ok(p) => p,
            Verdict::Refused(r) => return vec![format!("pick {x},{y}: refused: {}", r.reason)],
        };
        let id = match picks.first() {
            Some(Pick::Edge) => return vec![format!("pick {x},{y}: edge (cpu)")],
            Some(Pick::Owned(id)) => *id,
            Some(Pick::Background) | None => [0; 4],
        };
        let owner = if self.folded {
            self.fold.print_id(id)
        } else {
            self.strand.print_id(id)
        };
        self.confirm = Some(Confirm {
            x,
            y,
            cpu: owner.clone(),
        });
        let mut lines = vec![format!("pick {x},{y}: {owner} (cpu)")];
        if self.folded && id[0] == 1 && id[1] == 2 && id[2] & TAG_MASK == CODON_TAG {
            let node = u64::from(id[2] & !TAG_MASK);
            self.selected = Some(node);
            lines.push(format!("selected node {node}"));
        }
        lines
    }
}
