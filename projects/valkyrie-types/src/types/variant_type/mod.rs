use super::*;

pub struct ValkyrieVariantType {
    pub namespace: Vec<String>,
    pub name: String,
    pub types: Vec<ValkyrieMetaType>,
}

impl ValkyrieVariantType {
    pub fn new(namespace: Vec<String>, name: String, types: Vec<ValkyrieMetaType>) -> Self {
        Self { namespace, name, types }
    }
}

impl ValkyrieType for ValkyrieVariantType {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }
}
