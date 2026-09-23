//! `Parser::join_policy`.

#![allow(clippy::result_large_err)]

use crate::model::JoinPolicy;
use joinn_frame::Refusal;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn join_policy(&mut self) -> Result<JoinPolicy, Refusal> {
        let id = self.take_ident()?;
        match id.as_str() {
            "refuse" => Ok(JoinPolicy::Refuse),
            "latest" => Ok(JoinPolicy::Latest),
            "queue" => Ok(JoinPolicy::Queue),
            other => Err(self.refuse(format!("unknown join policy {other}"))),
        }
    }
}
