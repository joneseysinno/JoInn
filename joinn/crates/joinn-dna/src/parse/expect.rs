//! `Parser::expect`.

#![allow(clippy::result_large_err)]

use joinn_frame::Refusal;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn expect(&mut self, c: char) -> Result<(), Refusal> {
        self.skip();
        if self.peek() == Some(c) {
            self.advance();
            Ok(())
        } else {
            Err(self.refuse(format!("expected {c:?}, got {:?}", self.peek())))
        }
    }
}
