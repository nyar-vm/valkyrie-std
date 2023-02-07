use std::fmt::{Display, Formatter};

use ariadne::ReportKind;

use crate::{errors::ValkyrieReport, ValkyrieError, ValkyrieErrorKind};

#[derive(Debug)]
pub struct RuntimeError {
    message: String,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl RuntimeError {
    pub fn new(message: impl Display) -> Self {
        Self { message: message.to_string() }
    }
    pub fn as_report(&self, kind: ReportKind) -> ValkyrieReport {
        let mut report = ValkyrieReport::build(kind, 0usize, 0);
        report.set_message(self.to_string());
        report.finish()
    }
}

impl ValkyrieError {
    pub fn runtime_error(message: impl Into<String>) -> Self {
        let this = RuntimeError { message: message.into() };
        Self { kind: ValkyrieErrorKind::Runtime(Box::new(this)), level: ReportKind::Error }
    }
}
