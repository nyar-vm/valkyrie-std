use std::ops::Range;

use crate::{ValkyrieASTKind, ValkyrieASTNode};
use serde::{Deserialize, Serialize};
use valkyrie_errors::{FileID, FileSpan};

mod atomic;
mod binary;
mod unary;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AtomicExpression {
    Null,
    Boolean(bool),
    Identifier(String),
    Integer(String),
    Decimal(String),
    PureString(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BinaryExpression {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnaryExpression {}
