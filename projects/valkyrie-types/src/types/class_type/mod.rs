use std::hash::{Hash, Hasher};

use super::*;

pub trait ValkyrieClassType {
    // get namespace
    fn namespace(&self) -> Vec<String>;
    fn class_name(&self) -> String;
    // get namepath
    fn namepath(&self) -> Vec<String> where Self: Sized {
        let mut namepath = self.namespace();
        namepath.push(self.class_name());
        namepath
    }
    fn type_display(&self) -> String {
        format!("{}[{}]", self.class_name(), self.generic_types().join(", "))
    }

    fn generic_types(&self) -> Vec<ValkyrieType> {
        Vec::new()
    }

    // get methods
    fn methods(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<T> From<T> for ValkyrieType where T: ValkyrieClassType + 'static {
    fn from(t: T) -> Self {
        ValkyrieType::Class(Box::new(t))
    }
}

impl Hash for dyn ValkyrieClassType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for s in &self.namespace() {
            state.write(s.as_bytes());
        }
        state.write(self.class_name().as_bytes());
        state.write(self.type_display().as_bytes());
    }
}