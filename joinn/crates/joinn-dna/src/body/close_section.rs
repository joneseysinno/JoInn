//! `BodyParser::close_section`.

#![allow(clippy::result_large_err)]

use crate::body::BodyParser;
impl<'a> BodyParser<'a> {
    pub(in crate::body) fn close_section(&mut self, braced: bool) {
        self.skip();
        if braced && self.peek() == Some('}') {
            self.advance();
        }
    }
}
