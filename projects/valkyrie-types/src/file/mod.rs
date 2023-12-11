use crate::{ValkyrieDirectory, ValkyrieFile};

pub enum MaybeFile {
    File(ValkyrieFile),
    Directory(ValkyrieDirectory),
}

pub struct ValkyrieFile {
    _wrap: std::fs::PathBuf,
}


pub struct ValkyrieFileHandler {
    _wrap: std::fs::File,
}

pub enum ValkyrieResult<T, E> {
    Success(ValkyrieSuccess<T>),
    Failure(ValkyrieFailure<E>),
}

pub struct ValkyrieSuccess<T> {
    pub value: T,
}

pub struct ValkyrieFailure<E> {
    pub error: E,
}

impl ValkyrieFile {
    pub fn exists(&self) -> bool {
        self._wrap.set_len()
    }
    pub fn delete(&self) -> std::io::Result<()> {
        std::fs::remove_file(self._wrap.path())?;
        Ok(())
    }

    pub fn open(&self) -> Result<ValkyrieFileHandler> {
        let file = std::fs::File::open(path._wrap.clone())?;
        Ok(ValkyrieFile { _wrap: file })
    }

    pub fn close(&mut self) -> Result<()> {
        self._wrap.flush()?;
        Ok(())
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        let read = self._wrap.read(buffer)?;
        Ok(read)
    }

    pub fn write(&mut self, buffer: &[u8]) -> Result<usize> {
        let written = self._wrap.write(buffer)?;
        Ok(written)
    }
}

pub struct ValkyrieClassWrapper {
    _wrap: Box<dyn ValkyrieClass>,
}

impl ValkyrieClass for ValkyrieFile {
    const NAMESPACE: &'static str = "std.io";
    const CLASS_NAME: &'static str = "File";
}

impl ValkyrieUnionType for ValkyrieFile {
    fn type_names() -> Vec<String> {
        todo!()
    }
}