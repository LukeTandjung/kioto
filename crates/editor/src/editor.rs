use gpui::{
    App, ClipboardItem, Context, FocusHandle, Focusable, FontWeight, InteractiveElement as _,
    IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement as _,
    Render, ScrollHandle, ScrollWheelEvent, Styled as _, Window, div, px,
};

use crate::{
    actions::{
        AppendMode, Backspace, Copy, Cut, Delete, EDITOR_INSERT_CONTEXT, EDITOR_NORMAL_CONTEXT,
        EDITOR_SELECT_CONTEXT, Enter, InsertMode, MoveDown, MoveLeft, MoveLineEnd, MoveLineStart,
        MoveRight, MoveUp, NormalMode, Paste, Redo, SelectAll, SelectLine, SelectMode, Undo,
    },
    buffer::TextBuffer,
    display_map::{DisplayPoint, DisplaySnapshot, WrapMap},
    element::EditorElement,
    history::{Edit, History},
    position_map::PositionMap,
    selection::Selection,
    style::EditorStyle,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditorMode {
    Insert,
    Normal,
    Select,
}

impl EditorMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Insert => "INSERT",
            Self::Normal => "NORMAL",
            Self::Select => "SELECT",
        }
    }

    pub fn key_context(self) -> &'static str {
        match self {
            Self::Insert => EDITOR_INSERT_CONTEXT,
            Self::Normal => EDITOR_NORMAL_CONTEXT,
            Self::Select => EDITOR_SELECT_CONTEXT,
        }
    }
}

pub struct Editor {
    pub focus_handle: FocusHandle,
    pub buffer: TextBuffer,
    pub selection: Selection,
    pub mode: EditorMode,
    pub marked_range: Option<Selection>,
    pub last_position_map: Option<PositionMap>,
    pub scroll_handle: ScrollHandle,
    pub is_selecting: bool,
    pub style: EditorStyle,
    pub title: String,
    pub wrap_map: WrapMap,
    history: History,
}

