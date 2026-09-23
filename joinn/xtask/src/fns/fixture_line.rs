//! The first data row of the unresolvable-artifact fixture table.

use super::workspace_root;

pub(crate) fn fixture_line() -> Result<(String, String), String> {
    let text = std::fs::read_to_string(
        workspace_root()?
            .join("xtask")
            .join("gate_fixtures")
            .join("unresolvable.txt"),
    )
    .map_err(|e| e.to_string())?;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let name = parts
            .next()
            .ok_or_else(|| "fixture row has no item name".to_string())?
            .to_string();
        let path = parts
            .next()
            .ok_or_else(|| format!("fixture row {name} has no artifact"))?
            .to_string();
        if parts.next().is_some() {
            return Err(format!("fixture row {name} has extra fields"));
        }
        return Ok((name, path));
    }
    Err("fixture table is empty".into())
}
