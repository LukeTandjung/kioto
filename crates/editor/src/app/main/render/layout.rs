use std::ops::Range as ByteRange;
use std::sync::Arc;

use gpui::{Bounds, Pixels, Point, RenderImage, ShapedLine, Size, point, px, size};

use crate::core::position::Position;
use crate::core::preview_renderer::PreviewBlock;

/// One shaped display row of one block.
pub struct LayoutLine {
    pub block: usize,
    /// Range within the block's display text that this row shows.
    pub display_range: ByteRange<usize>,
    pub shaped: ShapedLine,
    /// y offset in unscrolled content coordinates.
    pub content_y: Pixels,
    pub height: Pixels,
    pub line_number: Option<ShapedLine>,
    /// Compiled fragment drawn in place of text, with its logical size.
    pub image: Option<(Arc<RenderImage>, Size<Pixels>)>,
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
        let column = line.shaped.closest_index_for_x(x);
        let display = line.display_range.start + column.min(line.display_range.len());
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
        let x = line.shaped.x_for_index(display - line.display_range.start);
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
            let left = display_start.max(line.display_range.start) - line.display_range.start;
            let right = display_end.min(line.display_range.end) - line.display_range.start;
            if display_start > line.display_range.end || display_end < line.display_range.start {
                continue;
            }
            let start_x = line.shaped.x_for_index(left.min(line.display_range.len()));
            let mut end_x = line.shaped.x_for_index(right.min(line.display_range.len()));
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
