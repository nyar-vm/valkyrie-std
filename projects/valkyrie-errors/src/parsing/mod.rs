use std::{
    char::ParseCharError,
    fmt::{Display, Formatter},
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};
#[cfg(feature = "peginator")]
mod for_peginator;

use ariadne::{Color, ReportKind};

use crate::{FileID, FileSpan, ValkyrieError, ValkyrieErrorKind, ValkyrieReport};

#[derive(Clone, Debug)]
pub struct ParseError {
    info: String,
    span: FileSpan,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.info)
    }
}

impl ParseError {
    pub fn new(info: impl Into<String>) -> Self {
        Self { span: FileSpan::default(), info: info.into() }
    }
    pub fn with_file(mut self, file: FileID) -> Self {
        self.span.file = file;
        self
    }
    pub fn with_range(mut self, range: (usize, usize)) -> Self {
        self.span.head = range.0;
        self.span.tail = range.1;
        self
    }
    pub fn as_report(&self, kind: ReportKind) -> ValkyrieReport {
        let mut report = ValkyrieReport::build(kind, self.span.file, self.span.head);
        report.set_message(self.to_string());
        report.add_label(self.span.as_label(self.to_string()).with_color(Color::Red));
        report.finish()
    }
}

impl From<ParseError> for ValkyrieError {
    fn from(value: ParseError) -> Self {
        ValkyrieError { kind: ValkyrieErrorKind::Parsing(Box::new(value)), level: ReportKind::Error }
    }
}

macro_rules! wrap_parse_error {
    ($($type:ty),*) => {
        $(
            impl From<$type> for ValkyrieError {
                fn from(value: $type) -> Self {
                    ParseError::new(value.to_string()).into()
                }
            }
        )*
    };
}

wrap_parse_error!(ParseIntError, ParseFloatError, ParseBoolError, ParseCharError, url::ParseError);

#[cfg(feature = "peginator")]
wrap_parse_error!(peginator::ParseError);
