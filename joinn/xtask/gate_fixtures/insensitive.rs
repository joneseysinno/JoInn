//! A control that ignores the subject it is given.
//! Kept as a fixture source; the byte-damage rule is gone (P52-03).

pub fn ignores_its_bytes(_art: &()) -> bool {
    false
}
