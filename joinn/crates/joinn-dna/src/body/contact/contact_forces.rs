//! `BodyParser::contact_forces`.

use joinn_frame::{FrameId, FrameRef, Refusal};

use crate::body::BodyParser;
use crate::body::contact::{Force, ForceKind};

use super::is_contact_section::is_contact_section;

impl<'a> BodyParser<'a> {
    /// `combine <frame> <version> cell:<hex> as <response> from <members>`
    /// lines. Codex 1 writes combine only.
    pub(in crate::body::contact) fn contact_forces(&mut self) -> Result<Vec<Force>, Refusal> {
        let braced = self.take_brace();
        let mut forces = Vec::new();
        loop {
            self.skip();
            let word = match self.peek_ident() {
                Some(word) if !is_contact_section(word) => self.ident(),
                _ => break,
            };
            let kind = ForceKind::Combine;
            if word != kind.word() {
                return Err(self.refuse(format!(
                    "contact codex 1 writes combine only; {word} is not written here (separate is combine read at a turn). acceptance is combine"
                )));
            }
            let frame_name = self.ident();
            let Some(id) = FrameId::parse(&frame_name) else {
                return Err(self.refuse(format!(
                    "contact: force frame {frame_name:?} is not a frame; acceptance is Text, ℤ or ℚ and a version"
                )));
            };
            self.skip();
            let version = self.number_u32()?;
            if self.ident() != "cell" {
                return Err(self.refuse(
                    "contact: a force pins its response as cell:<hash>; acceptance is combine <frame> <version> cell:<hash> as <name> from <members>",
                ));
            }
            if self.peek() == Some(':') {
                self.advance();
            }
            let response = self.hex_hash()?;
            self.skip();
            if self.peek_ident() == Some("as") {
                self.ident();
            }
            let name = self.ident();
            if name.is_empty() || name == "from" {
                return Err(self.refuse(
                    "contact: a force names its response; acceptance is combine <frame> <version> cell:<hash> as <name> from <members>",
                ));
            }
            self.skip();
            if self.peek_ident() == Some("from") {
                self.ident();
            }
            let members = self.contact_members(&name)?;
            forces.push(Force {
                kind,
                frame: FrameRef::new(id, version),
                response,
                name,
                members,
            });
        }
        self.close_section(braced);
        Ok(forces)
    }
}
