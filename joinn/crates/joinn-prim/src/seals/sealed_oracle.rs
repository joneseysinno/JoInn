//! Lookup sealed oracle by name.

use crate::alleles::{AddInt, AddRat, FormatInt, ParseText};
use joinn_gate::Oracle;

use super::mul_int_sealed::MulIntSealed;

pub(in crate::seals) fn sealed_oracle(id: &str) -> Option<&'static dyn Oracle> {
    match id {
        "add@ℤ" => Some(&AddInt),
        "parse@Text" => Some(&ParseText),
        "format@ℤ" => Some(&FormatInt),
        "mul@ℤ" => Some(&MulIntSealed),
        "add@ℚ" => Some(&AddRat),
        _ => None,
    }
}
