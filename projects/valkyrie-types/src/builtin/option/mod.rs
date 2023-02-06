use crate::{types::ValkyrieType, ValkyrieTypeInfo};

impl<T> ValkyrieTypeInfo for Option<T>
where
    T: ValkyrieTypeInfo + 'static,
{
    fn static_info() -> ValkyrieType
    where
        Self: Sized,
    {
        let mut meta = ValkyrieType::default();
        meta.set_namepath("std.primitive.Option");
        meta.mut_generic_types().push(T::static_info());
        meta
    }
}
