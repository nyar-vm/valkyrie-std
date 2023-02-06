use std::sync::Arc;

use crate::{types::ValkyrieMetaType, ValkyrieList, ValkyrieType, ValkyrieValue};

impl<T> ValkyrieType for Vec<T>
where
    T: ValkyrieType,
{
    fn boxed(self) -> ValkyrieValue {
        let mut out = ValkyrieList::list();
        for item in self {
            out.extend_one(item.boxed());
        }
        ValkyrieValue::List(Arc::new(out))
    }

    fn type_info(&self) -> Arc<ValkyrieMetaType> {
        let mut this = ValkyrieMetaType::default();
        this.set_namepath("std.collection.Vector");
        // meta.mut_generic_types().push(T::type_info());
        Arc::new(this)
    }
}
