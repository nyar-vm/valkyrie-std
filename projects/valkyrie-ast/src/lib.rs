#![feature(trivial_bounds)]
#![feature(never_type)]
#![feature(box_syntax)]

pub use builtin::{NamespaceDeclare, NamespaceKind};
use valkyrie_errors::FileSpan;

mod builtin;

pub struct ValkyrieASTNode {
    pub kind: ValkyrieASTKind,
    pub span: FileSpan,
}

pub enum ValkyrieASTKind {
    Statement(Vec<ValkyrieASTNode>),
    Namespace(Box<NamespaceDeclare>),
}
