//! The contact calculator, loaded the way the window loads it.

use joinn_dna::Contact;

use super::Desktop;
use crate::load::{find_corpus, load_cells, load_contact_file};

/// The desktop, and the contact it draws, for `Scene::regrow_contact`.
pub(super) fn calculator_contact() -> (Desktop, Contact) {
    let corpus = match find_corpus() {
        Ok(path) => path,
        Err(e) => panic!("{e}"),
    };
    let cells = match load_cells(&corpus) {
        Ok(cells) => cells,
        Err(e) => panic!("{e}"),
    };
    let (contact, body) =
        match load_contact_file(&corpus.join("phase7").join("calculator.contact"), &cells) {
            Ok(pair) => pair,
            Err(e) => panic!("{e}"),
        };
    match Desktop::open(body, cells, Some(&contact)) {
        Ok(desktop) => (desktop, contact),
        Err(e) => panic!("{e}"),
    }
}
