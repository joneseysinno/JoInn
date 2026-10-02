use super::Verdict;

pub fn admitted<T>(verdict: Verdict<T>) -> T {
    match verdict {
        Verdict::Admitted(value) => value,
        Verdict::Refused(refusal) => panic!("refused: {}", refusal.reason()),
    }
}

pub fn refused<T>(verdict: Verdict<T>) -> String {
    match verdict {
        Verdict::Admitted(_) => panic!("admitted, but a refusal was wanted"),
        Verdict::Refused(refusal) => refusal.reason().to_string(),
    }
}
