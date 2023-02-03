mod errors;

pub use errors::{Error, Result};


#[v::namepath(std::io::Path)]
pub struct ValkyriePath {
    _wrap: std::path::PathBuf
}


impl ValkyriePath {
    pub fn get_windows_path(&self) -> Result<String> {
        let path = self._wrap.to_str().ok_or(Error::UnknownError)?;
        Ok(path.to_string())
    }
}
