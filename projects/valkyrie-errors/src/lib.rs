pub use self::{
    duplicates::DuplicateError,
    errors::{ValkyrieError, ValkyrieErrorKind},
    managers::{list::TextSpan, FileID, FileSpan, TextManager},
};

mod errors;
// #[cfg(test)]
mod duplicates;
mod managers;
pub mod testing;
