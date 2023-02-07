use std::{
    collections::BTreeSet,
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::Range,
    sync::Arc,
};

use miette::{Diagnostic, LabeledSpan, Severity, SourceCode};
use url::Url;

use crate::{ValkyrieError, ValkyrieErrorKind};

#[derive(Debug)]
pub struct DuplicateError {
    name: String,
    this_item: DuplicateItem,
    last_item: Option<DuplicateItem>,
}

#[derive(Debug)]
pub struct DuplicateItem {
    text: Arc<String>,
    span: Range<usize>,
}

impl DuplicateItem {
    pub fn new(text: Arc<String>, span: (usize, usize)) -> Self {
        Self { text, span: Range { start: span.0, end: span.1 } }
    }
}

// impl FileSpan {
//     pub fn resolve_source(&self, text: &TextManager) -> NamedSource {}
// }

impl ValkyrieError {
    pub fn duplicate_type(name: String, this: (usize, usize), last: (usize, usize)) -> Self {
        let this = DuplicateError {
            name,
            this_item: DuplicateItem::new(Arc::new(include_str!("mod.rs").to_string()), this),
            last_item: Some(DuplicateItem::new(Arc::new(include_str!("../errors/mod.rs").to_string()), last)),
        };
        Self { kind: ValkyrieErrorKind::Duplicate(Box::new(this)), level: Severity::Error }
    }
}

impl Display for DuplicateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Duplicate item `{}`", self.name)
    }
}

impl Error for DuplicateError {}

impl Diagnostic for DuplicateError {
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new("E0001"))
    }
    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new("Remove one of the duplicate item"))
    }
    fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new("file:///C:/Users/Owner/Documents/Programming/Rust/valkyrie/projects/valkyrie-errors/src/errors/mod.rs"))
    }
    fn source_code(&self) -> Option<&dyn SourceCode> {
        Some(&self.this_item.text)
    }
    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        let this = LabeledSpan::new(Some("last defined at".to_string()), self.this_item.span.start, self.this_item.span.end);
        Some(Box::new(vec![this].into_iter()))
    }
    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        Some(Box::new(self.last_item.iter().map(|x| x as &dyn Diagnostic)))
    }
}

impl Error for DuplicateItem {}

impl Display for DuplicateItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("DuplicateRelated")
    }
}

impl Diagnostic for DuplicateItem {}

#[test]
fn main() {
    use ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind, Source};

    let mut colors = ColorGenerator::new();

    // Generate & choose some colours for each of our elements
    let a = colors.next();
    let b = colors.next();
    let out = Color::Fixed(81);

    Report::build(ReportKind::Error, "mod.rs", 12)
        .with_code(3)
        .with_message(format!("Incompatible types"))
        .with_label(Label::new(("mod.rs", 32..33)).with_message(format!("This is of type {}", "Nat".fg(a))).with_color(a))
        .with_label(Label::new(("mod.rs", 42..45)).with_message(format!("This is of type {}", "Str".fg(b))).with_color(b))
        .with_label(
            Label::new(("mod.rs", 11..48))
                .with_message(format!("The values are outputs of this {} expression", "match".fg(out),))
                .with_color(out),
        )
        .with_note(format!("Outputs of {} expressions must coerce to the same type", "match".fg(out)))
        .finish()
        .print(("mod.rs", Source::from(include_str!("mod.rs"))))
        .unwrap();
}
