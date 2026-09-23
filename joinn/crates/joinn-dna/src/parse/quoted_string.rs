//! `Parser::quoted_string`.

#![allow(clippy::result_large_err)]

use joinn_frame::{Refusal, nfc};

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn quoted_string(&mut self) -> Result<String, Refusal> {
        self.skip();
        if self.peek() != Some('"') {
            return Err(self.refuse("expected string"));
        }
        self.advance();
        let start = self.i;
        while let Some(c) = self.peek() {
            if c == '\\' {
                self.advance();
                self.advance();
            } else if c == '"' {
                let s = nfc(&self.src[start..self.i]);
                self.advance();
                return Ok(s);
            } else {
                self.advance();
            }
        }
        Err(self.refuse("unterminated string"))
    }
}
