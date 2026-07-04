use crate::core::position::Range;

/// The language-neutral buffer contract: text reads and primitive edits,
/// nothing else. Each supported language provides its own implementation
/// (e.g. `TypstDocument` wrapping `typst_syntax::Source`); the editor holds
/// this abstraction, never a concrete language type.
///
/// Contract: out-of-range and mid-character positions are errors defined out
/// of existence — implementations clamp to the buffer length and to the
/// previous char boundary. `clamp_range` implements that rule once.
pub trait EditableBuffer {
    fn text(&self) -> &str;

    /// The single write path: replaces `range` with `new_text`.
    fn replace(&mut self, range: Range, new_text: &str);

    fn len(&self) -> usize {
        self.text().len()
    }

    fn is_empty(&self) -> bool {
        self.text().is_empty()
    }
}

/// Clamps a range to `text`'s length and to char boundaries (flooring), the
/// shared implementation of the trait's clamping contract.
pub fn clamp_range(text: &str, range: Range) -> std::ops::Range<usize> {
    let start = clamp_offset(text, range.start.0);
    let end = clamp_offset(text, range.end.0).max(start);
    start..end
}

fn clamp_offset(text: &str, offset: usize) -> usize {
    let mut offset = offset.min(text.len());
    while offset > 0 && !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

#[cfg(test)]
#[path = "editable_buffer.test.rs"]
mod tests;
