use std::fmt::{Display, Formatter};
use crate::types::tuple_type::ValkyrieTuple;

pub use self::builtin::result::ValkyrieResult;
pub use self::types::union_type::ValkyrieUnionType;
pub use self::types::ValkyrieType;

mod types;
mod builtin;


pub trait ValkyrieFunction {}

pub struct ValkyrieMethod {}

pub trait ValkyrieVariant {
    fn type_names() -> Vec<String>;
}


// class Tensor[T, D] {
// const D: Tuple[..u64]
// }
pub struct ValkyrieTensor<T: ValkyrieType> {
    dimension: Vec<usize>,
    data: Vec<T>,
}

impl<T> ValkyrieTensor<T> where T: ValkyrieType {
    pub fn new(dimension: Vec<usize>) -> Self {
        Self { dimension, data: vec![] }
    }
    pub fn broadcast_add(&self, other: &Self) -> Self {
        let mut dimension = self.dimension.clone();
        for (i, d) in other.dimension.iter().enumerate() {
            if dimension[i] != *d {
                dimension[i] = 1;
            }
        }
        Self { dimension, data: vec![] }
    }
}

impl<T:ValkyrieType> ValkyrieType for ValkyrieTensor<T> {
    fn namespace(&self) -> Vec<String> {
        vec!["std".to_string(), "numeric".to_string()]
    }

    fn type_name(&self) -> String {
        "Tensor".to_string()
    }

    fn generic_types(&self) -> Vec<Box<dyn ValkyrieType>> {
        vec![
            // Box::new(T::type_name()),
            Box::new(ValkyrieTuple::from_literal(self.dimension.iter().cloned()))
        ]
    }
}


#[test]
fn test_broadcast() {
    let lhs: ValkyrieTensor<usize> = ValkyrieTensor::new(vec![2, 3, 4]);
    let rhs: ValkyrieTensor<usize> = ValkyrieTensor::new(vec![2, 1, 4]);
    let result: Box<dyn ValkyrieType> = Box::new(lhs.broadcast_add(&rhs));
    println!("{}", result);
    println!("{:?}", result);
}