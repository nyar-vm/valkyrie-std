use std::{ops::Deref, sync::Arc};

use crate::{types::ValkyrieMetaType, ValkyrieType, ValkyrieValue};

impl<T> ValkyrieType for Arc<T>
where
    T: ValkyrieType,
{
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn type_info(&self) -> Arc<ValkyrieMetaType> {
        self.as_ref().type_info()
    }
}
