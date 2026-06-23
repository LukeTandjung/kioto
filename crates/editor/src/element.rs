use gpui::{
    App, Bounds, Element, ElementId, ElementInputHandler, Entity, GlobalElementId, Hsla,
    IntoElement, LayoutId, PaintQuad, Pixels, ShapedLine, SharedString, Style, TextAlign, TextRun,
    UnderlineStyle, Window, fill, point, px, relative, rgb, rgba, size,
};

use crate::{Editor, editor::EditorMode, position_map::PositionLine, position_map::PositionMap};

const GUTTER_PADDING_LEFT: Pixels = px(8.);
const GUTTER_PADDING_RIGHT: Pixels = px(12.);
const CURSOR_WIDTH: Pixels = px(2.);

pub struct EditorElement {
    editor: Entity<Editor>,
}

impl EditorElement {
    pub fn new(editor: Entity<Editor>) -> Self {
        Self { editor }
    }
}

struct LayoutLine {
    row: usize,
    origin_y: Pixels,
    text: ShapedLine,
    line_number: ShapedLine,
}

pub struct PrepaintState {
    position_map: PositionMap,
    lines: Vec<LayoutLine>,
    selection_rects: Vec<Bounds<Pixels>>,
    cursor: Option<PaintQuad>,
    current_line: Option<Bounds<Pixels>>,
}

impl IntoElement for EditorElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for EditorElement {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = relative(1.).into();
        style.min_size.height = window.line_height().into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let editor = self.editor.read(cx);
        let style = window.text_style();
        let font_size = style.font_size.to_pixels(window.rem_size());
        let line_height = window.line_height();
        let total_lines = editor.buffer.line_count();
        let digits = total_lines.max(1).to_string().len().max(2);

        let number_sample: SharedString = "8".repeat(digits).into();
        let number_sample = window.text_system().shape_line(
            number_sample,
            font_size,
            &[TextRun {
                len: digits,
                font: style.font(),
                color: rgb(0x8b949e).into(),
                background_color: None,
                underline: None,
                strikethrough: None,
            }],
            None,
        );
        let gutter_width = GUTTER_PADDING_LEFT + number_sample.width + GUTTER_PADDING_RIGHT;

        let scroll_y = editor.scroll_handle.offset().y;
        let first_row = ((-scroll_y).as_f32() / line_height.as_f32())
            .floor()
            .max(0.) as usize;
        let visible_rows = (bounds.size.height.as_f32() / line_height.as_f32()).ceil() as usize + 1;
        let last_row = (first_row + visible_rows).min(total_lines);
        let line_infos = editor.buffer.line_infos();
        let mut layout_lines = Vec::with_capacity(last_row.saturating_sub(first_row));
        let mut position_lines = Vec::with_capacity(last_row.saturating_sub(first_row));

        for row in first_row..last_row {
            let line = &line_infos[row];
            let line_text = editor.buffer.line_text(row);
            let runs = text_runs_for_line(
                line.start,
                line.end,
                line_text.len(),
                editor.marked_range.as_ref().map(|range| range.range_ref()),
                style.font(),
                style.color,
            );
            let shaped_text = window.text_system().shape_line(
                SharedString::from(line_text.to_string()),
                font_size,
                &runs,
                None,
            );

            let line_number_text = format!("{:>width$}", row + 1, width = digits);
            let line_number = window.text_system().shape_line(
                SharedString::from(line_number_text.clone()),
                font_size,
                &[TextRun {
                    len: line_number_text.len(),
                    font: style.font(),
                    color: rgb(0x8b949e).into(),
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                }],
                None,
            );

            let visible_index = row - first_row;
            layout_lines.push(LayoutLine {
                row,
                origin_y: line_height * visible_index as f32,
                text: shaped_text.clone(),
                line_number,
            });
            position_lines.push(PositionLine {
                start: line.start,
                end: line.end,
                shaped_line: shaped_text,
            });
        }

        let position_map = PositionMap {
            bounds,
            gutter_width,
            line_height,
            lines: position_lines,
        };

        let selection_rects = position_map.selection_rects(editor.selected_range());
        let cursor = cursor_quad(
            editor.mode,
            editor.cursor(),
            &position_map,
            rgb(0xf0f6fc).into(),
        );
        let current_line = position_map.point_for_offset(editor.cursor()).map(|point| {
            Bounds::new(
                gpui::point(bounds.left(), point.y),
                gpui::size(bounds.size.width, line_height),
            )
        });

