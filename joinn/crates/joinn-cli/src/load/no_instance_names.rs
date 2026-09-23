//! Production sources name no instance from a corpus body.

#[cfg(test)]
mod tests {
    use joinn_dna::parse_body;
    use joinn_frame::{FrameRegistry, Verdict};
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::Path;

    const KEYWORDS: &[&str] = &[
        "self", "super", "crate", "type", "for", "where", "move", "ref", "mut", "pub", "mod",
        "use", "fn", "impl", "trait", "struct", "enum", "const", "static", "loop", "while",
        "match", "return", "true", "false", "let", "as", "in", "if", "else",
    ];

    #[test]
    fn cli_source_names_no_corpus_instance() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let corpus = root.join("..").join("..").join("corpus");
        let mut names = BTreeSet::new();
        let mut dirs = vec![corpus];
        while let Some(dir) = dirs.pop() {
            let rd = match fs::read_dir(&dir) {
                Ok(rd) => rd,
                Err(e) => panic!("{}: {e}", dir.display()),
            };
            for ent in rd {
                let ent = match ent {
                    Ok(e) => e,
                    Err(e) => panic!("{e}"),
                };
                let path = ent.path();
                if path.is_dir() {
                    dirs.push(path);
                    continue;
                }
                if path.extension().and_then(|e| e.to_str()) != Some("body") {
                    continue;
                }
                let src = match fs::read_to_string(&path) {
                    Ok(s) => s,
                    Err(e) => panic!("{}: {e}", path.display()),
                };
                let body = match parse_body(&src, &FrameRegistry::phase1()) {
                    Verdict::Ok(b) => b,
                    Verdict::Refused(_) => continue,
                };
                for entry in body.coding.genome {
                    for instance in entry.instances {
                        if instance.len() >= 3 && !KEYWORDS.contains(&instance.as_str()) {
                            names.insert(instance);
                        }
                    }
                }
            }
        }
        assert!(names.contains("cli_a"), "the scan's domain must include cli_a");

        let src_root = root.join("src");
        let mut files = Vec::new();
        let mut stack = vec![src_root];
        while let Some(dir) = stack.pop() {
            let rd = match fs::read_dir(&dir) {
                Ok(rd) => rd,
                Err(e) => panic!("{}: {e}", dir.display()),
            };
            for ent in rd {
                let ent = match ent {
                    Ok(e) => e,
                    Err(e) => panic!("{e}"),
                };
                let path = ent.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    files.push(path);
                }
            }
        }

        let mut hits = Vec::new();
        for path in &files {
            let text = match fs::read_to_string(path) {
                Ok(s) => s,
                Err(e) => panic!("{}: {e}", path.display()),
            };
            let production = strip_cfg_test(&text);
            for name in &names {
                if contains_ident(&production, name) {
                    hits.push(format!("{}: {name}", path.display()));
                }
            }
        }
        assert!(
            hits.is_empty(),
            "instance names in joinn-cli production source:\n{}",
            hits.join("\n")
        );
    }

    fn strip_cfg_test(text: &str) -> String {
        let mut out = String::new();
        let mut depth = 0i32;
        let mut skipping = false;
        let mut skip_depth = 0i32;
        let mut pending = false;
        for line in text.lines() {
            let trimmed = line.trim_start();
            if !skipping && trimmed.starts_with("#[cfg(test)]") {
                pending = true;
                continue;
            }
            let opens = line.chars().filter(|&c| c == '{').count() as i32;
            let closes = line.chars().filter(|&c| c == '}').count() as i32;
            if pending && opens > 0 {
                skipping = true;
                skip_depth = depth + opens - closes;
                pending = false;
                depth += opens - closes;
                continue;
            }
            if skipping {
                depth += opens - closes;
                if depth < skip_depth {
                    skipping = false;
                }
                continue;
            }
            depth += opens - closes;
            out.push_str(line);
            out.push('\n');
        }
        out
    }

    fn contains_ident(text: &str, name: &str) -> bool {
        let bytes = text.as_bytes();
        let needle = name.as_bytes();
        let mut i = 0;
        while i + needle.len() <= bytes.len() {
            if &bytes[i..i + needle.len()] == needle {
                let before = i == 0 || !is_ident_byte(bytes[i - 1]);
                let after = i + needle.len();
                let after_ok = after == bytes.len() || !is_ident_byte(bytes[after]);
                if before && after_ok {
                    return true;
                }
            }
            i += 1;
        }
        false
    }

    fn is_ident_byte(b: u8) -> bool {
        b.is_ascii_alphanumeric() || b == b'_'
    }
}
