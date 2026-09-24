//! `allow(modules): reason` silencer, mirrors vocab.

/// `allow(modules): reason` silencer, mirrors vocab.
pub fn allow_modules_reason(line: &str) -> Option<&str> {
    const MARK: &str = concat!("allow", "(modules)");
    let idx = line.find(MARK)?;
    let rest = &line[idx + MARK.len()..];
    if let Some(r) = rest.strip_prefix(':') {
        Some(r.trim())
    } else if rest.trim_start().starts_with(':') {
        Some(rest.trim_start().trim_start_matches(':').trim())
    } else {
        Some("")
    }
}
