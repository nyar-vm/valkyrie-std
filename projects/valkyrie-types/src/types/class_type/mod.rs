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
    fn type_name(&self) -> String;

    fn generic_types(&self) -> Vec<String> {
        Vec::new()
    }

    // get methods
    fn methods(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<T> From<T> for ValkyrieType where T: ValkyrieClassType {
    fn from(t: T) -> Self {
        ValkyrieType::Class(Box::new(t))
    }
}