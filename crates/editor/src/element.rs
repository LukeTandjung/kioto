use gpui::{
    App, Bounds, Element, ElementId, ElementInputHandler, Entity, GlobalElementId, HighlightStyle,
    Hsla, IntoElement, LayoutId, LineFragment, PaintQuad, Pixels, ShapedLine, SharedString, Style,
    TextAlign, TextRun, UnderlineStyle, Window, fill, point, px, relative, size,
};

use crate::{
    Editor,
    display_map::{DisplaySnapshot, WrapMap},
    editor::EditorMode,
    highlights::{Highlight, runs_for_line},
    position_map::{PositionLine, PositionMap},
};

const GUTTER_PADDING_LEFT: Pixels = px(8.);
const GUTTER_PADDING_RIGHT: Pixels = px(18.);
const TEXT_PADDING_RIGHT: Pixels = px(28.);
const BAR_CURSOR_WIDTH: Pixels = px(2.);

pub struct EditorElement {
    editor: Entity<Editor>,
}

impl EditorElement {
    pub fn new(editor: Entity<Editor>) -> Self {
        Self { editor }
    }
}

struct LayoutLine {
    origin_y: Pixels,
    text: ShapedLine,
    line_number: Option<ShapedLine>,
}

/// Where the cursor quad paints relative to the text layer: a block cursor
/// sits under the glyph (which is inverted to the background color), a bar
/// cursor paints on top.
enum CursorLayer {
    UnderText,
    OverText,
}

pub struct PrepaintState {
    position_map: PositionMap,
    lines: Vec<LayoutLine>,
    selection_rects: Vec<Bounds<Pixels>>,
    cursor: Option<(PaintQuad, CursorLayer)>,
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
        let style = window.text_style();
        let font_size = style.font_size.to_pixels(window.rem_size());
        let line_height = window.line_height();

        let shape_number = |text: String, color: Hsla, window: &Window| {
            let len = text.len();
            window.text_system().shape_line(
                SharedString::from(text),
                font_size,
                &[TextRun {
                    len,
                    font: style.font(),
                    color,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                }],
                None,
            )
        };

        let (buffer_lines, gutter_number_color) = {
            let editor = self.editor.read(cx);
            (
                editor.buffer.line_count(),
                Hsla::from(editor.style.gutter_number),
            )
        };
        let digits = buffer_lines.max(1).to_string().len().max(2);
        let number_sample = shape_number("8".repeat(digits), gutter_number_color, window);
        let gutter_width = GUTTER_PADDING_LEFT + number_sample.width + GUTTER_PADDING_RIGHT;
        let wrap_width = (bounds.size.width - gutter_width - TEXT_PADDING_RIGHT).max(px(0.));

        // Rebuild the wrap projection when the buffer or the width changed.
        let wrap_stale = {
            let editor = self.editor.read(cx);
            !editor
                .wrap_map
                .is_valid_for(&editor.buffer, wrap_width.as_f32())
        };
        if wrap_stale {
            let mut wrapper = window.text_system().line_wrapper(style.font(), font_size);
            self.editor.update(cx, |editor, _| {
                editor.wrap_map =
                    WrapMap::build(&editor.buffer, wrap_width.as_f32(), |line_text| {
                        wrapper
                            .wrap_line(&[LineFragment::text(line_text)], wrap_width)
                            .map(|boundary| boundary.ix)
                            .collect()
                    });
            });
        }

        let editor = self.editor.read(cx);
        let snapshot = DisplaySnapshot::new(&editor.buffer, &editor.wrap_map);
        let total_rows = snapshot.row_count();
        let cursor_offset = editor.cursor();
        let cursor_buffer_row = snapshot
            .row(snapshot.offset_to_display(cursor_offset).row)
            .buffer_row;
        let block_cursor = editor.mode != EditorMode::Insert;
        let accent: Hsla = editor.style.accent.into();
        let background: Hsla = editor.style.background.into();

        let scroll_y = editor.scroll_handle.offset().y;
        let first_row = ((-scroll_y).as_f32() / line_height.as_f32())
            .floor()
            .max(0.) as usize;
        let visible_rows = (bounds.size.height.as_f32() / line_height.as_f32()).ceil() as usize + 1;
        let last_row = (first_row + visible_rows).min(total_rows);
        let mut layout_lines = Vec::with_capacity(last_row.saturating_sub(first_row));
        let mut position_lines = Vec::with_capacity(last_row.saturating_sub(first_row));

