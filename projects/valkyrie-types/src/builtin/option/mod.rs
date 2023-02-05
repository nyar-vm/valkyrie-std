use crate::{ ValkyrieType, ValkyrieVariantType};


trait StaticType {
    const NAME: &'static str;
}
trait DynamicType {
    fn name(&self) -> String;
}

enum TypeWrapper {
    Static(Box<dyn StaticType>),
    Dynamic(Box<dyn DynamicType>),
}




impl<T> ValkyrieType for Option<T> where T: ValkyrieType + Clone + Default + 'static {
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

    fn is_variant_type(&self) -> bool {
        return true;
    }

    fn as_variant_type(self) -> ValkyrieVariantType {
        ValkyrieVariantType {
            namespace: self.namespace(),
            name: self.type_name(),
            types: vec![],
        }
    }
}
