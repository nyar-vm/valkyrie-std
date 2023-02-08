use super::*;

impl AtomicExpression {
    pub fn to_node(self, file: FileID, range: &Range<usize>) -> ValkyrieASTNode {
        ValkyrieASTNode { kind: ValkyrieASTKind::Atomic(box self), span: FileSpan { file, head: range.start, tail: range.end } }
    }
}
