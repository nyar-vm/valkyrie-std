use super::*;

pub struct ValkyrieVariant {
    namepath: Vec<String>,
    generics: Vec<Arc<ValkyrieMetaType>>,
    variants: Vec<Arc<ValkyrieClassType>>,
}

impl ValkyrieVariant {
    pub fn new(namepath: String) -> Self {
        Self { namepath: namepath.split('.').map(|s| s.to_string()).collect(), generics: vec![], variants: vec![] }
    }
    pub fn mut_generics(&mut self) -> &mut Vec<Arc<ValkyrieMetaType>> {
        &mut self.generics
    }
}

impl Default for ValkyrieVariant {
    fn default() -> Self {
        todo!()
    }
}

impl ValkyrieType for ValkyrieVariant {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Variant(Arc::new(self))
    }

    fn type_info(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        *this.mut_namepath() = self.namepath.clone();
        this.mut_generic_types().extend(self.generics.iter().cloned());
        Arc::new(this)
    }
}
