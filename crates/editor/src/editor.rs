use gpui::{
    App, ClipboardItem, Context, FocusHandle, Focusable, InteractiveElement as _, IntoElement,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement as _, Render,
    ScrollHandle, ScrollWheelEvent, Styled as _, Window, div, px, rgb,
};

use crate::{
    actions::{
        AppendMode, Backspace, Copy, Cut, Delete, EDITOR_INSERT_CONTEXT, EDITOR_NORMAL_CONTEXT,
        EDITOR_SELECT_CONTEXT, Enter, InsertMode, MoveDown, MoveLeft, MoveLineEnd, MoveLineStart,
        MoveRight, MoveUp, NormalMode, Paste, SelectAll, SelectLine, SelectMode,
    },
    buffer::TextBuffer,
    element::EditorElement,
    position_map::PositionMap,
    selection::Selection,
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
}

impl Editor {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle().tab_stop(true);
        focus_handle.focus(window, cx);
        Self {
            focus_handle,
            buffer: TextBuffer::default(),
            selection: Selection::caret(0),
            mode: EditorMode::Insert,
            marked_range: None,
            last_position_map: None,
            scroll_handle: ScrollHandle::new(),
            is_selecting: false,
        }
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.buffer = TextBuffer::new(text);
        self.selection.set_caret(0);
        self
    }

    pub fn text(&self) -> &str {
        self.buffer.as_str()
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
        let start = range.start;
        self.buffer.replace(range, new_text);
        let cursor = start + new_text.len();
        self.selection.set_caret(cursor.min(self.buffer.len()));
        self.marked_range = None;
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
        let start = range.start;
        self.buffer.replace(range, new_text);
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
        let (row, _) = self.buffer.offset_to_line_col(self.cursor());
        let offset = self.buffer.line_start_offset(row);
        self.apply_motion(offset, cx);
    }

    pub fn move_line_end(&mut self, _: &MoveLineEnd, _: &mut Window, cx: &mut Context<Self>) {
        let (row, _) = self.buffer.offset_to_line_col(self.cursor());
        let end = self.buffer.line_end_offset(row);
        let offset = if self.mode == EditorMode::Insert || end == self.buffer.line_start_offset(row)
        {
            end
        } else {
            self.buffer.previous_grapheme_boundary(end)
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
        let (row, _) = self.buffer.offset_to_line_col(self.cursor());
        let start = self.buffer.line_start_offset(row);
        let end = if row + 1 < self.buffer.line_count() {
            self.buffer.line_start_offset(row + 1)
        } else {
            self.buffer.line_end_offset(row)
        };
        self.selection.set_range(start..end);
        self.mode = EditorMode::Select;
        cx.notify();
    }

    pub fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        let range = self.effective_selection_range();
        if range.is_empty() {
            return;
        }
        cx.write_to_clipboard(ClipboardItem::new_string(
            self.buffer.slice(range).to_string(),
        ));
    }

    pub fn cut(&mut self, _: &Cut, _: &mut Window, cx: &mut Context<Self>) {
        let range = self.effective_selection_range();
        if range.is_empty() {
            return;
        }
        let start = range.start;
        cx.write_to_clipboard(ClipboardItem::new_string(
            self.buffer.slice(range.clone()).to_string(),
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
        let total_height = self.buffer.line_count() as f32 * line_height;
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
        let cursor = self.cursor();
        let (row, column) = self.buffer.offset_to_line_col(cursor);
        let target_row = if down {
            (row + 1).min(self.buffer.line_count().saturating_sub(1))
        } else {
            row.saturating_sub(1)
        };
        let offset = self.buffer.line_col_to_offset(target_row, column);
        self.apply_motion(offset, cx);
    }

    fn apply_motion(&mut self, offset: usize, cx: &mut Context<Self>) {
        let offset = offset.min(self.buffer.len());
        match self.mode {
            EditorMode::Insert => self.selection.set_caret(offset),
            EditorMode::Normal => self.set_normal_selection_at(offset),
            EditorMode::Select => self.selection.select_to(offset),
        }
        cx.notify();
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
}

impl Focusable for Editor {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Editor {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mode = self.mode;
        div()
            .size_full()
            .bg(rgb(0x0d1117))
            .text_color(rgb(0xc9d1d9))
            .text_size(px(14.))
            .line_height(px(20.))
            .key_context(mode.key_context())
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
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .on_scroll_wheel(cx.listener(Self::on_scroll_wheel))
            .child(EditorElement::new(cx.entity()))
            .child(
                div()
                    .absolute()
                    .right_2()
                    .bottom_2()
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .bg(match mode {
                        EditorMode::Insert => rgb(0x238636),
                        EditorMode::Normal => rgb(0x1f6feb),
                        EditorMode::Select => rgb(0x8957e5),
                    })
                    .text_color(rgb(0xffffff))
                    .text_size(px(11.))
                    .child(mode.label()),
            )
    }
}
