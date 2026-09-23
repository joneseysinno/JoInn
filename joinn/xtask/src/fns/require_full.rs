//! A gate command succeeds only when every item passed.

pub(crate) fn require_full(score: (u32, u32)) -> Result<(), String> {
    let (n, total) = score;
    if n == total {
        Ok(())
    } else {
        Err(format!("{n}/{total}"))
    }
}
