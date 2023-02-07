use valkyrie_errors::{TextManager, TextSpan, ValkyrieError};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn main() {
    use ariadne::{Color, ColorGenerator};

    let mut colors = ColorGenerator::new();

    // Generate & choose some colours for each of our elements
    let _a = colors.next();
    let _b = colors.next();
    let _out = Color::Fixed(81);

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
