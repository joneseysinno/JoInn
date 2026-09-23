//! `cargo xtask decisions` — print counts, write no file.

use super::workspace_root;
use std::fs;

/// `cargo xtask decisions` — print counts, write no file.
pub(crate) fn decisions() -> Result<(), String> {
    let path = workspace_root()?
        .parent()
        .ok_or_else(|| "workspace has no parent".to_string())?
        .join("docs/Findings/decisions.md");
    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut holds = 0u32;
    let mut open = 0u32;
    let mut reversed = 0u32;
    for line in text.lines() {
        let t = line.trim();
        if !t.starts_with('|') || t.starts_with("| id") || t.starts_with("|---") {
            continue;
        }
        let cols: Vec<&str> = t
            .split('|')
            .map(str::trim)
            .filter(|c| !c.is_empty())
            .collect();
        if cols.len() < 4 {
            continue;
        }
        let status = cols[3];
        match status {
            "holds" => holds += 1,
            "open" => open += 1,
            "reversed" => {
                reversed += 1;
                let rest = t.to_ascii_lowercase();
                if !rest.contains("findings/") {
                    return Err(format!("reversed line with no linked finding: {t}"));
                }
            }
            _ => {}
        }
    }
    println!("decisions holds: {holds} open: {open} reversed: {reversed}");
    Ok(())
}
