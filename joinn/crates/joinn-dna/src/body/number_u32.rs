//! `BodyParser::number_u32`.

#![allow(clippy::result_large_err)]

use joinn_frame::Refusal;

use crate::body::BodyParser;
impl<'a> BodyParser<'a> {
    pub(in crate::body) fn number_u32(&mut self) -> Result<u32, Refusal> {
        let n = self.number_u64()?;
        u32::try_from(n).map_err(|_| self.refuse("number out of range"))
    }
}
