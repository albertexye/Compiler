use crate::intern_pool::PathId;
use serde::Serialize;
use std::ops::Sub;

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub(crate) struct Span {
    pub(crate) path: PathId,
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) index: usize,
    pub(crate) size: usize,
}

impl Span {
    pub(crate) fn path_only(path_id: PathId) -> Span {
        Span {
            path: path_id,
            line: 0,
            column: 0,
            index: 0,
            size: 0,
        }
    }
}

impl Sub for Span {
    type Output = Span;

    fn sub(self, other: Span) -> Span {
        std::debug_assert!(self.index + self.size >= other.index);
        std::debug_assert!(self.path == other.path);
        Span {
            path: self.path,
            line: self.line,
            column: self.column,
            index: self.index,
            size: self.index + self.size - other.index,
        }
    }
}
