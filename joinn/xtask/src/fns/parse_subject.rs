//! Parse a control artifact's text into a Subject by file kind.

use joinn_dna::parse_body;
use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::parse_universe;

use super::parse_lock_scores::parse_lock_scores;
use super::subject::Subject;

/// Parse `text` from repo-relative `path` into a Subject.
/// On refusal, returns the parser's reason (or an unknown-kind reason).
pub(crate) fn parse_subject(path: &str, text: &str) -> Result<Subject, String> {
    if path.ends_with(".body") {
        return match parse_body(text, &FrameRegistry::phase1()) {
            Verdict::Ok(body) => Ok(Subject::Body(body)),
            Verdict::Refused(r) => Err(r.reason),
        };
    }
    if path.ends_with(".universe") {
        return match parse_universe(text) {
            Verdict::Ok(u) => Ok(Subject::Universe(u)),
            Verdict::Refused(r) => Err(r.reason),
        };
    }
    if path.ends_with(".lock") {
        return match parse_lock_scores(text) {
            Ok(rows) => Ok(Subject::Lock(rows)),
            Err(reason) => Err(reason),
        };
    }
    if path.ends_with(".txt") {
        let under_transcripts = path.contains("corpus/transcripts/");
        if under_transcripts {
            let lines = text.lines().map(str::to_owned).collect();
            return Ok(Subject::Transcript(lines));
        }
        return Ok(Subject::Text(text.trim().to_owned()));
    }
    Err(format!("unknown artifact kind for subject parse: {path}"))
}
