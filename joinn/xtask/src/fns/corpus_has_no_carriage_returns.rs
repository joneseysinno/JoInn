//! Corpus files are LF. A carriage return fails the build naming the file.

#[cfg(test)]
mod tests {
    use crate::fns::workspace_root;
    use std::fs;
    use std::path::Path;

    fn first_carriage_return(dir: &Path) -> Result<Option<String>, String> {
        let entries = fs::read_dir(dir).map_err(|e| e.to_string())?;
        let mut paths = Vec::new();
        for ent in entries {
            paths.push(ent.map_err(|e| e.to_string())?.path());
        }
        paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
        for path in paths {
            if path.is_dir() {
                if let Some(hit) = first_carriage_return(&path)? {
                    return Ok(Some(hit));
                }
            } else {
                let bytes = fs::read(&path).map_err(|e| e.to_string())?;
                if bytes.contains(&b'\r') {
                    return Ok(Some(path.display().to_string()));
                }
            }
        }
        Ok(None)
    }

    #[test]
    fn corpus_has_no_carriage_returns() {
        let root = workspace_root().unwrap_or_else(|e| panic!("{e}"));
        let corpus = root.join("corpus");
        match first_carriage_return(&corpus) {
            Ok(Some(path)) => panic!("{path} contains a carriage return"),
            Ok(None) => {}
            Err(e) => panic!("{e}"),
        }
    }
}
