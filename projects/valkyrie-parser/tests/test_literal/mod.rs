use super::*;

#[test]
fn test_basic() {
    run_parser(&["tests/test_literal/atomic.vk"]).unwrap();
}
