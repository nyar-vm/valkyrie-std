use super::*;

pub struct ValkyrieVariantType {
    pub namespace: Vec<String>,
    pub name: String,
    pub types: Vec<Box<dyn ValkyrieType>>,
}

impl ValkyrieVariantType {
    pub fn new(namespace: Vec<String>, name: String, types: Vec<Box<dyn ValkyrieType>>) -> Self {
        Self { namespace, name, types }
    }
}

impl ValkyrieType for ValkyrieVariantType {
    fn namespace(&self) -> Vec<String> {
        self.namespace.clone()
    }

    fn type_name(&self) -> String {
        self.name.clone()
    }

    fn as_variant_type(&self) -> Option<&ValkyrieVariantType> {
        Some(self)
    }
}