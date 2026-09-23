//! `Parser::eq`.

#![allow(clippy::result_large_err)]

use crate::formula::Formula;
use joinn_frame::Verdict;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn eq(&mut self) -> Verdict<Formula> {
        let left = match self.term() {
            Verdict::Ok(t) => t,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        self.skip();
        if self.peek() == Some('=') {
            self.advance();
            return self.term().map(|r| Formula::Eq(left, r));
        }
        Verdict::Refused(self.refuse("expected = in formula"))
    }
}
