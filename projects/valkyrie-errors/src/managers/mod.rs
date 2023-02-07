use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter},
    fs::read_to_string,
    path::{Path, PathBuf},
    sync::Arc,
};

use ariadne::{Cache, Source};
use url::Url;

pub type FileID = usize;

pub struct TextManager {
    // workspace root
    root: PathBuf,
    max_id: FileID,
    text_map: BTreeMap<FileID, TextItem>,
}

pub struct TextItem {
    path: String,
    source: Arc<Source>,
}

impl Debug for TextManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let files: Vec<&str> = self.text_map.values().map(|v| v.path.as_str()).collect();
        f.debug_struct("TextManager").field("root", &self.root.display()).field("files", &files).finish()
    }
}

pub struct FileSpan {
    file: usize,
    head: usize,
    tail: usize,
}

impl TextManager {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self { root: root.as_ref().canonicalize().unwrap(), max_id: 0, text_map: BTreeMap::default() }
    }
    pub fn add_file(&mut self, relative_path: &str) -> FileID {
        let file = self.root.join(&relative_path);
        let text = match read_to_string(&file) {
            Ok(o) => o,
            Err(_) => {
                panic!("File {} not found", file.display())
            }
        };
        self.add_text(relative_path, text)
    }
    pub fn add_text(&mut self, file: impl Into<String>, text: impl Into<String>) -> FileID {
        let id = self.max_id;
        self.max_id += 1;
        self.text_map.insert(id, TextItem { path: file.into(), source: Arc::new(Source::from(text.into())) });
        id
    }
}

impl Cache<FileID> for TextManager {
    fn fetch(&mut self, id: &FileID) -> Result<&Source, Box<dyn Debug + '_>> {
        match self.text_map.get(id) {
            Some(s) => Ok(s.source.as_ref()),
            None => Err(Box::new(format!("FileID `{}` not found", id))),
        }
    }

    fn display<'a>(&self, id: &'a FileID) -> Option<Box<dyn Display + 'a>> {
        let item = self.text_map.get(id)?;
        let url = Url::from_file_path(&self.root.join(&item.path)).ok()?;
        Some(Box::new(url))
    }
}
