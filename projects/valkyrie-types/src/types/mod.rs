use std::{
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
};

use itertools::Itertools;

use crate::{ValkyrieLiteralType, ValkyrieUnionType, ValkyrieVariantType};

pub mod class_type;
pub mod literal_type;
pub mod tuple_type;
pub mod union_type;
pub mod variant_type;

#[allow(clippy::wrong_self_convention)]
pub trait ValkyrieVariantTrait {
    const NAME: &'static str;

    fn as_type_module(self) -> ValkyrieMetaType
    where
        Self: Sized,
    {
        todo!()
    }
}

impl<T: ValkyrieType> ValkyrieVariantTrait for Option<T> {
    const NAME: &'static str = "Option";
}

// rtti of valktype
#[derive(Default)]
pub struct ValkyrieMetaType {
    namepath: Vec<String>,
}

impl ValkyrieMetaType {
    pub fn new(model: impl ValkyrieType) -> ValkyrieMetaType {
        let mut out = Self::default();
        out.namepath = model.namepath();
        return out;
    }

    pub fn with_namepath(&mut self, namepath: &str) {
        self.namepath.clear();
        for s in namepath.split('.') {
            self.namepath.push(s.to_string());
        }
    }
}

#[allow(clippy::wrong_self_convention)]

pub trait ValkyrieType {
    // get namespace
    fn namespace(&self) -> Vec<String>;
    fn type_name(&self) -> String;
    fn type_display(&self, full_path: bool) -> String {
        let this = match full_path {
            true => self.namepath().join("::"),
            false => self.type_name(),
        };
        let generics = self.generic_types();
        if generics.is_empty() {
            return this;
        }
        format!("{}[{}]", this, generics.iter().map(|f| f.type_display(full_path)).join(", "))
    }
    // get namepath
    fn namepath(&self) -> Vec<String> {
        let mut namepath = self.namespace();
        namepath.push(self.type_name());
        namepath
    }
    fn new<T>(value: T) -> Box<dyn ValkyrieType>
    where
        Self: Sized,
        T: ValkyrieType + 'static,
    {
        Box::new(value)
    }
    fn generic_types(&self) -> Vec<Box<dyn ValkyrieType>> {
        Vec::new()
    }

    // get methods
    fn methods(&self) -> Vec<String> {
        Vec::new()
    }

    fn static_type() -> ValkyrieMetaType
    where
        Self: Sized,
    {
        ValkyrieMetaType { namepath: vec![] }
    }
    fn dynamic_type(&self) -> ValkyrieMetaType
    where
        Self: Sized,
    {
        Self::static_type()
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
