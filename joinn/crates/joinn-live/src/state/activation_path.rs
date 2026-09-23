//! Activation path for budget refusals.

use crate::state::BodyState;

impl BodyState {
    pub(in crate::state) fn activation_path(&self) -> String {
        self.stack
            .iter()
            .map(|a| a.label.as_str())
            .collect::<Vec<_>>()
            .join(" ▸ ")
    }
}
