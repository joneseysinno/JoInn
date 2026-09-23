//! `Parser::frame_ref`.

#![allow(clippy::result_large_err)]

use joinn_frame::{FrameId, FrameRef, Refusal};

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn frame_ref(&mut self) -> Result<FrameRef, Refusal> {
        let name = self.take_ident()?;
        let Some(id) = FrameId::parse(&name) else {
            if matches!(name.as_str(), "int" | "text") {
                return Err(self.refuse(format!("primitive namespace {name} is not a frame name")));
            }
            return Err(self.refuse(format!("unknown frame {name}")));
        };
        let version = self.number()?;
        Ok(FrameRef::new(id, version))
    }
}
