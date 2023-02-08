#![feature(trivial_bounds)]
#![feature(never_type)]
#![feature(box_syntax)]

use serde::{Deserialize, Serialize};

use valkyrie_errors::FileSpan;

pub use crate::{
    expression_level::{AtomicExpression, BinaryExpression, UnaryExpression},
    package_level::{NamespaceDeclare, NamespaceKind},
};

mod display;
mod expression_level;
mod package_level;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValkyrieASTNode {
    pub kind: ValkyrieASTKind,
    pub span: FileSpan,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ValkyrieASTKind {
    Statement(Vec<ValkyrieASTNode>),
    Namespace(Box<NamespaceDeclare>),
    Atomic(Box<AtomicExpression>),
    Unary(Box<UnaryExpression>),
    Binary(Box<BinaryExpression>),
}
