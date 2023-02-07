#![feature(trivial_bounds)]

pub use self::{
    builtin::result::{ValkyrieFailure, ValkyrieSuccess},
    types::{
        literal_type::ValkyrieLiteralType, tuple_type::ValkyrieList, union_type::ValkyrieUnionType,
        variant_type::ValkyrieVariantType, ValkyrieType,
    },
    values::ValkyrieValue,
};

mod builtin;
mod codegen;
mod functions;
mod types;
mod values;
// #[cfg(test)]
pub mod testing;
