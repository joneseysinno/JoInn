//! Replace the remaining step budget. The universe runtime shares one budget.

use crate::state::BodyState;

impl BodyState {
    /// Set the step ceiling. Steps already taken still count.
    pub fn set_budget(&mut self, budget: u64) {
        self.budget = budget;
    }
}