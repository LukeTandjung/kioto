use crate::core::editable_buffer::EditableBuffer;
use crate::core::position::Position;
use crate::core::position::Range;
use crate::core::preview_renderer::{PreviewOutput, PreviewRenderer};

/// Runtime document object used by UI adapters: one value owns both the
/// canonical editable source and the preview renderer over the same parse
/// state.
pub trait EditorDocument: EditableBuffer + PreviewRenderer {}

impl<T> EditorDocument for T where T: EditableBuffer + PreviewRenderer {}

impl EditableBuffer for Box<dyn EditorDocument> {
    fn text(&self) -> &str {
        (**self).text()
    }

    fn replace(&mut self, range: Range, new_text: &str) {
        (**self).replace(range, new_text);
    }
}

impl PreviewRenderer for Box<dyn EditorDocument> {
    fn render_preview(&self, cursor_positions: &[Position]) -> PreviewOutput {
        (**self).render_preview(cursor_positions)
    }
}
