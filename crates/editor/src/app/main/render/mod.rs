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

mod layout;

use crate::app::editor::Editor;
use crate::app::style::EditorStyle;
use crate::core::document::EditorDocument;
use crate::core::editable_buffer::EditableBuffer;
use crate::core::mode::{EditorAction, Input};
use crate::core::motion::{self, Motion};
use crate::core::position::{Position, Range};
use crate::core::preview_renderer::{
    BlockKind, PreviewOutput, PreviewRenderer, RenderedFragment, SpanKind, StyleSpan,
};
use crate::core::selection::Selection;
use crate::port::clipboard::Clipboard;
use crate::port::document_store::{DocumentLocation, DocumentStore};

use layout::{LayoutLine, RowSegment, ViewLayout, display_rows};

const GUTTER_PADDING_LEFT: Pixels = px(8.);
const GUTTER_PADDING_RIGHT: Pixels = px(18.);
const BAR_CURSOR_WIDTH: Pixels = px(2.);
const BASE_LINE_HEIGHT: Pixels = px(25.);
const BASE_FONT_SIZE: Pixels = px(14.);
const CODE_COLOR: u32 = 0xE9C46A;
const CHIP_BACKGROUND: u32 = 0xFFFFFF18;

type ClipboardFactory = for<'a> fn(&'a mut App) -> Box<dyn Clipboard + 'a>;

/// GPU images for one block's compiled fragments, resolved in prepaint.
struct BlockImages {
    /// The whole-block image of a rendered math block.
    whole: Option<Arc<RenderImage>>,
    /// Images standing in for ranges of the display text (inline math).
    inline: Vec<InlineImage>,
}

struct InlineImage {
    display_range: ByteRange<usize>,
    image: Arc<RenderImage>,
    /// Logical draw size.
    size: Size<Pixels>,
}

/// The GPUI entity wrapping the pure `Editor`. Owns everything only the
/// view cares about: focus, scroll, the last layout for hit testing.
pub struct EditorView {
    editor: Editor<Box<dyn EditorDocument>>,
    style: EditorStyle,
    title: String,
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
    location: Option<DocumentLocation>,
    store: Box<dyn DocumentStore>,
    clipboard_factory: ClipboardFactory,
}

