use super::*;

pub struct ValkyrieUnionType {
    types: Vec<Box<dyn ValkyrieType>>,
}
