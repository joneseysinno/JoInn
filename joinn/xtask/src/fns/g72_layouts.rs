//! Gate 7.2: a universe admitted like any universe, every lens laid out.

use joinn_frame::Verdict;
use joinn_link::Universe;
use joinn_visual::{UniverseLayout, layout_universe};

use super::corpus_store::corpus_store;
use super::grove::admit_universe;

/// Each lens's layout, in declared order, of a universe `admit_universe`
/// admits (the one admission path), or the first refusal of admission or of
/// a layout.
pub(crate) fn g72_layouts(universe: &Universe) -> Result<Vec<UniverseLayout>, String> {
    let (_, store) = corpus_store()?;
    if let Verdict::Refused(r) = admit_universe(universe, store) {
        return Err(r.reason);
    }
    let mut out = Vec::new();
    for lens in &universe.coding.lenses {
        match layout_universe(universe, store, &lens.name) {
            Verdict::Ok(l) => out.push(l),
            Verdict::Refused(r) => return Err(r.reason),
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use super::g72_layouts;
    use crate::fns::grove::load_universe_arg;
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::subject::Subject;

    #[test]
    fn ordered_is_admitted_and_its_mutants_are_refused_at_admission() {
        let u = load_universe_arg("phase5/ordered.universe").unwrap_or_else(|e| panic!("{e}"));
        let names: Vec<String> = g72_layouts(&u)
            .unwrap_or_else(|e| panic!("{e}"))
            .into_iter()
            .map(|l| l.lens)
            .collect();
        assert_eq!(names, ["function"]);
        let real = Subject::Universe(u);
        let refusal = |m: Mutation| match mutate(&real, &m) {
            Verdict::Ok(Subject::Universe(x)) => g72_layouts(&x).err(),
            _ => panic!("{m:?} must give a universe"),
        };
        assert_eq!(
            refusal(Mutation::CopyMember("function", "units", "calculation")).as_deref(),
            Some(
                "body units belongs to systems calculation and measurement in lens function; acceptance is one system per body per lens"
            )
        );
        assert_eq!(
            refusal(Mutation::ShiftPort("path", "calc.sum@2", 9)).as_deref(),
            Some("link path member calc.sum@9 no such port; acceptance is a declared port")
        );
    }
}
