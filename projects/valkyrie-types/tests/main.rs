use valkyrie_types::testing::assert_type;
use valkyrie_types::ValkyrieType;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test_option() {
    let a: Option<usize> = Some(0);
    let b: Option<usize> = None;

    assert_type(a, "Option[u64]", "std::primitive::Option[std::primitive::u64]");
    format!("{}", Box::new(b) as Box<dyn ValkyrieType>);
    format!("{:?}", Box::new(b) as Box<dyn ValkyrieType>);
}

