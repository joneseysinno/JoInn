//! Print a protein list.

use super::Protein;

/// One line per protein, `protein at letter <n>: <acids>`; the empty protein
/// reads `nothing`, and a strand with none prints `no protein`. The reader's
/// version is not printed: it never changes what a strand means.
pub fn print_proteins(proteins: &[Protein]) -> String {
    if proteins.is_empty() {
        return "no protein\n".to_owned();
    }
    let mut out = String::new();
    for p in proteins {
        let acids: Vec<&str> = p.acids.iter().map(|a| a.name()).collect();
        let body = if acids.is_empty() {
            "nothing".to_owned()
        } else {
            acids.join(" ")
        };
        out.push_str(&format!("protein at letter {}: {body}\n", p.start));
    }
    out
}
