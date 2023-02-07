use std::sync::Arc;

use crate::{types, types::ValkyrieMetaType, ValkyrieType, ValkyrieValue};

pub struct ValkyrieSuccess<T> {
    pub value: T,
}

pub struct ValkyrieFailure<E> {
    pub error: E,
}

impl<T, E> ValkyrieType for Result<T, E>
where
    T: ValkyrieType,
    E: ValkyrieType,
{
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn type_info(&self) -> Arc<ValkyrieMetaType> {
        todo!()
    }
}
