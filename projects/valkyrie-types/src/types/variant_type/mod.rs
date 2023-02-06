use super::*;

pub struct ValkyrieVariantType {
    pub namespace: Vec<String>,
    pub name: String,
    pub types: Vec<ValkyrieType>,
}

impl ValkyrieVariantType {
    pub fn new(namespace: Vec<String>, name: String, types: Vec<ValkyrieType>) -> Self {
        Self { namespace, name, types }
    }
}

impl ValkyrieTypeInfo for ValkyrieVariantType {}
