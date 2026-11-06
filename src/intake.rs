use serde_json::{Value, json};
use std::{fs, path::Path};

// Explicit roots only. No symlinks, hidden directories or generated trees.
fn scan(dir: &Path, root: &Path, out: &mut Vec<Value>, warnings: &mut Vec<String>, depth: usize) {
    if depth > 12 || out.len() >= 500 {
        warnings.push(format!("Scan limit reached: {}", dir.display()));
        return;
    }
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            warnings.push(e.to_string());
            return;
        }
    };
    for entry in entries.flatten() {
        if out.len() >= 500 {
            break;
        }
        let p = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') || ["node_modules", "target", "vendor"].contains(&name.as_str()) {
            continue;
        }
        let Ok(meta) = fs::symlink_metadata(&p) else {
            continue;
        };
        if meta.file_type().is_symlink() {
            continue;
        }
        if meta.is_dir() {
            scan(&p, root, out, warnings, depth + 1);
            continue;
        }
        if !matches!(
            p.extension()
                .and_then(|e| e.to_str())
                .map(str::to_lowercase)
                .as_deref(),
            Some("md" | "markdown" | "txt")
        ) {
            continue;
        }
        if meta.len() > 1_000_000 {
            warnings.push(format!("Too large: {}", p.display()));
            continue;
        }
        match fs::read_to_string(&p) {
            Ok(body) => {
                let title = body
                    .lines()
                    .find_map(|l| l.strip_prefix("# "))
                    .unwrap_or(p.file_stem().unwrap().to_str().unwrap_or("Untitled"))
                    .trim();
                out.push(json!({"path":p.strip_prefix(root).unwrap().to_string_lossy(),"title":title,"bytes":meta.len(),"preview":body.chars().take(400).collect::<String>()}));
            }
            Err(e) => warnings.push(format!("{}: {e}", p.display())),
        }
    }
}
pub fn run(input: &Value, import: bool, store: &Path) -> Result<Value, String> {
    let root = fs::canonicalize(input["root"].as_str().ok_or("Folder required")?)
        .map_err(|e| e.to_string())?;
    if !root.is_dir() {
        return Err("Root must be a directory".into());
    }
    let mut files = vec![];
    let mut warnings = vec![];
    scan(&root, &root, &mut files, &mut warnings, 0);
    files.sort_by(|a, b| a["path"].as_str().cmp(&b["path"].as_str()));
    let mut notes: Vec<Value> =
        serde_json::from_slice(&fs::read(store).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    for f in &mut files {
        let source = root.join(f["path"].as_str().unwrap());
        f["imported"] = json!(
            notes
                .iter()
                .any(|n| n["source_path"].as_str() == source.to_str())
        );
    }
    if !import {
        return Ok(json!({"root":root,"files":files,"warnings":warnings,"limit":500}));
    }
    let selected = input["paths"]
        .as_array()
        .ok_or("Select paths from discovery")?;
    if selected.len() > 500 {
        return Err("Maximum 500 files per intake".into());
    }
    let mut imported = vec![];
    let mut skipped = vec![];
    for rel in selected {
        let Some(f) = files.iter().find(|f| f["path"] == *rel) else {
            skipped.push(json!({"path":rel,"reason":"Not a discoverable file"}));
            continue;
        };
        let source = root.join(f["path"].as_str().unwrap());
        if notes
            .iter()
            .any(|n| n["source_path"].as_str() == source.to_str())
        {
            skipped.push(json!({"path":rel,"reason":"Already imported; existing note preserved"}));
            continue;
        }
        let canonical = fs::canonicalize(&source).map_err(|e| e.to_string())?;
        if !canonical.starts_with(&root) || canonical != source {
            return Err("Source changed during intake".into());
        }
        let body = fs::read_to_string(&source).map_err(|e| e.to_string())?;
        if body.len() > 1_000_000 {
            return Err("Source exceeds size limit".into());
        }
        let id = notes
            .iter()
            .filter_map(|n| n["id"].as_u64())
            .max()
            .unwrap_or(0)
            + 1;
        notes.push(json!({"id":id,"title":f["title"],"body":body,"tags":["imported"],"links":[],"source_path":source,"imported_at":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()}));
        imported.push(json!({"path":rel,"id":id}));
    }
    let tmp = store.with_extension("tmp");
    fs::write(&tmp, serde_json::to_vec_pretty(&notes).unwrap()).map_err(|e| e.to_string())?;
    fs::rename(tmp, store).map_err(|e| e.to_string())?;
    Ok(json!({"imported":imported,"skipped":skipped,"warnings":warnings}))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn discovery_import_is_idempotent_and_scoped() {
        let root = std::env::temp_dir().join(format!("viki-intake-{}", std::process::id()));
        fs::create_dir_all(root.join(".hidden")).unwrap();
        fs::write(root.join("a.md"), "# Research\n\n[[Experiment]]").unwrap();
        fs::write(root.join(".hidden/secret.md"), "hidden").unwrap();
        let store = root.join("notes.json");
        fs::write(&store, "[]").unwrap();
        let input = json!({"root":root,"paths":["a.md","../escape.md"]});
        assert_eq!(
            run(&input, false, &store).unwrap()["files"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            run(&input, true, &store).unwrap()["imported"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            run(&input, true, &store).unwrap()["imported"]
                .as_array()
                .unwrap()
                .len(),
            0
        );
        let notes: Value = serde_json::from_slice(&fs::read(&store).unwrap()).unwrap();
        assert_eq!(notes[0]["body"], "# Research\n\n[[Experiment]]");
        fs::remove_dir_all(root).unwrap();
    }
}
