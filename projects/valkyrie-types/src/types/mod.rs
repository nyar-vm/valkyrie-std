use std::fmt::{Debug, Display, Write};
use std::hash::{Hash, Hasher};

use itertools::Itertools;

use crate::ValkyrieLiteralType;
use crate::ValkyrieUnionType;

pub mod class_type;
pub mod union_type;
pub mod tuple_type;
pub mod literal_type;
pub mod variant_type;


pub trait ValkyrieType {
    // get namespace
    fn namespace(&self) -> Vec<String>;
    fn type_name(&self) -> String;
    fn type_display(&self, full_path: bool) -> String {
        let this = match full_path {
            true => { self.namepath().join("::") }
            false => { self.type_name() }
        };
        let generics = self.generic_types();
        if generics.is_empty() {
            return this;
        }
        format!("{}[{}]", this, generics.iter().map(|f| f.as_ref().to_string()).join(", "))
    }
    // get namepath
    fn namepath(&self) -> Vec<String> {
        let mut namepath = self.namespace();
        namepath.push(self.type_name());
        namepath
    }
    fn generic_types(&self) -> Vec<Box<dyn ValkyrieType>> {
        Vec::new()
    }

    // get methods
    fn methods(&self) -> Vec<String> {
        Vec::new()
    }

    fn as_union_type(&self) -> Option<ValkyrieUnionType> {
        None
    }
    fn as_variant_type(&self) -> Option<ValkyrieVariantType> {
        None
    }
}



impl Display for dyn ValkyrieType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.type_display(false))
    }
}

impl Debug for dyn ValkyrieType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.type_display(true))
    }
}

impl Hash for dyn ValkyrieType + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.type_display(true).as_bytes());
    }
}
