//! `BodyParser::contact_members`.

use joinn_frame::Refusal;

use crate::body::BodyParser;
use crate::body::contact::Member;

impl<'a> BodyParser<'a> {
    /// `instance@port` members, separated by whitespace or commas. A member
    /// written twice in one force is refused.
    pub(in crate::body::contact) fn contact_members(
        &mut self,
        force: &str,
    ) -> Result<Vec<Member>, Refusal> {
        let mut members: Vec<Member> = Vec::new();
        loop {
            self.skip();
            if self.peek() == Some(',') {
                self.advance();
                self.skip();
            }
            let Some(word) = self.peek_ident() else {
                break;
            };
            if !self.rest().trim_start()[word.len()..].starts_with('@') {
                break;
            }
            let instance = self.ident();
            self.expect('@')?;
            let port = self.number_u32()?;
            let member = Member { instance, port };
            if members.contains(&member) {
                return Err(self.refuse(format!(
                    "contact: {member} is written twice in force {force}"
                )));
            }
            members.push(member);
        }
        Ok(members)
    }
}
