use crate::ValkyrieVariant;

pub enum ValkyrieResult<T, E> {
    Success(ValkyrieSuccess<T>),
    Failure(ValkyrieFailure<E>),
}

pub struct ValkyrieSuccess<T> {
    pub value: T,
}

pub struct ValkyrieFailure<E> {
    pub error: E,
}


impl ValkyrieVariant for ValkyrieResult<T, E> {
    fn type_names() -> Vec<String> {
        todo!()

    }
}

