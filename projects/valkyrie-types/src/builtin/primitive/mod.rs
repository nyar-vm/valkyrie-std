use crate::ValkyrieType;

impl ValkyrieType for f64 {
    fn namespace(&self) -> Vec<String> {
        vec!["std".to_string(), "primitive".to_string()]
    }

    fn type_name(&self) -> String {
        "f64".to_string()
    }
}

impl ValkyrieType for usize {
    fn namespace(&self) -> Vec<String>  {
        vec!["std".to_string(), "primitive".to_string()]
    }

    fn type_name(&self) -> String  {
        "u64".to_string()
    }
}

impl<T> ValkyrieType for Vec<T> {
    fn namespace(&self) -> Vec<String>  {
        vec!["std".to_string(), "numeric".to_string()]
    }

    fn type_name(&self) -> String  {
        "Vector".to_string()
    }
}