//! A corpus system, loaded the way the window loads it.

use super::Grower;
use crate::load::{find_corpus, load_cells, load_system_file};

pub(super) fn grower(name: &str) -> Grower {
    let corpus = match find_corpus() {
        Ok(path) => path,
        Err(e) => panic!("{e}"),
    };
    let cells = match load_cells(&corpus) {
        Ok(cells) => cells,
        Err(e) => panic!("{e}"),
    };
    let path = corpus.join("phase74").join(format!("{name}.system"));
    let (system, contacts, waiting) = match load_system_file(&path) {
        Ok(loaded) => loaded,
        Err(e) => panic!("{e}"),
    };
    match Grower::open(&system, &contacts, &cells, waiting) {
        Ok(g) => g,
        Err(e) => panic!("{e}"),
    }
}
