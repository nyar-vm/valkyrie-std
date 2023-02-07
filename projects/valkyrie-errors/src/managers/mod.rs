use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct TextManager {
    // workspace root
    root: PathBuf,
    map: BTreeMap<String, String>,
}

impl TextManager {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self { root: root.as_ref().canonicalize().unwrap(), map: BTreeMap::new() }
    }
    pub fn add_file(&mut self, relative_path: String) {
        let file = self.root.join(&relative_path);
        let text = std::fs::read_to_string(file).unwrap();
        self.map.insert(relative_path, text);
    }
    pub fn add_text(&mut self, file: impl Into<String>, text: impl Into<String>) {
        self.map.insert(file.into(), text.into());
    }
    pub fn get_text(&self, file: &str) -> Option<&str> {
        self.map.get(&file.into()).map(|s| s.as_str())
    }
}
