use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use miette::{Diagnostic, LabeledSpan, Severity, SourceCode};

use crate::DuplicateItem;

pub mod diagnostic;

pub enum ValkyrieErrorKind {
    Duplicate(Box<DuplicateItem>),
}

pub struct ValkyrieError {
    pub kind: ValkyrieErrorKind,
    pub level: Severity,
}
