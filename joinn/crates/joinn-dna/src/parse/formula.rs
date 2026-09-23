//! `Parser::formula`.

#![allow(clippy::result_large_err)]

use crate::formula::Formula;
use joinn_frame::Verdict;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn formula(&mut self) -> Verdict<Formula> {
        self.skip();
        if matches!(self.peek_ident(), Some("forall") | Some("∀")) || self.peek() == Some('∀') {
            if self.peek() == Some('∀') {
                self.advance();
            } else {
                self.ident();
            }
            return self.forall();
        }
        self.implies()
    }
}
