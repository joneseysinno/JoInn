//! `Parser::turn_decl`.

#![allow(clippy::result_large_err)]

use crate::model::TurnDecl;
use joinn_frame::Refusal;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn turn_decl(&mut self) -> Result<TurnDecl, Refusal> {
        let out = self.number()?;
        self.skip();
        let key = self.ident();
        if key != "from" {
            return Err(self.refuse(format!("expected 'from' in turn, got {key}")));
        }
        self.skip();
        let set = self.peek() == Some('{');
        if set {
            self.advance();
        }
        let mut from = Vec::new();
        loop {
            self.skip();
            if set && self.peek() == Some('}') {
                self.advance();
                break;
            }
            if self.peek() == Some(',') {
                self.advance();
                continue;
            }
            if self.peek().is_some_and(|c| c.is_ascii_digit()) {
                from.push(self.number()?);
                continue;
            }
            break;
        }
        from.sort();
        from.dedup();
        Ok(TurnDecl { out, from })
    }
}
