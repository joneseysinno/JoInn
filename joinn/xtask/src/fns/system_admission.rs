//! Admit a system subject against the corpus: its own bound contacts, every
//! corpus contact and system (a parent is named by lineage), and the corpus
//! cells.

use joinn_dna::{hash, parse_system};
use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::check_system;
use std::collections::BTreeMap;
use std::fs;

use super::contact::corpus_files;
use super::forces::corpus_cells;
use super::layout::corpus_contacts;
use super::subject::SystemSubject;
use super::workspace_root;

pub(crate) fn system_admission(s: &SystemSubject) -> Result<Verdict<()>, String> {
    let frames = FrameRegistry::phase1();
    let (parsed, _) = corpus_contacts()?;
    let mut contacts: BTreeMap<_, _> = parsed
        .into_iter()
        .filter_map(|(_, v)| match v {
            Verdict::Ok(c) => Some((hash(&c.coding), c)),
            Verdict::Refused(_) => None,
        })
        .collect();
    contacts.extend(s.contacts.values().map(|c| (hash(&c.coding), c.clone())));
    let corpus = workspace_root()?.join("corpus");
    let mut systems = BTreeMap::new();
    for path in corpus_files(&corpus, "system")? {
        let src = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        if let Verdict::Ok(system) = parse_system(&src) {
            systems.insert(hash(&system.coding), system);
        }
    }
    let cells = corpus_cells(&frames)?;
    Ok(check_system(
        &s.system, &contacts, &systems, &cells, &frames,
    ))
}

#[cfg(test)]
mod tests {
    use super::system_admission;
    use crate::fns::mutate::system_fixture::system_fixture;
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::refused_at_admission::refused_at_admission;
    use crate::fns::subject::Subject;
    use joinn_dna::Accept;
    use joinn_frame::Verdict;

    #[test]
    fn counting_is_admitted_and_its_mutants_answer_as_check_system_does() {
        let s = system_fixture();
        let Subject::System(sys) = &s else {
            panic!("system");
        };
        let admitted = system_admission(sys).unwrap_or_else(|e| panic!("{e}"));
        assert!(matches!(admitted, Verdict::Ok(())), "{admitted:?}");
        assert!(!refused_at_admission(&s));
        let cases = [
            (
                Mutation::DropForce("count"),
                Some(
                    "system: no force; acceptance is a force (a system is the bodies its forces reach)",
                ),
            ),
            (
                Mutation::ForceOn("count", "ghost"),
                Some(
                    "system: force count is on ghost, which is no body; acceptance is a bound alias",
                ),
            ),
            (Mutation::Accepts("numbers", Accept::Any), None),
        ];
        for (m, want) in cases {
            let Verdict::Ok(mutant) = mutate(&s, &m) else {
                panic!("{m:?}");
            };
            let Subject::System(ms) = &mutant else {
                panic!("system");
            };
            let got = system_admission(ms).unwrap_or_else(|e| panic!("{e}"));
            match (got, want) {
                (Verdict::Refused(r), Some(w)) => assert_eq!(r.reason, w, "{m:?}"),
                (Verdict::Ok(()), None) => {}
                (got, want) => panic!("{m:?}: {got:?}, want {want:?}"),
            }
            assert_eq!(refused_at_admission(&mutant), want.is_some(), "{m:?}");
        }
    }
}
