use super::*;

pub struct ValkyrieVariantType {
    pub namespace: Vec<String>,
    pub name: String,
    pub types: Vec<Box<dyn ValkyrieTypeModule>>,
}

impl ValkyrieVariantType {
    pub fn new(namespace: Vec<String>, name: String, types: Vec<Box<dyn ValkyrieTypeModule>>) -> Self {
        Self { namespace, name, types }
    }
}

impl ValkyrieTypeModule for ValkyrieVariantType {
    fn namespace(&self) -> Vec<String> {
        self.namespace.clone()
    }

    fn type_name(&self) -> String {
        self.name.clone()
    }

    fn is_variant_type(&self) -> bool {
        return true;
    }

    fn as_variant_type(self) -> ValkyrieVariantType {
        self
    }
}