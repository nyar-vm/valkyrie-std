use std::hash::Hash;
use super::*;

pub struct ValkyrieTuple {
    data: Vec<ValkyrieType>,
}



impl ValkyrieClassType for ValkyrieTuple {
    fn namespace(&self) -> Vec<String> {
        vec!["std".to_string(), "tuple".to_string()]
    }

    fn class_name(&self) -> String {
        "Tuple".to_string()
    }

    fn type_display(&self) -> String {
        format!("({})", self.generic_types().join(", "))
    }

    fn generic_types(&self) -> Vec<String> {
        vec![]
    }
}
