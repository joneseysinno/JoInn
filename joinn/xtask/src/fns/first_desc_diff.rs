//! Name the first field that differs between two canonical descriptions.

/// Name the first field that differs between two canonical descriptions.
pub(crate) fn first_desc_diff(want: &str, got: &str) -> Option<String> {
    for (a, b) in want.lines().zip(got.lines()) {
        if a == b {
            continue;
        }
        let ta: Vec<&str> = a.split_whitespace().collect();
        let tb: Vec<&str> = b.split_whitespace().collect();
        let n = ta.len().min(tb.len());
        for i in 0..n {
            if ta[i] != tb[i] {
                if i > 0 {
                    return Some(ta[i - 1].to_string());
                }
                return Some(ta[i].to_string());
            }
        }
        if ta.len() != tb.len() {
            let extra = if ta.len() > tb.len() { ta[n] } else { tb[n] };
            return Some(extra.to_string());
        }
        return Some("line".into());
    }
    if want != got {
        return Some("structure".into());
    }
    None
}
