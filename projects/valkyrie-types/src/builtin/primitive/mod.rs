use std::sync::Arc;
use crate::{types::ValkyrieMetaType, ValkyrieType};

impl ValkyrieType for u8 {
    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.f64");
        meta
    }
}

impl ValkyrieType for u16 {
    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.f64");
        meta
    }
}

impl ValkyrieType for u32 {
    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.f64");
        meta
    }
}

impl ValkyrieType for u64 {
    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.f64");
        meta
    }
}

impl ValkyrieType for u128 {
    fn static_info() -> ValkyrieMetaType
        where
            Self: Sized,
    {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.u64");
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


impl ValkyrieType for i8 {
    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.f64");
        meta
    }
}

impl ValkyrieType for i16 {
    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.f64");
        meta
    }
}

impl ValkyrieType for i32 {
    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.f64");
        meta
    }
}

impl ValkyrieType for i64 {
    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.f64");
        meta
    }
}

impl ValkyrieType for i128 {
    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.f64");
        meta
    }
}

impl ValkyrieType for isize {
    fn static_info() -> ValkyrieMetaType
        where
            Self: Sized,
    {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.u64");
        meta
    }
}

impl ValkyrieType for f32 {
    fn static_info() -> ValkyrieMetaType
        where
            Self: Sized,
    {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.u64");
        meta
    }
}

impl ValkyrieType for f64 {
    fn static_info() -> ValkyrieMetaType
        where
            Self: Sized,
    {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.u64");
        meta
    }
}

impl<T> ValkyrieType for Arc<T> {
    fn static_info() -> ValkyrieMetaType
        where
            Self: Sized,
    {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("core.Arc");
        meta
    }
}

impl ValkyrieType for String {
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
