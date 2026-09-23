//! `Genotype` for `Allele`.

use joinn_frame::{CanonWriter, TAG_ALLELE};

use crate::model::Allele;
use crate::print::print_allele;

use super::Genotype;

impl Genotype for Allele {
    fn domain_tag() -> &'static [u8] {
        TAG_ALLELE
    }

    fn encode(&self, w: &mut CanonWriter) {
        w.push_str(&print_allele(self));
    }
}
