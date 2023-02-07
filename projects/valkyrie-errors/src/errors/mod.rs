use std::{
    error::Error,
    fmt::{Debug, Display},
};

use ariadne::ReportKind;

use crate::DuplicateError;

pub mod diagnostic;

pub enum ValkyrieErrorKind {
    Duplicate(Box<DuplicateError>),
}

pub struct ValkyrieError {
    pub kind: ValkyrieErrorKind,
    pub level: ReportKind,
}
