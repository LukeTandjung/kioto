use unicode_segmentation::UnicodeSegmentation as _;

use crate::core::position::Position;

/// A cursor motion, named after what it means rather than which key
/// produced it. Resolution is a pure function of the source text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Motion {
    Left,
    Right,
    Up,
    Down,
    LineStart,
    LineEnd,
    WordForward,
    WordBack,
    WordEnd,
    DocumentStart,
    DocumentEnd,
}

/// Resolves a motion from `position` over `text`, clamped to the text and
/// to char boundaries. Horizontal steps are grapheme-aware; vertical steps
/// preserve the byte column, clamped to the target line's content.
pub fn resolve(text: &str, position: Position, motion: Motion) -> Position {
    let offset = clamp(text, position.0);
    let resolved = match motion {
        Motion::Left => previous_grapheme(text, offset),
        Motion::Right => next_grapheme(text, offset),
        Motion::Up => vertical(text, offset, false),
        Motion::Down => vertical(text, offset, true),
        Motion::LineStart => line_of(text, offset).start,
        Motion::LineEnd => line_of(text, offset).end,
        Motion::WordForward => word_forward(text, offset),
        Motion::WordBack => word_back(text, offset),
        Motion::WordEnd => word_end(text, offset),
        Motion::DocumentStart => 0,
        Motion::DocumentEnd => text.len(),
    };
    Position(resolved)
}

/// Byte ranges of each line's content, excluding line breaks. Always at
/// least one line. Shared by motion resolution and the view layer.
pub fn line_ranges(text: &str) -> Vec<std::ops::Range<usize>> {
    let mut lines = Vec::new();
    let mut start = 0;
    for (index, byte) in text.bytes().enumerate() {
        if byte == b'\n' {
            let end = if index > start && text.as_bytes()[index - 1] == b'\r' {
                index - 1
            } else {
                index
            };
            lines.push(start..end);
            start = index + 1;
        }
    }
    lines.push(start..text.len());
    lines
}

fn clamp(text: &str, offset: usize) -> usize {
    let mut offset = offset.min(text.len());
    while offset > 0 && !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

fn previous_grapheme(text: &str, offset: usize) -> usize {
    text.grapheme_indices(true)
        .rev()
        .find_map(|(index, _)| (index < offset).then_some(index))
        .unwrap_or(0)
}

fn next_grapheme(text: &str, offset: usize) -> usize {
    text.grapheme_indices(true)
        .find_map(|(index, _)| (index > offset).then_some(index))
        .unwrap_or(text.len())
}

fn line_of(text: &str, offset: usize) -> std::ops::Range<usize> {
    line_ranges(text)
        .into_iter()
        .rev()
        .find(|line| line.start <= offset)
        .unwrap_or(0..text.len())
}

fn vertical(text: &str, offset: usize, down: bool) -> usize {
    let lines = line_ranges(text);
    let row = lines
        .iter()
        .rposition(|line| line.start <= offset)
        .unwrap_or(0);
    let target = if down {
        (row + 1).min(lines.len() - 1)
    } else {
        row.saturating_sub(1)
    };
    let column = offset - lines[row].start;
    clamp(text, lines[target].start + column.min(lines[target].len()))
}

/// Start offsets of every non-whitespace word segment.
fn word_starts(text: &str) -> impl Iterator<Item = (usize, &str)> {
    text.split_word_bound_indices()
        .filter(|(_, word)| !word.trim().is_empty())
}

fn word_forward(text: &str, offset: usize) -> usize {
    word_starts(text)
        .map(|(start, _)| start)
        .find(|start| *start > offset)
        .unwrap_or(text.len())
}

fn word_back(text: &str, offset: usize) -> usize {
    word_starts(text)
        .map(|(start, _)| start)
        .take_while(|start| *start < offset)
        .last()
        .unwrap_or(0)
}

fn word_end(text: &str, offset: usize) -> usize {
    // The last character of the current-or-next word, vim-style.
    word_starts(text)
        .map(|(start, word)| (start + word.len(), start))
        .find_map(|(end, _)| {
            let last = previous_grapheme(text, end);
            (last > offset).then_some(last)
        })
        .unwrap_or_else(|| previous_grapheme(text, text.len()))
}

#[cfg(test)]
#[path = "motion.test.rs"]
mod tests;
