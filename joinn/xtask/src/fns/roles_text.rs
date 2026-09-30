//! `cargo xtask roles <path>`: each cell's role in an admitted contact body.

use joinn_dna::parse_contact;
use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::{BodyRole, Facing, Holding, body_roles, check_contact};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use super::forces::corpus_cells;
use super::workspace_root;

/// The path is read from the working directory, else from the workspace root.
/// A path that is not a `.contact`, or a contact that is refused, is the error.
pub(crate) fn roles_text(path: &str) -> Result<String, String> {
    if !path.ends_with(".contact") {
        return Err(format!(
            "roles: {path} is not a .contact; acceptance is a .contact file"
        ));
    }
    let given = Path::new(path);
    let file = if given.is_file() {
        given.to_path_buf()
    } else {
        workspace_root()?.join(path)
    };
    let src = fs::read_to_string(&file).map_err(|e| format!("roles: {path}: {e}"))?;
    let frames = FrameRegistry::phase1();
    let contact = match parse_contact(&src, &frames) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => return Err(format!("roles: {path}: refused: {}", r.reason)),
    };
    let cells = corpus_cells(&frames)?;
    if let Verdict::Refused(r) = check_contact(&contact, &cells, &frames) {
        return Err(format!("roles: {path}: refused: {}", r.reason));
    }
    let roles = match body_roles(&contact, &cells) {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => return Err(format!("roles: {path}: refused: {}", r.reason)),
    };
    let name = file
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let mut out = String::new();
    let _ = writeln!(out, "roles {name}");
    for (instance, role) in &roles {
        let _ = writeln!(
            out,
            "{instance} {} ({}, {})",
            role.word(),
            role.facing.word(),
            role.holding.word()
        );
    }
    let mut counts = Vec::new();
    for facing in [Facing::Out, Facing::In] {
        for holding in [Holding::Holds, Holding::Reacts] {
            let role = BodyRole { facing, holding };
            let n = roles.values().filter(|r| **r == role).count();
            counts.push(format!("{} {n}", role.word()));
        }
    }
    let _ = writeln!(out, "roles: {} cell(s); {}", roles.len(), counts.join(", "));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::roles_text;

    #[test]
    fn the_calculator_prints_the_predicted_block() {
        assert_eq!(
            roles_text("corpus/phase7/calculator.contact"),
            Ok("roles calculator.contact\n\
                cli_a protect (faces out, holds)\n\
                cli_b protect (faces out, holds)\n\
                sum carry (faces out, reacts)\n\
                roles: 3 cell(s); protect 2, carry 1, store 0, respond 0\n"
                .to_owned())
        );
    }

    #[test]
    fn a_body_file_is_not_a_contact() {
        let Err(e) = roles_text("corpus/phase2/calculator.body") else {
            panic!("a .body must be refused");
        };
        assert_eq!(
            e,
            "roles: corpus/phase2/calculator.body is not a .contact; acceptance is a .contact file"
        );
    }
}
