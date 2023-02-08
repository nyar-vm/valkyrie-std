use super::*;

impl AtomicExpression {
    pub fn to_node(self, file: FileID, range: &Range<usize>) -> ValkyrieASTNode {
        ValkyrieASTNode { kind: ValkyrieASTKind::Atomic(box self), span: FileSpan { file, head: range.start, tail: range.end } }
    }
}

impl ValkyrieASTNode {
    pub fn null(file: FileID, range: &Range<usize>) -> Self {
        AtomicExpression::Null.to_node(file, range)
    }
    pub fn boolean(b: bool, file: FileID, range: &Range<usize>) -> Self {
        AtomicExpression::Boolean(b).to_node(file, range)
    }
}
