//! An organelle's printed name.

use super::Organelle;

impl Organelle {
    /// Lower case, as in the list.
    pub fn name(self) -> &'static str {
        match self {
            Organelle::Screen => "screen",
            Organelle::Input => "input",
            Organelle::Store => "store",
            Organelle::Clock => "clock",
        }
    }
}
