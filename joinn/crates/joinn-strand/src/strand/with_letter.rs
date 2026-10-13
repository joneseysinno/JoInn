//! The same strand with one letter changed.

use super::Strand;
use crate::letter::Letter;

impl Strand {
    /// Letter `index` replaced by `letter`, or `None` past the tail.
    pub fn with_letter(&self, index: usize, letter: Letter) -> Option<Strand> {
        let mut letters = self.letters.clone();
        *letters.get_mut(index)? = letter;
        Some(Strand::new(letters))
    }
}
