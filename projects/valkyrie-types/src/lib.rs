
pub trait ValkyrieClass {
    // a namespace is a string split by `.`
    // save bytes then Vec<String>
    const NAMESPACE: &'static str;
    // display class name
    const CLASS_NAME: &'static str;
    // get namespace
    fn namespace() -> Vec<String> {
        Self::namespace().split(".").map(|s| s.to_string()).collect()
    }
    // get namepath
    fn namepath() -> Vec<String> {
        let mut path = Self::namespace();
        path.push(Self::CLASS_NAME.to_string());
        path
    }
}

pub trait ValkyrieVariant {
    fn type_names() -> Vec<String>;
}


pub trait ValkyrieUnionType {
    fn type_names() -> Vec<String>;
}
