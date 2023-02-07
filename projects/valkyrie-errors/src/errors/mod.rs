use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use miette::Diagnostic;

use crate::DuplicateDefinition;

pub enum ValkyrieErrorKind {
    Duplicate(Box<DuplicateDefinition>),
}

pub struct ValkyrieError {
    pub kind: ValkyrieErrorKind,
}

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
    // fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
    //     todo!()
    // }
    // fn severity(&self) -> Option<Severity> {
    //     todo!()
    // }
    // fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
    //     todo!()
    // }
    // fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
    //     todo!()
    // }
    // fn source_code(&self) -> Option<&dyn SourceCode> {
    //     todo!()
    // }
    // fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
    //     todo!()
    // }
    // fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
    //     todo!()
    // }
    // fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
    //     todo!()
    // }
}
