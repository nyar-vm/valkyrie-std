#![feature(trivial_bounds)]

pub use self::{
    builtin::result::{ValkyrieFailure, ValkyrieSuccess},
    types::{
        class_type::ValkyrieClassType, literal_type::ValkyrieLiteralType, tuple_type::ValkyrieClass,
        union_type::ValkyrieUnionType, variant_type::ValkyrieVariant, ValkyrieType,
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
