//! Hand-written turn alleles register.

/// Hand-written turn alleles, derived from the register. R31.
pub fn turn_register() -> Vec<(&'static str, u32, &'static str)> {
    vec![("add@ℤ", 0, "add@ℤ.turn0"), ("add@ℤ", 1, "add@ℤ.turn1")]
}
