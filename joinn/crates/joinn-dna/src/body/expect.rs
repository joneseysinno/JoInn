//! `BodyParser::expect`.

#![allow(clippy::result_large_err)]

use joinn_frame::Refusal;

use crate::body::BodyParser;
impl<'a> BodyParser<'a> {
    pub(in crate::body) fn expect(&mut self, c: char) -> Result<(), Refusal> {
        self.skip();
        if self.peek() == Some(c) {
            self.advance();
            Ok(())
        } else {
            Err(self.refuse(format!("expected {c:?}, got {:?}", self.peek())))
        }
    }
}
