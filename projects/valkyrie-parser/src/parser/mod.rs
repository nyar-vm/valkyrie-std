use std::{ops::Range, path::PathBuf};

use peginator::PegParser;

use valkyrie_ast::{NamespaceDeclare, ValkyrieASTNode};
use valkyrie_errors::{FileID, ParseError, ValkyrieError, ValkyrieResult};

use crate::{
    parser::valkyrie::{IdentifierNode, LetStatement, NamespaceDeclareNode, VkParser, VkStatements},
    ValkyrieParser,
};

mod expression;
#[allow(non_camel_case_types)]
mod valkyrie;

pub struct ParseContext {
    source: String,
    path: PathBuf,
}

impl ValkyrieParser {
    pub fn parse_file(&mut self, file: FileID, text: &str) -> ValkyrieResult<Vec<ValkyrieASTNode>> {
        self.file = file;
        let stmts = match VkParser::parse(text) {
            Ok(o) => o.statements,
            Err(e) => Err(ParseError::from(e).with_file(file))?,
        };
        let mut out = vec![];
        for s in stmts {
            s.visit(self, &mut out)?;
        }
        Ok(out)
    }
    pub fn take_errors(&mut self) -> Vec<ValkyrieError> {
        std::mem::take(&mut self.errors)
    }
    pub fn push_error(&self, msg: impl Into<String>, span: Range<usize>) -> ParseError {
        todo!()
        // let path = Url::from_file_path(&self.path).unwrap();
        // let path = path.to_string();
        // message: msg.into(), file: NamedSource::new(path, self.source.clone()), span
        // ParseError { info: "".to_string(), span: Default::default() }
    }
}

impl VkStatements {
    pub fn visit(&self, parser: &mut ValkyrieParser, out: &mut Vec<ValkyrieASTNode>) -> ValkyrieResult {
        match self {
            VkStatements::Semicolon(_) => {}
            VkStatements::ControlFlowNode(_) => {
                todo!()
            }
            VkStatements::DefStatement(_) => {
                todo!()
            }
            VkStatements::ExpressionNode(v) => out.push(v.visit(parser)?),
            VkStatements::LetStatement(v) => out.push(v.visit(parser)?),
            VkStatements::LoopStatement(_) => {
                todo!()
            }
            VkStatements::NamespaceDeclareNode(v) => out.push(v.visit(parser)?),

            VkStatements::WhileStatement(_) => {
                todo!()
            }
        }
        Ok(())
    }
}

impl NamespaceDeclareNode {
    pub fn visit(&self, parser: &mut ValkyrieParser) -> ValkyrieResult<ValkyrieASTNode> {
        let mut out = NamespaceDeclare::new(&self.kw);
        for name in &self.namespace.path {
            out.push_name(name.get_identifier());
        }
        Ok(out.to_node(parser.file, &self.namespace.position))
    }
}

impl LetStatement {
    pub fn visit(&self, parser: &mut ValkyrieParser) -> ValkyrieResult<ValkyrieASTNode> {
        todo!()
    }
}

impl IdentifierNode {
    pub fn get_identifier(&self) -> String {
        self.string.to_string()
    }
}

impl ValkyrieParser {}
