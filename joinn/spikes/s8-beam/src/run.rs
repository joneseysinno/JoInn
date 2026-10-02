//! The spike's run: every example, witness and plant, as printed text.

mod example_block;
mod plant_block;
mod whole;

pub use example_block::example_block;
pub use plant_block::plant_block;
pub use whole::run;

/// What a run printed, and whether every check outside the plants held.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Run {
    text: String,
    clean: bool,
}

/// One example's printed lines and what its checks found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    text: String,
    agree: usize,
    disagree: usize,
    refined: bool,
    clean: bool,
}

/// The plants' printed lines: how many were refused (ok), and how many not.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plants {
    text: String,
    refused: usize,
    admitted: usize,
}

impl Run {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn clean(&self) -> bool {
        self.clean
    }
}
