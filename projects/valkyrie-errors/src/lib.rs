pub use self::{
    duplicates::DuplicateItem,
    errors::{ValkyrieError, ValkyrieErrorKind},
    managers::{FileID, FileSpan, TextManager},
};

mod errors;
// #[cfg(test)]
mod duplicates;
mod managers;
pub mod testing;
