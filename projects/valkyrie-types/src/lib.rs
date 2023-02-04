mod result;
mod builtin;

pub use result::ValkyrieResult;

pub trait ValkyrieClass {
    // a namespace is a string split by `.`
    // save bytes then Vec<String>
    const NAMESPACE: &'static str;
    // display class name
    const CLASS_NAME: &'static str;
    // get namespace
    fn namespace() -> Vec<String> {
        Self::NAMESPACE.split('.').map(|s| s.to_string()).collect()
    }
    // get namepath
    fn namepath() -> Vec<String> {
        let mut path = Self::namespace();
        path.push(Self::CLASS_NAME.to_string());
        path
    }
    // get methods
    fn methods() -> Vec<String> {
        Vec::new()
    }

}

pub trait ValkyrieFunction {

}

pub struct ValkyrieMethod {

}

pub trait ValkyrieVariant {
    fn type_names() -> Vec<String>;
}


pub trait ValkyrieUnionType {
    fn type_names() -> Vec<String>;
}
