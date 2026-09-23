//! Unicode NFC.

use unicode_normalization::UnicodeNormalization;

/// NFC-normalize a string.
pub fn nfc(s: &str) -> String {
    s.nfc().collect()
}
