use std::ops::Range;

use unicode_segmentation::UnicodeSegmentation as _;

#[derive(Clone, Debug, Default)]
pub struct TextBuffer {
    text: String,
}

#[derive(Clone, Debug)]
pub struct LineInfo {
    pub row: usize,
    pub start: usize,
    pub end: usize,
}

impl TextBuffer {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn replace(&mut self, range: Range<usize>, new_text: &str) {
        let start = self.clip_to_boundary(range.start);
        let end = self.clip_to_boundary(range.end).max(start);
        self.text.replace_range(start..end, new_text);
    }

    pub fn slice(&self, range: Range<usize>) -> &str {
        &self.text[range]
    }

    pub fn line_infos(&self) -> Vec<LineInfo> {
        let mut lines = Vec::new();
        let mut start = 0;
        let mut row = 0;

        for (index, character) in self.text.char_indices() {
            if character == '\n' {
                lines.push(LineInfo {
                    row,
                    start,
                    end: index,
                });
                start = index + character.len_utf8();
                row += 1;
            }
        }

        lines.push(LineInfo {
            row,
            start,
            end: self.text.len(),
        });

        lines
    }

    pub fn line_count(&self) -> usize {
        self.text
            .chars()
            .filter(|character| *character == '\n')
            .count()
            + 1
    }

    pub fn line_text(&self, row: usize) -> &str {
        let lines = self.line_infos();
        let line = lines
            .get(row)
            .or_else(|| lines.last())
            .expect("line_infos always contains at least one line");
        &self.text[line.start..line.end]
    }

    pub fn line_start_offset(&self, row: usize) -> usize {
        self.line_infos()
            .get(row)
            .map(|line| line.start)
            .unwrap_or_else(|| self.text.len())
    }

    pub fn line_end_offset(&self, row: usize) -> usize {
        self.line_infos()
            .get(row)
            .map(|line| line.end)
            .unwrap_or_else(|| self.text.len())
    }

    pub fn offset_to_line_col(&self, offset: usize) -> (usize, usize) {
        let offset = offset.min(self.text.len());
        let lines = self.line_infos();
        let last_index = lines.len().saturating_sub(1);

        for line in &lines {
            if offset <= line.end || line.row == last_index {
                return (
                    line.row,
                    offset.saturating_sub(line.start).min(line.end - line.start),
                );
            }
        }

        (last_index, 0)
    }

    pub fn line_col_to_offset(&self, row: usize, column: usize) -> usize {
        let lines = self.line_infos();
        let line = lines
            .get(row)
            .or_else(|| lines.last())
            .expect("line_infos always contains at least one line");
        self.clip_to_boundary((line.start + column).min(line.end))
    }

    pub fn previous_grapheme_boundary(&self, offset: usize) -> usize {
        self.text
            .grapheme_indices(true)
            .rev()
            .find_map(|(index, _)| (index < offset).then_some(index))
            .unwrap_or(0)
    }

    pub fn next_grapheme_boundary(&self, offset: usize) -> usize {
        self.text
            .grapheme_indices(true)
            .find_map(|(index, _)| (index > offset).then_some(index))
            .unwrap_or(self.text.len())
    }

    pub fn offset_to_utf16(&self, offset: usize) -> usize {
        let offset = offset.min(self.text.len());
        let mut utf16_offset = 0;
        let mut utf8_offset = 0;

        for character in self.text.chars() {
            if utf8_offset >= offset {
                break;
            }
            utf8_offset += character.len_utf8();
            utf16_offset += character.len_utf16();
        }

        utf16_offset
    }

    pub fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_offset = 0;

        for character in self.text.chars() {
            if utf16_offset >= offset {
                break;
            }
            utf16_offset += character.len_utf16();
            utf8_offset += character.len_utf8();
        }

        utf8_offset.min(self.text.len())
    }

    pub fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    pub fn range_from_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range.start)..self.offset_from_utf16(range.end)
    }

    fn clip_to_boundary(&self, offset: usize) -> usize {
        let offset = offset.min(self.text.len());
        if self.text.is_char_boundary(offset) {
            return offset;
        }

        let mut clipped = offset;
        while clipped > 0 && !self.text.is_char_boundary(clipped) {
            clipped -= 1;
        }
        clipped
    }
}

#[cfg(test)]
mod tests {
    use super::TextBuffer;

    #[test]
    fn utf16_round_trip_handles_emoji() {
        let buffer = TextBuffer::new("a💝b");
        let emoji_end = "a💝".len();
        assert_eq!(buffer.offset_to_utf16(emoji_end), 3);
        assert_eq!(buffer.offset_from_utf16(3), emoji_end);
    }

    #[test]
    fn line_info_keeps_trailing_empty_line() {
        let buffer = TextBuffer::new("a\nb\n");
        assert_eq!(buffer.line_count(), 3);
        assert_eq!(buffer.line_text(2), "");
    }
}
