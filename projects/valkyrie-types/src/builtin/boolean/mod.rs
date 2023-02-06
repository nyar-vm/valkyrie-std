use crate::types::{ValkyrieType, ValkyrieValue};
use crate::ValkyrieTypeInfo;

impl ValkyrieTypeInfo for bool {
    fn static_info() -> ValkyrieType {
        let mut meta = ValkyrieType::default();
        meta.set_namepath("std.primitive.bool");
        meta
    }
}

pub struct ValkyrieFunction {
    document: String,
    parameters: Vec<ValkyrieType>,
    return_type: ValkyrieType,
    function_ptr: ValkyrieFunctionInstance,
}

pub enum ValkyrieFunctionInstance {
    Normal {
        apply: fn(Vec<ValkyrieValue>) -> ValkyrieValue
    },
    Curry {
        apply: fn(Vec<ValkyrieValue>) -> ValkyrieValue,
        parameters: Vec<ValkyrieValue>,
    },
}

// std.primitive.Boolean
pub fn not(p: Vec<ValkyrieValue>) -> ValkyrieType {
    match p {
        ValkyrieValue::Boolean(p) => {
            ValkyrieType::from(!p)
        }
        _ => panic!("Invalid type"),
    }
}

pub fn and(p: bool, q: bool) -> bool {
    p && q
}

pub fn or(p: bool, q: bool) -> bool {
    p || q
}

pub fn logic_gate(p: bool, q: bool, mask: u8) -> Result<bool, String> {
    let ok = match mask {
        0 => false,
        1 => p && q,
        2 => p && !q,
        3 => p,
        4 => !p && q,
        5 => q,
        6 => p ^ q,
        7 => p || q,
        8 => !p && !q,
        9 => p == q,
        10 => !q,
        11 => p || !q,
        12 => !p,
        13 => !p || q,
        14 => !p || !q,
        15 => true,
        _ => return Err(format!("Invalid mask: {}", mask)),
    };
    Ok(ok)
}