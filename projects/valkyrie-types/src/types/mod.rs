use std::fmt::Display;

use crate::{ValkyrieClassType, ValkyrieUnionType};

pub mod class_type;
pub mod union_type;


pub enum ValkyrieType {
    Class(Box<dyn ValkyrieClassType>),
    Union(Box<dyn ValkyrieUnionType>),
}

impl Display for ValkyrieType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieType::Class(v) => f.write_str(&v.type_name()),
            ValkyrieType::Union(v) => f.write_str(&v.type_name()),
        }
    }
}

impl ValkyrieType {}