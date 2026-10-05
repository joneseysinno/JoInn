//! Gate 7.2: a universe admitted as its picture needs, every lens laid out.

use joinn_frame::Verdict;
use joinn_link::{Universe, assemble_universe, bind, check_law4, check_lenses};
use joinn_visual::{UniverseLayout, layout_universe};

use super::corpus_store::corpus_store;

/// Each lens's layout, in declared order, or the first refusal of `bind`,
/// `check_law4`, `check_lenses`, `assemble_universe` or the layout. The
/// link-type check is not run: no picture reads a member's frame, and
/// `adversary.universe` is refused there and nowhere else.
pub(crate) fn g72_layouts(universe: &Universe) -> Result<Vec<UniverseLayout>, String> {
    let (_, store) = corpus_store()?;
    let bound = match bind(universe, store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Err(r.reason),
    };
    for verdict in [
        check_law4(universe),
        check_lenses(universe),
        assemble_universe(universe, &bound),
    ] {
        if let Verdict::Refused(r) = verdict {
            return Err(r.reason);
        }
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
    use crate::fns::corpus_store::corpus_store;
    use crate::fns::grove::{admit_universe, load_universe_arg};
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::subject::Subject;

    #[test]
    fn the_adversary_is_refused_only_by_link_types_and_its_mutants_structurally() {
        let (_, store) = corpus_store().unwrap_or_else(|e| panic!("{e}"));
        let u = load_universe_arg("phase5/adversary.universe").unwrap_or_else(|e| panic!("{e}"));
        let Verdict::Refused(r) = admit_universe(&u, store) else {
            panic!("adversary.universe must be refused by full admission");
        };
        assert_eq!(
            r.reason,
            "link bus member units.scale@1 is head but direction is Out; acceptance is In"
        );
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
            refusal(Mutation::ShiftPort("bus", "calc.sum@2", 9)).as_deref(),
            Some("no such port calc.sum@9; acceptance is a declared port on ∂(calc)")
        );
    }
}
