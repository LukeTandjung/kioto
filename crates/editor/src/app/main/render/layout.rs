use std::ops::Range as ByteRange;
use std::sync::Arc;

use gpui::{Bounds, Pixels, Point, RenderImage, ShapedLine, Size, point, px, size};

use crate::core::position::Position;
use crate::core::preview_renderer::PreviewBlock;

/// One horizontal piece of a display row: either shaped text or a compiled
/// fragment drawn as an image. Both cover a range of the block's display
/// text, so hit testing and cursor geometry work uniformly across them.
pub enum RowSegment {
    Text {
        display_range: ByteRange<usize>,
        // Boxed: a ShapedLine is ~3KB inline, dwarfing the Image variant.
        shaped: Box<ShapedLine>,
        x: Pixels,
    },
    Image {
        display_range: ByteRange<usize>,
        image: Arc<RenderImage>,
        size: Size<Pixels>,
        x: Pixels,
    },
}

impl RowSegment {
    fn display_range(&self) -> &ByteRange<usize> {
        match self {
            RowSegment::Text { display_range, .. } | RowSegment::Image { display_range, .. } => {
                display_range
            }
        }
    }

    fn x(&self) -> Pixels {
        match self {
            RowSegment::Text { x, .. } | RowSegment::Image { x, .. } => *x,
        }
    }

    fn width(&self) -> Pixels {
        match self {
            RowSegment::Text { shaped, .. } => shaped.width(),
            RowSegment::Image { size, .. } => size.width,
        }
    }
}

/// One display row of one block, as a sequence of segments.
pub struct LayoutLine {
    pub block: usize,
    /// Range within the block's display text that this row shows.
    pub display_range: ByteRange<usize>,
    pub segments: Vec<RowSegment>,
    /// y offset in unscrolled content coordinates.
    pub content_y: Pixels,
    pub height: Pixels,
    pub line_number: Option<ShapedLine>,
}

impl LayoutLine {
    /// x offset of a display position within this row, in row-local
    /// coordinates. Positions inside an image segment snap to its edges.
    pub fn x_for_display(&self, display: usize) -> Pixels {
        let mut end_x = px(0.);
        for segment in &self.segments {
            let range = segment.display_range();
            if display < range.start {
                return segment.x();
            }
            if display < range.end {
                return match segment {
                    RowSegment::Text { shaped, x, .. } => {
                        *x + shaped.x_for_index(display - range.start)
                    }
                    RowSegment::Image { x, .. } => *x,
                };
            }
            end_x = segment.x() + segment.width();
        }
        end_x
    }

    /// Display position of a row-local x offset. x inside an image segment
    /// resolves to the nearer edge of its display range.
    pub fn display_for_x(&self, x: Pixels) -> usize {
        let mut result = self.display_range.start;
        for segment in &self.segments {
            if x < segment.x() {
                break;
            }
            let range = segment.display_range();
            if x < segment.x() + segment.width() {
                return match segment {
                    RowSegment::Text {
                        shaped, x: seg_x, ..
                    } => range.start + shaped.closest_index_for_x(x - *seg_x),
                    RowSegment::Image { x: seg_x, size, .. } => {
                        if x - *seg_x < size.width / 2. {
                            range.start
                        } else {
                            range.end
                        }
                    }
                };
            }
            result = range.end;
        }
        result
    }
}

/// Geometry of one source position in the viewport.
pub struct SourceGeometry {
    pub origin: Point<Pixels>,
    pub height: Pixels,
    pub line_index: usize,
}

pub struct ViewLayout {
    pub bounds: Bounds<Pixels>,
    pub gutter_width: Pixels,
    pub scroll_y: Pixels,
    pub content_height: Pixels,
    pub blocks: Vec<PreviewBlock>,
    pub lines: Vec<LayoutLine>,
    /// Gutter numbers of the blank rows between blocks, as `(content_y,
    /// shaped number)` — the rows have no text but keep their numbers.
    pub gap_numbers: Vec<(Pixels, ShapedLine)>,
}

impl ViewLayout {
    pub fn line_y(&self, line: &LayoutLine) -> Pixels {
        self.bounds.top() + line.content_y + self.scroll_y
    }

    pub fn offset_for_point(&self, position: Point<Pixels>) -> Position {
        if self.lines.is_empty() {
            return Position(0);
        }
        let mut line = &self.lines[0];
        for candidate in &self.lines {
            if position.y >= self.line_y(candidate) {
                line = candidate;
            } else {
                break;
            }
        }

        let x = position.x - self.bounds.left() - self.gutter_width;
        let display = line.display_for_x(x);
        let block = &self.blocks[line.block];
        block.offset_map.display_to_source(display)
    }

    /// The viewport geometry of a source offset, resolved through its block's
    /// offset map.
    pub fn geometry_for_source(&self, source: usize) -> Option<SourceGeometry> {
        let block_index = self
            .blocks
            .iter()
            .rposition(|block| block.source_range.start.0 <= source)
            .unwrap_or(0);
        let block = self.blocks.get(block_index)?;
        let display = block
            .offset_map
            .source_to_display(Position(source.min(block.source_range.end.0)));

        let line_index = self.lines.iter().position(|line| {
            line.block == block_index
                && display >= line.display_range.start
                && display <= line.display_range.end
        })?;
        let line = &self.lines[line_index];
        let x = line.x_for_display(display);
        Some(SourceGeometry {
            origin: point(
                self.bounds.left() + self.gutter_width + x,
                self.line_y(line),
            ),
            height: line.height,
            line_index,
        })
    }

    pub fn selection_rects(&self, source: ByteRange<usize>) -> Vec<Bounds<Pixels>> {
        let mut rects = Vec::new();
        for line in &self.lines {
            let block = &self.blocks[line.block];
            let block_range = block.source_range.start.0..block.source_range.end.0;
            let start = source.start.max(block_range.start);
            let end = source.end.min(block_range.end);
            if start >= end {
                continue;
            }
            let display_start = block.offset_map.source_to_display(Position(start));
            let display_end = block.offset_map.source_to_display(Position(end));
            if display_start > line.display_range.end || display_end < line.display_range.start {
                continue;
            }
            let left = display_start.clamp(line.display_range.start, line.display_range.end);
            let right = display_end.clamp(line.display_range.start, line.display_range.end);
            let start_x = line.x_for_display(left);
            let mut end_x = line.x_for_display(right);
            if end_x <= start_x {
                if display_end <= line.display_range.start {
                    continue;
                }
                end_x = start_x + px(6.);
            }
            rects.push(Bounds::new(
                point(
                    self.bounds.left() + self.gutter_width + start_x,
                    self.line_y(line),
                ),
                size(end_x - start_x, line.height),
            ));
        }
        rects
    }
}

/// Byte ranges of each display row in a block's display text.
pub fn display_rows(display_text: &str) -> Vec<ByteRange<usize>> {
    let mut rows = Vec::new();
    let mut start = 0;
    for (index, byte) in display_text.bytes().enumerate() {
        if byte == b'\n' {
            rows.push(start..index);
            start = index + 1;
        }
    }
    rows.push(start..display_text.len());
    rows
}
