//! Print an owner under this scene's alias.

use super::Scene;
use crate::pick::{Owner, print_owner};

impl Scene {
    /// `body.cli_a`, `body.sum@2`, `body wire cli_a@1 -> sum@0`, `body surface`.
    pub fn print_owner(&self, o: &Owner) -> String {
        print_owner(&self.alias, o)
    }
}
