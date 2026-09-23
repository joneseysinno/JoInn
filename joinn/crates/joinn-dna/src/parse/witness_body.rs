//! `Parser::witness_body`.

#![allow(clippy::result_large_err)]

use crate::model::Witness;
use joinn_frame::Verdict;
use std::collections::BTreeMap;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn witness_body(&mut self) -> Verdict<Witness> {
        self.skip();
        if self.peek() == Some('{') {
            self.advance();
        }
        let mut inputs = BTreeMap::new();
        let mut outputs = BTreeMap::new();
        loop {
            self.skip();
            if self.peek() == Some('}') {
                self.advance();
                break;
            }
            match self.peek_ident() {
                Some("in") | Some("out") => {
                    let which = self.ident();
                    let pos = match self.number() {
                        Ok(n) => n,
                        Err(r) => return Verdict::Refused(r),
                    };
                    if let Err(r) = self.expect(':') {
                        return Verdict::Refused(r);
                    }
                    match self.literal_value() {
                        Verdict::Ok(v) => {
                            if which == "in" {
                                inputs.insert(pos, v);
                            } else {
                                outputs.insert(pos, v);
                            }
                        }
                        Verdict::Refused(r) => return Verdict::Refused(r),
                    }
                }
                _ => break,
            }
        }
        Verdict::Ok(Witness { inputs, outputs })
    }
}
