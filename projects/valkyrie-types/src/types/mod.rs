use std::{
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    sync::Arc,
};

use itertools::Itertools;

use crate::{ValkyrieList, ValkyrieValue, ValkyrieVariantType};

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
#[derive(Debug, Default)]
pub struct ValkyrieMetaType {
    namepath: Vec<String>,
    generic_types: Vec<Arc<ValkyrieMetaType>>,
}

impl ValkyrieType for () {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::List(Arc::new(ValkyrieList::tuple()))
    }

    fn type_info(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Unit");
        Arc::new(this)
    }
}

impl ValkyrieType for ValkyrieValue {
    fn boxed(self) -> ValkyrieValue {
        self
    }

    fn type_info(&self) -> Arc<ValkyrieMetaType> {
        match self {
            ValkyrieValue::Null(v) => v.type_info(),
            ValkyrieValue::Boolean(v) => v.type_info(),
            ValkyrieValue::Unsigned8(v) => v.type_info(),
            ValkyrieValue::Unsigned16(v) => v.type_info(),
            ValkyrieValue::Unsigned32(v) => v.type_info(),
            ValkyrieValue::Unsigned64(v) => v.type_info(),
            ValkyrieValue::Unsigned128(v) => v.type_info(),
            ValkyrieValue::Integer8(v) => v.type_info(),
            ValkyrieValue::Integer16(v) => v.type_info(),
            ValkyrieValue::Integer32(v) => v.type_info(),
            ValkyrieValue::Integer64(v) => v.type_info(),
            ValkyrieValue::Integer128(v) => v.type_info(),
            ValkyrieValue::Float32(v) => v.type_info(),
            ValkyrieValue::Float64(v) => v.type_info(),
            ValkyrieValue::Buffer(v) => v.type_info(),
            ValkyrieValue::String(v) => v.type_info(),
            ValkyrieValue::Option(v) => v.type_info(),
            ValkyrieValue::Result(v) => v.type_info(),
            ValkyrieValue::List(v) => v.type_info(),
        }
    }
}

impl ValkyrieMetaType {
    pub fn set_namepath(&mut self, namepath: &str) {
        self.namepath.clear();
        for s in namepath.split('.') {
            self.namepath.push(s.to_string());
        }
    }
    pub fn mut_namepath(&mut self) -> &mut Vec<String> {
        &mut self.namepath
    }
    pub fn mut_generic_types(&mut self) -> &mut Vec<Arc<ValkyrieMetaType>> {
        &mut self.generic_types
    }
}

#[allow(clippy::wrong_self_convention)]
pub trait ValkyrieType
where
    Self: Sized,
{
    fn boxed(self) -> ValkyrieValue;
    fn type_info(&self) -> Arc<ValkyrieMetaType>;
}

impl ValkyrieMetaType {
    pub fn name(&self) -> String {
        assert_ne!(self.namepath.len(), 0, "namepath `{:?}` is not valid", self.namepath);
        self.namepath.last().unwrap().to_owned()
    }
    pub fn namespace(&self) -> String {
        assert_ne!(self.namepath.len(), 0, "namepath `{:?}` is not valid", self.namepath);
        self.namepath[..self.namepath.len() - 1].join(".")
    }
    pub fn display_type(&self, full_path: bool) -> String {
        let this = match full_path {
            true => self.namepath.join("::"),
            false => self.name(),
        };
        if self.generic_types.is_empty() {
            return this;
        }
        format!("{}[{}]", this, self.generic_types.iter().map(|f| f.type_info().display_type(full_path)).join(", "))
    }
}

impl Display for ValkyrieMetaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.display_type(false))
    }
}

impl Hash for ValkyrieMetaType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.display_type(true).as_bytes());
    }
}

impl ValkyrieType for ValkyrieMetaType {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn type_info(&self) -> Arc<ValkyrieMetaType> {
        todo!()
    }
}
