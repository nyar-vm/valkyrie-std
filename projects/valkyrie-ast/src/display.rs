use std::fmt::{Debug, Display, Formatter};

use crate::ValkyrieASTKind;

impl Debug for ValkyrieASTKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieASTKind::Statement(v) => Debug::fmt(v, f),
            ValkyrieASTKind::Namespace(v) => Debug::fmt(v, f),
            ValkyrieASTKind::Atomic(v) => Debug::fmt(v, f),
            ValkyrieASTKind::Unary(v) => Debug::fmt(v, f),
            ValkyrieASTKind::Binary(v) => Debug::fmt(v, f),
        }
    }
}

// impl Display for ValkyrieASTKind {
//
// }
