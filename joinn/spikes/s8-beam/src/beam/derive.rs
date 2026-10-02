use super::{Derived, Example, Order, balance, closure, complex, walk};
use crate::bridge::Bridge;
use crate::verdict::{Verdict, admit};

/// One example, derived in order: the complex, balance, the walk and its
/// closure. `bridge` is the example's one bridge; its count after these
/// steps is what they read.
pub fn derive(
    example: &Example,
    refined: bool,
    order: Order,
    bridge: &mut Bridge,
) -> Verdict<Derived> {
    let complex = admit!(complex(example, refined));
    let balance = admit!(balance(example));
    let stations = admit!(walk(&complex, &balance, order));
    let end = admit!(closure(example.id(), &stations));
    let reads_before_bridge = bridge.reads();
    Verdict::Admitted(Derived {
        complex,
        balance,
        stations,
        end,
        reads_before_bridge,
    })
}

#[cfg(test)]
mod tests {
    use num_traits::Zero;

    use super::derive;
    use crate::beam::{Order, examples};
    use crate::bridge::{Bridge, Edition};
    use crate::tag::Tag;
    use crate::verdict::{admitted, refused};

    #[test]
    fn closure_is_zero_in_all_five_and_reads_no_bridge() {
        for e in examples() {
            let mut bridge = Bridge::new(Some(Edition::aisc_for_tests()));
            let d = admitted(derive(&e, false, Order::Faithful, &mut bridge));
            assert_eq!(d.end().tag(), Tag::MOMENT);
            assert!(d.end().value().is_zero(), "{} M at end", e.id());
            assert_eq!(d.reads_before_bridge(), 0, "{} reads", e.id());
            assert_eq!(bridge.reads(), 0);
        }
    }

    #[test]
    fn mixed_order_fails_the_closure_check() {
        let e1 = &examples()[0];
        let mut bridge = Bridge::new(None);
        assert_eq!(
            refused(derive(e1, false, Order::MixedOrder, &mut bridge)),
            "balance: E1 M at end 1728/5 kip·ft, want 0; acceptance is one order for every moment"
        );
    }
}
