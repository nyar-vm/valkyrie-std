use super::*;

impl From<peginator::ParseError> for ParseError {
    fn from(e: peginator::ParseError) -> Self {
        ParseError { info: e.specifics.to_string(), span: FileSpan { file: 0, head: e.position, tail: e.position } }
    }
}
