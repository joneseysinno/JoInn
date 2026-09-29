//! Load a body file and the corpus cells, the way the CLI loads them.

mod find_corpus;
mod load_body_file;
mod load_cells;

pub(crate) use find_corpus::find_corpus;
pub(crate) use load_body_file::load_body_file;
pub(crate) use load_cells::load_cells;
