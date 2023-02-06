use crate::{
    types::{ValkyrieMetaType, ValkyrieValue},
    ValkyrieType,
};
use std::sync::Arc;

impl ValkyrieType for u8 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Unsigned8(self)
    }

    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.Unsigned8");
        meta
    }
}

impl ValkyrieType for u16 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Unsigned16(self)
    }

    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.Unsigned16");
        meta
    }
}

impl ValkyrieType for u32 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Unsigned32(self)
    }

    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.Unsigned32");
        meta
    }
}

impl ValkyrieType for u64 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Unsigned64(self)
    }

    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.Unsigned64");
        meta
    }
}

impl ValkyrieType for u128 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Unsigned128(self)
    }

    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.Unsigned128");
        meta
    }
}

impl ValkyrieType for usize {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Unsigned64(self as u64)
    }

    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.Unsigned64");
        meta
    }
}

impl ValkyrieType for i8 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Integer8(self)
    }

    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.Integer8");
        meta
    }
}

impl ValkyrieType for i16 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Integer16(self)
    }

    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.Integer16");
        meta
    }
}

impl ValkyrieType for i32 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Integer32(self)
    }

    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.Integer32");
        meta
    }
}

impl ValkyrieType for i64 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Integer64(self)
    }

    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.Integer64");
        meta
    }
}

impl ValkyrieType for i128 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Integer128(self)
    }

    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.Integer128");
        meta
    }
}

impl ValkyrieType for isize {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Integer64(self as i64)
    }

    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.Integer64");
        meta
    }
}

impl ValkyrieType for f32 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Float32(self)
    }

    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.Float32");
        meta
    }
}

impl ValkyrieType for f64 {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::Float64(self)
    }

    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.Float64");
        meta
    }
}

// impl<T> ValkyrieType for Arc<T> {
//     fn static_info() -> ValkyrieMetaType
//         where
//             Self: Sized,
//     {
//         let mut meta = ValkyrieMetaType::default();
//         meta.set_namepath("core.Arc");
//         meta
//     }
// }

impl ValkyrieType for String {
    fn boxed(self) -> ValkyrieValue {
        ValkyrieValue::String(Arc::new(self))
    }

    fn static_info() -> ValkyrieMetaType
    where
        Self: Sized,
    {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.primitive.UTF8Text");
        meta
    }
}

impl<T> ValkyrieType for Vec<T>
where
    T: ValkyrieType,
{
    fn boxed(self) -> ValkyrieValue {
        todo!()
    }

    fn static_info() -> ValkyrieMetaType {
        let mut meta = ValkyrieMetaType::default();
        meta.set_namepath("std.collection.Vector");
        meta.mut_generic_types().push(T::static_info());
        meta
    }
}
