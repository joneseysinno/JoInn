//! Name an ID texel of a strand view.

use joinn_tower::short_hash;

use super::StrandScene;
use crate::pick::{CODON_TAG, TAG_MASK};

impl StrandScene {
    /// `<strand hash>.codon<k>` for codon k (V168's address, the hash cut to
    /// twelve hex digits), `strand <hash>` for the surface, `background` for
    /// zero, and the raw texel for anything this scene did not draw.
    pub fn print_id(&self, id: [u32; 4]) -> String {
        let head = short_hash(&self.hash);
        match (id[0], id[1], id[2] & TAG_MASK) {
            _ if id == [0; 4] => "background".to_owned(),
            (1, 0, 0) if id[2] == 0 => format!("strand {head}"),
            (1, 0, CODON_TAG) => format!("{head}.codon{}", id[2] & !TAG_MASK),
            _ => format!("[{} {} {:#x} {}]", id[0], id[1], id[2], id[3]),
        }
    }
}
