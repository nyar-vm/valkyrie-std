use std::fmt::Display;

use crate::{ValkyrieClassType, ValkyrieUnionType};

pub mod class_type;
pub mod union_type;
pub mod tuple_type;


pub enum ValkyrieType {
    Class(Box<dyn ValkyrieClassType>),
    // a | b
    Union(Box<dyn ValkyrieUnionType>),
}

impl Display for ValkyrieType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieType::Class(v) => f.write_str(&v.type_display()),
            ValkyrieType::Union(v) => f.write_str(&v.type_name()),
        }
    }
}

impl ValkyrieType {}