use std::fmt::{Debug, Display, };
use std::hash::{Hash, Hasher};

use itertools::Itertools;

use crate::{ValkyrieLiteralType, ValkyrieVariantType};
use crate::ValkyrieUnionType;

pub mod class_type;
pub mod union_type;
pub mod tuple_type;
pub mod literal_type;
pub mod variant_type;



#[allow(clippy::wrong_self_convention)]
pub trait ValkyrieVariantTrait {
    const NAME: &'static str;

    fn as_type_module(self) -> ValkyrieType {
        todo!()
    }
}


impl<T: ValkyrieTypeModule> ValkyrieVariantTrait for Option<T>{
    const NAME: &'static str = "";
}

// rtti of valktype
#[derive(Default)]
pub struct ValkyrieType {
    namepath: Vec<String>,
}

impl ValkyrieType {
    pub fn new(model: impl ValkyrieTypeModule) -> ValkyrieType {
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
pub trait ValkyrieTypeModule {
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
        format!("{}[{}]", this, generics.iter().map(|f| f.type_display(full_path)).join(", "))
    }
    // get namepath
    fn namepath(&self) -> Vec<String> {
        let mut namepath = self.namespace();
        namepath.push(self.type_name());
        namepath
    }
    fn new<T>(value: T) -> Box<dyn ValkyrieTypeModule> where Self: Sized , T: ValkyrieTypeModule +'static, {
        Box::new(value)
    }
    fn generic_types(&self) -> Vec<Box<dyn ValkyrieTypeModule>> {
        Vec::new()
    }

    // get methods
    fn methods(&self) -> Vec<String> {
        Vec::new()
    }

    fn is_union_type(&self) -> bool {
        false
    }
    fn as_union_type(self) -> ValkyrieUnionType where Self: Sized {
        panic!("Can't convert `{}` to union type", self.type_display(true))
    }
    fn is_variant_type(&self) -> bool {
        false
    }
    fn as_variant_type(self) -> ValkyrieVariantType where Self: Sized {
        panic!("Can't convert `{}` to variant type", self.type_display(true))
    }
}

impl Display for dyn ValkyrieTypeModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.type_display(false))
    }
}

impl Debug for dyn ValkyrieTypeModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.type_display(true))
    }
}

impl Hash for dyn ValkyrieTypeModule + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.type_display(true).as_bytes());
    }
}
