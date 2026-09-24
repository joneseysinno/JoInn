//! Kind-level neutral edit: one regulatory label, or a lock comment.

use crate::fns::subject::Subject;

use super::reparse::reparse;
use super::reprint::reprint;

/// One regulatory label changed (body/universe), or a comment line (lock).
/// Transcript and text have no neutral edit.
pub(crate) fn neutral(s: &Subject) -> Option<Subject> {
    match s {
        Subject::Body(body) => {
            let mut next = body.clone();
            if let Some((_, label)) = next.regulatory.labels.iter_mut().next() {
                label.push_str(" (neutral)");
            } else if let Some((_, name)) = next.regulatory.names.iter_mut().next() {
                name.push_str(" (neutral)");
            } else {
                return None;
            }
            let subject = Subject::Body(next);
            let text = reprint(&subject);
            match reparse(s, &text) {
                joinn_frame::Verdict::Ok(p) => Some(p),
                joinn_frame::Verdict::Refused(_) => None,
            }
        }
        Subject::Universe(u) => {
            let mut next = u.clone();
            if let Some((_, label)) = next.regulatory.labels.iter_mut().next() {
                label.push_str(" (neutral)");
            } else if let Some((_, name)) = next.regulatory.names.iter_mut().next() {
                name.push_str(" (neutral)");
            } else {
                return None;
            }
            let subject = Subject::Universe(next);
            let text = reprint(&subject);
            match reparse(s, &text) {
                joinn_frame::Verdict::Ok(p) => Some(p),
                joinn_frame::Verdict::Refused(_) => None,
            }
        }
        Subject::Lock(rows) => {
            let body = reprint(s);
            let text = format!("# neutral\n{body}");
            match reparse(s, &text) {
                joinn_frame::Verdict::Ok(p) => {
                    let _ = rows;
                    Some(p)
                }
                joinn_frame::Verdict::Refused(_) => None,
            }
        }
        Subject::Transcript(_) | Subject::Text(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::neutral;
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;

    #[test]
    fn neutral_edits_labels_and_skips_transcript() {
        let u = parse_subject(
            "phase5/universe.universe",
            include_str!("../../../../corpus/phase5/universe.universe"),
        )
        .expect("universe");
        let nu = neutral(&u).expect("universe neutral");
        let Subject::Universe(orig) = &u else {
            panic!("universe");
        };
        let Subject::Universe(next) = &nu else {
            panic!("universe");
        };
        assert_ne!(orig.regulatory.labels, next.regulatory.labels);

        let t = parse_subject(
            "corpus/transcripts/universe.txt",
            include_str!("../../../../corpus/transcripts/universe.txt"),
        )
        .expect("transcript");
        assert!(neutral(&t).is_none());
    }
}
