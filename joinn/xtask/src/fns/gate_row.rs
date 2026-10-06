//! One row of a gate's output, as the whole gate and `--item` both print it.

/// `<n> ok  <name>` or `<n> fail  <name>`.
pub(crate) fn gate_row(n: usize, name: &str, ok: bool) -> String {
    let word = if ok { "ok" } else { "fail" };
    format!("{n} {word}  {name}")
}
