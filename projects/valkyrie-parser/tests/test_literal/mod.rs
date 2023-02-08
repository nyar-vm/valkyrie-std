use super::*;

#[test]
fn test_basic() {
    run_parser(&["tests/test_literal/atomic.vk", "tests/test_literal/numbers.vk"]).unwrap();
}
