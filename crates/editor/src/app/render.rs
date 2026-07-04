//! GPUI render orchestration: `EditorView` (the entity), the block-based
//! custom `EditorElement`, keystroke translation into `core::mode::Input`,
//! and the platform text-input bridge.
//!
//! The render unit is the `PreviewBlock`, not the source line: each block
//! renders styled (markers hidden) unless the cursor is inside it, and all
//! cursor/mouse/selection geometry crosses the source ↔ display divide
//! through each block's `OffsetMap`.

use std::collections::HashMap;
use std::ops::Range as ByteRange;
use std::path::PathBuf;
use std::sync::Arc;

use gpui::{
    App, Bounds, Context, Corners, Element, ElementId, ElementInputHandler, Entity,
    EntityInputHandler, FocusHandle, Focusable, FontStyle, FontWeight, GlobalElementId, Hsla,
    InteractiveElement as _, IntoElement, KeyDownEvent, Keystroke, LayoutId, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement as _, Pixels, Point, Render,
    RenderImage, ScrollHandle, ScrollWheelEvent, ShapedLine, SharedString, Size, Style,
    Styled as _, TextAlign, TextRun, UTF16Selection, Window, div, fill, point, px, relative, rgb,
    rgba, size,
};
use image::{Frame, ImageBuffer, Rgba};
use smallvec::SmallVec;

use crate::adapters::gpui_clipboard::GpuiClipboard;
use crate::app::documents::typst::TypstDocument;
use crate::app::editor::Editor;
use crate::app::style::EditorStyle;
use crate::core::editable_buffer::EditableBuffer;
use crate::core::mode::{EditorAction, Input};
use crate::core::motion::{self, Motion};
use crate::core::position::{Position, Range};
use crate::core::preview_renderer::{
    BlockKind, PreviewBlock, PreviewOutput, PreviewRenderer as _, RenderedFragment, SpanKind,
    StyleSpan,
};
use crate::core::selection::Selection;
use crate::port::document_store::DocumentStore;

const GUTTER_PADDING_LEFT: Pixels = px(8.);
const GUTTER_PADDING_RIGHT: Pixels = px(18.);
const BAR_CURSOR_WIDTH: Pixels = px(2.);
const BASE_LINE_HEIGHT: Pixels = px(25.);
const BASE_FONT_SIZE: Pixels = px(14.);
const CODE_COLOR: u32 = 0xE9C46A;
const CHIP_BACKGROUND: u32 = 0xFFFFFF18;

/// The GPUI entity wrapping the pure `Editor`. Owns everything only the
/// view cares about: focus, scroll, the last layout for hit testing.
pub struct EditorView {
    pub editor: Editor<TypstDocument>,
    pub style: EditorStyle,
    pub title: String,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    last_layout: Option<ViewLayout>,
    drag_anchor: Option<Position>,
    /// GPU-side images for compiled fragments, keyed by the fragment's
    /// allocation — stable while the document's fragment cache holds it.
    fragment_images: HashMap<usize, Arc<RenderImage>>,
    /// The in-progress search query while the `/` bar is open; keystrokes
    /// feed it instead of the modal state machine.
    search_input: Option<String>,
    location: Option<PathBuf>,
    store: Box<dyn DocumentStore>,
}

impl EditorView {
    pub fn new(
        editor: Editor<TypstDocument>,
        style: EditorStyle,
        title: String,
        location: Option<PathBuf>,
        store: Box<dyn DocumentStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle().tab_stop(true);
        focus_handle.focus(window, cx);
        Self {
            editor,
            style,
            title,
            focus_handle,
            scroll_handle: ScrollHandle::new(),
            last_layout: None,
            drag_anchor: None,
            fragment_images: HashMap::new(),
            search_input: None,
            location,
            store,
        }
    }

