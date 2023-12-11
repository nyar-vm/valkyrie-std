use peginator::ParseError;

use super::*;

impl From<ParseError> for SyntaxError {
    fn from(e: ParseError) -> Self {
        SyntaxError { info: e.specifics.to_string(), span: FileSpan { file: 0, head: e.position, tail: e.position } }
    }
}
