//! `Parser::turn_block`.

#![allow(clippy::result_large_err)]

use crate::model::TurnDecl;
use joinn_frame::Verdict;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn turn_block(&mut self) -> Verdict<Vec<TurnDecl>> {
        self.skip();
        let braced = self.peek() == Some('{');
        if braced {
            self.advance();
        }
        let mut turns = Vec::new();
        loop {
            self.skip();
            if braced && self.peek() == Some('}') {
                self.advance();
                break;
            }
            if self.peek().is_some_and(|c| c.is_ascii_digit()) {
                match self.turn_decl() {
                    Ok(t) => turns.push(t),
                    Err(r) => return Verdict::Refused(r),
                }
                continue;
            }
            break;
        }
        Verdict::Ok(turns)
    }
}
