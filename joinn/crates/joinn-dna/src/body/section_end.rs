//! `BodyParser::section_end`.

#![allow(clippy::result_large_err)]

use crate::body::BodyParser;
impl<'a> BodyParser<'a> {
    pub(in crate::body) fn section_end(&self, braced: bool) -> bool {
        if braced {
            return self.peek() == Some('}');
        }
        matches!(
            self.peek_ident(),
            Some("budget")
                | Some("codex")
                | Some("genome")
                | Some("grants")
                | Some("lineage")
                | Some("read")
                | Some("wires")
                | Some("steps")
        )
    }
}
