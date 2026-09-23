//! `split_delimiter`.

#![allow(clippy::result_large_err)]

pub(in crate::parse) fn split_delimiter(src: &str) -> (&str, &str) {
    if let Some(idx) = src.find("\n---\n") {
        (&src[..idx], &src[idx + 5..])
    } else if let Some(idx) = src.find("\n---") {
        (&src[..idx], &src[idx + 4..])
    } else {
        (src, "")
    }
}
