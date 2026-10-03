//! Admit a universe exactly as any universe: bind, Law 4, lenses, link types,
//! assembly.

use joinn_frame::Verdict;
use joinn_link::{
    BodyStore, Universe, assemble_universe, bind, check_law4, check_lenses, check_link_types,
};

/// The first refusal of `bind`, `check_law4`, `check_lenses`,
/// `check_link_types` and `assemble_universe`, in that order.
pub(crate) fn admit_universe(universe: &Universe, store: &BodyStore) -> Verdict<()> {
    let bound = match bind(universe, store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    for verdict in [check_law4(universe), check_lenses(universe)] {
        if let Verdict::Refused(r) = verdict {
            return Verdict::Refused(r);
        }
    }
    if let Verdict::Refused(r) = check_link_types(universe, &bound) {
        return Verdict::Refused(r);
    }
    assemble_universe(universe, &bound)
}
