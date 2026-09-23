//! Floor membership.

use super::register::register;

/// True when `name` is in the floor.
pub fn contains(name: &str) -> bool {
    register().iter().any(|p| p.name() == name)
}

#[cfg(test)]
mod tests {
    use super::contains;

    #[test]
    fn succ_is_not_in_the_floor() {
        assert!(!contains("succ"));
        assert!(!contains("zero"));
        assert!(!contains("pred"));
        assert!(contains("build"));
        assert!(contains("case"));
        assert!(contains("resolve"));
    }
}
