use nyar_error::ReportResult;
use valkyrie_parser::ValkyrieParser;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() -> ReportResult {
    let _ = ValkyrieParser::parse_file("tests/test_expr/infix3.vk")?;

    Ok(())
}

#[test]
fn main() {
    let mut text = TextManager::new("./");
    let file1 = text.add_file("src/duplicates/parsing").unwrap();
    let file2 = text.add_file("src/errors/parsing").unwrap();

    ValkyrieError::duplicate_type(
        "Optional".to_string(),
        FileSpan { file: file1, head: 32, tail: 33 },
        FileSpan { file: file2, head: 42, tail: 45 },
    )
    .as_report()
    .print(&mut text)
    .unwrap();
    ValkyrieError::runtime_error("Optional".to_string()).as_report().print(&mut text).unwrap();
}
