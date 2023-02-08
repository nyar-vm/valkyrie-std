use std::ops::Range;

use num::BigInt;
use serde::{Deserialize, Serialize};

use valkyrie_errors::{FileID, FileSpan};

use crate::{ValkyrieASTKind, ValkyrieASTNode};

mod atomic;
mod binary;
mod unary;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValkyrieIntegerNode {
    hint: ValkyrieIdentifierNode,
    value: BigInt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValkyrieIdentifierNode {
    name: String,
    span: FileSpan,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BinaryExpression {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnaryExpression {}
