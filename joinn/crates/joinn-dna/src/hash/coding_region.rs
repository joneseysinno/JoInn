//! `Genotype` for `CodingRegion`.

use joinn_frame::{CanonWriter, TAG_CELL};

use crate::model::CodingRegion;
use crate::print::print_coding;

use super::Genotype;

impl Genotype for CodingRegion {
    fn domain_tag() -> &'static [u8] {
        TAG_CELL
    }

    fn encode(&self, w: &mut CanonWriter) {
        w.push_str(&print_coding(self));
    }
}
