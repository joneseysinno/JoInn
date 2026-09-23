//! `BodyParser::hex_hash`.

#![allow(clippy::result_large_err)]

use joinn_frame::{Hash, Refusal};

use crate::body::BodyParser;
impl<'a> BodyParser<'a> {
    pub(in crate::body) fn hex_hash(&mut self) -> Result<Hash, Refusal> {
        self.skip();
        let start = self.i;
        while matches!(self.peek(), Some(c) if c.is_ascii_hexdigit()) {
            self.advance();
        }
        let s = &self.src[start..self.i];
        Hash::parse_hex(s).ok_or_else(|| self.refuse(format!("expected 64-hex cell hash, got {s}")))
    }
}
