use super::*;

pub struct ValkyrieUnionType {
    terms: Vec<ValkyrieType>,
}

impl ValkyrieTypeInfo for ValkyrieUnionType {}
