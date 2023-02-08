#![feature(trivial_bounds)]
#![feature(never_type)]
#![feature(box_syntax)]

pub use package_level::{NamespaceDeclare, NamespaceKind};
use valkyrie_errors::FileSpan;
pub use values::{AtomicExpression, BinaryExpression, UnaryExpression};

mod package_level;
mod testing;
mod values;

#[derive(Clone, Debug)]
pub struct ValkyrieASTNode {
    pub kind: ValkyrieASTKind,
    pub span: FileSpan,
}

#[derive(Clone)]
pub enum ValkyrieASTKind {
    Statement(Vec<ValkyrieASTNode>),
    Namespace(Box<NamespaceDeclare>),
    Atomic(Box<AtomicExpression>),
    Unary(Box<UnaryExpression>),
    Binary(Box<BinaryExpression>),
}
