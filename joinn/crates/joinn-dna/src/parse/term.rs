//! `Parser::term`.

#![allow(clippy::result_large_err)]

use crate::formula::{Term_, VarId};
use joinn_frame::{FrameId, Verdict};

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn term(&mut self) -> Verdict<Term_> {
        self.skip();
        if self.peek_ident() == Some("self") {
            self.ident();
            return self.self_at();
        }
        if self.peek_ident() == Some("cell") {
            self.ident();
            return self.cell_at();
        }
        if self.peek_ident() == Some("port") {
            self.ident();
            return Verdict::Ok(Term_::Var(VarId("port".into())));
        }
        // Frame ref or variable. Look ahead: Frame names are Text, ℤ, ℚ, Z, Q.
        if let Some(id) = self.peek_ident() {
            if FrameId::parse(id).is_some() {
                return self.frame_term();
            }
            if matches!(id, "int" | "text") {
                return Verdict::Refused(self.refuse(format!(
                    "primitive name {id} is not allowed in a coding region"
                )));
            }
            let name = self.ident();
            // `int.add` style: ident immediately followed by `.`
            self.skip();
            if self.peek() == Some('.') {
                return Verdict::Refused(self.refuse(format!(
                    "primitive name {name}.* is not allowed in a coding region"
                )));
            }
            return Verdict::Ok(Term_::Var(VarId(name)));
        }
        Verdict::Refused(self.refuse("expected term"))
    }
}
