use std::fs;

use crate::{ValkyrieDirectory, ValkyrieFile};

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
    pub fn exists(&self) -> bool {
        self._path.set_len()
    }
    pub fn delete(&self) -> std::io::Result<()> {
        std::fs::remove_file(self._path.path())?;
        Ok(())
    }

    pub fn open(&self) -> std::io::Result<ValkyrieFileHandler> {
        let file = tokio::fs::File::open(path._wrap.clone())?;
        Ok(ValkyrieFileHandler { _file: file })
    }

    pub fn read_all_bytes(&mut self) -> std::io::Result<Vec<u8>> {
        return fs::read(&self._path)
    }

    pub fn write_all_bytes(&mut self, buffer: &[u8]) -> std::io::Result<()> {
        fs::write(&self._path, buffer)
    }
}

pub struct UTF8FileHandler {
    _wrap: tokio::fs::File,
}

pub struct ValkyrieClassWrapper {
    _wrap: Box<dyn ValkyrieClass>,
}

impl ValkyrieClass for ValkyrieFile {
    const NAMESPACE: &'static str = "std.io";
    const CLASS_NAME: &'static str = "File";
}

pub trait ValkyrieClass {
    // a namespace is a string split by `.`
    // save bytes then Vec<String>
    const NAMESPACE: &'static str;
    // display class name
    const CLASS_NAME: &'static str;
    // get namespace
    fn namespace() -> Vec<String> {
        Self::namespace().split(".").map(|s| s.to_string()).collect()
    }
    // get namepath
    fn namepath() -> Vec<String> {
        let mut path = Self::namespace();
        path.push(Self::CLASS_NAME.to_string());
        path
    }
}

pub trait ValkyrieVariant {
    fn type_names() -> Vec<String>;
}


pub trait ValkyrieUnionType {
    fn type_names() -> Vec<String>;
}

impl ValkyrieUnionType for ValkyrieFile {
    fn type_names() -> Vec<String> {
        todo!()
    }
}