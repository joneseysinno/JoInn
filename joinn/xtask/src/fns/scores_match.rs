//! The lock gate all wrote must equal the scores gate all returned.

use super::parse_lock_scores::LockRow;

pub(crate) fn scores_match(
    returned: &[(&str, bool, u32, u32)],
    rows: &[LockRow],
) -> Result<(), String> {
    for (phase, _ok, n, total) in returned {
        let Some(row) = rows.iter().find(|row| row.phase == *phase) else {
            return Err(format!("{phase}: lock has no row for the score just returned"));
        };
        if row.n != *n || row.total != *total {
            return Err(format!(
                "{phase}: lock recorded {}/{} and the gate returned {}/{}",
                row.n, row.total, n, total
            ));
        }
    }
    Ok(())
}
