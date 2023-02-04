


pub trait ValkyrieUnionType {
    fn type_names(&self) -> Vec<String>;

    fn type_name(&self) -> String {
        Self::namepath().join(".")
    }
}


