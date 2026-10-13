//! Name an ID texel of a fold view.

use joinn_tower::short_hash;

use super::FoldScene;
use crate::pick::{CODON_TAG, TAG_MASK};

impl FoldScene {
    /// `<strand hash>.codon<k>` for the codon on segment k (the strand view's
    /// address, V168), `<hash>.segment<s> (silent)`, `<hash>.node<d> gene
    /// <hash>` for a ring, `gene <hash> at <hash>.node<d>` for an opened gene,
    /// `fold <hash> floor <f>` for the surface, `background` for zero, and the
    /// raw texel for anything this scene did not draw.
    pub fn print_id(&self, id: [u32; 4]) -> String {
        let head = short_hash(&self.chromosome.identity());
        let raw = || format!("[{} {} {:#x} {}]", id[0], id[1], id[2], id[3]);
        let k = id[2] & !TAG_MASK;
        let gene = || self.chromosome.genes().get(&u64::from(k)).map(short_hash);
        match (id[0], id[1], id[2] & TAG_MASK) {
            _ if id == [0; 4] => "background".to_owned(),
            (1, 0, 0) if id[2] == 0 => format!("fold {head} floor {}", self.floor),
            (1, 0, CODON_TAG) => format!("{head}.codon{k}"),
            (1, 1, CODON_TAG) => format!("{head}.segment{k} (silent)"),
            (1, 2, CODON_TAG) => gene().map_or_else(raw, |g| format!("{head}.node{k} gene {g}")),
            (1, 3, CODON_TAG) => gene().map_or_else(raw, |g| format!("gene {g} at {head}.node{k}")),
            _ => raw(),
        }
    }
}
