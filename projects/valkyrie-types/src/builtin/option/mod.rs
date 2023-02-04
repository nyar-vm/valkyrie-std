use crate::ValkyrieType;

impl<T> ValkyrieType for Option<T> {
    fn namespace(&self) -> Vec<String> {
        vec!["std".to_string(), "primitive".to_string()]
    }

    fn type_name(&self) -> String {
        "Option".to_string()
    }
}