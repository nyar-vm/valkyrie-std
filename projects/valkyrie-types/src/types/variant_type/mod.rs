use super::*;

pub struct ValkyrieVariantType {
    pub namespace: Vec<String>,
    pub name: String,
    pub generics: Vec<ValkyrieMetaType>,
}

impl ValkyrieVariantType {
    pub fn new(namespace: Vec<String>, name: String, types: Vec<ValkyrieMetaType>) -> Self {
        Self { namespace, name, generics: types }
    }
}

impl ValkyrieType for ValkyrieVariantType {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn type_info(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
    }
}
