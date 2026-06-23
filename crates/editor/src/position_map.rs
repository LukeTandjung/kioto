use std::ops::Range;

use gpui::{Bounds, Pixels, Point, ShapedLine, point, size};

#[derive(Clone)]
pub struct PositionLine {
    pub start: usize,
    pub end: usize,
    pub shaped_line: ShapedLine,
}

#[derive(Clone)]
pub struct PositionMap {
    pub bounds: Bounds<Pixels>,
    pub gutter_width: Pixels,
    pub line_height: Pixels,
    pub lines: Vec<PositionLine>,
}

impl PositionMap {
    pub fn offset_for_point(&self, point: Point<Pixels>) -> Option<usize> {
        if self.lines.is_empty() {
            return Some(0);
        }

        let local = point - self.bounds.origin;
        let row_index = (local.y.as_f32() / self.line_height.as_f32()).floor() as isize;
        let row_index = row_index.clamp(0, self.lines.len().saturating_sub(1) as isize) as usize;
        let line = &self.lines[row_index];
        let x = local.x - self.gutter_width;
        let column = line.shaped_line.closest_index_for_x(x);
        Some((line.start + column).min(line.end))
    }

    pub fn point_for_offset(&self, offset: usize) -> Option<Point<Pixels>> {
        let line_index = self.line_index_for_offset(offset)?;
        let line = &self.lines[line_index];
        let column = offset.saturating_sub(line.start).min(line.end - line.start);
        Some(point(
            self.bounds.left() + self.gutter_width + line.shaped_line.x_for_index(column),
            self.bounds.top() + self.line_height * line_index as f32,
        ))
    }

    pub fn bounds_for_range(&self, range: Range<usize>) -> Option<Bounds<Pixels>> {
        let start = self.point_for_offset(range.start)?;
        let end = self.point_for_offset(range.end)?;
        Some(Bounds::from_corners(
            start,
            point(end.x, end.y + self.line_height),
        ))
    }

    pub fn selection_rects(&self, range: Range<usize>) -> Vec<Bounds<Pixels>> {
        if range.is_empty() {
            return Vec::new();
        }

        let mut rects = Vec::new();
        for (line_index, line) in self.lines.iter().enumerate() {
            if range.end < line.start || range.start > line.end {
                continue;
            }

            let start = range.start.max(line.start).saturating_sub(line.start);
            let end = range.end.min(line.end).saturating_sub(line.start);
            let start_x = line.shaped_line.x_for_index(start);
            let mut end_x = line.shaped_line.x_for_index(end);
            if start_x == end_x {
                end_x += gpui::px(6.);
            }

            let top = self.bounds.top() + self.line_height * line_index as f32;
            rects.push(Bounds::new(
                point(self.bounds.left() + self.gutter_width + start_x, top),
                size(end_x - start_x, self.line_height),
            ));
        }

        rects
    }

    fn line_index_for_offset(&self, offset: usize) -> Option<usize> {
        self.lines.iter().enumerate().find_map(|(index, line)| {
            (offset >= line.start && (offset <= line.end || index + 1 == self.lines.len()))
                .then_some(index)
        })
    }
}
