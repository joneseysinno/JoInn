//! `Parser::laws_body`.

#![allow(clippy::result_large_err)]

use crate::formula::{Law, LawName};
use joinn_frame::Verdict;
use std::collections::BTreeMap;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn laws_body(&mut self) -> Verdict<BTreeMap<LawName, Law>> {
        let mut laws = BTreeMap::new();
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
            let name = match self.peek_ident() {
                Some(n) => n.to_owned(),
                None => break,
            };
            if matches!(
                name.as_str(),
                "codex"
                    | "contract"
                    | "declarations"
                    | "founding"
                    | "frame"
                    | "lineage"
                    | "regulatory"
                    | "alleles"
            ) {
                break;
            }
            self.ident();
            if let Err(r) = self.expect(':') {
                return Verdict::Refused(r);
            }
            match self.formula() {
                Verdict::Ok(formula) => {
                    let law = Law {
                        name: LawName(name.clone()),
                        formula,
                    };
                    laws.insert(LawName(name), law);
                }
                Verdict::Refused(r) => return Verdict::Refused(r),
            }
        }
        Verdict::Ok(laws)
    }
}
