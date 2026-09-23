//! `Parser::name_map`.

#![allow(clippy::result_large_err)]

use joinn_frame::Verdict;
use std::collections::BTreeMap;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn name_map(&mut self) -> Verdict<BTreeMap<u32, String>> {
        let mut m = BTreeMap::new();
        self.skip();
        if self.peek() == Some('{') {
            self.advance();
        }
        loop {
            self.skip();
            if self.peek() == Some('}') {
                self.advance();
                break;
            }
            if !matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                break;
            }
            let pos = match self.number() {
                Ok(n) => n,
                Err(r) => return Verdict::Refused(r),
            };
            self.skip();
            let name = if self.peek() == Some('"') {
                match self.quoted_string() {
                    Ok(n) => n,
                    Err(r) => return Verdict::Refused(r),
                }
            } else {
                match self.take_ident() {
                    Ok(n) => n,
                    Err(r) => return Verdict::Refused(r),
                }
            };
            m.insert(pos, name);
        }
        Verdict::Ok(m)
    }
}
