//! `BodyParser::ident`.

#![allow(clippy::result_large_err)]

use crate::body::BodyParser;
impl<'a> BodyParser<'a> {
    pub(in crate::body) fn ident(&mut self) -> String {
        self.skip();
        if let Some(id) = self.peek_ident() {
            let s = id.to_owned();
            self.i += id.len();
            s
        } else {
            String::new()
        }
    }
}
