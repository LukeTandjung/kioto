use std::sync::Arc;

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
/// spans multiple visual rows. Each block owns the source range and
/// bidirectional source ↔ display map that make motions, clicks, and
/// selections work when markup is hidden. The block's content is an enum so
/// invalid states such as "rendered kind without a fragment" cannot exist.
#[derive(Clone, Debug)]
pub struct PreviewBlock {
    pub source_range: Range,
    pub offset_map: OffsetMap,
    content: PreviewBlockContent,
}

#[derive(Clone, Debug)]
enum PreviewBlockContent {
    Text {
        kind: BlockKind,
        display_text: String,
        spans: Vec<StyleSpan>,
        fragments: Vec<InlineFragment>,
    },
    Raw {
        display_text: String,
    },
    Rendered {
        fragment: Arc<RenderedFragment>,
        fallback_text: String,
        fallback_spans: Vec<StyleSpan>,
    },
}

impl PreviewBlock {
    pub fn text(
        source_range: Range,
        kind: BlockKind,
        display_text: String,
        spans: Vec<StyleSpan>,
        fragments: Vec<InlineFragment>,
        offset_map: OffsetMap,
    ) -> Self {
        debug_assert!(!matches!(kind, BlockKind::Raw | BlockKind::Rendered));
        Self {
            source_range,
            offset_map,
            content: PreviewBlockContent::Text {
                kind,
                display_text,
                spans,
                fragments,
            },
        }
    }

    pub fn raw(source_range: Range, display_text: String, offset_map: OffsetMap) -> Self {
        Self {
            source_range,
            offset_map,
            content: PreviewBlockContent::Raw { display_text },
        }
    }

    pub fn rendered(
        source_range: Range,
        fallback_text: String,
        fallback_spans: Vec<StyleSpan>,
        offset_map: OffsetMap,
        fragment: Arc<RenderedFragment>,
    ) -> Self {
        Self {
            source_range,
            offset_map,
            content: PreviewBlockContent::Rendered {
                fragment,
                fallback_text,
                fallback_spans,
            },
        }
    }

    pub fn kind(&self) -> BlockKind {
        match &self.content {
            PreviewBlockContent::Text { kind, .. } => *kind,
            PreviewBlockContent::Raw { .. } => BlockKind::Raw,
            PreviewBlockContent::Rendered { .. } => BlockKind::Rendered,
        }
    }

    /// Text shown for text/raw blocks and fallback text for rendered blocks.
    /// Rendered blocks keep fallback text so hit testing and text fallback use
    /// the same display coordinate system.
    pub fn display_text(&self) -> &str {
        match &self.content {
            PreviewBlockContent::Text { display_text, .. } => display_text,
            PreviewBlockContent::Raw { display_text } => display_text,
            PreviewBlockContent::Rendered { fallback_text, .. } => fallback_text,
        }
    }

    pub fn spans(&self) -> &[StyleSpan] {
        match &self.content {
            PreviewBlockContent::Text { spans, .. } => spans,
            PreviewBlockContent::Raw { .. } => &[],
            PreviewBlockContent::Rendered { fallback_spans, .. } => fallback_spans,
        }
    }

    /// Compiled inline fragments drawn in place of ranges of the display
    /// text (inline math). Empty for raw and whole-block-rendered blocks.
    pub fn inline_fragments(&self) -> &[InlineFragment] {
        match &self.content {
            PreviewBlockContent::Text { fragments, .. } => fragments,
            PreviewBlockContent::Raw { .. } | PreviewBlockContent::Rendered { .. } => &[],
        }
    }

    pub fn rendered_fragment(&self) -> Option<&Arc<RenderedFragment>> {
        match &self.content {
            PreviewBlockContent::Rendered { fragment, .. } => Some(fragment),
            PreviewBlockContent::Text { .. } | PreviewBlockContent::Raw { .. } => None,
        }
    }

    pub fn into_rendered(self, fragment: Arc<RenderedFragment>) -> Self {
        match self.content {
            PreviewBlockContent::Text {
                display_text,
                spans,
                ..
            } => Self::rendered(
                self.source_range,
                display_text,
                spans,
                self.offset_map,
                fragment,
            ),
            PreviewBlockContent::Raw { display_text } => Self::rendered(
                self.source_range,
                display_text,
                Vec::new(),
                self.offset_map,
                fragment,
            ),
            PreviewBlockContent::Rendered {
                fallback_text,
                fallback_spans,
                ..
            } => Self::rendered(
                self.source_range,
                fallback_text,
                fallback_spans,
                self.offset_map,
                fragment,
            ),
        }
    }
}

/// A compiled fragment standing in for a range of a text block's display
/// text — the range still exists in the display coordinate system (offset
/// maps and hit testing are unaffected), but the view draws the fragment's
/// bitmap instead of those bytes.
#[derive(Clone, Debug)]
pub struct InlineFragment {
    pub display_range: std::ops::Range<usize>,
    pub fragment: Arc<RenderedFragment>,
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
    /// A compiled fragment drawn as an image, with fallback text retained for
    /// hit testing and graceful degradation.
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
    pub fn source_to_display(&self, source: Position) -> usize {
        let source = source.0;
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
    pub fn display_to_source(&self, display: usize) -> Position {
        let mut result = self.segments.first().map_or(0, |s| s.source_start);
        for segment in &self.segments {
            if display < segment.display_start {
                return Position(segment.source_start);
            }
            if display < segment.display_start + segment.len {
                return Position(segment.source_start + (display - segment.display_start));
            }
            result = segment.source_start + segment.len;
        }
        Position(result)
    }
}