    /// Renders the preview and pairs each block with the GPU image for its
    /// compiled fragment, converting bitmaps on first sight and dropping
    /// images whose fragments are gone.
    fn preview_with_images(&mut self) -> (PreviewOutput, Vec<Option<Arc<RenderImage>>>) {
        let cursor = self.editor.cursor();
        let preview = self.editor.document.render_preview(&[cursor]);
        let images: Vec<_> = preview
            .blocks
            .iter()
            .map(|block| block.rendered.as_ref().and_then(|f| self.fragment_image(f)))
            .collect();
        let live: Vec<usize> = preview
            .blocks
            .iter()
            .filter_map(|block| block.rendered.as_ref())
            .map(|fragment| Arc::as_ptr(fragment) as usize)
            .collect();
        self.fragment_images.retain(|key, _| live.contains(key));
        (preview, images)
    }

    fn fragment_image(&mut self, fragment: &Arc<RenderedFragment>) -> Option<Arc<RenderImage>> {
        let key = Arc::as_ptr(fragment) as usize;
        if let Some(image) = self.fragment_images.get(&key) {
            return Some(image.clone());
        }
        // The buffer is typed `Rgba` (the container format `image` offers)
        // but carries BGRA bytes, which is what GPUI's renderer samples.
        let buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
            fragment.width,
            fragment.height,
            fragment.bgra.clone(),
        )?;
        let image = Arc::new(RenderImage::new(SmallVec::from_elem(Frame::new(buffer), 1)));
        self.fragment_images.insert(key, image.clone());
        Some(image)
    }

    fn on_key_down(&mut self, event: &KeyDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.search_input.is_some() {
            self.handle_search_key(&event.keystroke);
            self.autoscroll_to_cursor();
            cx.stop_propagation();
            cx.notify();
            return;
        }
        if event.keystroke.modifiers.control && event.keystroke.key == "s" {
            self.save();
            cx.stop_propagation();
            cx.notify();
            return;
        }

        let Some(input) = input_for_keystroke(&event.keystroke, self.editor.mode.is_insert())
        else {
            return;
        };
        let action = self.editor.handle_input(input, &mut GpuiClipboard::new(cx));
        if action == EditorAction::StartSearch {
            self.search_input = Some(String::new());
        }
        self.autoscroll_to_cursor();
        // Mark the key handled so it does not also reach the platform
        // text-input fallback (e.g. `i` entering insert mode must not
        // insert an "i").
        cx.stop_propagation();
        cx.notify();
    }

    /// Feeds one keystroke to the open search bar: printable characters
    /// build the query, Enter commits it, Escape cancels.
    fn handle_search_key(&mut self, keystroke: &Keystroke) {
        match keystroke.key.as_str() {
            "escape" => {
                self.search_input = None;
            }
            "enter" => {
                let query = self.search_input.take().expect("search bar is open");
                self.editor.search(&query);
            }
            "backspace" => {
                if let Some(query) = self.search_input.as_mut() {
                    query.pop();
                }
            }
            "space" => {
                if let Some(query) = self.search_input.as_mut() {
                    query.push(' ');
                }
            }
            key => {
                let mut chars = key.chars();
                if let (Some(character), None, Some(query)) =
                    (chars.next(), chars.next(), self.search_input.as_mut())
                {
                    query.push(if keystroke.modifiers.shift {
                        character.to_ascii_uppercase()
                    } else {
                        character
                    });
                }
            }
        }
    }

    /// Writes the document through the store port; a no-op for editors
    /// without a location.
    fn save(&mut self) {
        let Some(location) = &self.location else {
            return;
        };
        if let Err(error) = self.store.save(location, self.editor.document.text()) {
            eprintln!("{error}");
        }
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.focus_handle.focus(window, cx);
        let Some(offset) = self.offset_for_point(event.position) else {
            return;
        };
        if event.modifiers.shift || self.editor.mode.is_visual() {
            let anchor = self
                .editor
                .selections
                .first()
                .map(|selection| selection.from_position)
                .unwrap_or_else(|| self.editor.cursor());
            self.set_drag_selection(anchor, offset);
        } else {
            self.editor.selections.clear();
            self.editor.cursors[0].set(offset);
            self.drag_anchor = Some(offset);
        }
        self.editor.break_undo_group();
        cx.notify();
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        let Some(anchor) = self.drag_anchor else {
            return;
        };
        let Some(offset) = self.offset_for_point(event.position) else {
            return;
        };
        if offset != anchor {
            self.set_drag_selection(anchor, offset);
            cx.notify();
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _: &mut Window, _: &mut Context<Self>) {
        self.drag_anchor = None;
    }

    fn on_scroll_wheel(
        &mut self,
        event: &ScrollWheelEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(layout) = &self.last_layout else {
            return;
        };
        let delta = event.delta.pixel_delta(BASE_LINE_HEIGHT);
        let mut offset = self.scroll_handle.offset();
        offset.y += delta.y;
        let min_y = (-layout.content_height + layout.bounds.size.height).min(px(0.));
        offset.y = offset.y.clamp(min_y, px(0.));
        self.scroll_handle.set_offset(offset);
        cx.stop_propagation();
        cx.notify();
    }

    fn set_drag_selection(&mut self, anchor: Position, offset: Position) {
        self.editor.selections = vec![Selection::new(anchor, offset)];
        self.editor.cursors[0].set(offset);
        self.drag_anchor = Some(anchor);
    }

    fn offset_for_point(&self, position: Point<Pixels>) -> Option<Position> {
        self.last_layout
            .as_ref()
            .map(|layout| layout.offset_for_point(position))
    }

    /// Keeps the cursor's display line inside the viewport.
    fn autoscroll_to_cursor(&mut self) {
        let Some(layout) = &self.last_layout else {
            return;
        };
        let Some(geometry) = layout.geometry_for_source(self.editor.cursor().0) else {
            return;
        };
        let mut offset = self.scroll_handle.offset();
        // geometry.origin is in unscrolled content coordinates.
        let top = geometry.origin.y - layout.bounds.top() + offset.y;
        if top < px(0.) {
            offset.y -= top;
        } else if top + geometry.height > layout.bounds.size.height {
            offset.y -= top + geometry.height - layout.bounds.size.height;
        } else {
            return;
        }
        self.scroll_handle.set_offset(offset);
    }

    fn render_status_line(&self) -> impl IntoElement {
        let (mode_fg, mode_bg) = self.style.mode_segment(&self.editor.mode);
        let text = self.editor.document.text();
        let cursor = self.editor.cursor().0;
        let lines = motion::line_ranges(text);
        let row = lines
            .iter()
            .rposition(|line| line.start <= cursor)
            .unwrap_or(0);
        let column = cursor.saturating_sub(lines[row].start);

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
                    .child(self.editor.mode.label()),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .px_3()
                    .text_color(self.style.text)
                    .child(self.title.clone()),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .px_3()
                    .text_color(self.style.text)
                    .children(
                        self.search_input
                            .as_ref()
                            .map(|query| format!("/{query}\u{2588}")),
                    ),
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

impl Focusable for EditorView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for EditorView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(self.style.background)
            .text_color(self.style.text)
            .font_family("JetBrains Mono")
            .text_size(BASE_FONT_SIZE)
            .line_height(BASE_LINE_HEIGHT)
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::on_key_down))
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

