use serde::Serialize;
use std::ops::Sub;

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub(crate) struct Span {
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) index: usize,
    pub(crate) size: usize,
}

impl Sub for Span {
    type Output = Span;

    fn sub(self, other: Span) -> Span {
        std::debug_assert!(self.index + self.size >= other.index);
        Span {
            line: self.line,
            column: self.column,
            index: self.index,
            size: self.index + self.size - other.index,
        }
    }
}
