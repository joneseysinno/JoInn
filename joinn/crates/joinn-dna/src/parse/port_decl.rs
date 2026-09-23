//! `Parser::port_decl`.

#![allow(clippy::result_large_err)]

use crate::model::{Direction, PortDecl};
use joinn_frame::Refusal;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn port_decl(&mut self) -> Result<PortDecl, Refusal> {
        let position = self.number()?;
        let dir = self.take_ident()?;
        let direction = match dir.as_str() {
            "in" => Direction::In,
            "out" => Direction::Out,
            other => return Err(self.refuse(format!("unknown direction {other}"))),
        };
        let frame = self.frame_ref()?;
        let req = self.take_ident()?;
        let required = match req.as_str() {
            "required" => true,
            "optional" => false,
            other => return Err(self.refuse(format!("expected required/optional, got {other}"))),
        };
        Ok(PortDecl {
            position,
            direction,
            frame,
            required,
        })
    }
}
