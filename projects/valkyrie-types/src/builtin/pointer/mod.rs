use std::sync::Arc;

use crate::{types::ValkyrieValue, ValkyrieType};

impl<T> ValkyrieType for Arc<T> {
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }
}
