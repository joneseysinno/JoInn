//! Print an owner the way a click reports it.

use super::Owner;

/// `background`, `<alias> surface`, `<alias>.<instance>`, `<alias>.<instance>@<n>`,
/// or `<alias> wire <src> -> <dst>`.
pub fn print_owner(alias: &str, owner: &Owner) -> String {
    match owner {
        Owner::Background => "background".to_owned(),
        Owner::Surface => format!("{alias} surface"),
        Owner::Cell(instance) => format!("{alias}.{instance}"),
        Owner::Port(address) => format!("{alias}.{}", address.printed()),
        Owner::Wire { src, dst } => {
            format!("{alias} wire {} -> {}", src.printed(), dst.printed())
        }
    }
}
