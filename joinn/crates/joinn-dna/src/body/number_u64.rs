//! `BodyParser::number_u64`.

#![allow(clippy::result_large_err)]

use joinn_frame::Refusal;

use crate::body::BodyParser;
impl<'a> BodyParser<'a> {
    pub(in crate::body) fn number_u64(&mut self) -> Result<u64, Refusal> {
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
