//! `Parser::founding_body`.

#![allow(clippy::result_large_err)]

use crate::model::Witness;
use joinn_frame::Verdict;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn founding_body(&mut self) -> Verdict<Vec<Witness>> {
        let mut out = Vec::new();
        self.skip();
        let braced = self.peek() == Some('{');
        if braced {
            self.advance();
        }
        loop {
            self.skip();
            if braced && self.peek() == Some('}') {
                self.advance();
                break;
            }
            if self.peek_ident() == Some("witness") {
                self.ident();
                match self.witness_body() {
                    Verdict::Ok(w) => out.push(w),
                    Verdict::Refused(r) => return Verdict::Refused(r),
                }
            } else {
                break;
            }
        }
        Verdict::Ok(out)
    }
}
