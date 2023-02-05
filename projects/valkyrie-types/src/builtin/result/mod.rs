use crate::ValkyrieTypeModule;

pub struct ValkyrieSuccess<T> {
    pub value: T,
}

pub struct ValkyrieFailure<E> {
    pub error: E,
}


impl<T, E> ValkyrieTypeModule for Result<T, E> {
    fn namespace(&self) -> Vec<String> {
        vec!["std".to_string(), "primitive".to_string()]
    }

    fn type_name(&self) -> String {
        "Result".to_string()
    }
}

