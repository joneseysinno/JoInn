//! `cargo xtask layout`: a body's layout, then its four standard cameras.

mod corpus_bodies;
mod corpus_contacts;
mod corpus_growing_contacts;
mod layout_all;
mod layout_block;
mod layout_text;
mod lone_body;
mod universe_layout_text;

pub(crate) use corpus_bodies::corpus_bodies;
pub(crate) use corpus_contacts::corpus_contacts;
pub(crate) use corpus_growing_contacts::corpus_growing_contacts;
pub(crate) use layout_all::layout_all;
pub(crate) use layout_text::layout_text;
pub(crate) use universe_layout_text::universe_layout_text;
