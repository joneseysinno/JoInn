//! The grove grown from seed 7 and laid out in its one lens.

use std::sync::OnceLock;

use joinn_frame::Verdict;
use joinn_link::Universe;
use joinn_visual::{UniverseLayout, layout_universe};

use super::{GROVE_SEED, admit_universe, grow_grove};
use crate::fns::corpus_store::corpus_store;
use crate::fns::once::memo;

type Laid = Result<(Universe, UniverseLayout), String>;

static LAID: OnceLock<(Laid, Vec<String>)> = OnceLock::new();

/// Seed 7's grove, admitted by `admit_universe`, and its `function` lens laid
/// out, bodies bound from the corpus by hash. Grown and laid out once per
/// process.
pub(crate) fn grove_layout() -> Laid {
    memo(&LAID, || {
        let universe = grow_grove(GROVE_SEED)?;
        let (_, store) = corpus_store()?;
        if let Verdict::Refused(r) = admit_universe(&universe, store) {
            return Err(r.reason);
        }
        match layout_universe(&universe, store, "function") {
            Verdict::Ok(l) => Ok((universe, l)),
            Verdict::Refused(r) => Err(r.reason),
        }
    })
}
