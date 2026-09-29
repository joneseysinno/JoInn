//! Print the owner an ID texel names.

use joinn_frame::Verdict;

use super::Desktop;

impl Desktop {
    /// The printed owner, or the refusal's reason when the ID is stale.
    pub fn owner_line(&self, id: [u32; 4]) -> String {
        match self.scene.resolve(id) {
            Verdict::Ok(owner) => self.scene.print_owner(&owner),
            Verdict::Refused(r) => r.reason,
        }
    }
}
