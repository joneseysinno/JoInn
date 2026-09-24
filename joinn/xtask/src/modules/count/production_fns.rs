//! How many named production functions appear in `text`.

use super::is_fn_decl::is_fn_decl;
use super::line_has_allow_modules::line_has_allow_modules;
use super::strip_cfg_test_for_scan::strip_cfg_test_for_scan;

/// How many named production functions appear in `text`.
pub fn production_fns(text: &str) -> usize {
    let stripped = strip_cfg_test_for_scan(text);
    let mut count = 0;
    for line in stripped.lines() {
        if line_has_allow_modules(line) {
            continue;
        }
        let trimmed = line.trim_start();
        if is_fn_decl(trimmed) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::production_fns;

    #[test]
    fn counts_two_top_level_fns() {
        let src = "fn a() {}\nfn b() {}\n";
        assert_eq!(production_fns(src), 2);
    }

    #[test]
    fn ignores_cfg_test_block() {
        let src = "fn a() {}\n#[cfg(test)]\nmod tests {\n    fn b() {}\n}\n";
        assert_eq!(production_fns(src), 1);
    }
}
