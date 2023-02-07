use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use miette::{Diagnostic, LabeledSpan, NamedSource, Severity};

use crate::{TextManager, ValkyrieError, ValkyrieErrorKind};

#[derive(Debug)]
pub struct DuplicateItem {
    name: String,
    first: (String, usize, usize),
    duplicate: (String, usize, usize),
}

pub struct Span {
    file: usize,
    head: usize,
    tail: usize,
}

impl ValkyrieError {
    pub fn duplicate_type(name: String, text: &TextManager) -> Self {
        let this = DuplicateItem {
            name,
            first: NamedSource::new("first", (0usize, 4)),
            duplicate: NamedSource::new("second", (9usize, 4)),
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
    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        todo!()
    }
}
