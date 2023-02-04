use std::fmt::{Display, Formatter};
use crate::types::ValkyrieType;

pub use self::types::class_type::ValkyrieClassType;
pub use self::types::union_type::ValkyrieUnionType;

pub use self::builtin::result::ValkyrieResult;

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
pub struct ValkyrieTensor<T> {
    dimension: Vec<usize>,
    data: Vec<T>,
}

impl<T> ValkyrieTensor<T> {
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


impl<T> Display for ValkyrieTensor<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Tensor<{}, {}>", std::any::type_name::<T>(), self.dimension.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(", ")))
    }
}

impl<T> ValkyrieClassType for ValkyrieTensor<T> {
    const NAMESPACE: &'static str = "std.tensor";
    const CLASS_NAME: &'static str = "Tensor";
    fn generic_types(&self) -> Vec<String> {
        vec![
            std::any::type_name::<T>().to_string(),
            self.dimension.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(", "),
        ]
    }
}


pub struct ValkyrieTuple {
    data: Vec<ValkyrieType>,
}

impl ValkyrieClassType for ValkyrieTuple {
    const NAMESPACE: &'static str = "std.tuple";
    const CLASS_NAME: &'static str = "Tuple";

    fn generic_types(&self) -> Vec<String> {
        self.data.iter().map(|d| d.generic_types()).collect::<Vec<Vec<String>>>().join(", ")
    }
}

#[test]
fn test_broadcast() {
    let lhs: ValkyrieTensor<usize> = ValkyrieTensor::new(vec![2, 3, 4]);
    let rhs: ValkyrieTensor<usize> = ValkyrieTensor::new(vec![2, 1, 4]);
    let result = lhs.broadcast_add(&rhs);
    println!("{:?}", result.generic_types());
    assert_eq!(result.dimension, vec![2, 3, 4]);
}