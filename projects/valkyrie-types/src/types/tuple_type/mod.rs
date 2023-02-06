use super::*;

pub struct ValkyrieTuple {
    items: Vec<ValkyrieValue>,
}

impl ValkyrieTuple {
    pub fn empty() -> Self {
        Self { items: Vec::new() }
    }

    pub fn from_literal<I, T>(data: I) -> Self
    where
        I: Iterator<Item = T>,
        T: Display + 'static,
    {
        todo!()
        // Self { tuple: data.map(|d| Box::new(ValkyrieLiteralType::new(d)) as ValkyrieMetaType).collect() }
    }
}

impl ValkyrieType for ValkyrieTuple {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Tuple(Arc::new(self))
    }
}
