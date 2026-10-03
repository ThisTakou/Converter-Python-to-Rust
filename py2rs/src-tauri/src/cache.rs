use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Clone)]
pub struct CacheEntry {
    pub python_hash: String,
    pub rust_code: String,
    pub checked: bool,
    pub timestamp: u64,
}

pub struct Cache {
    path: PathBuf,
    entries: HashMap<String, CacheEntry>,
}

impl Cache {
    pub fn new(project_root: &Path) -> Self {
        let path = PathBuf::from(format!("{}_rs/.cache.json", project_root.display()));
        let entries = if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            HashMap::new()
        };
        Cache { path, entries }
    }

    pub fn get(&self, file: &str, python_code: &str) -> Option<&CacheEntry> {
        let hash = format!("{:x}", md5::compute(python_code));
        self.entries.get(file).filter(|e| e.python_hash == hash)
    }

    pub fn set(&mut self, file: String, python_code: &str, rust_code: String, checked: bool) {
        let hash = format!("{:x}", md5::compute(python_code));
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.entries.insert(
            file,
            CacheEntry {
                python_hash: hash,
                rust_code,
                checked,
                timestamp,
            },
        );
    }

    pub fn save(&self) -> anyhow::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.path, serde_json::to_string_pretty(&self.entries)?)?;
        Ok(())
    }
}
