//! What an out-port value prints: at most four glyphs.

use super::VALUE_GLYPHS;

/// The value as printed: itself when it is at most four characters, otherwise
/// its first three and `…`.
pub fn value_text(value: &str) -> String {
    if value.chars().count() <= VALUE_GLYPHS {
        return value.to_owned();
    }
    let mut out: String = value.chars().take(VALUE_GLYPHS - 1).collect();
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::value_text;

    #[test]
    fn four_glyphs_print_whole_and_five_print_three_and_an_ellipsis() {
        assert_eq!(value_text("12345"), "123…");
        assert_eq!(value_text("1234"), "1234");
        assert_eq!(value_text("-123"), "-123");
        assert_eq!(value_text("-1234"), "-12…");
        assert_eq!(value_text("5"), "5");
    }
}
