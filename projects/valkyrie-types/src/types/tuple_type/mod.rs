use super::*;

pub struct ValkyrieTuple {
    tuple: Vec<ValkyrieMetaType>,
}

impl ValkyrieTuple {
    pub fn from_literal<I, T>(data: I) -> Self
    where
        I: Iterator<Item = T>,
        T: Display + 'static,
    {
        todo!()
        // Self { tuple: data.map(|d| Box::new(ValkyrieLiteralType::new(d)) as ValkyrieMetaType).collect() }
    }
}

impl ValkyrieType for ValkyrieTuple {}
