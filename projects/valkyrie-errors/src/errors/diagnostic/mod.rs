use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use crate::{ValkyrieError, ValkyrieErrorKind};

impl Error for ValkyrieError {}

impl Debug for ValkyrieError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ValkyrieErrorKind::Duplicate(v) => Debug::fmt(v, f),
        }
    }
}

impl Display for ValkyrieError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ValkyrieErrorKind::Duplicate(v) => Display::fmt(v, f),
        }
    }
}

impl Diagnostic for ValkyrieError {
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        match &self.kind {
            ValkyrieErrorKind::Duplicate(v) => v.code(),
        }
    }
    fn severity(&self) -> Option<Severity> {
        Some(self.level)
    }
    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        match &self.kind {
            ValkyrieErrorKind::Duplicate(v) => v.help(),
        }
    }
    fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        match &self.kind {
            ValkyrieErrorKind::Duplicate(v) => v.url(),
        }
    }
    fn source_code(&self) -> Option<&dyn SourceCode> {
        match &self.kind {
            ValkyrieErrorKind::Duplicate(v) => v.source_code(),
        }
    }
    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        match &self.kind {
            ValkyrieErrorKind::Duplicate(v) => v.labels(),
        }
    }
    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        match &self.kind {
            ValkyrieErrorKind::Duplicate(v) => v.related(),
        }
    }
    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        match &self.kind {
            ValkyrieErrorKind::Duplicate(v) => v.diagnostic_source(),
        }
    }
}
