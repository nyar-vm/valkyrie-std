use std::fmt::Display;

pub use self::builtin::result::ValkyrieResult;
pub use self::types::literal_type::ValkyrieLiteralType;
pub use self::types::union_type::ValkyrieUnionType;
pub use self::types::ValkyrieType;
pub use self::types::tuple_type::ValkyrieTuple;
pub use self::types::variant_type::ValkyrieVariantType;

mod types;
mod builtin;


pub trait ValkyrieFunction {}

pub struct ValkyrieMethod {}

pub trait ValkyrieVariant {
    fn type_names() -> Vec<String>;
}
