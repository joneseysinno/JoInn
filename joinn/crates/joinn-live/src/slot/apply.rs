//! The only function that mutates a port slot (V35).

use joinn_dna::JoinPolicy;
use joinn_frame::{CheckId, Refusal, Subject, Verdict};

use crate::activation::Instance;
use crate::slot::{Slot, SlotWrite};

/// The only function that mutates a port slot (V35).
pub(crate) fn apply_slot(
    inst: &mut Instance,
    port: u32,
    write: SlotWrite,
    policy: JoinPolicy,
    dest: &str,
    seed: u64,
) -> Verdict<()> {
    match write {
        SlotWrite::Consume => {
            match inst.slots.get_mut(&port) {
                Some(slot @ Slot::Filled(_)) => {
                    *slot = Slot::Empty;
                }
                Some(Slot::Queue(q)) if !q.is_empty() => {
                    q.remove(0);
                }
                _ => {}
            }
            Verdict::Ok(())
        }
        SlotWrite::Fill(value) => match inst.slots.get_mut(&port) {
            Some(Slot::Filled(_)) if matches!(policy, JoinPolicy::Refuse) => {
                Verdict::Refused(Refusal {
                    check: CheckId::Join,
                    subject: Subject::Other(dest.into()),
                    reason: format!("join refuse at {dest} port {port}"),
                    counterexample: None,
                    seed,
                })
            }
            Some(slot @ Slot::Filled(_)) if matches!(policy, JoinPolicy::Latest) => {
                *slot = Slot::Filled(value);
                Verdict::Ok(())
            }
            Some(Slot::Queue(q)) => {
                q.push(value);
                Verdict::Ok(())
            }
            Some(slot @ Slot::Empty) => {
                *slot = match policy {
                    JoinPolicy::Queue => Slot::Queue(vec![value]),
                    _ => Slot::Filled(value),
                };
                Verdict::Ok(())
            }
            Some(Slot::Filled(_)) => Verdict::Ok(()),
            None => {
                inst.slots.insert(port, Slot::Filled(value));
                Verdict::Ok(())
            }
        },
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn deliver_is_the_only_slot_writer() {
        let manifest = env!("CARGO_MANIFEST_DIR");
        let src_dir = std::path::Path::new(manifest).join("src");
        let mut writers = Vec::new();
        let mut saw_apply = false;
        fn walk(dir: &std::path::Path, writers: &mut Vec<String>, saw_apply: &mut bool) {
            let Ok(entries) = std::fs::read_dir(dir) else {
                return;
            };
            let mut ents: Vec<_> = entries.flatten().collect();
            ents.sort_by_key(|e| e.file_name());
            for entry in ents {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, writers, saw_apply);
                    continue;
                }
                if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                    continue;
                }
                let Ok(text) = std::fs::read_to_string(&path) else {
                    continue;
                };
                if text.contains("fn apply_slot") {
                    *saw_apply = true;
                }
                let in_apply = path.file_name().and_then(|n| n.to_str()) == Some("apply.rs");
                if in_apply {
                    continue;
                }
                for (i, line) in text.lines().enumerate() {
                    let is_fill_insert = line.contains("slots.insert") && line.contains("Filled");
                    let is_star_assign = line.contains("Slot::Filled") && line.contains('*');
                    if is_fill_insert || is_star_assign {
                        writers.push(format!("{}:{}", path.display(), i + 1));
                    }
                }
            }
        }
        walk(&src_dir, &mut writers, &mut saw_apply);
        assert!(
            saw_apply,
            "V35: apply_slot must exist as the only slot writer"
        );
        assert!(
            writers.is_empty(),
            "V35: slot write outside apply_slot at {:?}",
            writers
        );
    }
}
