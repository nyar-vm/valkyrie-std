use crate::ValkyrieTypeInfo;

pub struct ValkyrieSuccess<T> {
    pub value: T,
}

pub struct ValkyrieFailure<E> {
    pub error: E,
}

impl<T, E> ValkyrieTypeInfo for Result<T, E> {}
