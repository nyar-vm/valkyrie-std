use std::ops::Range;

use ariadne::{Report, ReportKind};

use crate::{DuplicateError, FileID, RuntimeError};

pub mod display;

pub type ValkyrieReport = Report<(FileID, Range<usize>)>;

pub type ValkyrieResult<T = ()> = Result<T, ValkyrieError>;

pub struct ValkyrieError {
    pub kind: ValkyrieErrorKind,
    pub level: ReportKind,
}

pub enum ValkyrieErrorKind {
    Duplicate(Box<DuplicateError>),
    Runtime(Box<RuntimeError>),
}

impl ValkyrieError {
    pub fn as_report(&self) -> ValkyrieReport {
        match &self.kind {
            ValkyrieErrorKind::Duplicate(e) => e.as_report(self.level),
            ValkyrieErrorKind::Runtime(e) => e.as_report(self.level),
        }
    }
}
