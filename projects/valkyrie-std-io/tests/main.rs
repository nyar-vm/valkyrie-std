use valkyrie_types::ValkyrieTypeModule;
use valkyrie_types::ValkyrieTuple;

#[test]
fn ready() {
    println!("it works!")
}



// class Tensor[T, D] {
// const D: Tuple[..u64]
// }
pub struct ValkyrieTensor<T: ValkyrieTypeModule + Clone> {
    dimension: Vec<usize>,
    default: T,
}

impl<T> ValkyrieTensor<T> where T: ValkyrieTypeModule + Clone {
    pub fn new(dimension: Vec<usize>, default: T) -> Self {
        Self { dimension, default }
    }
    pub fn broadcast_add(&self, other: &Self) -> Self {
        let mut dimension = self.dimension.clone();
        for (i, d) in other.dimension.iter().enumerate() {
            if dimension[i] != *d {
                dimension[i] = 1;
            }
        }
        Self { dimension, default: self.default.clone() }
    }
}

impl<T> ValkyrieTypeModule for ValkyrieTensor<T> where T: ValkyrieTypeModule + Clone + 'static {
    fn namespace(&self) -> Vec<String> {
        vec!["std".to_string(), "numeric".to_string()]
    }

    fn type_name(&self) -> String {
        "Tensor".to_string()
    }

    fn generic_types(&self) -> Vec<Box<dyn ValkyrieTypeModule>> {
        vec![
            Box::new(self.default.clone()),
            Box::new(ValkyrieTuple::from_literal(self.dimension.iter().cloned())),
        ]
    }
}


#[test]
fn test_broadcast() {
    let lhs = ValkyrieTensor::new(vec![2, 3, 4], 0.0);
    let rhs = ValkyrieTensor::new(vec![2, 1, 4], 0.0);
    let result: Box<dyn ValkyrieTypeModule> = Box::new(lhs.broadcast_add(&rhs));
    println!("{}", result);
    println!("{:?}", result);
}