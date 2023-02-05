use crate::{types::ValkyrieMetaType, ValkyrieType};

impl<T> ValkyrieType for Option<T>
where
    T: ValkyrieType + 'static,
{
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
