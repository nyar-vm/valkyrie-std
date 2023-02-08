use valkyrie_ast::{BinaryExpression, UnaryExpression};

use crate::parser::valkyrie::{ExprNode, ExpressionNode, TermNode};

use super::*;

impl ExpressionNode {
    pub fn visit(&self, parser: &mut ValkyrieParser) -> ValkyrieResult<ValkyrieASTNode> {
        let lhs = self.expr.visit(parser)?;
        if self.infix.is_empty() {
            return Ok(lhs);
        }
        else {
            let binary = BinaryExpression {};
            Ok(binary.to_node(parser.file, &Range::default()))
        }
    }
}

impl ExprNode {
    pub fn visit(&self, parser: &mut ValkyrieParser) -> ValkyrieResult<ValkyrieASTNode> {
        if self.prefix.is_empty() && self.suffix.is_empty() {
            // must automic
            self.term.visit(parser)
        }
        else {
            let unary = UnaryExpression {};
            Ok(unary.to_node(parser.file, &Range::default()))
        }
    }
}

impl TermNode {
    pub fn visit(&self, parser: &mut ValkyrieParser) -> ValkyrieResult<ValkyrieASTNode> {
        match self {
            TermNode::ExpressionNode(_) => {
                todo!()
            }
            TermNode::IdentifierNode(_) => {
                todo!()
            }
            TermNode::NumberNode(_) => {
                todo!()
            }
            TermNode::StringNode(_) => {
                todo!()
            }
        }
    }
}
