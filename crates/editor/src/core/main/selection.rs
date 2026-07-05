use crate::core::position::Position;

/// A directed selection: `from_position` is the anchor, `to_position` the
/// moving end, so `to < from` is a valid backwards selection.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Selection {
    pub from_position: Position,
    pub to_position: Position,
}

impl Selection {
    pub fn new(from_position: Position, to_position: Position) -> Self {
        Self {
            from_position,
            to_position,
        }
    }

    /// Updates either endpoint. `None` for `from_position` means the start
    /// of the file; `None` for `to_position` means the end of the file
    /// (`Position(usize::MAX)`, clamped by the buffer at use).
    pub fn set(&mut self, from_position: Option<Position>, to_position: Option<Position>) {
        self.from_position = from_position.unwrap_or(Position(0));
        self.to_position = to_position.unwrap_or(Position(usize::MAX));
    }
}
