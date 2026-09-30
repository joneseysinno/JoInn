//! `Genotype` for `ContactCoding`.

use joinn_frame::{CanonWriter, TAG_CONTACT};

use crate::body::contact::{ContactCoding, print_contact};

use super::Genotype;

impl Genotype for ContactCoding {
    fn domain_tag() -> &'static [u8] {
        TAG_CONTACT
    }

    fn encode(&self, w: &mut CanonWriter) {
        w.push_str(&print_contact(self));
    }
}
