//! What `cargo xtask strand <file>` prints.

use joinn_codex::{Codon, side};
use joinn_strand::{Strand, print_letters};

use super::read_strand_file;

/// The strand and its partner with their hashes, each one's side from its
/// head codon, and the partner read from its tail, where START reads even.
pub(crate) fn strand_text(path: &str) -> Result<String, String> {
    let file = read_strand_file(path)?;
    let s = &file.strand;
    let p = s.partner();
    let tail_first = Strand::new(p.letters().iter().rev().copied().collect());
    let side_line = |x: &Strand| match (side(x), x.letters()) {
        (Some(sd), [a, b, c, ..]) => {
            let head = Codon::from_letters([*a, *b, *c]);
            format!(
                "{} (head {} {} {})",
                sd.name(),
                head.text(),
                head.bit_text(),
                head.parity().name()
            )
        }
        _ => "none (shorter than a codon)".to_owned(),
    };
    let mut out = format!("file      {path}\ncodex {}\n", file.codex);
    out.push_str(&format!("strand    {}\n", print_letters(s)));
    out.push_str(&format!("hash      {}\n", s.identity().to_hex()));
    out.push_str(&format!("partner   {}\n", print_letters(&p)));
    out.push_str(&format!("hash      {}\n", p.identity().to_hex()));
    out.push_str(&format!("side      {}\n", side_line(s)));
    out.push_str(&format!("partner side {}\n", side_line(&p)));
    out.push_str(&format!(
        "partner from its tail {}: side {}\n",
        print_letters(&tail_first),
        side_line(&tail_first)
    ));
    out.push_str(&format!(
        "strand: {} letters, {} codons, side {}\n",
        s.len(),
        s.len() / 3,
        side(s).map_or("none", |sd| sd.name())
    ));
    Ok(out)
}
