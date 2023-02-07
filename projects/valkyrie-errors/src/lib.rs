pub use url::Url;

pub use crate::{
    duplicates::DuplicateError,
    errors::{ValkyrieError, ValkyrieErrorKind, ValkyrieReport, ValkyrieResult},
    managers::{
        list::{FileID, FileSpan},
        TextManager,
    },
    runtime::RuntimeError,
};

mod errors;
// #[cfg(test)]
mod duplicates;
mod managers;
mod runtime;
pub mod testing;
