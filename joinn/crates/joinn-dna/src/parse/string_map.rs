//! `Parser::string_map`.

#![allow(clippy::result_large_err)]

use joinn_frame::Verdict;
use std::collections::BTreeMap;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn string_map(&mut self) -> Verdict<BTreeMap<String, String>> {
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
            let key = match self.peek_ident() {
                Some(k) => k.to_owned(),
                None => break,
            };
            self.ident();
            match self.quoted_string() {
                Ok(v) => {
                    m.insert(key, v);
                }
                Err(r) => return Verdict::Refused(r),
            }
        }
        Verdict::Ok(m)
    }
}
