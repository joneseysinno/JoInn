//! Phase 2 item: the hand-written turn register.

pub(crate) fn g2_alleles() -> bool {
    let n = joinn_prim::handwritten_turn_alleles();
    println!("hand-written turn alleles: {n}");
    n == 2
}
