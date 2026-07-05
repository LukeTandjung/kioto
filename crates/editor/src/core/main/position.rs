/// A byte offset into a document's UTF-8 source text. Line/column values are
/// derived, never stored.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(pub usize);

/// A normalized span over source text. It owns the editor's source-range
/// boundary rules: unordered endpoints normalize, and callers can ask for a
/// byte range or slice only in the context of concrete text, where the span
/// is clamped to the document and to UTF-8 character boundaries.
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

    /// Returns a safe byte range for this span in `text`, clamped to the text
    /// length and floored to UTF-8 character boundaries. This is the common
    /// path before slicing or replacing source text.
    pub fn byte_range_in(&self, text: &str) -> std::ops::Range<usize> {
        let start = clamp_offset(text, self.start.0);
        let end = clamp_offset(text, self.end.0).max(start);
        start..end
    }

    /// Returns the text covered by this span after applying the same clamping
    /// rules as [`byte_range_in`](Self::byte_range_in).
    pub fn slice<'a>(&self, text: &'a str) -> &'a str {
        &text[self.byte_range_in(text)]
    }

    /// Converts an IME/platform UTF-16 range into a source range. Offsets in
    /// the middle of a surrogate pair floor to the previous source boundary,
    /// matching the source-edit clamping rule.
    pub fn from_utf16(text: &str, range: std::ops::Range<usize>) -> Self {
        Self::new(
            Position(offset_from_utf16(text, range.start)),
            Position(offset_from_utf16(text, range.end)),
        )
    }

    /// Converts this source range into the UTF-16 offsets expected by IME and
    /// platform text-input APIs.
    pub fn to_utf16(&self, text: &str) -> std::ops::Range<usize> {
        let range = self.byte_range_in(text);
        offset_to_utf16(text, range.start)..offset_to_utf16(text, range.end)
    }
}

fn clamp_offset(text: &str, offset: usize) -> usize {
    let mut offset = offset.min(text.len());
    while offset > 0 && !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

fn offset_to_utf16(text: &str, offset: usize) -> usize {
    let offset = clamp_offset(text, offset);
    let mut utf16 = 0;
    let mut utf8 = 0;
    for character in text.chars() {
        if utf8 >= offset {
            break;
        }
        utf8 += character.len_utf8();
        utf16 += character.len_utf16();
    }
    utf16
}

fn offset_from_utf16(text: &str, offset: usize) -> usize {
    let mut utf16 = 0;
    let mut utf8 = 0;
    for character in text.chars() {
        let width = character.len_utf16();
        if utf16 + width > offset {
            break;
        }
        utf16 += width;
        utf8 += character.len_utf8();
    }
    utf8.min(text.len())
}
