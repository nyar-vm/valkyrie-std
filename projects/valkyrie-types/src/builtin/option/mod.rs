use crate::{types::ValkyrieMetaType, ValkyrieType, ValkyrieVariantType};

impl<T> ValkyrieType for Option<T>
where
    T: ValkyrieType + Clone + Default + 'static,
{
    fn static_type() -> ValkyrieMetaType
    where
        Self: Sized,
    {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.Option");
        meta.mut_generic_types().push(T::static_type());
        meta
    }
}
