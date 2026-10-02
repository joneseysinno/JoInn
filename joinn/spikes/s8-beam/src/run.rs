//! The spike's run: every example, witness and plant, as printed text.

mod whole;

pub use whole::run;

/// What a run printed, and whether every check outside the plants held.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Run {
    text: String,
    clean: bool,
}

impl Run {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn clean(&self) -> bool {
        self.clean
    }
}
