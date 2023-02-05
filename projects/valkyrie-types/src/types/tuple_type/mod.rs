use super::*;

pub struct ValkyrieTuple {
    tuple: Vec<Box<dyn ValkyrieTypeModule>>,
}

impl ValkyrieTuple {
    pub fn from_literal<I, T>(data: I) -> Self
        where
            I: Iterator<Item=T>,
            T: Display + 'static
    {
        Self {
            tuple: data
                .map(|d| Box::new(ValkyrieLiteralType::new(d)) as Box<dyn ValkyrieTypeModule>)
                .collect()
        }
    }
}


impl ValkyrieTypeModule for ValkyrieTuple {
    fn namespace(&self) -> Vec<String> {
        vec!["std".to_string(), "tuple".to_string()]
    }

    fn type_name(&self) -> String {
        "Tuple".to_string()
    }
    fn type_display(&self, full_path: bool) -> String {
        format!("({})", self.tuple.iter().map(|t| t.type_display(full_path)).join(", "))
    }
}
