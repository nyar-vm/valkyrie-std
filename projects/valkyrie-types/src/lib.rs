#![feature(trivial_bounds)]

pub use self::{
    builtin::result::{ValkyrieFailure, ValkyrieSuccess},
    types::{
        literal_type::ValkyrieLiteralType, tuple_type::ValkyrieTuple, union_type::ValkyrieUnionType,
        variant_type::ValkyrieVariantType, ValkyrieTypeInfo,
    },
};

mod builtin;
mod types;
mod functions;
// #[cfg(test)]
pub mod testing;
