//! A refusal is a value: `Verdict::Refused`, never a Rust `Err` and never a panic.

mod refuse;
#[cfg(test)]
mod testing;

pub use refuse::refuse;
#[cfg(test)]
pub use testing::{admitted, refused};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verdict<T> {
    Admitted(T),
    Refused(Refusal),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    reason: String,
}

impl Refusal {
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

/// Takes an admitted value, or returns the refusal from the enclosing function.
macro_rules! admit {
    ($verdict:expr) => {
        match $verdict {
            $crate::verdict::Verdict::Admitted(value) => value,
            $crate::verdict::Verdict::Refused(refusal) => {
                return $crate::verdict::Verdict::Refused(refusal);
            }
        }
    };
}

pub(crate) use admit;
