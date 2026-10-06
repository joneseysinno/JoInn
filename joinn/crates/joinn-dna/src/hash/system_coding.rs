//! `Genotype` for `SystemCoding`.

use joinn_frame::{CanonWriter, TAG_SYSTEM};

use crate::body::system::{SystemCoding, print_system};

use super::Genotype;

impl Genotype for SystemCoding {
    fn domain_tag() -> &'static [u8] {
        TAG_SYSTEM
    }

    fn encode(&self, w: &mut CanonWriter) {
        w.push_str(&print_system(self));
    }
}
