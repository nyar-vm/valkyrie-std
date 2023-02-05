use valkyrie_types::testing::assert_type;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_primitive() {
    let value: usize = 0;
    assert_type(value, "u64", "std::primitive::u64");
}

#[test]
fn test_option() {
    let value: Option<usize> = Some(0);
    assert_type(value, "Option[u64]", "std::primitive::Option[std::primitive::u64]");
    let value: Option<usize> = None;
    assert_type(value, "Option[u64]", "std::primitive::Option[std::primitive::u64]");
    let value: Option<Option<usize>> = Some(None);
    assert_type(value, "Option[Option[u64]]", "std::primitive::Option[std::primitive::Option]");
}
