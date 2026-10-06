//! A universe for `links`: loaded, admitted, its `function` lens laid out.

use joinn_frame::Verdict;
use joinn_link::Universe;
use joinn_visual::{UniverseLayout, layout_universe};

use crate::fns::corpus_store::corpus_store;
use crate::fns::grove::{admit_universe, grove_layout, load_universe_arg};

/// `grove` (seed 7, from `grove_layout`) or a universe path, admitted through
/// `admit_universe` and laid out in its `function` lens; with the name the
/// lines print (`grove`, or the file name).
pub(crate) fn laid_universe(arg: &str) -> Result<(String, Universe, UniverseLayout), String> {
    if arg == "grove" {
        let (u, l) = grove_layout()?;
        return Ok(("grove".to_owned(), u, l));
    }
    let universe = load_universe_arg(arg)?;
    let (_, store) = corpus_store()?;
    let name = arg.rsplit(['/', '\\']).next().unwrap_or(arg).to_owned();
    if let Verdict::Refused(r) = admit_universe(&universe, store) {
        return Err(format!("{name}: {}", r.reason));
    }
    match layout_universe(&universe, store, "function") {
        Verdict::Ok(l) => Ok((name, universe, l)),
        Verdict::Refused(r) => Err(format!("{name}: {}", r.reason)),
    }
}
