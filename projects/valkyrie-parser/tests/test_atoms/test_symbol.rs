use super::*;

const SYMBOLS: &str = r#"
true;
false;
a;
a::b;
a::b::c;
我;
我::的;
我::的::库;
"#;

#[test]
fn debug_symbols() -> Result<()> {
    let ast: ASTKind = ASTDump::parse(SYMBOLS);
    ast.save("tests/test_atoms/debug_symbols.clj")
}
