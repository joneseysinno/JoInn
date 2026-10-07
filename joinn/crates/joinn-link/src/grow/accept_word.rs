//! What a body that accepts so is called in refusals.

use joinn_dna::Accept;

/// `accepts one` is counting; `accepts any` is adding.
pub fn accept_word(accepts: Accept) -> &'static str {
    match accepts {
        Accept::One => "counting",
        Accept::Any => "adding",
    }
}
