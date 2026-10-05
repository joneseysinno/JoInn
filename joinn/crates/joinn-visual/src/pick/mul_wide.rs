//! A 128 × 128 → 256-bit product.

/// `a·b` as `(high, low)` 128-bit words, so two products compare as tuples.
pub(crate) fn mul_wide(a: u128, b: u128) -> (u128, u128) {
    const LOW: u128 = u64::MAX as u128;
    let (a1, a0) = (a >> 64, a & LOW);
    let (b1, b0) = (b >> 64, b & LOW);
    let (p00, p01, p10, p11) = (a0 * b0, a0 * b1, a1 * b0, a1 * b1);
    let mid = (p00 >> 64) + (p01 & LOW) + (p10 & LOW);
    let low = (p00 & LOW) | (mid << 64);
    let high = p11 + (p01 >> 64) + (p10 >> 64) + (mid >> 64);
    (high, low)
}

#[cfg(test)]
mod tests {
    use super::mul_wide;

    #[test]
    fn products_past_128_bits_carry_into_the_high_word() {
        assert_eq!(mul_wide(3, 5), (0, 15));
        assert_eq!(mul_wide(1 << 64, 1 << 64), (1, 0));
        assert_eq!(mul_wide(u128::MAX, 2), (1, u128::MAX - 1));
        assert_eq!(mul_wide(u128::MAX, u128::MAX), (u128::MAX - 1, 1));
    }
}
