use crate::core::position::{Position, Range};

/// The live-preview contract: the editor asks for preview output without
/// knowing Typst, Markdown, or LaTeX. Rendering is a function of the
/// document *plus the cursor positions* — the block containing a cursor
/// renders in raw-markup mode while every other block renders styled.
///
/// Implemented by the same document-model type that implements
/// `EditableBuffer` (the buffer/renderer pairing decision), so it reads its
/// own text and parse state.
pub trait PreviewRenderer {
    fn render_preview(&self, cursor_positions: &[Position]) -> PreviewOutput;
}

#[derive(Clone, Debug, Default)]
pub struct PreviewOutput {
    pub blocks: Vec<PreviewBlock>,
}

/// One render unit. Blocks, not lines: a paragraph, heading, or math block
/// spans multiple visual rows. `display_text` is what is shown (markers
/// hidden unless the block is in raw mode); `offset_map` is the
/// bidirectional source ↔ display mapping that makes motions, clicks, and
/// selections work when the two diverge.
#[derive(Clone, Debug)]
pub struct PreviewBlock {
    pub source_range: Range,
    pub kind: BlockKind,
    pub display_text: String,
    pub spans: Vec<StyleSpan>,
    pub offset_map: OffsetMap,
    /// Compiled bitmap for `BlockKind::Rendered` blocks; `display_text` and
    /// `offset_map` still describe the source so hit testing keeps working.
    pub rendered: Option<std::sync::Arc<RenderedFragment>>,
}

/// A compiled preview fragment as raw pixels — language- and UI-neutral, so
/// `core` stays free of both `typst-*` and `gpui`. Producers rasterize at a
/// device resolution; consumers draw at the logical size.
#[derive(Clone, Debug)]
pub struct RenderedFragment {
    /// Bitmap dimensions in device pixels.
    pub width: u32,
    pub height: u32,
    /// Size to draw at, in logical pixels.
    pub logical_width: f32,
    pub logical_height: f32,
    /// Straight-alpha BGRA bytes, row-major, `width * height * 4` long.
    pub bgra: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockKind {
    Heading(u8),
    Paragraph,
    ListItem,
    CodeBlock,
    MathBlock,
    /// The cursor is inside: verbatim source markup, identity-mapped.
    Raw,
    /// Milestone 4: compiled fragment drawn as an image.
    Rendered,
}

/// Styling for a range of `display_text`, in display offsets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StyleSpan {
    pub range: std::ops::Range<usize>,
    pub kind: SpanKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpanKind {
    Plain,
    Strong,
    Emphasis,
    InlineCode,
    InlineMath,
}

/// Bidirectional source ↔ display offset mapping within one block, as a
/// sorted list of 1:1 segments. Source positions with no display image
/// (hidden markers) clamp to the nearest mapped display position, and vice
/// versa — errors defined out of existence.
#[derive(Clone, Debug, Default)]
pub struct OffsetMap {
    segments: Vec<MapSegment>,
}

#[derive(Clone, Copy, Debug)]
struct MapSegment {
    source_start: usize,
    display_start: usize,
    len: usize,
}

impl OffsetMap {
    /// Identity map for raw blocks: display text == source slice.
    pub fn identity(source_range: &Range) -> Self {
        let mut map = Self::default();
        map.push(source_range.start.0, 0, source_range.len());
        map
    }

    /// Records that `len` bytes at absolute source offset `source_start`
    /// appear verbatim at `display_start` in the block's display text.
    /// Segments must be pushed in ascending order on both axes.
    pub fn push(&mut self, source_start: usize, display_start: usize, len: usize) {
        if len == 0 {
            return;
        }
        // Coalesce with a directly adjacent previous segment.
        if let Some(last) = self.segments.last_mut()
            && last.source_start + last.len == source_start
            && last.display_start + last.len == display_start
        {
            last.len += len;
            return;
        }
        self.segments.push(MapSegment {
            source_start,
            display_start,
            len,
        });
    }

    /// Maps an absolute source offset to a display offset in the block.
    /// Offsets exactly on a segment boundary map into the later segment, so
    /// positions after hidden markers resolve past them.
    pub fn source_to_display(&self, source: usize) -> usize {
        let mut result = 0;
        for segment in &self.segments {
            if source < segment.source_start {
                // In a hidden gap: clamp to the segment ahead.
                return segment.display_start;
            }
            if source < segment.source_start + segment.len {
                return segment.display_start + (source - segment.source_start);
            }
            result = segment.display_start + segment.len;
        }
        result
    }

    /// Maps a display offset back to an absolute source offset, with the
    /// same later-segment boundary rule.
    pub fn display_to_source(&self, display: usize) -> usize {
        let mut result = self.segments.first().map_or(0, |s| s.source_start);
        for segment in &self.segments {
            if display < segment.display_start {
                return segment.source_start;
            }
            if display < segment.display_start + segment.len {
                return segment.source_start + (display - segment.display_start);
            }
            result = segment.source_start + segment.len;
        }
        result
    }
}

#[cfg(test)]
#[path = "preview_renderer.test.rs"]
mod tests;
