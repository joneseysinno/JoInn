//! `Parser::formula_map`.

#![allow(clippy::result_large_err)]

use crate::formula::Formula;
use joinn_frame::Verdict;
use std::collections::BTreeMap;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn formula_map(&mut self) -> Verdict<BTreeMap<u32, Formula>> {
        let mut map = BTreeMap::new();
        loop {
            self.skip();
            if !matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                break;
            }
            let pos = match self.number() {
                Ok(n) => n,
                Err(r) => return Verdict::Refused(r),
            };
            if let Err(r) = self.expect(':') {
                return Verdict::Refused(r);
            }
            match self.formula() {
                Verdict::Ok(f) => {
                    map.insert(pos, f);
                }
                Verdict::Refused(r) => return Verdict::Refused(r),
            }
            self.skip();
        }
        Verdict::Ok(map)
    }
}
