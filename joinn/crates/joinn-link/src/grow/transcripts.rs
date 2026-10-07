//! The fixed witness transcripts of plan 7.4 §3.

use joinn_dna::Accept;

/// A counting body's transcripts; an adding body's are its own, then every
/// counting transcript but the empty one it already has. Written here, seeded
/// nowhere.
pub fn transcripts(accepts: Accept) -> Vec<Vec<i64>> {
    let counting = vec![vec![], vec![1, 1, 1], vec![1, 1, 3], vec![1; 12]];
    match accepts {
        Accept::One => counting,
        Accept::Any => {
            let mut out = vec![vec![], vec![2, 3, 4], vec![5, -2, 0, 7]];
            out.extend(counting.into_iter().filter(|t| !t.is_empty()));
            out
        }
    }
}