/// Translates a GPUI keystroke into a modal `Input`. Printable keys in
/// insert mode return `None` — they arrive as committed text through the
/// platform input path instead.
fn input_for_keystroke(keystroke: &Keystroke, insert_mode: bool) -> Option<Input> {
    let named = match keystroke.key.as_str() {
        "escape" => Some(Input::Escape),
        "enter" => Some(Input::Enter),
        "backspace" => Some(Input::Backspace),
        "delete" => Some(Input::Delete),
        "left" => Some(Input::Left),
        "right" => Some(Input::Right),
        "up" => Some(Input::Up),
        "down" => Some(Input::Down),
        "home" => Some(Input::Home),
        "end" => Some(Input::End),
        _ => None,
    };
    if named.is_some() {
        return named;
    }
    if insert_mode
        || keystroke.modifiers.control
        || keystroke.modifiers.alt
        || keystroke.modifiers.platform
        || keystroke.modifiers.function
    {
        return None;
    }

    let mut chars = keystroke.key.chars();
    let character = chars.next()?;
    if chars.next().is_some() {
        return None;
    }
    Some(Input::Char(if keystroke.modifiers.shift {
        character.to_ascii_uppercase()
    } else {
        character
    }))
}

// ---------------------------------------------------------------------------
// Platform text input (IME bridge)
// ---------------------------------------------------------------------------

