//! Closed mutation catalogue: damage that keeps the form.

mod apply;
mod coding_hash;
mod copy_member;
mod corrupt_hash;
mod drop_genome;
mod drop_grant;
mod drop_lens;
mod drop_line;
mod drop_link;
mod drop_wire;
mod flip_mark;
mod mutate_fn;
mod neutral;
mod parse_endpoint;
mod parse_member_ref;
mod refuse;
mod rename_alias;
mod rename_link;
mod reparse;
mod replace;
mod reprint;
mod set_score;
mod shift_port;
mod swap_binding;
mod swap_cell;
mod swap_lines;
mod wire_across;

pub(crate) use mutate_fn::mutate;
pub(crate) use neutral::neutral;
pub(crate) use reparse::reparse;
pub(crate) use reprint::reprint;

/// One catalogue mutation. Closed; every non-legacy control opposes one.
#[allow(dead_code)] // closed catalogue; not every variant is declared by a gate row yet
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Mutation {
    DropLink(&'static str),
    FlipMark(&'static str, &'static str),
    ShiftPort(&'static str, &'static str, u32),
    SwapBinding(&'static str, &'static str),
    CorruptHash(&'static str),
    CopyMember(&'static str, &'static str, &'static str),
    DropLens(&'static str),
    WireAcross(&'static str),
    DropGrant(&'static str),
    RenameLink(&'static str, &'static str),
    RenameAlias(&'static str, &'static str),
    DropWire(&'static str, &'static str),
    DropGenome(&'static str),
    SwapCell(&'static str, &'static str),
    SetScore(&'static str, u32, u32),
    DropLine(usize),
    SwapLines(usize, usize),
    Replace(&'static str),
}
