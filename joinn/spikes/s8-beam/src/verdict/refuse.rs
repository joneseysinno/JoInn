use super::{Refusal, Verdict};

pub fn refuse<T>(reason: String) -> Verdict<T> {
    Verdict::Refused(Refusal { reason })
}
