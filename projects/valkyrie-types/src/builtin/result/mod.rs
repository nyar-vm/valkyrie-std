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
        match self {
            Ok(o) => ValkyrieValue::Result(Ok(Arc::new(o.boxed()))),
            Err(e) => ValkyrieValue::Result(Err(Arc::new(e.boxed()))),
        }
    }

    fn type_info(&self) -> Arc<ValkyrieMetaType> {
        todo!()
    }
}
