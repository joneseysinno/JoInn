//! `Parser::and`.

#![allow(clippy::result_large_err)]

use crate::formula::Formula;
use joinn_frame::Verdict;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn and(&mut self) -> Verdict<Formula> {
        let mut xs = vec![match self.not() {
            Verdict::Ok(f) => f,
            Verdict::Refused(r) => return Verdict::Refused(r),
        }];
        loop {
            self.skip();
            if self.peek() == Some('∧') || self.peek_ident() == Some("and") {
                if self.peek() == Some('∧') {
                    self.advance();
                } else {
                    self.ident();
                }
                match self.not() {
                    Verdict::Ok(f) => xs.push(f),
                    Verdict::Refused(r) => return Verdict::Refused(r),
                }
            } else {
                break;
            }
        }
        if xs.len() == 1 {
            Verdict::Ok(xs.remove(0))
        } else {
            Verdict::Ok(Formula::And(xs))
        }
    }
}
