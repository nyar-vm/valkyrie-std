use super::*;
use crate::parser::valkyrie::{NumberNode, NumberVariant};

impl NumberNode {
    pub fn visit(&self, parser: &mut ValkyrieParser) -> ValkyrieResult<ValkyrieASTNode> {
        let out = match &self.variant {
            NumberVariant::DecimalNode(_) => {}
            NumberVariant::IntegerNode(s) => {
                s.string
                s.position

            }
        };
        Ok(out)
    }
}
