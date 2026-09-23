//! joinn-host production source has no render function and no IO.

use std::fs;
use std::path::{Path, PathBuf};

fn walk_rs(dir: &Path, hits: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_rs(&path, hits);
            continue;
        }
        if path.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let render = concat!("fn ", "render");
        for (i, line) in text.lines().enumerate() {
            if line.contains(render) {
                hits.push(format!("{}:{}: {render}", path.display(), i + 1));
            }
        }
    }
}

#[test]
fn no_string_returning_render_function() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut hits = Vec::new();
    walk_rs(&src, &mut hits);
    assert!(
        hits.is_empty(),
        "joinn-host must not contain a render function; got {hits:?}"
    );
}
