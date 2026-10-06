//! `BodyParser::system_forces`.

use joinn_frame::{FrameId, FrameRef, Refusal};

use crate::body::BodyParser;
use crate::body::contact::ForceKind;
use crate::body::system::SystemForce;

impl<'a> BodyParser<'a> {
    /// `combine <frame> <version> cell:<hex> as <response> on <alias>` lines.
    pub(in crate::body::system) fn system_forces(&mut self) -> Result<Vec<SystemForce>, Refusal> {
        const SHAPE: &str =
            "acceptance is combine <frame> <version> cell:<hash> as <name> on <body alias>";
        let braced = self.take_brace();
        let mut forces = Vec::new();
        loop {
            self.skip();
            let word = match self.peek_ident() {
                Some(w) if !matches!(w, "bodies" | "codex" | "forces" | "lineage") => self.ident(),
                _ => break,
            };
            let kind = ForceKind::Combine;
            if word != kind.word() {
                return Err(self.refuse(format!(
                    "system codex 1 writes combine only; {word} is not written here. acceptance is combine"
                )));
            }
            let frame_name = self.ident();
            let Some(id) = FrameId::parse(&frame_name) else {
                return Err(self.refuse(format!(
                    "system: force frame {frame_name:?} is not a frame; acceptance is Text, ℤ or ℚ and a version"
                )));
            };
            self.skip();
            let version = self.number_u32()?;
            if self.ident() != "cell" {
                return Err(self.refuse(format!(
                    "system: a force pins its response as cell:<hash>; {SHAPE}"
                )));
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
            if name.is_empty() || name == "on" {
                return Err(self.refuse(format!("system: a force names its response; {SHAPE}")));
            }
            self.skip();
            if self.peek_ident() == Some("from") {
                return Err(self.refuse(format!(
                    "system: force {name} names no instance (it reaches every grown instance of its body); {SHAPE}"
                )));
            }
            if self.ident() != "on" {
                return Err(self.refuse(format!("system: force {name} says which body; {SHAPE}")));
            }
            let on = self.ident();
            if on.is_empty() {
                return Err(self.refuse(format!("system: force {name} says which body; {SHAPE}")));
            }
            forces.push(SystemForce {
                kind,
                frame: FrameRef::new(id, version),
                response,
                name,
                on,
            });
        }
        self.close_section(braced);
        Ok(forces)
    }
}
