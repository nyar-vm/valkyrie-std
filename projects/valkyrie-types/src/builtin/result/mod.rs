use crate::ValkyrieType;

pub struct ValkyrieSuccess<T> {
    pub value: T,
}

pub struct ValkyrieFailure<E> {
    pub error: E,
}

impl<T, E> ValkyrieType for Result<T, E> {}
