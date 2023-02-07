use std::{
    fmt::{Debug, Display, Formatter},
};

use ariadne::{Color, Report, ReportKind};

use crate::{errors::ValkyrieReport, TextSpan, ValkyrieError, ValkyrieErrorKind};

mod kind;

#[derive(Copy, Clone)]
pub enum DuplicateKind {
    Type = 1002,
    Function = 1003,
    Variable = 1004,
}

#[derive(Debug)]
pub struct DuplicateError {
    kind: DuplicateKind,
    name: String,
    this_item: TextSpan,
    last_item: TextSpan,
}

impl Display for DuplicateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Duplicate {} `{}`", self.kind, self.name)
    }
}

impl DuplicateError {
    pub fn as_report(&self, level: ReportKind) -> Result<ValkyrieReport, String> {
        let mut report = Report::build(level, self.this_item.file, 12).with_code(self.kind as u32);
        report.set_message(self.to_string());
        report.add_label(
            self.this_item.as_label(format!("{:?} `{}` is defined here.", self.kind, self.name)).with_color(Color::Blue),
        );
        report.add_label(
            self.last_item
                .as_label(format!("But {} `{}` has been defined here.", self.kind, self.name))
                .with_color(Color::Cyan),
        );
        report.set_help(format!("Items must have unique names, rename one of the items to have a unique name"));
        Ok(report.finish())
    }
}

impl ValkyrieError {
    pub fn duplicate_type(name: String, this: TextSpan, last: TextSpan) -> Self {
        let this = DuplicateError { kind: DuplicateKind::Type, name, this_item: this, last_item: last };
        Self { kind: ValkyrieErrorKind::Duplicate(Box::new(this)), level: ReportKind::Error }
    }
}
