use std::hash::Hash;

use super::*;

pub struct ValkyrieTuple {
    tuple: Vec<Box<dyn ValkyrieType>>,
}

impl ValkyrieTuple {
    pub fn from_literal<I, T>(data: I) -> Self
        where
            I: Iterator<Item=T>,
            T: Display + 'static
    {
        Self {
            tuple: data
                .map(|d| Box::new(ValkyrieLiteralType::new(d)) as Box<dyn ValkyrieType>)
                .collect()
        }
    }
}

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


impl ValkyrieType for ValkyrieTuple {
    fn namespace(&self) -> Vec<String> {
        vec!["std".to_string(), "tuple".to_string()]
    }

    fn type_name(&self) -> String {
        "Tuple".to_string()
    }
    fn type_display(&self, full_path: bool) -> String {
        format!("({})", self.generic_types().iter().map(|f| f.as_ref().to_string()).join(", "))
    }
}
