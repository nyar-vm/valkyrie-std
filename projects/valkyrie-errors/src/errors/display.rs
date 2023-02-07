use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use crate::{ValkyrieError, ValkyrieErrorKind};

impl Error for ValkyrieError {}

impl Debug for ValkyrieError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ValkyrieErrorKind::Duplicate(v) => Debug::fmt(v, f),
            ValkyrieErrorKind::Runtime(v) => Debug::fmt(v, f),
        }
    }
}

impl Display for ValkyrieError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ValkyrieErrorKind::Duplicate(v) => Display::fmt(v, f),
            ValkyrieErrorKind::Runtime(v) => Display::fmt(v, f),
        }
    }
}