        // Highlight sources merged into text runs at layout time. The IME
        // underline and block-cursor glyph inversion never overlap (marked
        // text exists only in Insert mode, the block cursor only outside
        // it); syntax, diagnostics, and search highlights join this list
        // later.
        let mut highlights = Vec::new();
        if let Some(marked) = &editor.marked_range {
            highlights.push(Highlight {
                range: marked.range_ref(),
                style: HighlightStyle {
                    underline: Some(UnderlineStyle {
                        color: Some(style.color),
                        thickness: px(1.),
                        wavy: false,
                    }),
                    ..Default::default()
                },
            });
        }
        if block_cursor {
            // The block cursor inverts its glyph to the background color.
            highlights.push(Highlight {
                range: cursor_offset..editor.buffer.next_grapheme_boundary(cursor_offset),
                style: HighlightStyle {
                    color: Some(background),
                    ..Default::default()
                },
            });
        }

        for index in first_row..last_row {
            let row = snapshot.row(index).clone();
            let row_text = snapshot.row_text(index);
            let runs = runs_for_line(&row.range, style.font(), style.color, &highlights);
            let shaped_text = window.text_system().shape_line(
                SharedString::from(row_text.into_owned()),
                font_size,
                &runs,
                None,
            );

            let line_number = row.is_line_start.then(|| {
                let number_color = if row.buffer_row == cursor_buffer_row {
                    editor.style.gutter_number_current.into()
                } else {
                    gutter_number_color
                };
                shape_number(
                    format!("{:>width$}", row.buffer_row + 1, width = digits),
                    number_color,
                    window,
                )
            });

            let visible_index = index - first_row;
            layout_lines.push(LayoutLine {
                origin_y: line_height * visible_index as f32,
                text: shaped_text.clone(),
                line_number,
            });
            position_lines.push(PositionLine {
                start: row.range.start,
                end: row.range.end,
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
        let cursor = cursor_quad(editor.mode, cursor_offset, &position_map, font_size, accent);
        let current_line = position_map.point_for_offset(cursor_offset).map(|point| {
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
        let (focus_handle, focused, editor_style_background, selection_color) = {
            let editor = self.editor.read(cx);
            (
                editor.focus_handle.clone(),
                editor.focus_handle.is_focused(window),
                editor.style.background,
                editor.style.selection(editor.mode),
            )
        };
        let current_line_color = self.editor.read(cx).style.current_line;

        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.editor.clone()),
            cx,
        );

        window.paint_quad(fill(bounds, editor_style_background));

        if let Some(current_line) = prepaint.current_line {
            window.paint_quad(fill(current_line, current_line_color));
        }

        for rect in prepaint.selection_rects.drain(..) {
            window.paint_quad(fill(rect, selection_color));
        }

        if focused && let Some((cursor, CursorLayer::UnderText)) = &prepaint.cursor {
            window.paint_quad(cursor.clone());
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

            if let Some(line_number) = &line.line_number {
                line_number
                    .paint(
                        line_number_origin,
                        prepaint.position_map.line_height,
                        TextAlign::Right,
                        Some(
                            prepaint.position_map.gutter_width
                                - GUTTER_PADDING_LEFT
                                - GUTTER_PADDING_RIGHT,
                        ),
                        window,
                        cx,
                    )
                    .ok();
            }
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

        if focused && let Some((cursor, CursorLayer::OverText)) = prepaint.cursor.take() {
            window.paint_quad(cursor);
        }

        self.editor.update(cx, |editor, _| {
            editor.last_position_map = Some(prepaint.position_map.clone());
        });
    }
}

/// The single warm note in the editor: the cursor. Insert mode paints a 2px
/// accent bar over the text; Normal/Select paint an accent block under the
/// (inverted) glyph.
fn cursor_quad(
    mode: EditorMode,
    offset: usize,
    position_map: &PositionMap,
    font_size: Pixels,
    accent: Hsla,
) -> Option<(PaintQuad, CursorLayer)> {
    let origin = position_map.point_for_offset(offset)?;
    match mode {
        EditorMode::Insert => Some((
            fill(
                Bounds::new(origin, size(BAR_CURSOR_WIDTH, position_map.line_height)),
                accent,
            ),
            CursorLayer::OverText,
        )),
        EditorMode::Normal | EditorMode::Select => {
            let line = position_map
                .lines
                .iter()
                .find(|line| offset >= line.start && offset <= line.end)?;
            let local = offset.saturating_sub(line.start).min(line.end - line.start);
            let next = (local + 1).min(line.end - line.start);
            let start_x = line.shaped_line.x_for_index(local);
            let end_x = line
                .shaped_line
                .x_for_index(next)
                .max(start_x + font_size * 0.6);
            Some((
                fill(
                    Bounds::new(
                        point(origin.x, origin.y),
                        size(end_x - start_x, position_map.line_height),
                    ),
                    accent,
                ),
                CursorLayer::UnderText,
            ))
        }
    }
}
