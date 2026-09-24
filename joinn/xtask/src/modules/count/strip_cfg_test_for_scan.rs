//! Strip `#[cfg(test)]` regions for scanning.

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
