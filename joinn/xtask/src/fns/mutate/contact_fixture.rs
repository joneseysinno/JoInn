//! Test fixture: `corpus/phase7/calculator.contact` as a subject.

use crate::fns::parse_subject::parse_subject;
use crate::fns::subject::Subject;

pub(crate) fn contact_fixture() -> Subject {
    let src = include_str!("../../../../corpus/phase7/calculator.contact");
    parse_subject("phase7/calculator.contact", src)
        .unwrap_or_else(|e| panic!("parse calculator.contact: {e}"))
}
