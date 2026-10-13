//! A strand from its letters.

use super::Strand;
use crate::letter::Letter;

impl Strand {
    /// The letters in head-to-tail order.
    pub fn new(letters: Vec<Letter>) -> Strand {
        Strand { letters }
    }
}
