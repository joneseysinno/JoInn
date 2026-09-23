//! `strip_comments`.

#![allow(clippy::result_large_err)]

pub(in crate::parse) fn strip_comments(src: &str) -> String {
    let mut out = String::new();
    for line in src.lines() {
        let cut = match line.find('#') {
            Some(i) => &line[..i],
            None => line,
        };
        out.push_str(cut.trim_end());
        out.push('\n');
    }
    out
}
