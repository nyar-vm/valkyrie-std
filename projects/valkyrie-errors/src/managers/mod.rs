use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Arc,
};

pub type FileID = usize;

#[derive(Debug)]
pub struct TextManager {
    // workspace root
    root: PathBuf,
    max_id: FileID,
    text_map: BTreeMap<FileID, Arc<String>>,
    file_map: BTreeMap<String, FileID>,
}

pub struct FileSpan {
    file: usize,
    head: usize,
    tail: usize,
}

impl TextManager {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self {
            root: root.as_ref().canonicalize().unwrap(),
            max_id: 0,
            text_map: BTreeMap::default(),
            file_map: BTreeMap::default(),
        }
    }
    pub fn add_file(&mut self, relative_path: String) {
        let file = self.root.join(&relative_path);
        let text = std::fs::read_to_string(file).unwrap();
        self.add_text(relative_path, text)
    }
    pub fn add_text(&mut self, file: impl Into<String>, text: impl Into<String>) {
        let id = self.max_id;
        self.max_id += 1;
        self.file_map.insert(file.into(), id);
        self.text_map.insert(id, Arc::new(text.into()));
    }
    pub fn get_text(&self, file: FileID) -> Option<Arc<String>> {
        self.text_map.get(&file).cloned()
    }
}
