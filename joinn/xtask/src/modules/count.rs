//! Count production `fn` items, ignoring `#[cfg(test)]` regions.

/// How many named production functions appear in `text`.
pub fn production_fns(text: &str) -> usize {
    let stripped = strip_cfg_test(text);
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

fn strip_cfg_test(text: &str) -> String {
    strip_cfg_test_for_scan(text)
}

/// Strip `#[cfg(test)]` regions for scanning (also used by the layout scan).
pub fn strip_cfg_test_for_scan(text: &str) -> String {
    let mut out = String::new();
    let mut depth = 0i32;
    let mut skipping = false;
    let mut skip_depth = 0i32;
    let mut pending_cfg_test = false;
    for line in text.lines() {
        let trimmed = line.trim_start();
        if !skipping && trimmed.starts_with("#[cfg(test)]") {
            pending_cfg_test = true;
            continue;
        }
        let opens = line.chars().filter(|&c| c == '{').count() as i32;
        let closes = line.chars().filter(|&c| c == '}').count() as i32;
        if pending_cfg_test {
            if opens > 0 {
                skipping = true;
                skip_depth = depth + opens - closes;
                if skip_depth <= depth {
                    skipping = false;
                }
                pending_cfg_test = false;
                depth = depth + opens - closes;
                continue;
            }
            // Attribute applies to next item; keep waiting for `{` or single-line item.
            if trimmed.starts_with("mod ") || trimmed.starts_with("fn ") || trimmed.contains("fn ")
            {
                if opens == 0 && !trimmed.ends_with('{') {
                    // single-line item under cfg(test) — skip this line only
                    pending_cfg_test = false;
                    continue;
                }
            }
        }
        if skipping {
            depth = depth + opens - closes;
            if depth < skip_depth {
                skipping = false;
                skip_depth = 0;
            }
            continue;
        }
        depth = depth + opens - closes;
        out.push_str(line);
        out.push('\n');
    }
    out
}

fn is_fn_decl(trimmed: &str) -> bool {
    let t = trimmed
        .trim_start_matches("pub(crate) ")
        .trim_start_matches("pub(super) ")
        .trim_start_matches("pub ")
        .trim_start_matches("async ")
        .trim_start_matches("const ")
        .trim_start_matches("unsafe ");
    t.starts_with("fn ") || t.starts_with("fn\t")
}

fn line_has_allow_modules(line: &str) -> bool {
    allow_modules_reason(line).is_some()
}

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

#[cfg(test)]
mod tests {
    use super::*;

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
