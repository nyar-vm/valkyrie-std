use ariadne::ReportKind;
use std::{
    collections::BTreeSet,
    error::Error,
    fmt::{Debug, Display, Formatter},
    ops::Range,
    sync::Arc,
};

use crate::{TextManager, ValkyrieError, ValkyrieErrorKind};

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
        Self { kind: ValkyrieErrorKind::Duplicate(Box::new(this)), level: ReportKind::Error }
    }
}

impl Display for DuplicateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Duplicate item `{}`", self.name)
    }
}

impl Error for DuplicateError {}

impl Error for DuplicateItem {}

impl Display for DuplicateItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("DuplicateRelated")
    }
}

#[test]
fn main() {
    use ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind, Source};

    let mut colors = ColorGenerator::new();

    // Generate & choose some colours for each of our elements
    let a = colors.next();
    let b = colors.next();
    let out = Color::Fixed(81);

    let mut text = TextManager::new("./");
    let file1 = text.add_file("src/duplicates/mod.rs");
    let file2 = text.add_file("src/errors/mod.rs");

    println!("{:#?}", text);

    Report::build(ReportKind::Error, file1, 12)
        .with_code(3)
        .with_message(format!("Incompatible types"))
        .with_label(Label::new((file1, 32..33)).with_message(format!("This is of type {}", "Nat".fg(a))).with_color(a))
        .with_label(Label::new((file1, 42..45)).with_message(format!("This is of type {}", "Str".fg(b))).with_color(b))
        .with_label(
            Label::new((file2, 11..48))
                .with_message(format!("The values are outputs of this {} expression", "match".fg(out),))
                .with_color(out),
        )
        .with_note(format!("Outputs of {} expressions must coerce to the same type", "match".fg(out)))
        .finish()
        .print(text)
        .unwrap();
}
