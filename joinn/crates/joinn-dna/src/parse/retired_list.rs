//! `Parser::retired_list`.

#![allow(clippy::result_large_err)]

use joinn_frame::Refusal;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn retired_list(&mut self) -> Result<Vec<u32>, Refusal> {
        let mut out = Vec::new();
        self.skip();
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            out.push(self.number()?);
            self.skip();
        }
        Ok(out)
    }
}
