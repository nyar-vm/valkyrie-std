use super::*;

pub struct ValkyrieUnionType {
    terms: Vec<Box<dyn ValkyrieTypeModule>>,
}

impl ValkyrieTypeModule for ValkyrieUnionType {
    fn namespace(&self) -> Vec<String> {
        vec!["std".to_string(), "collection".to_string()]
    }

    fn type_name(&self) -> String {
        "Union".to_string()
    }
    fn type_display(&self, full_path: bool) -> String {
        let mut result = String::new();
        for term in &self.terms {
            result.push_str(&term.type_display(full_path));
            result.push_str(" | ");
        }
        result.pop();
        result.pop();
        result
    }
    fn is_union_type(&self) -> bool {
        return true;
    }

    fn as_union_type(self) -> ValkyrieUnionType {
        self
    }
}
