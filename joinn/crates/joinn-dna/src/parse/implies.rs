//! `Parser::implies`.

#![allow(clippy::result_large_err)]

use crate::formula::Formula;
use joinn_frame::Verdict;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn implies(&mut self) -> Verdict<Formula> {
        let left = match self.or() {
            Verdict::Ok(f) => f,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        self.skip();
        if self.peek() == Some('→') || self.peek_ident() == Some("implies") {
            if self.peek() == Some('→') {
                self.advance();
            } else {
                self.ident();
            }
            return self
                .implies()
                .map(|r| Formula::Implies(Box::new(left), Box::new(r)));
        }
        Verdict::Ok(left)
    }
}