impl EntityInputHandler for EditorView {
    fn text_for_range(
        &mut self,
        range_utf16: ByteRange<usize>,
        adjusted_range: &mut Option<ByteRange<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let text = self.editor.document.text();
        let range = range_from_utf16(text, &range_utf16);
        adjusted_range.replace(range_to_utf16(text, &range));
        Some(text[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        let text = self.editor.document.text();
        let (range, reversed) = match self.editor.selections.first() {
            Some(selection) => (
                Range::new(selection.from_position, selection.to_position),
                selection.to_position < selection.from_position,
            ),
            None => (Range::caret(self.editor.cursor()), false),
        };
        Some(UTF16Selection {
            range: range_to_utf16(text, &range.into()),
            reversed,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<ByteRange<usize>> {
        // Composition currently commits directly; marked-text preview is a
        // known regression tracked for a later milestone.
        None
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<ByteRange<usize>>,
        new_text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.editor.mode.is_insert() {
            return;
        }
        if let Some(range_utf16) = &range_utf16 {
            let text = self.editor.document.text();
            let range = range_from_utf16(text, range_utf16);
            self.editor.selections =
                vec![Selection::new(Position(range.start), Position(range.end))];
        }
        self.editor.insert_text(new_text);
        self.autoscroll_to_cursor();
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<ByteRange<usize>>,
        new_text: &str,
        _new_selected_range_utf16: Option<ByteRange<usize>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.replace_text_in_range(range_utf16, new_text, window, cx);
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: ByteRange<usize>,
        _bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let range = range_from_utf16(self.editor.document.text(), &range_utf16);
        let layout = self.last_layout.as_ref()?;
        let start = layout.geometry_for_source(range.start)?;
        let end = layout.geometry_for_source(range.end)?;
        Some(Bounds::from_corners(
            start.origin,
            point(end.origin.x, end.origin.y + end.height),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let offset = self.last_layout.as_ref()?.offset_for_point(point);
        let text = self.editor.document.text();
        Some(range_to_utf16(text, &(offset.0..offset.0)).start)
    }
}

fn range_to_utf16(text: &str, range: &ByteRange<usize>) -> ByteRange<usize> {
    offset_to_utf16(text, range.start)..offset_to_utf16(text, range.end)
}

fn range_from_utf16(text: &str, range: &ByteRange<usize>) -> ByteRange<usize> {
    offset_from_utf16(text, range.start)..offset_from_utf16(text, range.end)
}

fn offset_to_utf16(text: &str, offset: usize) -> usize {
    let mut utf16 = 0;
    let mut utf8 = 0;
    for character in text.chars() {
        if utf8 >= offset {
            break;
        }
        utf8 += character.len_utf8();
        utf16 += character.len_utf16();
    }
    utf16
}

fn offset_from_utf16(text: &str, offset: usize) -> usize {
    let mut utf16 = 0;
    let mut utf8 = 0;
    for character in text.chars() {
        if utf16 >= offset {
            break;
        }
        utf16 += character.len_utf16();
        utf8 += character.len_utf8();
    }
    utf8.min(text.len())
}

// ---------------------------------------------------------------------------
// Block layout and hit testing
// ---------------------------------------------------------------------------

/// One shaped display row of one block.
struct LayoutLine {
    block: usize,
    /// Range within the block's display text that this row shows.
    display_range: ByteRange<usize>,
    shaped: ShapedLine,
    /// y offset in unscrolled content coordinates.
    content_y: Pixels,
    height: Pixels,
    line_number: Option<ShapedLine>,
    /// Compiled fragment drawn in place of text, with its logical size.
    image: Option<(Arc<RenderImage>, Size<Pixels>)>,
}

/// Geometry of one source position in the viewport.
struct SourceGeometry {
    origin: Point<Pixels>,
    height: Pixels,
    line_index: usize,
}

struct ViewLayout {
    bounds: Bounds<Pixels>,
    gutter_width: Pixels,
    scroll_y: Pixels,
    content_height: Pixels,
    blocks: Vec<PreviewBlock>,
    lines: Vec<LayoutLine>,
}

impl ViewLayout {
    fn line_y(&self, line: &LayoutLine) -> Pixels {
        self.bounds.top() + line.content_y + self.scroll_y
    }

    fn offset_for_point(&self, position: Point<Pixels>) -> Position {
        let Some((index, line)) = self.line_for_y(position.y) else {
            return Position(0);
        };
        let _ = index;
        let x = position.x - self.bounds.left() - self.gutter_width;
        let column = line.shaped.closest_index_for_x(x);
        let display = line.display_range.start + column.min(line.display_range.len());
        let block = &self.blocks[line.block];
        Position(block.offset_map.display_to_source(display))
    }

    /// The viewport geometry of a source offset, resolved through its
    /// block's offset map.
    fn geometry_for_source(&self, source: usize) -> Option<SourceGeometry> {
        let block_index = self
            .blocks
            .iter()
            .rposition(|block| block.source_range.start.0 <= source)
            .unwrap_or(0);
        let block = self.blocks.get(block_index)?;
        let display = block
            .offset_map
            .source_to_display(source.min(block.source_range.end.0));

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

    fn selection_rects(&self, source: ByteRange<usize>) -> Vec<Bounds<Pixels>> {
        let mut rects = Vec::new();
        for line in &self.lines {
            let block = &self.blocks[line.block];
            let block_range: ByteRange<usize> = block.source_range.into();
            let start = source.start.max(block_range.start);
            let end = source.end.min(block_range.end);
            if start >= end {
                continue;
            }
            let display_start = block.offset_map.source_to_display(start);
            let display_end = block.offset_map.source_to_display(end);
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

    fn line_for_y(&self, y: Pixels) -> Option<(usize, &LayoutLine)> {
        if self.lines.is_empty() {
            return None;
        }
        let mut candidate = 0;
        for (index, line) in self.lines.iter().enumerate() {
            if y >= self.line_y(line) {
                candidate = index;
            } else {
                break;
            }
        }
        Some((candidate, &self.lines[candidate]))
    }
}

// ---------------------------------------------------------------------------
// Custom element
// ---------------------------------------------------------------------------

/// Per-block typography: `(font size, line height, weight, base color
/// override)`.
fn block_metrics(kind: BlockKind, style: &EditorStyle) -> (Pixels, Pixels, FontWeight, Hsla) {
    let text: Hsla = style.text.into();
    match kind {
        BlockKind::Heading(1) => (px(22.), px(36.), FontWeight::BOLD, gpui::white()),
        BlockKind::Heading(2) => (px(18.), px(31.), FontWeight::BOLD, gpui::white()),
        BlockKind::Heading(_) => (px(15.), px(27.), FontWeight::BOLD, gpui::white()),
        BlockKind::CodeBlock => (px(13.), px(22.), FontWeight::NORMAL, rgb(CODE_COLOR).into()),
        BlockKind::MathBlock => (
            BASE_FONT_SIZE,
            BASE_LINE_HEIGHT,
            FontWeight::NORMAL,
            rgb(CODE_COLOR).into(),
        ),
        BlockKind::Paragraph | BlockKind::ListItem | BlockKind::Raw | BlockKind::Rendered => {
            (BASE_FONT_SIZE, BASE_LINE_HEIGHT, FontWeight::NORMAL, text)
        }
    }
}

struct EditorElement {
    view: Entity<EditorView>,
}

impl EditorElement {
    fn new(view: Entity<EditorView>) -> Self {
        Self { view }
    }
}

enum CursorLayer {
    UnderText,
    OverText,
}

struct PrepaintState {
    layout: ViewLayout,
    selection_rects: Vec<Bounds<Pixels>>,
    cursor: Option<(Bounds<Pixels>, CursorLayer)>,
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
        let (preview, images) = self.view.update(cx, |view, _| view.preview_with_images());
        let view = self.view.read(cx);
        let text_style = window.text_style();
        let base_font = text_style.font();

        let text = view.editor.document.text();
        let source_lines = motion::line_ranges(text);
        let cursor = view.editor.cursor();
        let insert_mode = view.editor.mode.is_insert();

        // Gutter sized by the source line count.
        let digits = source_lines.len().to_string().len().max(2);
        let shape_plain = |text: SharedString, size: Pixels, color: Hsla, window: &Window| {
            let len = text.len();
            window.text_system().shape_line(
                text,
                size,
                &[TextRun {
                    len,
                    font: base_font.clone(),
                    color,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                }],
                None,
            )
        };
        let sample = shape_plain(
            "8".repeat(digits).into(),
            BASE_FONT_SIZE,
            view.style.gutter_number.into(),
            window,
        );
        let gutter_width = GUTTER_PADDING_LEFT + sample.width + GUTTER_PADDING_RIGHT;

        let cursor_source_row = source_lines
            .iter()
            .rposition(|line| line.start <= cursor.0)
            .unwrap_or(0);

        // Lay every block out top to bottom. Gap rows between blocks come
        // from the source's blank lines so lists stay tight and paragraphs
        // breathe.
        let mut lines: Vec<LayoutLine> = Vec::new();
        let mut content_y = px(0.);
        let mut previous_end: usize = 0;
        let mut last_numbered_row: Option<usize> = None;

        for (block_index, block) in preview.blocks.iter().enumerate() {
            let gap_text =
                &text[previous_end.min(block.source_range.start.0)..block.source_range.start.0];
            let gap_rows = gap_text
                .matches('\n')
                .count()
                .saturating_sub(if block_index == 0 { 0 } else { 1 });
            content_y += BASE_LINE_HEIGHT * gap_rows as f32;
            previous_end = block.source_range.end.0;

            let (font_size, line_height, weight, base_color) =
                block_metrics(block.kind, &view.style);

            // A source line number for the row showing display offset
            // `display`, once per source row.
            let number_for_display =
                |display: usize, last: &mut Option<usize>, window: &Window| -> Option<ShapedLine> {
                    let source = block.offset_map.display_to_source(display);
                    let source_row = source_lines
                        .iter()
                        .rposition(|line| line.start <= source)
                        .unwrap_or(0);
                    let number = (*last != Some(source_row)).then(|| {
                        let color = if source_row == cursor_source_row {
                            view.style.gutter_number_current.into()
                        } else {
                            view.style.gutter_number.into()
                        };
                        shape_plain(
                            format!("{:>width$}", source_row + 1, width = digits).into(),
                            px(12.),
                            color,
                            window,
                        )
                    });
                    *last = Some(source_row);
                    number
                };

            // A compiled fragment renders as one image line; the styled text
            // below is the fallback when compilation failed.
            if let Some(image) = &images[block_index]
                && let Some(fragment) = &block.rendered
            {
                let logical = size(px(fragment.logical_width), px(fragment.logical_height));
                let height = logical.height.max(line_height);
                let line_number = number_for_display(0, &mut last_numbered_row, window);
                lines.push(LayoutLine {
                    block: block_index,
                    display_range: 0..0,
                    shaped: window.text_system().shape_line(
                        SharedString::default(),
                        font_size,
                        &row_runs(0, &(0..0), &[], &base_font, weight, base_color),
                        None,
                    ),
                    content_y,
                    height,
                    line_number,
                    image: Some((image.clone(), logical)),
                });
                content_y += height;
                continue;
            }

            for row_range in display_rows(&block.display_text) {
                let row_text = &block.display_text[row_range.clone()];
                let runs = row_runs(
                    row_text.len(),
                    &row_range,
                    &block.spans,
                    &base_font,
                    weight,
                    base_color,
                );
                let shaped = window.text_system().shape_line(
                    SharedString::from(row_text.to_string()),
                    font_size,
                    &runs,
                    None,
                );

                let line_number =
                    number_for_display(row_range.start, &mut last_numbered_row, window);

                lines.push(LayoutLine {
                    block: block_index,
                    display_range: row_range,
                    shaped,
                    content_y,
                    height: line_height,
                    line_number,
                    image: None,
                });
                content_y += line_height;
            }
        }
        let content_height = content_y + BASE_LINE_HEIGHT;

        let layout = ViewLayout {
            bounds,
            gutter_width,
            scroll_y: view.scroll_handle.offset().y,
            content_height,
            blocks: preview.blocks,
            lines,
        };

        let selection_rects = view
            .editor
            .selected_range()
            .map(|range| layout.selection_rects(range.into()))
            .unwrap_or_default();

        let cursor_quad = layout.geometry_for_source(cursor.0).map(|geometry| {
            if insert_mode {
                (
                    Bounds::new(geometry.origin, size(BAR_CURSOR_WIDTH, geometry.height)),
                    CursorLayer::OverText,
                )
            } else {
                let line = &layout.lines[geometry.line_index];
                let block = &layout.blocks[line.block];
                let next = motion::resolve(text, cursor, Motion::Right).0;
                let display_next = block
                    .offset_map
                    .source_to_display(next.min(block.source_range.end.0))
                    .clamp(line.display_range.start, line.display_range.end)
                    - line.display_range.start;
                let end_x = line.shaped.x_for_index(display_next).max(
                    geometry.origin.x - layout.bounds.left() - layout.gutter_width
                        + BASE_FONT_SIZE * 0.6,
                );
                let width =
                    (layout.bounds.left() + layout.gutter_width + end_x) - geometry.origin.x;
                (
                    Bounds::new(geometry.origin, size(width, geometry.height)),
                    CursorLayer::UnderText,
                )
            }
        });

        let current_line = layout.geometry_for_source(cursor.0).map(|geometry| {
            Bounds::new(
                point(bounds.left(), geometry.origin.y),
                size(bounds.size.width, geometry.height),
            )
        });

        PrepaintState {
            layout,
            selection_rects,
            cursor: cursor_quad,
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
        let (focus_handle, focused, background, selection_color, current_line_color, accent) = {
            let view = self.view.read(cx);
            (
                view.focus_handle.clone(),
                view.focus_handle.is_focused(window),
                view.style.background,
                view.style.selection(&view.editor.mode),
                view.style.current_line,
                Hsla::from(view.style.accent),
            )
        };

        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.view.clone()),
            cx,
        );

        window.paint_quad(fill(bounds, background));

        if let Some(current_line) = prepaint.current_line {
            window.paint_quad(fill(current_line, current_line_color));
        }
        for rect in prepaint.selection_rects.drain(..) {
            window.paint_quad(fill(rect, selection_color));
        }
        if focused && let Some((cursor, CursorLayer::UnderText)) = &prepaint.cursor {
            window.paint_quad(fill(*cursor, accent));
        }

        let top = bounds.top();
        let bottom = bounds.bottom();
        for line in &prepaint.layout.lines {
            let y = prepaint.layout.line_y(line);
            if y + line.height < top || y > bottom {
                continue;
            }
            if let Some(number) = &line.line_number {
                number
                    .paint(
                        point(bounds.left() + GUTTER_PADDING_LEFT, y),
                        line.height,
                        TextAlign::Right,
                        Some(
                            prepaint.layout.gutter_width
                                - GUTTER_PADDING_LEFT
                                - GUTTER_PADDING_RIGHT,
                        ),
                        window,
                        cx,
                    )
                    .ok();
            }
            if let Some((image, logical)) = &line.image {
                let origin = point(
                    bounds.left() + prepaint.layout.gutter_width,
                    y + (line.height - logical.height) / 2.,
                );
                window
                    .paint_image(
                        Bounds::new(origin, *logical),
                        Corners::default(),
                        image.clone(),
                        0,
                        false,
                    )
                    .ok();
                continue;
            }
            line.shaped
                .paint(
                    point(bounds.left() + prepaint.layout.gutter_width, y),
                    line.height,
                    TextAlign::Left,
                    None,
                    window,
                    cx,
                )
                .ok();
        }

        if focused && let Some((cursor, CursorLayer::OverText)) = &prepaint.cursor {
            window.paint_quad(fill(*cursor, accent));
        }

        let layout = std::mem::replace(
            &mut prepaint.layout,
            ViewLayout {
                bounds,
                gutter_width: px(0.),
                scroll_y: px(0.),
                content_height: px(0.),
                blocks: Vec::new(),
                lines: Vec::new(),
            },
        );
        self.view.update(cx, |view, _| {
            view.last_layout = Some(layout);
        });
    }
}

/// Byte ranges of each display row in a block's display text.
fn display_rows(display_text: &str) -> Vec<ByteRange<usize>> {
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

/// Builds the styled text runs for one display row by cutting the row at
/// span boundaries. Spans are non-overlapping (the walker emits them that
/// way); unstyled segments use the block's base weight and color.
fn row_runs(
    row_len: usize,
    row_range: &ByteRange<usize>,
    spans: &[StyleSpan],
    base_font: &gpui::Font,
    base_weight: FontWeight,
    base_color: Hsla,
) -> Vec<TextRun> {
    let mut base_font = base_font.clone();
    base_font.weight = base_weight;

    let make = |len: usize, kind: Option<SpanKind>| {
        let mut font = base_font.clone();
        let mut color = base_color;
        let mut background = None;
        match kind {
            Some(SpanKind::Strong) => font.weight = FontWeight::BOLD,
            Some(SpanKind::Emphasis) => font.style = FontStyle::Italic,
            Some(SpanKind::InlineCode) | Some(SpanKind::InlineMath) => {
                color = rgb(CODE_COLOR).into();
                background = Some(rgba(CHIP_BACKGROUND).into());
            }
            Some(SpanKind::Plain) | None => {}
        }
        TextRun {
            len,
            font,
            color,
            background_color: background,
            underline: None,
            strikethrough: None,
        }
    };

    if row_len == 0 {
        return vec![make(0, None)];
    }

    let mut cuts = vec![0, row_len];
    for span in spans {
        let start = span.range.start.clamp(row_range.start, row_range.end) - row_range.start;
        let end = span.range.end.clamp(row_range.start, row_range.end) - row_range.start;
        if start < end {
            cuts.push(start);
            cuts.push(end);
        }
    }
    cuts.sort_unstable();
    cuts.dedup();

    let mut runs = Vec::with_capacity(cuts.len() - 1);
    for pair in cuts.windows(2) {
        let (start, end) = (pair[0], pair[1]);
        if start == end {
            continue;
        }
        let absolute = row_range.start + start;
        let kind = spans
            .iter()
            .find(|span| span.range.start <= absolute && absolute < span.range.end)
            .map(|span| span.kind);
        runs.push(make(end - start, kind));
    }
    runs
}