impl EditorView {
    pub fn new(
        editor: Editor<Box<dyn EditorDocument>>,
        style: EditorStyle,
        title: String,
        location: Option<DocumentLocation>,
        store: Box<dyn DocumentStore>,
        clipboard_factory: ClipboardFactory,
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
            clipboard_factory,
        }
    }

    /// Renders the preview and pairs each block with the GPU images for its
    /// compiled fragments — the whole-block one for rendered math blocks and
    /// the inline ones for inline math — converting bitmaps on first sight
    /// and dropping images whose fragments are gone.
    fn preview_with_images(&mut self) -> (PreviewOutput, Vec<BlockImages>) {
        let cursor = self.editor.cursor();
        let preview = self.editor.document.render_preview(&[cursor]);
        let images: Vec<_> = preview
            .blocks
            .iter()
            .map(|block| BlockImages {
                whole: block
                    .rendered_fragment()
                    .and_then(|f| self.fragment_image(f)),
                inline: block
                    .inline_fragments()
                    .iter()
                    .filter_map(|inline| {
                        let image = self.fragment_image(&inline.fragment)?;
                        Some(InlineImage {
                            display_range: inline.display_range.clone(),
                            image,
                            size: size(
                                px(inline.fragment.logical_width),
                                px(inline.fragment.logical_height),
                            ),
                        })
                    })
                    .collect(),
            })
            .collect();
        let live: Vec<usize> = preview
            .blocks
            .iter()
            .flat_map(|block| {
                block.rendered_fragment().into_iter().chain(
                    block
                        .inline_fragments()
                        .iter()
                        .map(|inline| &inline.fragment),
                )
            })
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
        let action = {
            let mut clipboard = (self.clipboard_factory)(cx);
            self.editor.handle_input(input, clipboard.as_mut())
        };
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
        let Some(layout) = self.last_layout.as_ref() else {
            return;
        };
        let offset = layout.offset_for_point(event.position);
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
        let Some(layout) = self.last_layout.as_ref() else {
            return;
        };
        let offset = layout.offset_for_point(event.position);
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
        let range = Range::from_utf16(text, range_utf16);
        adjusted_range.replace(range.to_utf16(text));
        Some(range.slice(text).to_string())
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
            range: range.to_utf16(text),
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
            let range = Range::from_utf16(text, range_utf16.clone());
            self.editor.selections = vec![Selection::new(range.start, range.end)];
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
        let text = self.editor.document.text();
        let range = Range::from_utf16(text, range_utf16);
        let byte_range = range.byte_range_in(text);
        let layout = self.last_layout.as_ref()?;
        let start = layout.geometry_for_source(byte_range.start)?;
        let end = layout.geometry_for_source(byte_range.end)?;
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
        Some(Range::caret(offset).to_utf16(text).start)
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
        let source_row_of = |offset: usize| {
            source_lines
                .iter()
                .rposition(|line| line.start <= offset)
                .unwrap_or(0)
        };
        let shape_number = |source_row: usize, window: &Window| {
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
        };

        // Lay every block out top to bottom. Gap rows between blocks come
        // from the source's blank lines so lists stay tight and paragraphs
        // breathe; they carry no text but still show their line numbers.
        let mut lines: Vec<LayoutLine> = Vec::new();
        let mut gap_numbers: Vec<(Pixels, ShapedLine)> = Vec::new();
        let mut content_y = px(0.);
        let mut previous_row: usize = 0;
        let mut last_numbered_row: Option<usize> = None;

        for (block_index, block) in preview.blocks.iter().enumerate() {
            let block_row = source_row_of(block.source_range.start.0);
            let gap_start_row = if block_index == 0 {
                0
            } else {
                previous_row + 1
            };
            for gap_row in gap_start_row..block_row {
                gap_numbers.push((content_y, shape_number(gap_row, window)));
                content_y += BASE_LINE_HEIGHT;
            }
            previous_row = source_row_of(block.source_range.end.0);

            let (font_size, line_height, weight, base_color) =
                block_metrics(block.kind(), &view.style);

            // A source line number for the row showing display offset
            // `display`, once per source row.
            let number_for_display =
                |display: usize, last: &mut Option<usize>, window: &Window| -> Option<ShapedLine> {
                    let source = block.offset_map.display_to_source(display);
                    let source_row = source_row_of(source.0);
                    let number =
                        (*last != Some(source_row)).then(|| shape_number(source_row, window));
                    *last = Some(source_row);
                    number
                };

            // A whole-block compiled fragment renders as one image row
            // covering the full display text; the styled text below is the
            // fallback when compilation failed.
            if let Some(image) = &images[block_index].whole
                && let Some(fragment) = block.rendered_fragment()
            {
                let logical = size(px(fragment.logical_width), px(fragment.logical_height));
                let height = logical.height.max(line_height);
                let display_range = 0..block.display_text().len();
                let line_number = number_for_display(0, &mut last_numbered_row, window);
                lines.push(LayoutLine {
                    block: block_index,
                    display_range: display_range.clone(),
                    segments: vec![RowSegment::Image {
                        display_range,
                        image: image.clone(),
                        size: logical,
                        x: px(0.),
                    }],
                    content_y,
                    height,
                    line_number,
                });
                content_y += height;
                continue;
            }

            for row_range in display_rows(block.display_text()) {
                // Cut the row at inline-image boundaries: text pieces are
                // shaped, image pieces stand in for their display bytes.
                let mut segments = Vec::new();
                let mut x = px(0.);
                let mut height = line_height;
                let mut piece_start = row_range.start;
                let shape_text = |range: ByteRange<usize>, x: Pixels| {
                    let piece_text = &block.display_text()[range.clone()];
                    let runs = row_runs(
                        piece_text.len(),
                        &range,
                        block.spans(),
                        &base_font,
                        weight,
                        base_color,
                    );
                    let shaped = window.text_system().shape_line(
                        SharedString::from(piece_text.to_string()),
                        font_size,
                        &runs,
                        None,
                    );
                    let width = shaped.width();
                    (
                        RowSegment::Text {
                            display_range: range,
                            shaped: Box::new(shaped),
                            x,
                        },
                        width,
                    )
                };
                for inline in &images[block_index].inline {
                    if inline.display_range.start < row_range.start
                        || inline.display_range.end > row_range.end
                    {
                        continue;
                    }
                    if piece_start < inline.display_range.start {
                        let (segment, width) =
                            shape_text(piece_start..inline.display_range.start, x);
                        segments.push(segment);
                        x += width;
                    }
                    segments.push(RowSegment::Image {
                        display_range: inline.display_range.clone(),
                        image: inline.image.clone(),
                        size: inline.size,
                        x,
                    });
                    x += inline.size.width;
                    height = height.max(inline.size.height);
                    piece_start = inline.display_range.end;
                }
                let (segment, _) = shape_text(piece_start..row_range.end, x);
                segments.push(segment);

                let line_number =
                    number_for_display(row_range.start, &mut last_numbered_row, window);

                lines.push(LayoutLine {
                    block: block_index,
                    display_range: row_range,
                    segments,
                    content_y,
                    height,
                    line_number,
                });
                content_y += height;
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
            gap_numbers,
        };

        let selection_rects = view
            .editor
            .selected_range()
            .map(|range| layout.selection_rects(range.byte_range_in(text)))
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
                    .source_to_display(Position(next.min(block.source_range.end.0)))
                    .clamp(line.display_range.start, line.display_range.end);
                let end_x = line.x_for_display(display_next).max(
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
        for (content_y, number) in &prepaint.layout.gap_numbers {
            let y = top + *content_y + prepaint.layout.scroll_y;
            if y + BASE_LINE_HEIGHT < top || y > bottom {
                continue;
            }
            number
                .paint(
                    point(bounds.left() + GUTTER_PADDING_LEFT, y),
                    BASE_LINE_HEIGHT,
                    TextAlign::Right,
                    Some(prepaint.layout.gutter_width - GUTTER_PADDING_LEFT - GUTTER_PADDING_RIGHT),
                    window,
                    cx,
                )
                .ok();
        }
        for line in &prepaint.layout.lines {
            let y = prepaint.layout.line_y(line);
            if y + line.height < top || y > bottom {
                continue;
            }
            if let Some(number) = &line.line_number {
                // Numbers align to the first text baseline even when an
                // image makes the row taller than a text line.
                number
                    .paint(
                        point(bounds.left() + GUTTER_PADDING_LEFT, y),
                        BASE_LINE_HEIGHT.min(line.height),
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
            let left = bounds.left() + prepaint.layout.gutter_width;
            for segment in &line.segments {
                match segment {
                    RowSegment::Text { shaped, x, .. } => {
                        shaped
                            .paint(
                                point(left + *x, y),
                                line.height,
                                TextAlign::Left,
                                None,
                                window,
                                cx,
                            )
                            .ok();
                    }
                    RowSegment::Image { image, size, x, .. } => {
                        let origin = point(left + *x, y + (line.height - size.height) / 2.);
                        window
                            .paint_image(
                                Bounds::new(origin, *size),
                                Corners::default(),
                                image.clone(),
                                0,
                                false,
                            )
                            .ok();
                    }
                }
            }
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
                gap_numbers: Vec::new(),
            },
        );
        self.view.update(cx, |view, _| {
            view.last_layout = Some(layout);
        });
    }
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
