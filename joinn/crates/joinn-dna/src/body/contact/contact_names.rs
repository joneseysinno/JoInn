//! `BodyParser::contact_names`.

use crate::body::BodyParser;

use super::is_contact_section::is_contact_section;

impl<'a> BodyParser<'a> {
    /// Names separated by whitespace or commas, up to a section word, a `}`, a
    /// `cell`/`prim` entry, or anything that is not a name.
    pub(in crate::body::contact) fn contact_names(&mut self) -> Vec<String> {
        let mut names = Vec::new();
        loop {
            self.skip();
            if self.peek() == Some(',') {
                self.advance();
                self.skip();
            }
            match self.peek_ident() {
                Some(word) if is_contact_section(word) || word == "cell" || word == "prim" => {
                    break;
                }
                Some(_) => names.push(self.ident()),
                None => break,
            }
        }
        names
    }
}
