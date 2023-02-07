pub use self::{
    duplicates::DuplicateItem,
    errors::{ValkyrieError, ValkyrieErrorKind},
    managers::TextManager,
};

mod errors;
// #[cfg(test)]
mod duplicates;
mod managers;
pub mod testing;
