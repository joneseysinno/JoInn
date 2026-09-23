//! `Parser::native_id`.

#![allow(clippy::result_large_err)]

use joinn_frame::{Refusal, nfc};

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn native_id(&mut self) -> Result<String, Refusal> {
        self.skip();
        let start = self.i;
        while matches!(
            self.peek(),
            Some(c) if c.is_alphanumeric()
                || c == '_'
                || c == '@'
                || c == '.'
                || c == 'ℤ'
                || c == 'ℚ'
        ) {
            self.advance();
        }
        if start == self.i {
            return Err(self.refuse("expected native id"));
        }
        Ok(nfc(&self.src[start..self.i]))
    }
}
