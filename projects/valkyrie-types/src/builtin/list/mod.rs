use std::sync::Arc;

use crate::{types::ValkyrieMetaType, ValkyrieClass, ValkyrieType, ValkyrieValue};

impl<T> ValkyrieType for Vec<T>
where
    T: ValkyrieType,
{
    fn boxed(self) -> ValkyrieValue {
        let mut out = ValkyrieClass::list();
        for item in self {
            out.extend_one(item.boxed());
        }
        ValkyrieValue::Class(Arc::new(out))
    }

    fn type_info(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.collection.Vector");
        // meta.mut_generic_types().push(T::type_info());
        Arc::new(this)
    }
}
