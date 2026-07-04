use super::CoreActions;
use crate::core::editable_buffer::{EditableBuffer, clamp_range};
use crate::core::position::{Position, Range};

/// Minimal String-backed buffer proving the trait is implementable without
/// any language machinery.
#[derive(Default)]
struct PlainBuffer {
    text: String,
}

impl EditableBuffer for PlainBuffer {
    fn text(&self) -> &str {
        &self.text
    }

    fn replace(&mut self, range: Range, new_text: &str) {
        let clamped = clamp_range(&self.text, range);
        self.text.replace_range(clamped, new_text);
    }
}

#[test]
fn insert_replace_delete_round_trip() {
    let mut buffer = PlainBuffer::default();
    CoreActions::insert_chars(&mut buffer, Position(0), "hello world");
    assert_eq!(buffer.text(), "hello world");

    CoreActions::replace_chars(&mut buffer, Range::new(Position(0), Position(5)), "goodbye");
    assert_eq!(buffer.text(), "goodbye world");

    CoreActions::delete_chars(&mut buffer, Range::new(Position(7), Position(13)));
    assert_eq!(buffer.text(), "goodbye");
}

#[test]
fn edits_clamp_rather_than_panic() {
    let mut buffer = PlainBuffer::default();
    CoreActions::insert_chars(&mut buffer, Position(99), "a💝");
    assert_eq!(buffer.text(), "a💝");

    // Deleting a range that starts inside the emoji floors to its boundary.
    CoreActions::delete_chars(&mut buffer, Range::new(Position(2), Position(99)));
    assert_eq!(buffer.text(), "a");
}
