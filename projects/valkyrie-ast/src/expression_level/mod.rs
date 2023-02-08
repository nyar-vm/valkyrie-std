use std::ops::Range;

use serde::{Deserialize, Serialize};

use valkyrie_errors::{FileID, FileSpan};

use crate::{ValkyrieASTKind, ValkyrieASTNode};

mod atomic;
mod binary;
pub mod integer;
mod unary;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ValkyrieIdentifierNode {
    name: String,
    span: FileSpan,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BinaryExpression {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnaryExpression {}
