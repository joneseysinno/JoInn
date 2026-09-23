//! `Parser::cell_at`.

#![allow(clippy::result_large_err)]

use crate::formula::Term_;
use joinn_frame::Verdict;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn cell_at(&mut self) -> Verdict<Term_> {
        if let Err(r) = self.expect(':') {
            return Verdict::Refused(r);
        }
        let cell = match self.hash_hex() {
            Ok(h) => h,
            Err(r) => return Verdict::Refused(r),
        };
        if let Err(r) = self.expect('@') {
            return Verdict::Refused(r);
        }
        let out = match self.number() {
            Ok(n) => n,
            Err(r) => return Verdict::Refused(r),
        };
        match self.arg_map() {
            Verdict::Ok(args) => Verdict::Ok(Term_::CellAt { cell, out, args }),
            Verdict::Refused(r) => Verdict::Refused(r),
        }
    }
}
