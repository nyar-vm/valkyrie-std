use std::{
    fs::read_to_string,
    ops::Range,
    path::{Path, PathBuf},
};

use peginator::PegParser;

use valkyrie_errors::{FileID, ParseError, Url, ValkyrieError, ValkyrieResult};

use crate::{
    parser::valkyrie::{VkParser, VkStatements},
    ValkyrieParser,
};

#[allow(non_camel_case_types)]
mod valkyrie;

pub struct ParseContext {
    source: String,
    path: PathBuf,
}

impl ValkyrieParser {
    pub fn parse_file<P: AsRef<Path>>(&mut self, file: FileID, text: &str) -> ValkyrieResult<()> {
        self.file = file;
        let stmts = match VkParser::parse(text) {
            Ok(o) => o.statements,
            Err(e) => Err(ParseError::from(e).with_file(file))?,
        };
        self.parse_root(&stmts)?;
        Ok(())
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

impl ValkyrieParser {
    pub fn parse_root(&mut self, root: &[VkStatements]) -> ValkyrieResult<()> {
        for stmt in root {
            println!("{:#?}", stmt);
        }
        Ok(())
    }
}

impl ValkyrieParser {}
