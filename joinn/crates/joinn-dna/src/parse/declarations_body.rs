//! `Parser::declarations_body`.

#![allow(clippy::result_large_err)]

use crate::model::Declaration;
use joinn_frame::Verdict;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn declarations_body(&mut self) -> Verdict<Vec<Declaration>> {
        let mut out = Vec::new();
        self.skip();
        if self.peek() == Some('{') {
            self.advance();
            while !self.eof() && self.peek() != Some('}') {
                let id = self.ident();
                if !id.is_empty() {
                    out.push(Declaration { text: id });
                } else {
                    self.advance();
                }
                self.skip();
            }
            if self.peek() == Some('}') {
                self.advance();
            }
        }
        Verdict::Ok(out)
    }
}
