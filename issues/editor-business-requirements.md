# Kioto Editor Abstraction Skeleton

This document captures the rough software-design skeleton for the Kioto GPUI editor. These snippets are code-shaped notes for reasoning about the abstraction before adapting it to concrete GPUI, Typst, and Rust crate APIs.

## Hexagonal architecture folder shape

Each architecture layer is a separate top-level folder under `src/`.

```text
crates/editor/src/
  lib.rs      # exports the public API
  api/        # public crate-facing API
  impl/       # implementation of the public API; delegates into app
  app/        # concrete editor model, modal state machine, render orchestration
  core/       # stateless primitive operations and general traits
  port/       # external capabilities expressed in editor/domain terms
  adapters/   # side-effecting implementations of ports
```

- `lib.rs` exports the API surface.
- `api` defines what callers can do with this crate.
- `impl` implements the API surface and delegates into `app`.
- `app` contains the concrete editor state, cursor/selection model, concrete modal structs, render orchestration, and concrete preview renderers.
- `core` contains stateless primitive editor operations and general traits, including the `Mode` trait.
- `port` contains traits for external capabilities such as persistence.
- `adapters` contains concrete side-effecting implementations, such as filesystem persistence.

## File organization

```text
crates/editor/src/
  lib.rs

  api/
    mod.rs                # public exports / API wrapper functions
    editor.rs             # public editor-facing API, e.g. create_editor(...)

  impl/
    mod.rs
    editor.rs             # implementation of api/editor.rs; delegates into app

  app/
    mod.rs
    editor.rs             # Editor state and Editor::create_editor(...)
    cursor.rs             # Cursor model
    selection.rs          # Selection range model
    mode.rs               # EditorMode enum plus concrete mode structs
    render.rs             # Editor::render(...) orchestration
    renderers/
      mod.rs
      typst.rs            # Typst renderer implementing PreviewRenderer
      markdown.rs         # Markdown renderer implementing PreviewRenderer

  core/
    mod.rs
    actions.rs            # CoreActions: stateless primitive edit operations
    editable_buffer.rs    # EditableBuffer trait / buffer-facing abstraction
    mode.rs               # Mode trait
    preview_renderer.rs   # PreviewRenderer trait

  port/
    mod.rs
    document_store.rs     # file reader/writer persistence port

  adapters/
    mod.rs
    filesystem_store.rs   # filesystem-backed DocumentStore implementation
```

`mod.rs` files should stay barrel-only.

## API skeleton

### `lib.rs`

```rust
pub use api::*;
```

### `api`

The API layer defines what callers can do with this crate. It should expose actions such as editor construction without owning the actual implementation logic.

```rust
pub fn create_editor(config: EditorConfig) -> Editor {
    impl::create_editor(config)
}
```

## Impl skeleton

The `impl` layer implements the public API and delegates to the `app` layer. It exists so the API surface can stay clean even if app construction/orchestration becomes more involved.

```rust
pub fn create_editor(config: EditorConfig) -> Editor {
    Editor::create_editor(config)
}
```

## Core skeleton

### Editable buffer

```rust
trait EditableBuffer {
    // Rough placeholder trait.
    // Exact bounds/API are intentionally TBD.
}
```

### Core actions

`CoreActions` is stateless. It does not own editor data, mode, cursors, GPUI state, or persistence. It only performs primitive edits against a mutable buffer-like object.

```rust
struct CoreActions;

impl CoreActions {
    fn insert_chars<B: EditableBuffer>(buffer: &mut B, range: Range, text: Text) { ... }
    fn replace_chars<B: EditableBuffer>(buffer: &mut B, range: Range, text: Text) { ... }
    fn delete_chars<B: EditableBuffer>(buffer: &mut B, range: Range) { ... }
}
```

For the first abstraction pass, this can target `Editor.screen_buffer`. If the design later splits canonical document storage from derived display storage, these methods should target the canonical editable buffer.

### Mode trait

The `Mode` trait belongs in `core`. Concrete modes live in `app` and implement this trait.

```rust
trait Mode {
    fn handle_input(&self, input: Input) -> EditorAction;
}
```

### Preview renderer trait

The renderer trait exists so the editor can ask for preview output without knowing Typst, Markdown, LaTeX, or other language-specific parsing/rendering details.

The renderer should return a language-neutral preview model. Different markup/rendering languages share common concepts such as headings, paragraphs, text spans, math blocks, code blocks, lists, and preview blocks. That common shape should influence `PreviewOutput` so `Editor::render()` can compose rendered preview output without caring which language produced it.

```rust
trait PreviewRenderer {
    fn render_preview<B: EditableBuffer>(&self, buffer: &B) -> PreviewOutput;
}
```

```rust
struct PreviewOutput {
    blocks: Vec<PreviewBlock>,
}

enum PreviewBlock {
    Heading { level: HeadingLevel, text: Text },
    Paragraph { spans: Vec<PreviewSpan> },
    CodeBlock { language: Option<Text>, text: Text },
    MathBlock { text: Text },
    List { items: Vec<PreviewBlock> },
    RenderedBlock { source_range: Range },
}

enum PreviewSpan {
    Text(Text),
    Emphasis(Text),
    Strong(Text),
    InlineCode(Text),
    InlineMath(Text),
}
```

