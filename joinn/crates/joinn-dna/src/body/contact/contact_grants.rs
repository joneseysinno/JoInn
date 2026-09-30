//! `BodyParser::contact_grants`.

use std::collections::BTreeMap;

use crate::body::BodyParser;

use super::is_contact_section::is_contact_section;

impl<'a> BodyParser<'a> {
    /// `<capability>: <instances>` lines, instances in grant order.
    pub(in crate::body::contact) fn contact_grants(&mut self) -> BTreeMap<String, Vec<String>> {
        let braced = self.take_brace();
        let mut grants = BTreeMap::new();
        loop {
            self.skip();
            match self.peek_ident() {
                Some(word) if !is_contact_section(word) => {
                    let cap = self.ident();
                    if self.peek() == Some(':') {
                        self.advance();
                    }
                    grants.insert(cap, self.contact_names());
                }
                _ => break,
            }
        }
        self.close_section(braced);
        grants
    }
}
