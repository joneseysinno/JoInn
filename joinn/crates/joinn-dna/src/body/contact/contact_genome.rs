//! `BodyParser::contact_genome`.

use joinn_frame::Refusal;

use crate::body::BodyParser;
use crate::body::contact::CellEntry;

impl<'a> BodyParser<'a> {
    /// `cell:<hex> as <instances>` entries. A `prim:` entry is refused: it
    /// belongs to `.body`.
    pub(in crate::body::contact) fn contact_genome(&mut self) -> Result<Vec<CellEntry>, Refusal> {
        let braced = self.take_brace();
        let mut genome = Vec::new();
        loop {
            self.skip();
            match self.peek_ident() {
                Some("cell") => {
                    self.ident();
                    if self.peek() == Some(':') {
                        self.advance();
                    }
                    let cell = self.hex_hash()?;
                    self.skip();
                    if self.peek_ident() == Some("as") {
                        self.ident();
                    }
                    let instances = self.contact_names();
                    genome.push(CellEntry { cell, instances });
                }
                Some("prim") => {
                    self.ident();
                    if self.peek() == Some(':') {
                        self.advance();
                    }
                    let name = self.ident();
                    return Err(self.refuse(format!(
                        "contact: a genome entry is a cell; prim:{name} belongs to .body. acceptance is cell:<hash>"
                    )));
                }
                _ => break,
            }
        }
        self.close_section(braced);
        Ok(genome)
    }
}
