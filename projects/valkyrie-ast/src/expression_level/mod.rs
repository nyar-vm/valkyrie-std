use std::ops::Range;

use crate::{ValkyrieASTKind, ValkyrieASTNode};
use serde::{Deserialize, Serialize};
use valkyrie_errors::{FileID, FileSpan};

mod atomic_type;
mod class_type;
mod literal_type;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BinaryExpression {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnaryExpression {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AtomicExpression {
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl BinaryExpression {
    pub fn to_node(self, file: FileID, range: &Range<usize>) -> ValkyrieASTNode {
        ValkyrieASTNode { kind: ValkyrieASTKind::Binary(box self), span: FileSpan { file, head: range.start, tail: range.end } }
    }
}

impl UnaryExpression {
    pub fn to_node(self, file: FileID, range: &Range<usize>) -> ValkyrieASTNode {
        ValkyrieASTNode { kind: ValkyrieASTKind::Unary(box self), span: FileSpan { file, head: range.start, tail: range.end } }
    }
}

impl AtomicExpression {
    pub fn to_node(self, file: FileID, range: &Range<usize>) -> ValkyrieASTNode {
        ValkyrieASTNode { kind: ValkyrieASTKind::Atomic(box self), span: FileSpan { file, head: range.start, tail: range.end } }
    }
}
