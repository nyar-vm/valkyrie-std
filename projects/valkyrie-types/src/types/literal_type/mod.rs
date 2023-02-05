use super::*;

pub struct ValkyrieLiteralType<T> {
    data: T,
}

pub struct ValkyriePhantomType<'r, T> {
    namespace: Vec<String>,
    name: String,
    data: &'r T,
}

impl<T> ValkyrieLiteralType<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> ValkyriePhantomType<T> {
    pub fn new(namespace: Vec<String>, name: String, data: &T) -> Self {
        Self { namespace, name, data }
    }
}


impl<T: Display> ValkyrieTypeModule for ValkyrieLiteralType<T> {
    fn namespace(&self) -> Vec<String> {
        vec!["std".to_string(), "literal".to_string()]
    }

    fn type_name(&self) -> String {
        self.data.to_string()
    }

    fn type_display(&self, full_path: bool) -> String {
        self.data.to_string()
    }
}

