//! `joinn run corpus/<rel>` under the script against the stem's transcript.

use std::fs;
use std::path::Path;

use super::SCRIPT;
use crate::fns::run_body_bin;

/// `transcript equal` when the CLI prints `transcripts/<stem>.txt` byte for byte.
pub(crate) fn transcript_line(corpus: &Path, rel: &str, stem: &str) -> Result<String, String> {
    let golden = corpus.join("transcripts").join(format!("{stem}.txt"));
    let want = fs::read_to_string(&golden)
        .map_err(|e| format!("transcript: {}: {e}", golden.display()))?
        .replace("\r\n", "\n");
    let mut stdin = String::new();
    for (_, _, line) in SCRIPT {
        stdin.push_str(line);
        stdin.push('\n');
    }
    let got = run_body_bin(&format!("corpus/{rel}"), stdin.as_bytes())?.replace("\r\n", "\n");
    if got == want {
        return Ok("transcript equal".into());
    }
    let first = got
        .lines()
        .zip(want.lines())
        .position(|(g, w)| g != w)
        .map_or_else(|| "length".to_owned(), |i| format!("line {}", i + 1));
    Err(format!(
        "transcript differs from transcripts/{stem}.txt at {first}:\n{got}"
    ))
}
