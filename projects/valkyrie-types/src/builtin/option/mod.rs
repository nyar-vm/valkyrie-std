use std::sync::Arc;

use crate::{types::ValkyrieMetaType, ValkyrieType, ValkyrieValue, ValkyrieVariant};

impl<T> ValkyrieType for Option<T>
where
    T: ValkyrieType + 'static,
{
    fn boxed(self) -> ValkyrieValue {
        let this = ValkyrieVariant::new("std.primitive.Option".to_string());
        this.boxed()
    }
    fn type_info(&self) -> Arc<ValkyrieMetaType>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
