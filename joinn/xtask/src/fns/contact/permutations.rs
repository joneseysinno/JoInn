//! Every order of `0 … k−1`, in lexicographic order.

pub(super) fn permutations(k: u32) -> Vec<Vec<u32>> {
    let mut out: Vec<Vec<u32>> = vec![Vec::new()];
    for _ in 0..k {
        let mut next = Vec::new();
        for prefix in &out {
            for i in 0..k {
                if !prefix.contains(&i) {
                    let mut p = prefix.clone();
                    p.push(i);
                    next.push(p);
                }
            }
        }
        out = next;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::permutations;

    #[test]
    fn orders_are_all_there_once_each() {
        assert_eq!(permutations(0), vec![Vec::<u32>::new()]);
        assert_eq!(permutations(2), vec![vec![0, 1], vec![1, 0]]);
        let three = permutations(3);
        assert_eq!(three.len(), 6);
        assert_eq!(three[0], [0, 1, 2]);
        assert_eq!(three[5], [2, 1, 0]);
    }
}
