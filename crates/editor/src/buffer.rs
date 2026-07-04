use std::borrow::Cow;
use std::ops::Range;

use ropey::Rope;
use unicode_segmentation::UnicodeSegmentation as _;

/// Rope-backed text storage. All public offsets are UTF-8 byte offsets;
/// UTF-16 conversions exist only for the platform input boundary.
#[derive(Clone, Debug, Default)]
pub struct TextBuffer {
    rope: Rope,
    version: u64,
}

/// One line of the buffer: its byte range (excluding the trailing line
/// break) and its text.
#[derive(Clone, Debug)]
pub struct Line<'a> {
    pub range: Range<usize>,
    pub text: Cow<'a, str>,
}

impl Line<'_> {
    pub fn len(&self) -> usize {
        self.range.len()
    }

    pub fn is_empty(&self) -> bool {
        self.range.is_empty()
    }
}

impl TextBuffer {
    pub fn new(text: impl AsRef<str>) -> Self {
        Self {
            rope: Rope::from_str(text.as_ref()),
            version: 0,
        }
    }

    /// Bumped on every mutation; lets derived state (wrap maps, future
    /// caches) detect staleness cheaply.
    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn text(&self) -> String {
        self.rope.to_string()
    }

    pub fn len(&self) -> usize {
        self.rope.len_bytes()
    }

    pub fn is_empty(&self) -> bool {
        self.rope.len_bytes() == 0
    }

    /// The single write path: replaces `range` with `new_text`. Offsets that
    /// fall inside a character are clipped to the previous boundary.
    pub fn replace(&mut self, range: Range<usize>, new_text: &str) {
        let start = self.rope.byte_to_char(self.clip(range.start));
        let end = self.rope.byte_to_char(self.clip(range.end)).max(start);
        self.rope.remove(start..end);
        self.rope.insert(start, new_text);
        self.version += 1;
    }

    pub fn slice(&self, range: Range<usize>) -> Cow<'_, str> {
        Cow::from(self.rope.byte_slice(range))
    }

    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    /// The line at `row`, clamped to the last line.
    pub fn line(&self, row: usize) -> Line<'_> {
        let row = row.min(self.line_count().saturating_sub(1));
        let start = self.rope.line_to_byte(row);
        let slice = self.rope.line(row);
        let mut end = start + slice.len_bytes();
        for character in slice.chars_at(slice.len_chars()).reversed() {
            if character == '\n' || character == '\r' {
                end -= character.len_utf8();
            } else {
                break;
            }
        }
        Line {
            range: start..end,
            text: Cow::from(self.rope.byte_slice(start..end)),
        }
    }

    /// Maps a byte offset to `(row, column)`, where column is a byte offset
    /// within the line clamped to the line's content.
    pub fn offset_to_point(&self, offset: usize) -> (usize, usize) {
        let offset = self.clip(offset);
        let row = self.rope.byte_to_line(offset);
        let line = self.line(row);
        (row, offset.saturating_sub(line.range.start).min(line.len()))
    }

    /// Maps `(row, column)` back to a byte offset, clamping row to the last
    /// line and column to the line's content.
    pub fn point_to_offset(&self, row: usize, column: usize) -> usize {
        let line = self.line(row);
        self.clip(line.range.start + column.min(line.len()))
    }

    pub fn previous_grapheme_boundary(&self, offset: usize) -> usize {
        let offset = self.clip(offset);
        if offset == 0 {
            return 0;
        }

        let row = self.rope.byte_to_line(offset);
        let line = self.line(row);
        if offset == line.range.start {
            // Step over the previous line's line break as one grapheme.
            return self.line(row - 1).range.end;
        }

        let local = offset - line.range.start;
        line.text
            .grapheme_indices(true)
            .rev()
            .find_map(|(index, _)| (index < local).then_some(line.range.start + index))
            .unwrap_or(line.range.start)
    }

    pub fn next_grapheme_boundary(&self, offset: usize) -> usize {
        let offset = self.clip(offset);
        if offset >= self.len() {
            return self.len();
        }

        let row = self.rope.byte_to_line(offset);
        let line = self.line(row);
        if offset >= line.range.end {
            // Inside the line break: the next boundary is the next line start.
            return if row + 1 < self.line_count() {
                self.rope.line_to_byte(row + 1)
            } else {
                self.len()
            };
        }

        let local = offset - line.range.start;
        line.text
            .grapheme_indices(true)
            .find_map(|(index, _)| (index > local).then_some(line.range.start + index))
            .unwrap_or(line.range.end)
    }

    pub fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    pub fn range_from_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range.start)..self.offset_from_utf16(range.end)
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let character = self.rope.byte_to_char(self.clip(offset));
        self.rope.char_to_utf16_cu(character)
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let offset = offset.min(self.rope.len_utf16_cu());
        let character = self.rope.utf16_cu_to_char(offset);
        self.rope.char_to_byte(character)
    }

    /// Clamps to buffer length and clips to the previous char boundary.
    fn clip(&self, offset: usize) -> usize {
        let offset = offset.min(self.len());
        let character = self.rope.byte_to_char(offset);
        self.rope.char_to_byte(character)
    }
}

#[cfg(test)]
#[path = "buffer.test.rs"]
mod tests;
