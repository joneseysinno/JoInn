//! Mask standard-library calls before concept-word scans.

/// Replace identifiers that are std method/path segments with spaces.
pub(crate) fn mask_std_calls(line: &str) -> String {
    let b = line.as_bytes();
    let mut mask = vec![false; b.len()];
    let is_start = |c: u8| c.is_ascii_alphabetic() || c == b'_';
    let is_cont = |c: u8| c.is_ascii_alphanumeric() || c == b'_';

    let mut i = 0;
    while i < b.len() {
        if b[i] == b'.' && i + 1 < b.len() && is_start(b[i + 1]) {
            let start = i + 1;
            let mut end = start;
            while end < b.len() && is_cont(b[end]) {
                end += 1;
            }
            for slot in &mut mask[start..end] {
                *slot = true;
            }
            i = end;
            continue;
        }
        i += 1;
    }

    for prefix in [
        "std::", "core::", "fs::", "io::", "u32::", "u64::", "usize::", "i64::",
    ] {
        let mut search = 0;
        while let Some(rel) = line[search..].find(prefix) {
            let at = search + rel + prefix.len();
            if at < b.len() && is_start(b[at]) {
                let mut end = at;
                while end < b.len() && is_cont(b[end]) {
                    end += 1;
                }
                for slot in &mut mask[at..end] {
                    *slot = true;
                }
            }
            search += rel + 1;
        }
    }

    let mut out = String::with_capacity(line.len());
    for (i, ch) in line.char_indices() {
        if mask.get(i).copied().unwrap_or(false) {
            out.push(' ');
        } else {
            out.push(ch);
        }
    }
    out
}
