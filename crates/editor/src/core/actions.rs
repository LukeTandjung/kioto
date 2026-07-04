use crate::core::editable_buffer::EditableBuffer;
use crate::core::position::{Position, Range};

/// Stateless primitive edit operations. `CoreActions` owns no editor data,
/// mode, cursors, GPUI state, or persistence — it only performs primitive
/// edits against the canonical buffer for the active language.
pub struct CoreActions;

impl CoreActions {
    pub fn insert_chars<B: EditableBuffer>(buffer: &mut B, at: Position, text: &str) {
        buffer.replace(Range::caret(at), text);
    }

    pub fn replace_chars<B: EditableBuffer>(buffer: &mut B, range: Range, text: &str) {
        buffer.replace(range, text);
    }

    pub fn delete_chars<B: EditableBuffer>(buffer: &mut B, range: Range) {
        buffer.replace(range, "");
    }
}

#[cfg(test)]
#[path = "actions.test.rs"]
mod tests;
