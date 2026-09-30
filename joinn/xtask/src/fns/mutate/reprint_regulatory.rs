//! A body's or contact's regulatory region, printed after `---`.

use joinn_dna::BodyRegulatory;
use std::fmt::Write as _;

/// Empty when the region holds nothing.
pub(super) fn reprint_regulatory(reg: &BodyRegulatory) -> String {
    let mut out = String::new();
    if reg.prompts.is_empty()
        && reg.present.is_empty()
        && reg.names.is_empty()
        && reg.labels.is_empty()
    {
        return out;
    }
    out.push_str("---\nregulatory {\n");
    for (section, map) in [
        ("prompts", &reg.prompts),
        ("present", &reg.present),
        ("names", &reg.names),
        ("labels", &reg.labels),
    ] {
        if map.is_empty() {
            continue;
        }
        let _ = write!(out, "  {section} {{");
        for (k, v) in map {
            let _ = write!(out, " {k} \"{v}\"");
        }
        out.push_str(" }\n");
    }
    out.push_str("}\n");
    out
}
