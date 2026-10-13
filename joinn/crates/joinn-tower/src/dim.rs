//! The fold's dimension: a parameter of the API, built at 2 only.

mod check_dim;

pub use check_dim::check_dim;

/// A dimension. Only `Dim(2)` is built.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Dim(pub u32);

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use super::{Dim, check_dim};

    #[test]
    fn two_is_built_and_four_is_refused() {
        assert!(check_dim(Dim(2)).is_ok());
        match check_dim(Dim(4)) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "tower: dimension 4 is not built; acceptance is dimension 2"
            ),
            Verdict::Ok(()) => panic!("dimension 4 was accepted"),
        }
    }
}
