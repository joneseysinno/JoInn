//! Load the body, contact, system, universe, strand or chromosome and run the
//! window until it closes.

use std::path::Path;

use super::run_view::run_view;
use crate::load::{
    find_corpus, load_body_file, load_cells, load_contact_file, load_dna, load_store,
    load_system_file, load_universe_file,
};
use crate::session::{Atlas, Desktop, Dna, Grower, View};

/// `adapter: <line>`, then `surface: <format>`, then one tick per change. A
/// `.universe` opens `lens`, or its first lens in canonical order; a `.system`
/// opens at size 0 with its waiting boxes; a `.strand` opens its strand view
/// and a `.chromosome` its fold.
pub fn run(path: &str, lens: Option<&str>) -> Result<(), String> {
    let corpus = find_corpus()?;
    let cells = load_cells(&corpus)?;
    let view: Box<dyn View> = if path.ends_with(".universe") {
        let universe = load_universe_file(Path::new(path))?;
        let store = load_store(&corpus, &cells)?;
        Box::new(Atlas::open(&universe, &store, lens, 1280, 720)?)
    } else if path.ends_with(".system") {
        let (system, contacts, waiting) = load_system_file(Path::new(path))?;
        Box::new(Grower::open(&system, &contacts, &cells, waiting)?)
    } else if path.ends_with(".contact") {
        let (contact, body) = load_contact_file(Path::new(path), &cells)?;
        Box::new(Desktop::open(body, cells, Some(&contact))?)
    } else if path.ends_with(".strand") || path.ends_with(".chromosome") {
        let (chromosome, pool) = load_dna(Path::new(path), &corpus)?;
        Box::new(Dna::open(chromosome, pool, path.ends_with(".chromosome"))?)
    } else {
        Box::new(Desktop::open(
            load_body_file(Path::new(path))?,
            cells,
            None,
        )?)
    };
    run_view(view)
}
