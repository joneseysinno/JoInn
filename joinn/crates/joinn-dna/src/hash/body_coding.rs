//! `Genotype` for `BodyCoding`.

use joinn_frame::{CanonWriter, TAG_BODY};

use crate::body::{BodyCoding, print_body};

use super::Genotype;

impl Genotype for BodyCoding {
    fn domain_tag() -> &'static [u8] {
        TAG_BODY
    }

    fn encode(&self, w: &mut CanonWriter) {
        w.push_str(&print_body(self));
    }
}
