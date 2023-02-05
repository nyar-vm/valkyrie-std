use crate::ValkyrieTypeModule;

#[track_caller]
pub fn assert_type<T>(value: T, short: &str, long: &str) where T: ValkyrieTypeModule + 'static {
    let v = Box::new(value) as Box<dyn ValkyrieTypeModule>;
    assert_eq!(format!("{}", v), short);
    assert_eq!(format!("{:?}", v), long);
}

