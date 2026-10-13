//! The facts the fold must witness (plan 8 §2.6), each checked for every node
//! and segment of an order, never sampled. Each takes the order's node list,
//! so a broken list shows which facts it breaks.

mod all;
mod t1_ends;
mod t2_steps;
mod t3_nesting;
mod t4_levels;
mod t5_filter;
mod t6_mirror;
mod t7_top_floor;

pub use all::facts;
pub use t5_filter::t5_filter;

/// The facts, in order.
pub const FACT_IDS: [&str; 7] = ["T1", "T2", "T3", "T4", "T5", "T6", "T7"];

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use super::facts;
    use crate::dim::Dim;
    use crate::point::nodes;

    fn order(n: u32) -> Vec<crate::point::Point> {
        match nodes(Dim(2), n) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn orders_1_to_6_hold_t1_to_t7() {
        for n in 1..=6 {
            let (this, below) = (order(n), order(n - 1));
            for (id, v) in facts(n, &this, &below) {
                if let Verdict::Refused(r) = v {
                    panic!("order {n} {id}: {}", r.reason);
                }
            }
        }
    }

    #[test]
    fn the_top_floor_at_order_3() {
        let (this, below) = (order(3), order(2));
        let all = facts(3, &this, &below);
        let Some((_, Verdict::Ok(line))) = all.iter().find(|(id, _)| *id == "T7") else {
            panic!("T7 holds");
        };
        assert_eq!(
            line,
            "top floor (level 2): segments 15, 31, 47; the middle, 31, is the centre"
        );
    }

    #[test]
    fn swapping_two_nodes_breaks_the_steps_and_the_filter() {
        let (mut this, below) = (order(3), order(2));
        this.swap(2, 3);
        let broken: Vec<&str> = facts(3, &this, &below)
            .into_iter()
            .filter(|(_, v)| v.is_refused())
            .map(|(id, _)| id)
            .collect();
        assert!(broken.contains(&"T2"), "{broken:?}");
        assert!(broken.contains(&"T5"), "{broken:?}");
    }
}
