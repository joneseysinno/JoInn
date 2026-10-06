//! `BodyParser::system_bodies`.

use joinn_frame::Refusal;

use crate::body::BodyParser;
use crate::body::system::SystemBody;

impl<'a> BodyParser<'a> {
    /// `contact:<hex> as <alias>` entries.
    pub(in crate::body::system) fn system_bodies(&mut self) -> Result<Vec<SystemBody>, Refusal> {
        let braced = self.take_brace();
        let mut bodies: Vec<SystemBody> = Vec::new();
        while self.peek_ident() == Some("contact") {
            self.ident();
            if self.peek() == Some(':') {
                self.advance();
            }
            let contact = self.hex_hash()?;
            self.skip();
            if self.peek_ident() == Some("as") {
                self.ident();
            }
            let alias = self.ident();
            if alias.is_empty() {
                return Err(self.refuse(
                    "system: a body has an alias; acceptance is contact:<hash> as <alias>",
                ));
            }
            if bodies.iter().any(|b| b.alias == alias) {
                return Err(self.refuse(format!(
                    "system: {alias} names two bodies; acceptance is a new alias"
                )));
            }
            bodies.push(SystemBody { contact, alias });
            self.skip();
        }
        self.close_section(braced);
        Ok(bodies)
    }
}
