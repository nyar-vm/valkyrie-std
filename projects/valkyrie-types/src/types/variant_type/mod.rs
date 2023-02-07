use super::*;
use std::any::type_name;

pub struct ValkyrieVariantType {
    pub namepath: String,
    pub generics: Vec<Arc<ValkyrieMetaType>>,
    pub variants: Vec<Arc<ValkyrieClassType>>,
}

pub struct ValkyrieClassType {}

impl ValkyrieVariantType {}

impl ValkyrieType for ValkyrieVariantType {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn type_info(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath(&self.namepath);
        this.mut_generic_types().extend(self.generics.iter().cloned());
        Arc::new(this)
    }
}
