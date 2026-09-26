//! Fraction-free rank over ℚ. Overflow of an intermediate is a refusal.

use joinn_frame::{CheckId, Refusal, Verdict};

/// Rank of `a` over ℚ by Bareiss elimination. `blocks` is named if `i128` overflows.
pub(super) fn bareiss_rank(mut a: Vec<Vec<i128>>, blocks: u32) -> Verdict<u32> {
    let overflow = || {
        Verdict::Refused(Refusal::structural(
            CheckId::Other,
            format!(
                "complex too large for exact elimination at {blocks} blocks; acceptance is a complex whose elimination fits i128"
            ),
        ))
    };
    if a.is_empty() {
        return Verdict::Ok(0);
    }
    let Some(width) = a.first().map(Vec::len) else {
        return Verdict::Ok(0);
    };
    if width == 0 {
        return Verdict::Ok(0);
    }
    let m = a.len();
    let n = width;
    let mut prev: i128 = 1;
    let mut rank: u32 = 0;
    let mut row = 0_usize;
    for col in 0..n {
        let mut pivot_at = None;
        for r in row..m {
            let Some(entry) = a.get(r).and_then(|line| line.get(col)) else {
                return overflow();
            };
            if *entry != 0 {
                pivot_at = Some(r);
                break;
            }
        }
        let Some(pr) = pivot_at else {
            continue;
        };
        if pr != row {
            a.swap(pr, row);
        }
        let Some(&pivot) = a.get(row).and_then(|line| line.get(col)) else {
            return overflow();
        };
        for r in (row + 1)..m {
            let Some(&down) = a.get(r).and_then(|line| line.get(col)) else {
                return overflow();
            };
            for c in (col + 1)..n {
                let Some(&ij) = a.get(r).and_then(|line| line.get(c)) else {
                    return overflow();
                };
                let Some(&pj) = a.get(row).and_then(|line| line.get(c)) else {
                    return overflow();
                };
                let left = match pivot.checked_mul(ij) {
                    Some(v) => v,
                    None => return overflow(),
                };
                let right = match down.checked_mul(pj) {
                    Some(v) => v,
                    None => return overflow(),
                };
                let Some(num) = left.checked_sub(right) else {
                    return overflow();
                };
                if prev == 0 || num.checked_rem(prev) != Some(0) {
                    return Verdict::Refused(Refusal::structural(
                        CheckId::Other,
                        format!(
                            "elimination at {blocks} blocks was not exact; acceptance is a divisible Bareiss step"
                        ),
                    ));
                }
                let Some(next) = num.checked_div(prev) else {
                    return overflow();
                };
                if let Some(slot) = a.get_mut(r).and_then(|line| line.get_mut(c)) {
                    *slot = next;
                }
            }
            if let Some(slot) = a.get_mut(r).and_then(|line| line.get_mut(col)) {
                *slot = 0;
            }
        }
        prev = pivot;
        rank += 1;
        row += 1;
        if row == m {
            break;
        }
    }
    Verdict::Ok(rank)
}