        PrepaintState {
            position_map,
            lines: layout_lines,
            selection_rects,
            cursor,
            current_line,
        }
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let (focus_handle, focused) = {
            let editor = self.editor.read(cx);
            (
                editor.focus_handle.clone(),
                editor.focus_handle.is_focused(window),
            )
        };

        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.editor.clone()),
            cx,
        );

        window.paint_quad(fill(bounds, rgb(0x0d1117)));

        if let Some(current_line) = prepaint.current_line {
            window.paint_quad(fill(current_line, rgba(0xffffff08)));
        }

        for rect in prepaint.selection_rects.drain(..) {
            window.paint_quad(fill(rect, rgba(0x2f81f755)));
        }

        for line in &prepaint.lines {
            let line_number_origin = point(
                bounds.left() + GUTTER_PADDING_LEFT,
                bounds.top() + line.origin_y,
            );
            let text_origin = point(
                bounds.left() + prepaint.position_map.gutter_width,
                bounds.top() + line.origin_y,
            );

            if line.row % 2 == 0 {
                window.paint_quad(fill(
                    Bounds::new(
                        point(bounds.left(), bounds.top() + line.origin_y),
                        size(
                            prepaint.position_map.gutter_width,
                            prepaint.position_map.line_height,
                        ),
                    ),
                    rgba(0xffffff03),
                ));
            }

            line.line_number
                .paint(
                    line_number_origin,
                    prepaint.position_map.line_height,
                    TextAlign::Right,
                    Some(prepaint.position_map.gutter_width - GUTTER_PADDING_RIGHT),
                    window,
                    cx,
                )
                .ok();
            line.text
                .paint(
                    text_origin,
                    prepaint.position_map.line_height,
                    TextAlign::Left,
                    None,
                    window,
                    cx,
                )
                .ok();
        }

        if focused && let Some(cursor) = prepaint.cursor.take() {
            window.paint_quad(cursor);
        }

        self.editor.update(cx, |editor, _| {
            editor.last_position_map = Some(prepaint.position_map.clone());
        });
    }
}

fn text_runs_for_line(
    line_start: usize,
    line_end: usize,
    line_len: usize,
    marked_range: Option<std::ops::Range<usize>>,
    font: gpui::Font,
    color: Hsla,
) -> Vec<TextRun> {
    let base = TextRun {
        len: line_len,
        font,
        color,
        background_color: None,
        underline: None,
        strikethrough: None,
    };

    let Some(marked_range) = marked_range else {
        return vec![base];
    };

    let mark_start = marked_range.start.max(line_start);
    let mark_end = marked_range.end.min(line_end);
    if mark_start >= mark_end {
        return vec![base];
    }

    let local_start = mark_start - line_start;
    let local_end = mark_end - line_start;
    vec![
        TextRun {
            len: local_start,
            ..base.clone()
        },
        TextRun {
            len: local_end - local_start,
            underline: Some(UnderlineStyle {
                color: Some(color),
                thickness: px(1.),
                wavy: false,
            }),
            ..base.clone()
        },
        TextRun {
            len: line_len - local_end,
            ..base
        },
    ]
    .into_iter()
    .filter(|run| run.len > 0 || line_len == 0)
    .collect()
}

fn cursor_quad(
    mode: EditorMode,
    offset: usize,
    position_map: &PositionMap,
    color: Hsla,
) -> Option<PaintQuad> {
    let origin = position_map.point_for_offset(offset)?;
    match mode {
        EditorMode::Insert => Some(fill(
            Bounds::new(origin, size(CURSOR_WIDTH, position_map.line_height)),
            color,
        )),
        EditorMode::Normal | EditorMode::Select => {
            let line = position_map
                .lines
                .iter()
                .find(|line| offset >= line.start && offset <= line.end)?;
            let local = offset.saturating_sub(line.start).min(line.end - line.start);
            let next = (local + 1).min(line.end - line.start);
            let start_x = line.shaped_line.x_for_index(local);
            let end_x = line.shaped_line.x_for_index(next).max(start_x + px(8.));
            Some(fill(
                Bounds::new(
                    point(origin.x, origin.y),
                    size(end_x - start_x, position_map.line_height),
                ),
                color.opacity(0.35),
            ))
        }
    }
}
