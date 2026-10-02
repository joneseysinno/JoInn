use crate::beam::{Example, Order, balance, complex, refinement, walk};
use crate::verdict::{Verdict, admit};

/// E1 walked with the load's moment term omitted, on the 2-line and the
/// refined complex. Refinement must refuse it.
pub fn rectangle(e1: &Example) -> Verdict<usize> {
    let balance = admit!(balance(e1));
    let coarse = admit!(walk(
        &admit!(complex(e1, false)),
        &balance,
        Order::Rectangle
    ));
    let fine = admit!(walk(&admit!(complex(e1, true)), &balance, Order::Rectangle));
    refinement(e1.id(), &coarse, &fine)
}

#[cfg(test)]
mod tests {
    use super::rectangle;
    use crate::beam::examples;
    use crate::plants::line;

    #[test]
    fn the_rectangle_plant_is_refused_with_the_plan_line() {
        let (text, ok) = line("rectangle rule", rectangle(&examples()[0]));
        assert_eq!(
            text,
            "plant rectangle rule: refused (ok): refinement: E1 M(12 ft) 864/5 kip·ft with 2 lines, 468/5 kip·ft with 24 lines; acceptance is a derivation the complex cannot change"
        );
        assert!(ok);
    }
}
