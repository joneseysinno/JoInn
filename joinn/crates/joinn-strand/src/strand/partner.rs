//! The partner strand.

use super::Strand;

impl Strand {
    /// Every letter's partner, in reverse order (Part VIII §3.3).
    pub fn partner(&self) -> Strand {
        Strand::new(self.letters.iter().rev().map(|l| l.partner()).collect())
    }
}
