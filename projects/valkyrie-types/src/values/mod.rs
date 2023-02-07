use std::sync::Arc;

use num::BigInt;

use crate::{types::ValkyrieMetaType, ValkyrieList, ValkyrieVariantType};

pub enum ValkyrieValue {
    Boolean(bool),
    Unsigned8(u8),
    Unsigned16(u16),
    Unsigned32(u32),
    Unsigned64(u64),
    Unsigned128(u128),
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    Integer128(i128),
    Float32(f32),
    Float64(f64),
    String(Arc<String>),
    Buffer(Arc<Vec<u8>>),
    Variant(Arc<ValkyrieVariantType>),
    List(Arc<ValkyrieList>),
}
