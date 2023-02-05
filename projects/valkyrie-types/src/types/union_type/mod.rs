use super::*;

pub struct ValkyrieUnionType {
    terms: Vec<ValkyrieMetaType>,
}

impl ValkyrieType for ValkyrieUnionType {}
