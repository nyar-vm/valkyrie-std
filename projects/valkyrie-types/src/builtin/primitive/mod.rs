use crate::{types::ValkyrieType, ValkyrieTypeInfo};

impl ValkyrieTypeInfo for f64 {
    fn static_info() -> ValkyrieType {
        let mut meta = ValkyrieType::default();
        meta.set_namepath("std.primitive.f64");
        meta
    }
}

impl ValkyrieTypeInfo for usize {
    fn static_info() -> ValkyrieType
    where
        Self: Sized,
    {
        let mut meta = ValkyrieType::default();
        meta.set_namepath("std.primitive.u64");
        meta
    }
}

impl<T> ValkyrieTypeInfo for Vec<T>
where
    T: ValkyrieTypeInfo,
{
    fn static_info() -> ValkyrieType {
        let mut meta = ValkyrieType::default();
        meta.set_namepath("std.collection.Vector");
        meta.mut_generic_types().push(T::static_info());
        meta
    }
}
