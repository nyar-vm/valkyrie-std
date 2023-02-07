use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use ariadne::{Label, Report, ReportKind};

use crate::{FileID, TextManager, ValkyrieError, ValkyrieErrorKind};

#[derive(Debug)]
pub struct DuplicateError {
    king: String,
    name: String,
    this_item: TextSpan,
    last_item: TextSpan,
}

impl Display for DuplicateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Duplicate {} `{}`", self.kind, self.name)
    }
}

pub type ValkyrieReport = Report<(FileID, Range<usize>)>;

impl DuplicateError {
    pub fn as_report(&self, level: ReportKind) -> Result<ValkyrieReport, String> {
        let mut report = Report::build(level, self.this_item.file, 12).with_code(3);
        report.set_message(self.to_string());
        report.add_label(self.this_item.as_label(format!("This is the first item named `{}`", self.name)));
        report.add_label(self.last_item.as_label(format!("This is the second item named `{}`", self.name)));
        report.set_help(format!("Rename one of the items to have a unique name"));
        report.set_note(format!("Items must have unique names"));
        Ok(report.finish())
    }
}

impl ValkyrieError {
    pub fn as_report(&self) -> Result<ValkyrieReport, String> {
        match &self.kind {
            ValkyrieErrorKind::Duplicate(dup) => dup.as_report(self.level),
        }
    }
}

impl ValkyrieError {
    pub fn duplicate_type(name: String, this: TextSpan, last: TextSpan) -> Self {
        let this = DuplicateError { name, this_item: this, last_item: last };
        Self { kind: ValkyrieErrorKind::Duplicate(Box::new(this)), level: ReportKind::Error }
    }
}

#[test]
fn main() {
    use ariadne::{Color, ColorGenerator};

    let mut colors = ColorGenerator::new();

    // Generate & choose some colours for each of our elements
    let a = colors.next();
    let b = colors.next();
    let out = Color::Fixed(81);

    let mut text = TextManager::new("./");
    let file1 = text.add_file("src/duplicates/mod.rs");
    let file2 = text.add_file("src/errors/mod.rs");

    ValkyrieError::duplicate_type(
        "Optional".to_string(),
        TextSpan { file: file1, head: 32, tail: 33 },
        TextSpan { file: file2, head: 42, tail: 45 },
    )
    .as_report()
    .unwrap()
    .print(text)
    .unwrap();
}
