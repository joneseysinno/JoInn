//! CSR modules never hash.

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn csr_module_does_not_call_hash() {
        let root = match crate::fns::workspace_root() {
            Ok(r) => r,
            Err(e) => panic!("{e}"),
        };
        let dir = root.join("crates").join("joinn-link").join("src").join("csr");
        let mut hits = Vec::new();
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
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let text = match fs::read_to_string(&path) {
                Ok(s) => s,
                Err(e) => panic!("{}: {e}", path.display()),
            };
            for (i, line) in text.lines().enumerate() {
                if line.contains("hash(") {
                    hits.push(format!("{}:{}: {line}", path.display(), i + 1));
                }
            }
        }
        let root_csr = root.join("crates").join("joinn-link").join("src").join("csr.rs");
        let text = match fs::read_to_string(&root_csr) {
            Ok(s) => s,
            Err(e) => panic!("{}: {e}", root_csr.display()),
        };
        for (i, line) in text.lines().enumerate() {
            if line.contains("hash(") {
                hits.push(format!("{}:{}: {line}", root_csr.display(), i + 1));
            }
        }
        assert!(hits.is_empty(), "CSR must not hash:\n{}", hits.join("\n"));
    }
}
