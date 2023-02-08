const TABLES: &str = r#"

"#;

#[test]
fn debug_table() -> Result<()> {
    let ast: ASTKind = ASTDump::parse(TABLES);
    ast.save("tests/test_literal/table.vk")
}
