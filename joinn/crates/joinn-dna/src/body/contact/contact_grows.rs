//! `BodyParser::contact_grows`.

use joinn_frame::Refusal;

use crate::body::BodyParser;
use crate::body::contact::{Accept, Grows};

/// The corpus input cell (`corpus/phase0/cli_input.cell`, the calculator's
/// `cli_a`).
const INPUT_CELL: &str = "c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e";

impl<'a> BodyParser<'a> {
    /// `cell:<hex> as <growth name> accepts one|any`. In Phase 7.4 the cell is
    /// the input cell.
    pub(in crate::body::contact) fn contact_grows(&mut self) -> Result<Grows, Refusal> {
        let braced = self.take_brace();
        self.skip();
        if self.ident() != "cell" {
            return Err(self.refuse(
                "contact: grows names the cell each growth adds; acceptance is grows { cell:<hash> as <name> accepts one|any }",
            ));
        }
        if self.peek() == Some(':') {
            self.advance();
        }
        let cell = self.hex_hash()?;
        if cell.to_hex() != INPUT_CELL {
            return Err(self.refuse(format!(
                "contact: grows cell:{}; acceptance is the input cell cell:{INPUT_CELL} in Phase 7.4",
                cell.to_hex()
            )));
        }
        self.skip();
        if self.peek_ident() == Some("as") {
            self.ident();
        }
        let name = self.ident();
        if name.is_empty() || name == "accepts" {
            return Err(self.refuse(
                "contact: grows names its growth; acceptance is grows { cell:<hash> as <name> accepts one|any }",
            ));
        }
        self.skip();
        if self.ident() != "accepts" {
            return Err(self.refuse(format!(
                "contact: grows {name} says what it accepts; acceptance is accepts one or accepts any"
            )));
        }
        let word = self.ident();
        let accepts = match word.as_str() {
            "one" => Accept::One,
            "any" => Accept::Any,
            other => {
                return Err(self.refuse(format!(
                    "contact: accepts {other:?}; acceptance is one or any"
                )));
            }
        };
        self.close_section(braced);
        Ok(Grows {
            cell,
            name,
            accepts,
        })
    }
}
