//! Identity of one block in a complex.

/// A block of some dimension. The number identifies it inside one complex.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BlockId(pub u32);
