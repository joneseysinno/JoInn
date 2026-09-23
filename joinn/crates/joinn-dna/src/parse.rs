//! Source and canonical parser. Primitive names are a parse refusal.

#![allow(clippy::result_large_err)]

mod allele_body;
mod alleles_body;
mod and;
mod arg_map;
mod cell_at;
mod coding_body;
mod contract_body;
mod declarations_body;
mod eq;
mod expect;
mod forall;
mod formula;
mod formula_map;
mod founding_body;
mod frame_ref;
mod frame_term;
mod hash_hex;
mod ident;
mod implies;
mod join_policy;
mod laws_body;
mod literal_value;
mod name_map;
mod native_id;
mod not;
mod number;
mod or;
mod parse_cell;
mod parse_coding;
mod parse_tail;
mod peek_ident;
mod port_decl;
mod quoted_string;
mod regulatory_body;
mod retired_list;
mod self_at;
mod split_delimiter;
mod string_map;
mod strip_comments;
mod take_ident;
mod term;
mod turn_block;
mod turn_decl;
mod witness_body;

pub use parse_cell::parse_cell;
pub use parse_coding::parse_coding;

use joinn_frame::{CheckId, FrameRegistry, Refusal, Subject};

/// Cursor over NFC-normalized cell / coding source.
pub(in crate::parse) struct Parser<'a> {
    src: &'a str,
    i: usize,
    frames: &'a FrameRegistry,
}

impl<'a> Parser<'a> {
    pub(in crate::parse) fn new(src: &'a str, frames: &'a FrameRegistry) -> Self {
        Self { src, i: 0, frames }
    }

    pub(in crate::parse) fn eof(&self) -> bool {
        self.i >= self.src.len()
    }

    pub(in crate::parse) fn rest(&self) -> &'a str {
        &self.src[self.i..]
    }

    pub(in crate::parse) fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }

    pub(in crate::parse) fn advance(&mut self) {
        if let Some(c) = self.peek() {
            self.i += c.len_utf8();
        }
    }

    pub(in crate::parse) fn skip(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.advance();
        }
    }

    pub(in crate::parse) fn refuse(&self, reason: impl Into<String>) -> Refusal {
        Refusal {
            check: CheckId::Parse,
            subject: Subject::Other(String::new()),
            reason: reason.into(),
            counterexample: None,
            seed: 0,
        }
    }
}
