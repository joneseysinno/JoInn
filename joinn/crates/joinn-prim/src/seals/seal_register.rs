//! The seal register.

use super::SealSpec;

/// The seal register. The instrument does not own this table.
pub fn seal_register() -> Vec<SealSpec> {
    vec![
        SealSpec {
            reference_file: "int_add_ref",
            counterfeit_file: "int_add_carry",
            sealed: "add@ℤ",
            drive_port: 1,
            drive_bound: 32,
            one_way: false,
        },
        SealSpec {
            reference_file: "text_parse_ref",
            counterfeit_file: "text_parse",
            sealed: "parse@Text",
            drive_port: 0,
            drive_bound: 8,
            one_way: true,
        },
        SealSpec {
            reference_file: "int_format_ref",
            counterfeit_file: "int_format",
            sealed: "format@ℤ",
            drive_port: 0,
            drive_bound: 32,
            one_way: false,
        },
        SealSpec {
            reference_file: "int_mul_ref",
            counterfeit_file: "int_mul",
            sealed: "mul@ℤ",
            drive_port: 1,
            drive_bound: 8,
            one_way: false,
        },
        SealSpec {
            reference_file: "rat_add_ref",
            counterfeit_file: "rat_add",
            sealed: "add@ℚ",
            drive_port: 1,
            drive_bound: 8,
            one_way: false,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::seal_register;

    #[test]
    fn seal_register_is_not_empty() {
        assert!(!seal_register().is_empty());
        assert!(seal_register().iter().any(|s| s.sealed == "mul@ℤ"));
    }
}
