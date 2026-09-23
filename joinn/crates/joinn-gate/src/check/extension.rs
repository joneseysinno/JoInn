//! Check 4: conservative extension and the six cross-frame obligations of §2.5.

mod check_cross_frame;
mod check_parent;
mod named;

pub use check_cross_frame::check_cross_frame;
pub use check_parent::check_parent;
