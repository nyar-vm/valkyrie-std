use std::{ops::Range, str::FromStr};

use valkyrie_errors::{FileID, FileSpan};

use crate::{ValkyrieASTKind, ValkyrieASTNode};

#[derive(Clone, Debug)]
pub struct NamespaceDeclare {
    pub kind: NamespaceKind,
    pub name: Vec<String>,
}

#[derive(Copy, Clone, Debug)]
pub enum NamespaceKind {
    // In the v language, there only one shared namespace
    Shared,
    // In the v language, there only one shared namespace
    Unique,
    // In the v language, there only one shared namespace
    Test,
}

impl NamespaceDeclare {
    pub fn new(kind: &str) -> Self {
        Self { kind: NamespaceKind::from_str(kind).unwrap(), name: Vec::new() }
    }
    pub fn push_name(&mut self, name: impl Into<String>) {
        self.name.push(name.into());
    }
    pub fn to_node(self, file: FileID, range: &Range<usize>) -> ValkyrieASTNode {
        ValkyrieASTNode {
            kind: ValkyrieASTKind::Namespace(box self),
            span: FileSpan { file, head: range.start, tail: range.end },
        }
    }
}

impl FromStr for NamespaceKind {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s {
            "namespace!" => Self::Shared,
            "namespace*" => Self::Test,
            _ => Self::Unique,
        };
        Ok(out)
    }
}
