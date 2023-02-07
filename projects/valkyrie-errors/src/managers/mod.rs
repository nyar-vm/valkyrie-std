use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
    path::{Path, PathBuf},
    sync::Arc,
};

use ariadne::{Cache, Source};

pub type FileID = usize;

pub struct TextManager {
    // workspace root
    root: PathBuf,
    max_id: FileID,
    file_map: BTreeMap<String, FileID>,
    text_map: BTreeMap<FileID, Arc<Source>>,
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
        self.text_map.insert(id, Arc::new(Source::from(text.into())));
    }
}

impl Cache<FileID> for TextManager {
    fn fetch(&mut self, id: &FileID) -> Result<&Source, Box<dyn Debug + '_>> {
        match self.text_map.get(id) {
            Some(s) => Ok(s.as_ref()),
            None => Err(Box::new(format!("FileID {} not found", id))),
        }
    }

    fn display<'a>(&self, id: &'a FileID) -> Option<Box<dyn Display + 'a>> {
        let o = self.text_map.get(id)?;
        Some(Box::new(o.chars().collect::<String>()))
    }
}
