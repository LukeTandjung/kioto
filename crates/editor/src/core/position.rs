/// A byte offset into a document's UTF-8 source text. Line/column values are
/// derived, never stored.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(pub usize);

/// A byte range over the source text, normalized so `start <= end`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    /// Builds a range from two endpoints in either order.
    pub fn new(a: Position, b: Position) -> Self {
        if a <= b {
            Self { start: a, end: b }
        } else {
            Self { start: b, end: a }
        }
    }

    pub fn caret(position: Position) -> Self {
        Self {
            start: position,
            end: position,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn len(&self) -> usize {
        self.end.0 - self.start.0
    }
}

impl From<Range> for std::ops::Range<usize> {
    fn from(range: Range) -> Self {
        range.start.0..range.end.0
    }
}

#[cfg(test)]
#[path = "position.test.rs"]
mod tests;
