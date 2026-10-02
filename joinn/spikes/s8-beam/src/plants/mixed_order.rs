use crate::beam::{Derived, Example, Order, derive};
use crate::bridge::Bridge;
use crate::verdict::Verdict;

/// E1 with the load's moment taken F ∧ r in the walk while the reactions stay
/// r ∧ F. The closure check must refuse it.
pub fn mixed_order(e1: &Example) -> Verdict<Derived> {
    derive(e1, false, Order::MixedOrder, &mut Bridge::new(None))
}

#[cfg(test)]
mod tests {
    use super::mixed_order;
    use crate::beam::examples;
    use crate::plants::line;

    #[test]
    fn the_mixed_order_plant_is_refused_with_the_plan_line() {
        let (text, ok) = line("mixed order", mixed_order(&examples()[0]));
        assert_eq!(
            text,
            "plant mixed order: refused (ok): balance: E1 M at end 1728/5 kip·ft, want 0; acceptance is one order for every moment"
        );
        assert!(ok);
    }
}
