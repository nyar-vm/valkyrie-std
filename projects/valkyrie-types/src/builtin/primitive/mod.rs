use crate::{types::ValkyrieMetaType, ValkyrieType};

impl ValkyrieType for f64 {
    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.f64");
        meta
    }
}

impl ValkyrieType for usize {
    fn static_info() -> ValkyrieMetaType
    where
        Self: Sized,
    {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.u64");
        meta
    }
}

impl<T> ValkyrieType for Vec<T>
where
    T: ValkyrieType,
{
    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.collection.Vector");
        meta.mut_generic_types().push(T::static_info());
        meta
    }
}
