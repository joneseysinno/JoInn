//! A segment's level.

/// The number of trailing 3s in `i` written in base 4.
pub fn level(i: u64) -> u32 {
    let mut t = i;
    let mut n = 0;
    while t % 4 == 3 {
        n += 1;
        t /= 4;
    }
    n
}