This is still only a rough skeleton. The important design point is that Typst/Markdown/LaTeX-specific complexity is pushed into concrete renderers, while the editor receives a shared preview model.

## App skeleton

### Editor

```rust
struct Editor {
    screen_buffer: Vec<_>,
    cursors: Vec<Cursor>,
    selections: Vec<Selection>,
    mode: EditorMode,
}

impl Editor {
    pub fn create_editor(config: EditorConfig) -> Editor { ... }

    fn change_mode(&mut self, mode: EditorMode) { ... }

    fn handle_input(&mut self, input: Input) {
        let action = self.mode.handle_input(input);
        self.apply(action);
    }

    fn apply(&mut self, action: EditorAction) { ... }

    fn render(&self) -> RenderedEditor { ... }
}
```

### Cursor

```rust
struct Cursor {
    position: Position,
}

impl Cursor {
    fn new(position: Position) -> Cursor { ... }
    fn set(&mut self, position: Position) { ... }
}
```

### Selection

```rust
struct Selection {
    from_position: Position,
    to_position: Position,
}

impl Selection {
    fn new(from_position: Position, to_position: Position) -> Selection { ... }

    fn set(&mut self, from_position: Option<Position>, to_position: Option<Position>) { ... }
}
```

In `Selection::set(...)`, `None` for `from_position` means the start of the file, and `None` for `to_position` means the end of the file.

### Mode state machine

Modes are complex enough to deserve concrete mode structs. The app layer keeps an enum wrapper for the active editor mode, while each concrete mode implements the `core::Mode` trait.

```rust
enum EditorMode {
    Insert(InsertMode),
    Normal(NormalMode),
    Visual(VisualMode),
}

struct InsertMode;
struct NormalMode;
struct VisualMode;

impl Mode for InsertMode {
    fn handle_input(&self, input: Input) -> EditorAction { ... }
}

impl Mode for NormalMode {
    fn handle_input(&self, input: Input) -> EditorAction { ... }
}

impl Mode for VisualMode {
    fn handle_input(&self, input: Input) -> EditorAction { ... }
}

impl EditorMode {
    fn handle_input(&self, input: Input) -> EditorAction {
        match self {
            EditorMode::Insert(mode) => mode.handle_input(input),
            EditorMode::Normal(mode) => mode.handle_input(input),
            EditorMode::Visual(mode) => mode.handle_input(input),
        }
    }
}
```

### Basic modal keybind targets

Rough first-pass keybinds to support:

| Mode | Keybind | Intended action |
|---|---|---|
| Normal | `h` / `j` / `k` / `l` | move cursor left/down/up/right |
| Normal | `w` / `b` / `e` | word-forward / word-back / word-end movement |
| Normal | `0` / `$` | move to line start / line end |
| Normal | `g g` / `G` | move to document start / document end |
| Normal | `i` | enter insert mode before cursor |
| Normal | `a` | enter insert mode after cursor |
| Normal | `v` | enter visual mode |
| Normal | `x` | select current line |
| Normal | `d` | delete current selection/range |
| Normal | `y` | yank/copy current selection/range |
| Normal | `p` | paste after cursor/selection |
| Normal | `u` | undo |
| Normal | `/` | start search |
| Normal | `n` / `N` | next / previous search match |
| Insert | text input | insert committed text |
| Insert | `Esc` | return to normal mode |
| Visual | `h` / `j` / `k` / `l` | extend selection left/down/up/right |
| Visual | `w` / `b` / `e` | extend selection by word motion |
| Visual | `d` | delete selection and return to normal mode |
| Visual | `y` | yank/copy selection and return to normal mode |
| Visual | `Esc` | collapse/exit visual mode back to normal mode |

Counts and multi-key operator state should be modeled after the basic mode/key dispatch path is clear.

### Concrete preview renderers

```rust
struct TypstRenderer;

impl PreviewRenderer for TypstRenderer {
    fn render_preview<B: EditableBuffer>(&self, buffer: &B) -> PreviewOutput { ... }
}
```

```rust
struct MarkdownRenderer;

impl PreviewRenderer for MarkdownRenderer {
    fn render_preview<B: EditableBuffer>(&self, buffer: &B) -> PreviewOutput { ... }
}
```

`Editor::render()` should hide preview parsing/rendering complexity behind the renderer trait and return something GPUI can render.

## Port skeleton

### Document persistence port

```rust
trait DocumentStore {
    fn load(&self, location: DocumentLocation) -> Text;
    fn save(&self, location: DocumentLocation, text: Text);
}
```

## Adapters skeleton

### Filesystem persistence adapter

```rust
struct FilesystemStore;

impl DocumentStore for FilesystemStore {
    fn load(&self, location: DocumentLocation) -> Text { ... }
    fn save(&self, location: DocumentLocation, text: Text) { ... }
}
```
