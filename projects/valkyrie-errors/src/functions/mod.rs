use std::fmt::{Debug, Display, Formatter};

use miette::NamedSource;

use crate::{ValkyrieError, ValkyrieErrorKind};

#[derive(Debug)]
pub struct DuplicateDefinition {
    name: String,
    first: NamedSource,
    second: NamedSource,
}

impl ValkyrieError {
    pub fn duplicate_type(name: String, location: String) -> Self {
        let this = DuplicateDefinition {
            name,
            first: NamedSource::new("first", (0usize, 4)),
            second: NamedSource::new("second", (9usize, 4)),
        };
        Self { kind: ValkyrieErrorKind::Duplicate(Box::new(this)) }
    }
}

impl Display for DuplicateDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
