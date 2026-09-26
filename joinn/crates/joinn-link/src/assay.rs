//! Fast derivation: regions, links, and the three filling rules.

mod assay_run;
mod print_assay;
mod roundtrip;

pub use assay_run::assay;
pub use print_assay::print_assay;

/// One connected piece of a body's inside, instances in name order.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RegionPieces {
    /// Body alias.
    pub alias: String,
    /// One entry per piece. A single piece is printed without braces.
    pub pieces: Vec<Vec<String>>,
}

/// What the assay measured. It does not refuse a loop; it records it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AssayReport {
    /// `assay body` or `assay universe`.
    pub headline: String,
    /// One entry per body alias, alias order.
    pub regions: Vec<RegionPieces>,
    /// Pieces of the complex with the outside removed.
    pub islands: u32,
    /// Dimension of the cycle space.
    pub loops: u32,
    /// Filled loops, canonical order, without the `filled:` prefix.
    pub filled: Vec<String>,
    /// Open loops, without the `open:` prefix.
    pub open: Vec<String>,
    /// Loops the rules do not judge, and links with more than two members.
    pub not_measured: u32,
    /// Betti numbers, outside included in b₀.
    pub b0: u32,
    /// Betti numbers.
    pub b1: u32,
    /// Betti numbers.
    pub b2: u32,
    /// 0-blocks.
    pub v: u32,
    /// 1-blocks.
    pub e: u32,
    /// 2-blocks.
    pub f: u32,
}
