use super::{Edition, parse};

/// Reads the spike's one edition, `bridges.edition`, beside its Cargo.toml.
pub fn pinned() -> Result<Edition, String> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/bridges.edition");
    let text = std::fs::read_to_string(path).map_err(|e| format!("edition: {path}: {e}"))?;
    parse(&text)
}
