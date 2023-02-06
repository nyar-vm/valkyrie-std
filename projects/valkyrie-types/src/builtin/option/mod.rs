use std::sync::Arc;

use crate::{
    types::{ValkyrieMetaType, ValkyrieValue},
    ValkyrieType,
};

impl<T> ValkyrieType for Option<T>
where
    T: ValkyrieType + 'static,
{
    fn boxed(self) -> ValkyrieValue {
        match self {
            Some(s) => ValkyrieValue::Option(Some(Arc::new(s.boxed()))),
            None => ValkyrieValue::Option(None),
        }
    }

    fn static_info() -> ValkyrieMetaType
    where
        Self: Sized,
    {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.Option");
        meta.mut_generic_types().push(T::static_info());
        meta
    }
}
