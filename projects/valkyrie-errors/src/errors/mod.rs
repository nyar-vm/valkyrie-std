use std::{
    ops::Range,
};

use ariadne::{Report, ReportKind};

use crate::{DuplicateError, FileID};

pub mod diagnostic;

pub type ValkyrieReport = Report<(FileID, Range<usize>)>;
pub type ValkyrieResult<T = ()> = Result<T, ValkyrieError>;

pub struct ValkyrieError {
    pub kind: ValkyrieErrorKind,
    pub level: ReportKind,
}

pub enum ValkyrieErrorKind {
    Duplicate(Box<DuplicateError>),
}

impl ValkyrieError {
    pub fn as_report(&self) -> Result<ValkyrieReport, String> {
        match &self.kind {
            ValkyrieErrorKind::Duplicate(e) => e.as_report(self.level),
        }
    }
}
