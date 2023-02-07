use std::sync::Arc;

use crate::{types::ValkyrieMetaType, ValkyrieType, ValkyrieValue};

impl<T> ValkyrieType for Option<T>
where
    T: ValkyrieType + 'static,
{
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn type_info(&self) -> Arc<ValkyrieMetaType>
    where
        Self: Sized,
    {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.primitive.Option");
        // meta.mut_generic_types().push(T::type_info());
        Arc::new(this)
    }
}
