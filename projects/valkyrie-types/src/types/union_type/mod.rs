


pub trait ValkyrieUnionType {
    fn type_names(&self) -> Vec<String>;

    fn type_name(&self) -> String {
        format!("({})", self.type_names().join(", "))
    }
}


