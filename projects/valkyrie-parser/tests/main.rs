use valkyrie_errors::{TextManager, Url, ValkyrieResult};
use valkyrie_parser::ValkyrieParser;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_basic() {
    run_parser(&["tests/basic.vk", "tests/yield.vk"]).unwrap();
}

fn run_parser(files: &[&str]) -> ValkyrieResult {
    let mut parser = ValkyrieParser::default();
    let mut text = TextManager::new("./");
    for file in files {
        println!("Parsing file: {}", Url::from_file_path(text.resolve_file(file))?);
        let basic = text.add_file(file)?;
        if let Err(e) = parser.parse_file(basic, &text.get_text(basic)) {
            e.as_report().print(&mut text)?;
        }
    }
    for error in parser.take_errors() {
        error.as_report().print(&mut text)?;
    }
    Ok(())
}
