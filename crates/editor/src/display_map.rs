use std::borrow::Cow;
use std::ops::Range;

use crate::buffer::TextBuffer;

/// A position in display space: the coordinates the element lays out and
/// paints. With soft wrap, one buffer line spans one or more display rows,
/// so display space and buffer space genuinely diverge; all rendering code
/// must go through this projection rather than assuming they match.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisplayPoint {
    pub row: usize,
    pub column: usize,
}

/// One display row: a slice of a buffer line (the whole line when unwrapped).
#[derive(Clone, Debug)]
pub struct DisplayRow {
    pub buffer_row: usize,
    pub range: Range<usize>,
    /// True for the first display row of a buffer line; the gutter shows the
    /// line number only there.
    pub is_line_start: bool,
}

/// Buffer-to-display projection: which byte range each display row shows.
/// Built from the buffer plus a wrap function (pixel measurement lives with
/// the caller so this stays testable), cached against the buffer version and
/// wrap width.
pub struct WrapMap {
    rows: Vec<DisplayRow>,
    buffer_version: u64,
    wrap_width: f32,
}

impl WrapMap {
    /// Identity map: one display row per buffer line. Used before the first
    /// layout pass has a real wrap width; invalid against any real width, so
    /// the first prepaint replaces it.
    pub fn unwrapped(buffer: &TextBuffer) -> Self {
        Self::build(buffer, f32::NEG_INFINITY, |_| Vec::new())
    }

    /// Builds the projection. `wrap_line` returns the byte offsets within
    /// one line's text where new display rows start.
    pub fn build(
        buffer: &TextBuffer,
        wrap_width: f32,
        mut wrap_line: impl FnMut(&str) -> Vec<usize>,
    ) -> Self {
        let mut rows = Vec::with_capacity(buffer.line_count());
        for buffer_row in 0..buffer.line_count() {
            let line = buffer.line(buffer_row);
            let boundaries = if line.is_empty() {
                Vec::new()
            } else {
                wrap_line(&line.text)
            };

            let mut start = line.range.start;
            for boundary in boundaries {
                let end = line.range.start + boundary;
                if end <= start || end >= line.range.end {
                    continue;
                }
                rows.push(DisplayRow {
                    buffer_row,
                    range: start..end,
                    is_line_start: start == line.range.start,
                });
                start = end;
            }
            rows.push(DisplayRow {
                buffer_row,
                range: start..line.range.end,
                is_line_start: start == line.range.start,
            });
        }

        Self {
            rows,
            buffer_version: buffer.version(),
            wrap_width,
        }
    }

    pub fn is_valid_for(&self, buffer: &TextBuffer, wrap_width: f32) -> bool {
        self.buffer_version == buffer.version() && self.wrap_width == wrap_width
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
}

/// Immutable view over the buffer through the wrap projection, taken once
/// per layout pass or motion. Folds will compose inside this type without
/// changing its API.
pub struct DisplaySnapshot<'a> {
    buffer: &'a TextBuffer,
    rows: &'a [DisplayRow],
}

impl<'a> DisplaySnapshot<'a> {
    pub fn new(buffer: &'a TextBuffer, wrap_map: &'a WrapMap) -> Self {
        Self {
            buffer,
            rows: &wrap_map.rows,
        }
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// The display row at `index`, clamped to the last row.
    pub fn row(&self, index: usize) -> &DisplayRow {
        &self.rows[index.min(self.rows.len() - 1)]
    }

    pub fn row_text(&self, index: usize) -> Cow<'a, str> {
        self.buffer.slice(self.row(index).range.clone())
    }

    pub fn offset_to_display(&self, offset: usize) -> DisplayPoint {
        let index = self
            .rows
            .partition_point(|row| row.range.start <= offset)
            .saturating_sub(1);
        let row = &self.rows[index];
        DisplayPoint {
            row: index,
            column: offset.saturating_sub(row.range.start).min(row.range.len()),
        }
    }

    /// Maps a display point back to a clipped buffer offset.
    pub fn display_to_offset(&self, point: DisplayPoint) -> usize {
        let row = self.row(point.row);
        let offset = row.range.start + point.column.min(row.range.len());
        // Round-trip through buffer points to clip mid-character columns.
        let (buffer_row, column) = self.buffer.offset_to_point(offset);
        self.buffer.point_to_offset(buffer_row, column)
    }
}

#[cfg(test)]
#[path = "display_map.test.rs"]
mod tests;
