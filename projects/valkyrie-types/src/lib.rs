pub use self::builtin::result::ValkyrieResult;

mod result;
mod builtin;

pub trait ValkyrieClass {
    // a namespace is a string split by `.`
    // save bytes then Vec<String>
    const NAMESPACE: &'static str;
    // display class name
    const CLASS_NAME: &'static str;
    // get namespace
    fn namespace() -> Vec<String> {
        Self::NAMESPACE.split('.').map(|s| s.to_string()).collect()
    }
    // get namepath
    fn namepath() -> Vec<String> {
        let mut path = Self::namespace();
        path.push(Self::CLASS_NAME.to_string());
        path
    }
    fn generic_types(&self) -> Vec<String> {
        Vec::new()
    }

    // get methods
    fn methods() -> Vec<String> {
        Vec::new()
    }
}

pub trait ValkyrieFunction {}

pub struct ValkyrieMethod {}

pub trait ValkyrieVariant {
    fn type_names() -> Vec<String>;
}


pub trait ValkyrieUnionType {
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


impl<T> ValkyrieClass for ValkyrieTensor<T> {
    const NAMESPACE: &'static str = "std.tensor";
    const CLASS_NAME: &'static str = "Tensor";
    fn generic_types(&self) -> Vec<String> {
        vec![
            std::any::type_name::<T>().to_string(),
            self.dimension.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(", "),
        ]
    }
}

pub enum ValkyrieType {
    Class(Box<dyn ValkyrieClass>),
}

pub struct ValkyrieTuple {
    data: Vec<ValkyrieType>,
}

impl ValkyrieClass for ValkyrieTuple {
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