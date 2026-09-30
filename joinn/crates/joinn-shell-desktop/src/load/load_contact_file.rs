//! Parse, admit and lower the `.contact` file the shell was given.

use joinn_dna::{Body, Cell, Contact, parse_contact};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_link::lower;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Parse `path` as a contact, admit it against `cells`, and derive the body
/// the engine runs. The contact is what is drawn; the lowered body is only run
/// (rule 61). The argument is a path, not a stem.
pub(crate) fn load_contact_file(
    path: &Path,
    cells: &BTreeMap<Hash, Cell>,
) -> Result<(Contact, Body), String> {
    let src = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let frames = FrameRegistry::phase1();
    let contact = match parse_contact(&src, &frames) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => return Err(r.reason),
    };
    match lower(&contact, cells, &frames) {
        Verdict::Ok(body) => Ok((contact, body)),
        Verdict::Refused(r) => Err(r.reason),
    }
}

#[cfg(test)]
mod tests {
    use super::load_contact_file;
    use crate::load::{find_corpus, load_body_file, load_cells};
    use joinn_dna::{hash, print_body};

    #[test]
    fn the_contact_calculator_loads_as_the_wired_calculator() {
        let corpus = find_corpus().unwrap_or_else(|e| panic!("{e}"));
        let cells = load_cells(&corpus).unwrap_or_else(|e| panic!("{e}"));
        let (_, lowered) =
            load_contact_file(&corpus.join("phase7").join("calculator.contact"), &cells)
                .unwrap_or_else(|e| panic!("{e}"));
        let wired = load_body_file(&corpus.join("phase2").join("calculator.body"))
            .unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(print_body(&lowered.coding), print_body(&wired.coding));
        assert_eq!(hash(&lowered.coding), hash(&wired.coding));
        assert_eq!(lowered.regulatory, wired.regulatory);
    }

    #[test]
    fn a_contact_whose_cells_are_missing_is_refused() {
        let corpus = find_corpus().unwrap_or_else(|e| panic!("{e}"));
        let Err(e) = load_contact_file(
            &corpus.join("phase7").join("calculator.contact"),
            &Default::default(),
        ) else {
            panic!("no cells: must be refused");
        };
        assert_eq!(
            e,
            "unknown cell hash c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e"
        );
    }
}
