//! `Parser::number`.

#![allow(clippy::result_large_err)]

use joinn_frame::Refusal;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn number(&mut self) -> Result<u32, Refusal> {
        self.skip();
        let start = self.i;
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            self.advance();
        }
        if start == self.i {
            return Err(self.refuse("expected number"));
        }
        self.src[start..self.i]
            .parse()
            .map_err(|_| self.refuse("number out of range"))
    }
}
