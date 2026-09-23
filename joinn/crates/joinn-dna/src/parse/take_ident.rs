//! `Parser::take_ident`.

#![allow(clippy::result_large_err)]

use joinn_frame::Refusal;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn take_ident(&mut self) -> Result<String, Refusal> {
        let id = self.ident();
        if id.is_empty() {
            Err(self.refuse("expected identifier"))
        } else {
            Ok(id)
        }
    }
}
