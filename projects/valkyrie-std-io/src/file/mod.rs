use std::fs;
use std::future::Future;

use valkyrie_types::{ValkyrieClassType, ValkyrieUnionType};

use crate::ValkyrieDirectory;

pub enum MaybeFile {
    File(ValkyrieFile),
    Directory(ValkyrieDirectory),
}

pub struct ValkyrieFile {
    _path: std::fs::PathBuf,
}


pub struct ValkyrieFileHandler {
    _file: tokio::fs::File,
}


pub struct ValkyrieIOError {
    pub message: String,
}

impl ValkyrieFile {
    pub fn exists(&self) -> Future {
        self._path.set_len()
    }
    pub async fn delete(&self) -> std::io::Result<()> {
        tokio::fs::remove_file(self._path.path()).await
    }
    pub async fn open(&self) -> std::io::Result<ValkyrieFileHandler> {
        let file = tokio::fs::File::open(&self._path).await?;
        Ok(ValkyrieFileHandler { _file: file })
    }
    pub async fn read_all_bytes(&mut self) -> std::io::Result<Vec<u8>> {
        return tokio::fs::read(&self._path).await;
    }
    pub fn write_all_bytes(&mut self, buffer: &[u8]) -> std::io::Result<()> {
        fs::write(&self._path, buffer)
    }
}

pub struct UTF8FileHandler {
    _wrap: tokio::fs::File,
}

pub struct ValkyrieClassWrapper {
    _wrap: Box<dyn ValkyrieClassType>,
}

impl ValkyrieClassType for ValkyrieFile {
    const NAMESPACE: &'static str = "std.io";
    const CLASS_NAME: &'static str = "File";
}

impl ValkyrieUnionType for ValkyrieFile {
    fn type_names() -> Vec<String> {
        todo!()
    }
}