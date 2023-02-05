use crate::{types::ValkyrieMetaType, ValkyrieType};

impl ValkyrieType for f64 {}

impl ValkyrieType for usize {
    fn static_type() -> ValkyrieMetaType
    where
        Self: Sized,
    {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.u64");

        meta
    }
}

impl<T> ValkyrieType for Vec<T> {}
