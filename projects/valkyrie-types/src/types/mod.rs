use std::{
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
};
use std::sync::Arc;

use itertools::Itertools;

use crate::ValkyrieVariantType;

pub mod class_type;
pub mod literal_type;
pub mod tuple_type;
pub mod union_type;
pub mod variant_type;

#[allow(clippy::wrong_self_convention)]
pub trait ValkyrieVariantTrait {
    const NAME: &'static str;

    fn as_type_module(self) -> ValkyrieType
        where
            Self: Sized,
    {
        todo!()
    }
}

impl<T: ValkyrieTypeInfo> ValkyrieVariantTrait for Option<T> {
    const NAME: &'static str = "Option";
}

// rtti of valktype
#[derive(Debug, Default)]
pub struct ValkyrieType {
    namepath: Vec<String>,
    generic_types: Vec<ValkyrieType>,
}

pub enum ValkyrieValue {
    Unit,
    Boolean(bool),
    Unsigned8(u8),
    Unsigned16(u16),
    Unsigned32(u32),
    Unsigned64(u64),
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    Integer128(i128),
    Float32(f32),
    Float64(f64),
    String(String),
    Result(Result<ValkyrieValue, ValkyrieValue>),
}

impl ValkyrieTypeInfo for () {}

impl ValkyrieTypeInfo for ValkyrieValue {
    fn static_info() -> ValkyrieType {
        let mut meta = ValkyrieType::default();
        meta.set_namepath("core.ValkyrieValue");
        meta
    }
    fn dynamic_info(&self) -> ValkyrieType {
        match self {
            ValkyrieValue::Unit => { ().dynamic_info() }
            ValkyrieValue::Boolean(v) => { v.dynamic_info() }
            ValkyrieValue::Unsigned8(v) => { v.dynamic_info() }
            ValkyrieValue::Unsigned16(v) => { v.dynamic_info() }
            ValkyrieValue::Unsigned32(v) => { v.dynamic_info() }
            ValkyrieValue::Unsigned64(v) => { v.dynamic_info() }
            ValkyrieValue::Integer8(v) => {}
            ValkyrieValue::Integer16(v) => { v.dynamic_info() }
            ValkyrieValue::Integer32(v) => { v.dynamic_info() }
            ValkyrieValue::Integer64(v) => { v.dynamic_info() }
            ValkyrieValue::Integer128(v) => { v.dynamic_info() }
            ValkyrieValue::Float32(v) => { v.dynamic_info() }
            ValkyrieValue::Float64(v) => { v.dynamic_info() }
            ValkyrieValue::String(v) => { v.dynamic_info() }
            ValkyrieValue::Result(v) => { v.dynamic_info() }
        }
    }
}

impl ValkyrieType {
    pub fn new(model: impl ValkyrieTypeInfo) -> ValkyrieType {
        let mut out = Self::default();
        return out;
    }
    pub fn set_namepath(&mut self, namepath: &str) {
        self.namepath.clear();
        for s in namepath.split('.') {
            self.namepath.push(s.to_string());
        }
    }
    pub fn mut_namepath(&mut self) -> &mut Vec<String> {
        &mut self.namepath
    }
    pub fn set_generic_types(&mut self, generic_types: Vec<ValkyrieType>) {
        self.generic_types = generic_types;
    }
    pub fn mut_generic_types(&mut self) -> &mut Vec<ValkyrieType> {
        &mut self.generic_types
    }
}

#[allow(clippy::wrong_self_convention)]
pub trait ValkyrieTypeInfo
    where
        Self: Sized,
{
    fn static_info() -> ValkyrieType {
        ValkyrieType::default()
    }
    fn dynamic_info(&self) -> ValkyrieType {
        Self::static_info()
    }
}

impl ValkyrieType {
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
        format!("{}[{}]", this, self.generic_types.iter().map(|f| f.dynamic_info().display_type(full_path)).join(", "))
    }
}

impl Display for ValkyrieType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.display_type(false))
    }
}

impl Hash for ValkyrieType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.display_type(true).as_bytes());
    }
}

impl ValkyrieTypeInfo for ValkyrieType {}
