#![feature(trivial_bounds)]
use std::fmt::Display;

pub use self::{
    builtin::result::{ValkyrieFailure, ValkyrieSuccess},
    types::{
        literal_type::ValkyrieLiteralType, tuple_type::ValkyrieTuple, union_type::ValkyrieUnionType,
        variant_type::ValkyrieVariantType, ValkyrieType,
    },
};

mod builtin;
mod types;
// #[cfg(test)]
pub mod testing;
