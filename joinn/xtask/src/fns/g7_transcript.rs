//! `joinn run` on a contact subject, under §2.12's script.

use std::env;
use std::fs;
use std::process;

use joinn_dna::Contact;

use super::mutate::reprint;
use super::run_body_bin;
use super::subject::Subject;

/// §2.12's three lines, as typed.
const STDIN: &str = "two\n2\n3\n";

/// The subject is reprinted to a temporary `.contact` (removed after the run),
/// so a mutant runs through the same CLI path as the corpus file.
pub(crate) fn g7_transcript(contact: &Contact) -> Result<String, String> {
    let text = reprint(&Subject::Contact(contact.clone()));
    let path = env::temp_dir().join(format!("joinn-gate7-{}.contact", process::id()));
    fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
    let out = run_body_bin(&path.to_string_lossy(), STDIN.as_bytes());
    let _ = fs::remove_file(&path);
    Ok(out?.replace("\r\n", "\n"))
}
