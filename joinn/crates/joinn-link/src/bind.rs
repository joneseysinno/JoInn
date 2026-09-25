//! Binding is a type: a Bound comes only from bind.

mod bind_fn;
mod get;
mod iter;

use joinn_dna::{Body, Cell};
use joinn_frame::Hash;
use std::collections::BTreeMap;

pub(crate) type BodyWithCells = (Body, BTreeMap<Hash, Cell>);

/// Alias → body, produced only by [`bind`].
pub struct Bound {
    pub(crate) by_alias: BTreeMap<String, BodyWithCells>,
}

pub use bind_fn::bind;
