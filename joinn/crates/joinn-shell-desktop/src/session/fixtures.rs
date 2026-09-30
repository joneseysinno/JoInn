//! The calculator, loaded the way the window loads it.

use super::Desktop;
use crate::load::{find_corpus, load_body_file, load_cells};

pub(super) fn calculator() -> Desktop {
    let corpus = match find_corpus() {
        Ok(path) => path,
        Err(e) => panic!("{e}"),
    };
    let body = match load_body_file(&corpus.join("phase2").join("calculator.body")) {
        Ok(body) => body,
        Err(e) => panic!("{e}"),
    };
    let cells = match load_cells(&corpus) {
        Ok(cells) => cells,
        Err(e) => panic!("{e}"),
    };
    match Desktop::open(body, cells, None) {
        Ok(desktop) => desktop,
        Err(e) => panic!("{e}"),
    }
}
