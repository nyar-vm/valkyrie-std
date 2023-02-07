use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::Range,
    sync::Arc,
};

use miette::{Diagnostic, Severity, SourceCode};

use crate::{ValkyrieError, ValkyrieErrorKind};

#[derive(Debug)]
pub struct DuplicateItem {
    name: String,
    this_item: Arc<String>,
    this_span: Range<usize>,
    last_item: Arc<String>,
    last_span: Range<usize>,
}

// impl FileSpan {
//     pub fn resolve_source(&self, text: &TextManager) -> NamedSource {}
// }

impl ValkyrieError {
    pub fn duplicate_type(name: String) -> Self {
        let this = DuplicateItem {
            name,
            this_item: Arc::new("this_item".to_string()),
            this_span: Range { start: 20, end: 30 },
            last_item: Arc::new("last_item".to_string()),
            last_span: Range { start: 0, end: 10 },
        };
        Self { kind: ValkyrieErrorKind::Duplicate(Box::new(this)), level: Severity::Error }
    }
}

impl Display for DuplicateItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for DuplicateItem {}

impl Diagnostic for DuplicateItem {
    fn source_code(&self) -> Option<&dyn SourceCode> {
        todo!()
    }
}
