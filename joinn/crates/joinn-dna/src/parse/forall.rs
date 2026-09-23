//! `Parser::forall`.

#![allow(clippy::result_large_err)]

use crate::formula::{Formula, VarId};
use joinn_frame::Verdict;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn forall(&mut self) -> Verdict<Formula> {
        let mut vars = Vec::new();
        loop {
            self.skip();
            if self.peek() == Some('.') {
                self.advance();
                break;
            }
            let name = match self.take_ident() {
                Ok(n) => n,
                Err(r) => return Verdict::Refused(r),
            };
            if let Err(r) = self.expect(':') {
                return Verdict::Refused(r);
            }
            match self.frame_ref() {
                Ok(fr) => vars.push((VarId(name), fr)),
                Err(r) => return Verdict::Refused(r),
            }
        }
        self.formula().map(|body| Formula::ForAll {
            vars,
            body: Box::new(body),
        })
    }
}
