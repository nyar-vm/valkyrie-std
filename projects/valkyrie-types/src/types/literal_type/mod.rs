use std::marker::PhantomData;
use super::*;

pub struct ValkyrieLiteralType<T> {
    data: T,
}

impl<T> ValkyrieLiteralType<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T: Display> ValkyrieType for ValkyrieLiteralType<T> {
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

