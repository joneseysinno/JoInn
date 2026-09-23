//! `BodyParser::take_brace`.

#![allow(clippy::result_large_err)]

use crate::body::BodyParser;
impl<'a> BodyParser<'a> {
    pub(in crate::body) fn take_brace(&mut self) -> bool {
        self.skip();
        if self.peek() == Some('{') {
            self.advance();
            true
        } else {
            false
        }
    }
}
