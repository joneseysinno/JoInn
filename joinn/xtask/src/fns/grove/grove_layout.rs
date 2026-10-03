//! The grove grown from seed 7 and laid out in its one lens.

use joinn_frame::Verdict;
use joinn_link::Universe;
use joinn_visual::{UniverseLayout, layout_universe};

use super::{GROVE_SEED, grow_grove};
use crate::fns::corpus_store::corpus_store;

/// Seed 7's grove and its `function` lens laid out, bodies bound from the
/// corpus by hash.
pub(crate) fn grove_layout() -> Result<(Universe, UniverseLayout), String> {
    let universe = grow_grove(GROVE_SEED)?;
    let (_, store) = corpus_store()?;
    match layout_universe(&universe, store, "function") {
        Verdict::Ok(l) => Ok((universe, l)),
        Verdict::Refused(r) => Err(r.reason),
    }
}
