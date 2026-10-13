//! A click on a strand: the CPU pick, waiting for the GPU to agree.

use joinn_frame::Verdict;
use joinn_visual::{Pick, cpu_pick_sample};

use super::{Confirm, Dna};

impl Dna {
    /// `pick x,y: <hash>.codon<k> (cpu)` on a codon; an edge pixel is named
    /// and not confirmed.
    pub fn click(&mut self, x: u32, y: u32) -> Vec<String> {
        self.confirm = None;
        if x >= self.camera.width || y >= self.camera.height {
            return vec![format!("pick {x},{y}: background (cpu)")];
        }
        let picks = match cpu_pick_sample(&self.strand.shapes(), &self.camera, &[(x, y)]) {
            Verdict::Ok(p) => p,
            Verdict::Refused(r) => return vec![format!("pick {x},{y}: refused: {}", r.reason)],
        };
        let id = match picks.first() {
            Some(Pick::Edge) => return vec![format!("pick {x},{y}: edge (cpu)")],
            Some(Pick::Owned(id)) => *id,
            Some(Pick::Background) | None => [0; 4],
        };
        let owner = self.strand.print_id(id);
        self.confirm = Some(Confirm {
            x,
            y,
            cpu: owner.clone(),
        });
        vec![format!("pick {x},{y}: {owner} (cpu)")]
    }
}
