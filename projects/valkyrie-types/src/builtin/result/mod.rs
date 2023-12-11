use std::sync::Arc;

use crate::{types, types::ValkyrieMetaType, ValkyrieType, ValkyrieValue};

pub struct ValkyrieSuccess<T> {
    pub value: T,
}

pub struct ValkyrieFailure<E> {
    pub error: E,
}
