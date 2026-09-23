//! `Parser::arg_map`.

#![allow(clippy::result_large_err)]

use crate::formula::Term_;
use joinn_frame::Verdict;
use std::collections::BTreeMap;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn arg_map(&mut self) -> Verdict<BTreeMap<u32, Term_>> {
        if let Err(r) = self.expect('(') {
            return Verdict::Refused(r);
        }
        let mut args = BTreeMap::new();
        loop {
            self.skip();
            if self.peek() == Some(')') {
                self.advance();
                break;
            }
            let pos = match self.number() {
                Ok(n) => n,
                Err(r) => return Verdict::Refused(r),
            };
            if let Err(r) = self.expect(':') {
                return Verdict::Refused(r);
            }
            match self.term() {
                Verdict::Ok(t) => {
                    args.insert(pos, t);
                }
                Verdict::Refused(r) => return Verdict::Refused(r),
            }
            self.skip();
            if self.peek() == Some(',') {
                self.advance();
            }
        }
        Verdict::Ok(args)
    }
}
