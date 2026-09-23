//! The only way to obtain a `Hash` of DNA.

use joinn_frame::{CanonWriter, Hash, keyed_hash};

use super::Genotype;

/// The only way to obtain a `Hash` of DNA.
pub fn hash<G: Genotype>(g: &G) -> Hash {
    let mut w = CanonWriter::new();
    g.encode(&mut w);
    keyed_hash(G::domain_tag(), &w.finish())
}
