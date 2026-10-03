//! SplitMix64: the grove's only generator.

/// One draw. Every operation wraps in u64.
pub(crate) fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[cfg(test)]
mod tests {
    use super::splitmix64;

    #[test]
    fn seed_7_draws_the_plan_s_first_three() {
        let mut s = 7;
        assert_eq!(splitmix64(&mut s), 7_191_089_600_892_374_487);
        assert_eq!(splitmix64(&mut s), 309_689_372_594_955_804);
        assert_eq!(splitmix64(&mut s), 16_616_101_746_815_609_346);
    }
}
