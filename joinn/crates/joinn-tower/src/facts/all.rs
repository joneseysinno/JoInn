//! Every fact for one order.

use joinn_frame::Verdict;

use super::{
    t1_ends::t1_ends, t2_steps::t2_steps, t3_nesting::t3_nesting, t4_levels::t4_levels,
    t5_filter::t5_filter, t6_mirror::t6_mirror, t7_top_floor::t7_top_floor,
};
use crate::point::Point;

/// T1–T7 for order `order` with node list `nodes`; `below` is order
/// `order − 1`'s nodes. Each is its line when it holds, its refusal when not.
pub fn facts(order: u32, nodes: &[Point], below: &[Point]) -> Vec<(&'static str, Verdict<String>)> {
    vec![
        ("T1", t1_ends(order, nodes)),
        ("T2", t2_steps(order, nodes)),
        ("T3", t3_nesting(order, nodes, below)),
        ("T4", t4_levels(order)),
        ("T5", t5_filter(order, nodes, below, 1)),
        ("T6", t6_mirror(order, nodes)),
        ("T7", t7_top_floor(order)),
    ]
}
