use crate::core::position::Position;

/// A single caret. Pure data with pure operations, so it lives in `core`.
/// Grapheme-aware motion arrives with the modal layer (milestone 2).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Cursor {
    pub position: Position,
}

impl Cursor {
    pub fn new(position: Position) -> Self {
        Self { position }
    }

    pub fn set(&mut self, position: Position) {
        self.position = position;
    }
}
