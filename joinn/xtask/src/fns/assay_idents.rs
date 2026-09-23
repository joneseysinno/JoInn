//! Scans for assay types in the gate, and for homology words in the assay.

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn assay_depends_on_frame_alone() {
        let root = match crate::fns::workspace_root() {
            Ok(r) => r,
            Err(e) => panic!("{e}"),
        };
        let toml = match fs::read_to_string(root.join("crates").join("joinn-assay").join("Cargo.toml"))
        {
            Ok(s) => s,
            Err(e) => panic!("{e}"),
        };
        let names = dependency_names(&toml);
        assert_eq!(names, vec!["joinn-frame".to_string()]);
    }

    #[test]
    fn assay_names_no_homology() {
        let root = match crate::fns::workspace_root() {
            Ok(r) => r,
            Err(e) => panic!("{e}"),
        };
        let dir = root.join("crates").join("joinn-assay");
        let banned = ["homology", "betti", "rank", "H"];
        let mut hits = Vec::new();
        for (path, text) in rs_files(&dir) {
            for word in banned {
                if contains_ident(&text, word) {
                    hits.push(format!("{}: {word}", path.display()));
                }
            }
        }
        assert!(hits.is_empty(), "{}", hits.join("\n"));
    }

    #[test]
    fn gate_names_no_assay_type() {
        let root = match crate::fns::workspace_root() {
            Ok(r) => r,
            Err(e) => panic!("{e}"),
        };
        let dir = root.join("crates").join("joinn-gate");
        let banned = ["joinn_assay", "joinn-assay", "BlockId", "Complex"];
        let mut hits = Vec::new();
        for (path, text) in rs_files(&dir) {
            for word in banned {
                if word.contains('-') {
                    if text.contains(word) {
                        hits.push(format!("{}: {word}", path.display()));
                    }
                } else if contains_ident(&text, word) {
                    hits.push(format!("{}: {word}", path.display()));
                }
            }
        }
        assert!(hits.is_empty(), "{}", hits.join("\n"));
    }

    fn dependency_names(toml: &str) -> Vec<String> {
        let mut names = Vec::new();
        let mut in_deps = false;
        for line in toml.lines() {
            let t = line.trim();
            if t.starts_with('[') {
                in_deps = t == "[dependencies]";
                continue;
            }
            if !in_deps || t.is_empty() || t.starts_with('#') {
                continue;
            }
            if let Some((name, _)) = t.split_once('=') {
                names.push(name.trim().to_string());
            }
        }
        names
    }

    fn rs_files(dir: &Path) -> Vec<(std::path::PathBuf, String)> {
        let mut out = Vec::new();
        let mut dirs = vec![dir.to_path_buf()];
        while let Some(d) = dirs.pop() {
            let rd = match fs::read_dir(&d) {
                Ok(rd) => rd,
                Err(e) => panic!("{}: {e}", d.display()),
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
                if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                    continue;
                }
                let text = match fs::read_to_string(&path) {
                    Ok(s) => s,
                    Err(e) => panic!("{}: {e}", path.display()),
                };
                out.push((path, text));
            }
        }
        out
    }

    fn contains_ident(text: &str, name: &str) -> bool {
        let bytes = text.as_bytes();
        let needle = name.as_bytes();
        if needle.is_empty() {
            return false;
        }
        let mut i = 0;
        while i + needle.len() <= bytes.len() {
            if &bytes[i..i + needle.len()] == needle {
                let before = i == 0 || !is_ident(bytes[i - 1]);
                let after = i + needle.len();
                let after_ok = after == bytes.len() || !is_ident(bytes[after]);
                if before && after_ok {
                    return true;
                }
            }
            i += 1;
        }
        false
    }

    fn is_ident(b: u8) -> bool {
        b.is_ascii_alphanumeric() || b == b'_'
    }
}
