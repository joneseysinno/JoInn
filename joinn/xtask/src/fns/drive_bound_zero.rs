//! Load the empty-bound SealSpec artifact for gate 2.2 item 3.

use super::workspace_root;

/// `(sealed name, drive_bound)` from `corpus/phase22/drive_bound_zero.sealspec`.
pub(crate) fn drive_bound_zero() -> Result<(String, u32), String> {
    let path = workspace_root()?
        .join("corpus")
        .join("phase22")
        .join("drive_bound_zero.sealspec");
    let src = std::fs::read_to_string(&path).map_err(|e| format!("{path:?}: {e}"))?;
    let mut sealed = None;
    let mut bound = None;
    for line in src.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((k, v)) = line.split_once(':') else {
            continue;
        };
        match k.trim() {
            "sealed" => sealed = Some(v.trim().to_owned()),
            "drive_bound" => {
                bound = Some(
                    v.trim()
                        .parse::<u32>()
                        .map_err(|e| format!("drive_bound: {e}"))?,
                );
            }
            _ => {}
        }
    }
    match (sealed, bound) {
        (Some(s), Some(b)) => Ok((s, b)),
        _ => Err("drive_bound_zero.sealspec missing sealed or drive_bound".into()),
    }
}
