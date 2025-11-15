/// This file defines Span.
use crate::intern_pool::PathId;
use serde::Serialize;
use std::ops::Sub;

/// A Span holds the file path and a text span within that file.
/// So using a Span, you can locate a specific chunk of the text.
/// This is used to make error messages specific.
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub(crate) struct Span {
    /// The interned file path.
    pub(crate) path: PathId,
    /// Text span.
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) index: usize,
    /// If size is 0, the Span is path-only.
    pub(crate) size: usize,
}

impl Span {
    /// Sometimes, an error occurs not because of a particular
    ///     part of a file. So a path-only Span can be created.
    /// Internally, all spans with a size of 0 are path-only.
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

    /// It's useful to implement sub for Span because
    ///     spans can be merged.
    /// Sub is used instead of plus because the order matters.
    /// Note that both spans must be in the same file and the
    ///     subtracting span must appear before the end of the
    ///     subtracted span.
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
