//! No .body file declares a membrane. The membrane is computed.

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn no_body_declares_a_membrane_field() {
        let root = match crate::fns::workspace_root() {
            Ok(r) => r,
            Err(e) => panic!("{e}"),
        };
        let corpus = root.join("corpus");
        let mut hits = Vec::new();
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
                let text = match fs::read_to_string(&path) {
                    Ok(s) => s,
                    Err(e) => panic!("{}: {e}", path.display()),
                };
                if contains_ident(&text, "membrane") {
                    hits.push(path.display().to_string());
                }
            }
        }
        assert!(
            hits.is_empty(),
            "a .body file declares a membrane:\n{}",
            hits.join("\n")
        );
    }

    fn contains_ident(text: &str, name: &str) -> bool {
        let bytes = text.as_bytes();
        let needle = name.as_bytes();
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
