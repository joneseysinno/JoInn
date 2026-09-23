//! `Parser::not`.

#![allow(clippy::result_large_err)]

use crate::formula::Formula;
use joinn_frame::Verdict;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn not(&mut self) -> Verdict<Formula> {
        self.skip();
        if self.peek() == Some('¬') || self.peek_ident() == Some("not") {
            if self.peek() == Some('¬') {
                self.advance();
            } else {
                self.ident();
            }
            return self.not().map(|f| Formula::Not(Box::new(f)));
        }
        if self.peek() == Some('(') {
            self.advance();
            let f = self.formula();
            return match f {
                Verdict::Ok(f) => match self.expect(')') {
                    Ok(()) => Verdict::Ok(f),
                    Err(r) => Verdict::Refused(r),
                },
                other => other,
            };
        }
        self.eq()
    }
}
