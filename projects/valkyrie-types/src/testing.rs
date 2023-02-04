use crate::ValkyrieType;

#[track_caller]
pub fn assert_type<T>(value: T, short: &str, long: &str) where T: ValkyrieType + 'static {
    let v = Box::new(value) as Box<dyn ValkyrieType>;
    assert_eq!(format!("{}", v), short);
    assert_eq!(format!("{:?}", v), long);
}