//! `Parser::ident`.

#![allow(clippy::result_large_err)]

use joinn_frame::nfc;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn ident(&mut self) -> String {
        self.skip();
        let start = self.i;
        if let Some(id) = self.peek_ident() {
            self.i += id.len();
            nfc(&self.src[start..self.i])
        } else {
            String::new()
        }
    }
}
