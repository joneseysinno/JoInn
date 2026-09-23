//! One production function — modules check must accept this leaf shape.
fn alone() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fn_does_not_count() {
        assert!(true);
    }
}
