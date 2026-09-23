//! Turn admission: generated round-trip law, never a solver.

mod add_int_turn0;
mod add_int_turn1;
mod admit_add_turn0;
mod admit_turn;
mod admit_turn_on;
mod admit_turn_on_frame;
#[cfg(any(test, feature = "mutants"))]
mod admit_turn_positive_only_is_refused;
mod five_and_three_at_turn;
mod handwritten_turn_alleles;
mod refuse;
mod turn_register;

pub use add_int_turn0::AddIntTurn0;
pub use add_int_turn1::AddIntTurn1;
pub use admit_add_turn0::admit_add_turn0;
pub use admit_turn::admit_turn;
pub use admit_turn_on::admit_turn_on;
pub use admit_turn_on_frame::admit_turn_on_frame;
#[cfg(any(test, feature = "mutants"))]
pub use admit_turn_positive_only_is_refused::admit_turn_positive_only_is_refused;
pub use five_and_three_at_turn::five_and_three_at_turn;
pub use handwritten_turn_alleles::handwritten_turn_alleles;
pub use turn_register::turn_register;