impl Editor {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle().tab_stop(true);
        focus_handle.focus(window, cx);
        let buffer = TextBuffer::default();
        Self {
            focus_handle,
            wrap_map: WrapMap::unwrapped(&buffer),
            buffer,
            selection: Selection::caret(0),
            mode: EditorMode::Insert,
            marked_range: None,
            last_position_map: None,
            scroll_handle: ScrollHandle::new(),
            is_selecting: false,
            style: EditorStyle::default(),
            title: "scratch".into(),
            history: History::default(),
        }
    }

    pub fn with_text(mut self, text: impl AsRef<str>) -> Self {
        self.buffer = TextBuffer::new(text);
        self.wrap_map = WrapMap::unwrapped(&self.buffer);
        self.selection.set_caret(0);
        self
    }

    pub fn with_style(mut self, style: EditorStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn text(&self) -> String {
        self.buffer.text()
    }

    pub fn mode(&self) -> EditorMode {
        self.mode
    }

    pub fn selection(&self) -> Selection {
        self.selection.clone()
    }

    pub fn focus(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.focus_handle.focus(window, cx);
    }

    pub fn selected_range(&self) -> std::ops::Range<usize> {
        self.selection.range_ref()
    }

    pub fn cursor(&self) -> usize {
        self.marked_range
            .as_ref()
            .map(Selection::end)
            .unwrap_or_else(|| self.selection.cursor())
    }

    pub fn range_to_utf16(&self, range: &std::ops::Range<usize>) -> std::ops::Range<usize> {
        self.buffer.range_to_utf16(range)
    }

    pub fn range_from_utf16(&self, range: &std::ops::Range<usize>) -> std::ops::Range<usize> {
        self.buffer.range_from_utf16(range)
    }

    pub fn replace_range(
        &mut self,
        range: std::ops::Range<usize>,
        new_text: &str,
        cx: &mut Context<Self>,
    ) {
        let selection_before = self.selection.clone();
        let old_text = self.buffer.slice(range.clone()).into_owned();
        let start = range.start;
        self.buffer.replace(range.clone(), new_text);
        let cursor = start + new_text.len();
        self.selection.set_caret(cursor.min(self.buffer.len()));
        self.marked_range = None;
        self.history.record(Edit {
            range,
            old_text,
            new_text: new_text.to_string(),
            selection_before,
            selection_after: self.selection.clone(),
        });
        self.autoscroll_to_cursor();
        cx.notify();
    }

    pub fn replace_selected_text(&mut self, new_text: &str, cx: &mut Context<Self>) {
        self.replace_range(self.selection.range_ref(), new_text, cx);
    }

    pub fn set_marked_text(
        &mut self,
        range: std::ops::Range<usize>,
        new_text: &str,
        selected_range: Option<std::ops::Range<usize>>,
        cx: &mut Context<Self>,
    ) {
        let selection_before = self.selection.clone();
        let old_text = self.buffer.slice(range.clone()).into_owned();
        let start = range.start;
        self.buffer.replace(range.clone(), new_text);
        if new_text.is_empty() {
            self.marked_range = None;
            self.selection.set_caret(start);
        } else {
            let marked = start..start + new_text.len();
            self.marked_range = Some(Selection::range(marked.clone()));
            let selection = selected_range
                .map(|range| start + range.start..start + range.end)
                .unwrap_or_else(|| marked.end..marked.end);
            self.selection.set_range(selection);
        }
        self.history.record(Edit {
            range,
            old_text,
            new_text: new_text.to_string(),
            selection_before,
            selection_after: self.selection.clone(),
        });
        cx.notify();
    }

    pub fn undo(&mut self, _: &Undo, window: &mut Window, cx: &mut Context<Self>) {
        let Some(transaction) = self.history.undo() else {
            window.play_system_bell();
            return;
        };
        for edit in transaction.iter().rev() {
            let start = edit.range.start;
            self.buffer
                .replace(start..start + edit.new_text.len(), &edit.old_text);
        }
        if let Some(first) = transaction.first() {
            self.selection = first.selection_before.clone();
        }
        self.marked_range = None;
        self.ensure_normal_selection();
        self.autoscroll_to_cursor();
        cx.notify();
    }

    pub fn redo(&mut self, _: &Redo, window: &mut Window, cx: &mut Context<Self>) {
        let Some(transaction) = self.history.redo() else {
            window.play_system_bell();
            return;
        };
        for edit in &transaction {
            self.buffer.replace(edit.range.clone(), &edit.new_text);
        }
        if let Some(last) = transaction.last() {
            self.selection = last.selection_after.clone();
        }
        self.marked_range = None;
        self.ensure_normal_selection();
        self.autoscroll_to_cursor();
        cx.notify();
    }

    pub fn move_left(&mut self, _: &MoveLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.move_horizontal(false, cx);
    }

    pub fn move_right(&mut self, _: &MoveRight, _: &mut Window, cx: &mut Context<Self>) {
        self.move_horizontal(true, cx);
    }

    pub fn move_up(&mut self, _: &MoveUp, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertical(false, cx);
    }

    pub fn move_down(&mut self, _: &MoveDown, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertical(true, cx);
    }

    pub fn move_line_start(&mut self, _: &MoveLineStart, _: &mut Window, cx: &mut Context<Self>) {
        let (row, _) = self.buffer.offset_to_point(self.cursor());
        let offset = self.buffer.line(row).range.start;
        self.apply_motion(offset, cx);
    }

    pub fn move_line_end(&mut self, _: &MoveLineEnd, _: &mut Window, cx: &mut Context<Self>) {
        let (row, _) = self.buffer.offset_to_point(self.cursor());
        let line = self.buffer.line(row);
        let offset = if self.mode == EditorMode::Insert || line.is_empty() {
            line.range.end
        } else {
            self.buffer.previous_grapheme_boundary(line.range.end)
        };
        self.apply_motion(offset, cx);
    }

    pub fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        if self.mode != EditorMode::Insert {
            return;
        }

        if self.selection.is_empty() {
            let cursor = self.cursor();
            let previous = self.buffer.previous_grapheme_boundary(cursor);
            if previous == cursor {
                window.play_system_bell();
                return;
            }
            self.selection.set_range(previous..cursor);
        }
        self.replace_selected_text("", cx);
    }

    pub fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.mode != EditorMode::Insert {
            return;
        }

        if self.selection.is_empty() {
            let cursor = self.cursor();
            let next = self.buffer.next_grapheme_boundary(cursor);
            if next == cursor {
                window.play_system_bell();
                return;
            }
            self.selection.set_range(cursor..next);
        }
        self.replace_selected_text("", cx);
    }

    pub fn enter(&mut self, _: &Enter, _: &mut Window, cx: &mut Context<Self>) {
        if self.mode == EditorMode::Insert {
            self.replace_selected_text("\n", cx);
        }
    }

    pub fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.selection.set_range(0..self.buffer.len());
        self.mode = EditorMode::Select;
        cx.notify();
    }

    pub fn normal_mode(&mut self, _: &NormalMode, _: &mut Window, cx: &mut Context<Self>) {
        self.mode = EditorMode::Normal;
        self.marked_range = None;
        self.history.break_group();
        self.ensure_normal_selection();
        cx.notify();
    }

    pub fn insert_mode(&mut self, _: &InsertMode, _: &mut Window, cx: &mut Context<Self>) {
        let offset = self.selection.start();
        self.selection.set_caret(offset);
        self.mode = EditorMode::Insert;
        cx.notify();
    }

    pub fn append_mode(&mut self, _: &AppendMode, _: &mut Window, cx: &mut Context<Self>) {
        let offset = self.selection.end();
        self.selection.set_caret(offset);
        self.mode = EditorMode::Insert;
        cx.notify();
    }

    pub fn select_mode(&mut self, _: &SelectMode, _: &mut Window, cx: &mut Context<Self>) {
        self.mode = EditorMode::Select;
        self.ensure_normal_selection();
        cx.notify();
    }

    pub fn select_line(&mut self, _: &SelectLine, _: &mut Window, cx: &mut Context<Self>) {
        let (row, _) = self.buffer.offset_to_point(self.cursor());
        let line = self.buffer.line(row);
        let end = if row + 1 < self.buffer.line_count() {
            self.buffer.line(row + 1).range.start
        } else {
            line.range.end
        };
        self.selection.set_range(line.range.start..end);
        self.mode = EditorMode::Select;
        cx.notify();
    }

    pub fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        let range = self.effective_selection_range();
        if range.is_empty() {
            return;
        }
        cx.write_to_clipboard(ClipboardItem::new_string(
            self.buffer.slice(range).into_owned(),
        ));
    }

    pub fn cut(&mut self, _: &Cut, _: &mut Window, cx: &mut Context<Self>) {
        let range = self.effective_selection_range();
        if range.is_empty() {
            return;
        }
        let start = range.start;
        cx.write_to_clipboard(ClipboardItem::new_string(
            self.buffer.slice(range.clone()).into_owned(),
        ));
        self.replace_range(range, "", cx);
        if self.mode != EditorMode::Insert {
            self.mode = EditorMode::Normal;
            self.set_normal_selection_at(start);
        }
    }

    pub fn paste(&mut self, _: &Paste, _: &mut Window, cx: &mut Context<Self>) {
        let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) else {
            return;
        };

        if self.mode == EditorMode::Insert {
            self.replace_selected_text(&text, cx);
            return;
        }

        let insert_at = self.selection.end();
        self.replace_range(insert_at..insert_at, &text, cx);
        self.selection.set_range(insert_at..insert_at + text.len());
        self.mode = EditorMode::Normal;
        self.ensure_normal_selection();
        cx.notify();
    }

    pub fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.focus(window, cx);
        self.is_selecting = true;
        self.history.break_group();
        let offset = self.offset_for_mouse_position(event.position);
        if event.modifiers.shift || self.mode == EditorMode::Select {
            self.selection.select_to(offset);
        } else if self.mode == EditorMode::Insert {
            self.selection.set_caret(offset);
        } else {
            self.set_normal_selection_at(offset);
        }
        cx.notify();
    }

    pub fn on_mouse_up(&mut self, _: &MouseUpEvent, _: &mut Window, _: &mut Context<Self>) {
        self.is_selecting = false;
    }

    pub fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.is_selecting {
            return;
        }
        let offset = self.offset_for_mouse_position(event.position);
        self.selection.select_to(offset);
        if self.mode == EditorMode::Normal {
            self.mode = EditorMode::Select;
        }
        cx.notify();
    }

    pub fn on_scroll_wheel(
        &mut self,
        event: &ScrollWheelEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let line_height = self
            .last_position_map
            .as_ref()
            .map(|map| map.line_height)
            .unwrap_or_else(|| window.line_height());
        let delta = event.delta.pixel_delta(line_height);
        let mut offset = self.scroll_handle.offset();
        offset.y += delta.y;
        let total_height = self.wrap_map.row_count() as f32 * line_height;
        let viewport_height = self
            .last_position_map
            .as_ref()
            .map(|map| map.bounds.size.height)
            .unwrap_or(total_height);
        let min_y = (-total_height + viewport_height).min(px(0.));
        offset.y = offset.y.clamp(min_y, px(0.));
        self.scroll_handle.set_offset(offset);
        cx.stop_propagation();
        cx.notify();
    }

    fn move_horizontal(&mut self, forward: bool, cx: &mut Context<Self>) {
        let cursor = match self.mode {
            EditorMode::Insert => self.selection.cursor(),
            EditorMode::Normal => {
                if forward {
                    self.selection.end()
                } else {
                    self.selection.start()
                }
            }
            EditorMode::Select => self.selection.cursor(),
        };

        let offset = if forward {
            self.buffer.next_grapheme_boundary(cursor)
        } else {
            self.buffer.previous_grapheme_boundary(cursor)
        };
        self.apply_motion(offset, cx);
    }

    fn move_vertical(&mut self, down: bool, cx: &mut Context<Self>) {
        // Vertical motion works in display space so wrapped lines step one
        // visual row at a time.
        let snapshot = DisplaySnapshot::new(&self.buffer, &self.wrap_map);
        let point = snapshot.offset_to_display(self.cursor());
        let target_row = if down {
            (point.row + 1).min(snapshot.row_count().saturating_sub(1))
        } else {
            point.row.saturating_sub(1)
        };
        let offset = snapshot.display_to_offset(DisplayPoint {
            row: target_row,
            column: point.column,
        });
        self.apply_motion(offset, cx);
    }

    fn apply_motion(&mut self, offset: usize, cx: &mut Context<Self>) {
        let offset = offset.min(self.buffer.len());
        match self.mode {
            EditorMode::Insert => self.selection.set_caret(offset),
            EditorMode::Normal => self.set_normal_selection_at(offset),
            EditorMode::Select => self.selection.select_to(offset),
        }
        self.history.break_group();
        self.autoscroll_to_cursor();
        cx.notify();
    }

    /// Keeps the cursor's line inside the viewport after motions and edits.
    fn autoscroll_to_cursor(&mut self) {
        let Some(map) = &self.last_position_map else {
            return;
        };
        let snapshot = DisplaySnapshot::new(&self.buffer, &self.wrap_map);
        let row = snapshot.offset_to_display(self.cursor()).row;
        let mut offset = self.scroll_handle.offset();
        // The cursor row's y position relative to the viewport top.
        let cursor_top = map.line_height * row as f32 + offset.y;
        if cursor_top < px(0.) {
            offset.y -= cursor_top;
        } else if cursor_top + map.line_height > map.bounds.size.height {
            offset.y -= cursor_top + map.line_height - map.bounds.size.height;
        } else {
            return;
        }
        self.scroll_handle.set_offset(offset);
    }

    fn ensure_normal_selection(&mut self) {
        if self.mode == EditorMode::Insert || !self.selection.is_empty() || self.buffer.is_empty() {
            return;
        }
        self.set_normal_selection_at(self.selection.cursor());
    }

    fn set_normal_selection_at(&mut self, offset: usize) {
        if self.buffer.is_empty() {
            self.selection.set_caret(0);
            return;
        }

        let start = if offset >= self.buffer.len() {
            self.buffer.previous_grapheme_boundary(self.buffer.len())
        } else {
            offset
        };
        let end = self.buffer.next_grapheme_boundary(start);
        self.selection.set_range(start..end.max(start));
    }

    fn effective_selection_range(&mut self) -> std::ops::Range<usize> {
        if !self.selection.is_empty() {
            return self.selection.range_ref();
        }
        if self.mode == EditorMode::Insert || self.buffer.is_empty() {
            return self.selection.range_ref();
        }
        let cursor = self.selection.cursor();
        let start = if cursor >= self.buffer.len() {
            self.buffer.previous_grapheme_boundary(self.buffer.len())
        } else {
            cursor
        };
        start..self.buffer.next_grapheme_boundary(start)
    }

    fn offset_for_mouse_position(&self, position: gpui::Point<gpui::Pixels>) -> usize {
        self.last_position_map
            .as_ref()
            .and_then(|map| map.offset_for_point(position))
            .unwrap_or(0)
    }

    fn render_status_line(&self) -> impl IntoElement {
        let (mode_fg, mode_bg) = self.style.mode_segment(self.mode);
        let (row, column) = self.buffer.offset_to_point(self.selection.cursor());

        div()
            .flex()
            .items_stretch()
            .h(px(27.))
            .flex_none()
            .bg(self.style.status_background)
            .border_t_1()
            .border_color(self.style.status_border)
            .text_size(px(12.))
            .child(
                div()
                    .flex()
                    .items_center()
                    .px_4()
                    .bg(mode_bg)
                    .text_color(mode_fg)
                    .text_size(px(11.))
                    .font_weight(FontWeight::BOLD)
                    .child(self.mode.label()),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .px_3()
                    .text_color(self.style.text)
                    .child(self.title.clone()),
            )
            .child(div().flex_1())
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    .px_4()
                    .text_color(self.style.text_muted)
                    .child("1 sel")
                    .child(format!("Ln {}, Col {}", row + 1, column + 1))
                    .child("UTF-8")
                    .child("LF"),
            )
    }
}

impl Focusable for Editor {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Editor {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(self.style.background)
            .text_color(self.style.text)
            .font_family("JetBrains Mono")
            .text_size(px(14.))
            .line_height(px(25.))
            .key_context(self.mode.key_context())
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::enter))
            .on_action(cx.listener(Self::move_left))
            .on_action(cx.listener(Self::move_right))
            .on_action(cx.listener(Self::move_up))
            .on_action(cx.listener(Self::move_down))
            .on_action(cx.listener(Self::move_line_start))
            .on_action(cx.listener(Self::move_line_end))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::copy))
            .on_action(cx.listener(Self::cut))
            .on_action(cx.listener(Self::paste))
            .on_action(cx.listener(Self::normal_mode))
            .on_action(cx.listener(Self::insert_mode))
            .on_action(cx.listener(Self::append_mode))
            .on_action(cx.listener(Self::select_mode))
            .on_action(cx.listener(Self::select_line))
            .on_action(cx.listener(Self::undo))
            .on_action(cx.listener(Self::redo))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .on_scroll_wheel(cx.listener(Self::on_scroll_wheel))
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .child(EditorElement::new(cx.entity())),
            )
            .child(self.render_status_line())
    }
}
