use crate::{ValkyrieType, ValkyrieVariantType};

impl<T> ValkyrieType for Option<T>
where
    T: ValkyrieType + Clone + Default + 'static,
{
    fn namespace(&self) -> Vec<String> {
        vec!["std".to_string(), "primitive".to_string()]
    }

    fn type_name(&self) -> String {
        "Option".to_string()
    }

    fn generic_types(&self) -> Vec<Box<dyn ValkyrieType>> {
        match self {
            None => {
                vec![Box::new(T::default())]
            }
            Some(s) => {
                vec![Box::new(s.clone())]
            }
        }
    }
}
