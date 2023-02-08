#[derive(Clone, Debug)]
pub struct BinaryExpression {}

#[derive(Clone, Debug)]
pub struct UnaryExpression {}

#[derive(Clone, Debug)]
pub enum AtomicExpression {
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}
