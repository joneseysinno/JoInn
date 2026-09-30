//! Every corpus contact as a scene, with its CPU pick at each standard viewport.

use joinn_frame::Verdict;
use joinn_visual::{STANDARD_VIEWPORTS, Scene, cpu_pick, fit, shapes_of_tables};

use super::ContactSubject;
use crate::fns::layout::corpus_contacts;

/// The label is the file name (`calculator.contact`). A contact that is refused
/// or doesn't grow fails `pick`: every corpus contact is admitted (P7-07).
pub(crate) fn contact_subjects() -> Result<Vec<ContactSubject>, String> {
    let (contacts, cells) = corpus_contacts()?;
    let mut out = Vec::new();
    for (rel, parsed) in contacts {
        let grown = parsed.and_then(|c| Scene::grow_contact("body", &c, &cells));
        let scene = match grown {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => return Err(format!("pick: {rel}: {}", r.reason)),
        };
        let mut picks = Vec::new();
        for (w, h) in STANDARD_VIEWPORTS {
            let camera = fit(scene.layout(), w, h);
            match cpu_pick(&shapes_of_tables(scene.tables()), &camera) {
                Verdict::Ok(p) => picks.push((camera, p)),
                Verdict::Refused(r) => return Err(format!("pick: {rel}: {}", r.reason)),
            }
        }
        let label = rel.rsplit('/').next().unwrap_or(&rel).to_owned();
        out.push((label, scene, picks));
    }
    Ok(out)
}
