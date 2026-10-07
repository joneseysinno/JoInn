//! A click on a system: the CPU pick, then selection when it is a waiting box.

use joinn_visual::{PORT_TAG, Pick, TAG_MASK, cpu_pick_sample};

use joinn_frame::Verdict;

use super::{Confirm, Grower};

impl Grower {
    /// A waiting box, or its in-port, is selected: what is typed there grows
    /// the body. Any other owner clears the selection; an edge pixel is named
    /// and not confirmed.
    pub fn click(&mut self, x: u32, y: u32) -> Vec<String> {
        self.confirm = None;
        if x >= self.camera.width || y >= self.camera.height {
            return vec![format!("pick {x},{y}: background (cpu)")];
        }
        let picks = match cpu_pick_sample(&self.scene.shapes(), &self.camera, &[(x, y)]) {
            Verdict::Ok(p) => p,
            Verdict::Refused(r) => return vec![format!("pick {x},{y}: refused: {}", r.reason)],
        };
        let id = match picks.first() {
            Some(Pick::Edge) => return vec![format!("pick {x},{y}: edge (cpu)")],
            Some(Pick::Owned(id)) => *id,
            Some(Pick::Background) | None => [0; 4],
        };
        let owner = self.scene.print_id(id);
        let mut lines = vec![format!("pick {x},{y}: {owner} (cpu)")];
        self.confirm = Some(Confirm { x, y, cpu: owner });
        self.buffer.clear();
        let filled = self.scene.grown().inputs().len();
        let tag = id[2] & TAG_MASK;
        let waiting = (id[1] as usize)
            .checked_sub(2)
            .filter(|k| id[0] == 1 && *k >= filled && (tag == 0 || tag == PORT_TAG));
        self.selected = waiting.map(|k| self.scene.grown().instance(k));
        if let Some(instance) = &self.selected {
            lines.push(format!("selected {instance}"));
        }
        lines
    }
}
