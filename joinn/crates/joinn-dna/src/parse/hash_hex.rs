//! `Parser::hash_hex`.

#![allow(clippy::result_large_err)]

use joinn_frame::{Hash, Refusal};

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn hash_hex(&mut self) -> Result<Hash, Refusal> {
        self.skip();
        let start = self.i;
        while matches!(self.peek(), Some(c) if c.is_ascii_hexdigit()) {
            self.advance();
        }
        let hex = &self.src[start..self.i];
        Hash::parse_hex(hex).ok_or_else(|| self.refuse("expected 64-hex hash"))
    }
}
