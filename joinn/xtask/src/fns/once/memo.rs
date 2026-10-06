//! Run pure work once per process, and print its lines on every call.

use std::sync::OnceLock;

use super::{SINK, say};

/// The first call runs `work` with its `say` lines captured; every call
/// (the first included) prints those lines and returns the stored value.
pub(crate) fn memo<T: Clone>(cell: &OnceLock<(T, Vec<String>)>, work: impl FnOnce() -> T) -> T {
    let (out, lines) = cell.get_or_init(|| {
        SINK.with(|sink| sink.borrow_mut().push(Vec::new()));
        let out = work();
        let lines = SINK
            .with(|sink| sink.borrow_mut().pop())
            .unwrap_or_default();
        (out, lines)
    });
    for line in lines {
        say(line);
    }
    out.clone()
}

#[cfg(test)]
mod tests {
    use std::sync::OnceLock;
    use std::sync::atomic::{AtomicU32, Ordering};

    use super::memo;
    use crate::fns::once::say;

    #[test]
    fn work_runs_once_and_every_call_prints_its_lines() {
        static INNER: OnceLock<(u32, Vec<String>)> = OnceLock::new();
        static OUTER: OnceLock<(u32, Vec<String>)> = OnceLock::new();
        let runs = AtomicU32::new(0);
        let sum = memo(&OUTER, || {
            let first = memo(&INNER, || {
                runs.fetch_add(1, Ordering::SeqCst);
                say("one");
                say("two");
                7
            });
            let again = memo(&INNER, || {
                runs.fetch_add(1, Ordering::SeqCst);
                9
            });
            first + again
        });
        assert_eq!(sum, 14);
        assert_eq!(runs.load(Ordering::SeqCst), 1);
        assert_eq!(
            OUTER.get().map(|(_, lines)| lines.clone()),
            Some(vec![
                "one".to_owned(),
                "two".to_owned(),
                "one".to_owned(),
                "two".to_owned()
            ])
        );
    }
}
