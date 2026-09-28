//! `cargo xtask layout`: a body's layout, then its four standard cameras.

mod corpus_bodies;
mod layout_all;
mod layout_block;
mod layout_text;
mod lone_body;

pub(crate) use corpus_bodies::corpus_bodies;
pub(crate) use layout_all::layout_all;
pub(crate) use layout_text::layout_text;
