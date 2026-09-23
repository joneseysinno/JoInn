//! Test-only mutant register. These are not alleles of the floor.

mod difference;
mod impostor;
mod leading;
mod max;
mod no_restrict;
mod panic_neg;
mod plus1;
mod refuse;
mod saturating;
mod times;
mod turn_pos_only;
mod two_in;
mod wrapping;
mod wrong_frame;
mod wrong_rat;
mod zero;

pub use difference::Difference;
pub use impostor::Impostor;
pub use leading::Leading;
pub use max::Max;
pub use no_restrict::NoRestrict;
pub use panic_neg::PanicNeg;
pub use plus1::Plus1;
pub use saturating::Saturating;
pub use times::Times;
pub use turn_pos_only::TurnPosOnly;
pub use wrapping::Wrapping;
pub use wrong_frame::WrongFrame;
pub use wrong_rat::WrongRat;
pub use zero::Zero;
