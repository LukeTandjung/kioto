# Kioto Editor Abstraction Skeleton

This document captures the rough software-design skeleton for the Kioto GPUI editor. These snippets are code-shaped notes for reasoning about the abstraction before adapting it to concrete GPUI, Typst, and Rust crate APIs.

## Hexagonal architecture folder shape

Each architecture layer is a separate top-level folder under `src/`.

```text
crates/editor/src/
  lib.rs      # exports the public API
  api/        # public crate-facing API
  imp/        # implementation of the public API; delegates into app
  app/        # concrete editor model, use-case orchestration, document models
  core/       # pure, generalizable editor domain logic and general traits
  port/       # external capabilities expressed in editor/domain terms
  adapters/   # side-effecting implementations of ports
```

- `lib.rs` exports the API surface.
- `api` defines what callers can do with this crate.
- `imp` implements the API surface and delegates into `app`. It is also the composition root — see the Imp skeleton section for why the slight pass-through is deliberate. (Originally named `impl`, but `impl` is a reserved Rust keyword and cannot be a module name; `imp` is the conventional substitute.)
- `app` contains the concrete editor state and use-case orchestration: it composes `core` functions into application use cases, interprets `EditorAction`s whose effects require ports (paste, save), owns render orchestration, and hosts the concrete document models.
- `core` contains pure, generalizable editor domain logic: primitive edit operations, the cursor and selection models, the modal state machine, and the general traits (`EditableBuffer`, `PreviewRenderer`, `Mode`).
- `port` contains traits for external capabilities such as persistence and the system clipboard.
- `adapters` contains concrete side-effecting implementations, such as filesystem persistence.

The layering rule of thumb: pure, generalizable business logic that uses no `port`/`adapters` capability moves down into `core`. `app` is the use-case layer, where ports get wired in (constructed in `imp`) and `core` functions are composed. When something in `app` turns out to be pure and general, it migrates down.

## File organization

```text
crates/editor/src/
  lib.rs

  api/
    mod.rs                # public exports / API wrapper functions
    editor.rs             # public editor-facing API, e.g. create_editor(...)

  imp/
    mod.rs
    editor.rs             # implementation of api/editor.rs; composition root wiring adapters into ports, delegates into app

  app/
    mod.rs
    editor.rs             # Editor state and Editor::create_editor(...)
    render.rs             # Editor::render(...) orchestration
    documents/
      mod.rs
      typst.rs            # Typst document model: Source-backed EditableBuffer + PreviewRenderer
      markdown.rs         # Markdown document model: EditableBuffer + PreviewRenderer

  core/
    mod.rs
    actions.rs            # CoreActions: stateless primitive edit operations
    cursor.rs             # Cursor model
    selection.rs          # Selection range model
    mode.rs               # Mode trait, EditorMode enum, concrete mode structs
    editable_buffer.rs    # EditableBuffer trait / buffer-facing abstraction
    preview_renderer.rs   # PreviewRenderer trait

  port/
    mod.rs
    document_store.rs     # file reader/writer persistence port
    clipboard.rs          # system clipboard port

  adapters/
    mod.rs
    filesystem_store.rs   # filesystem-backed DocumentStore implementation
    gpui_clipboard.rs     # GPUI ClipboardItem-backed Clipboard implementation
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
    imp::create_editor(config)
}
```

## Imp skeleton

The `imp` layer implements the public API and delegates to the `app` layer. It is the composition root: construction here chooses concrete adapters (e.g. `FilesystemStore`) and injects them into the ports that `app` depends on. The slight pass-through cost is accepted deliberately — it keeps side effects out of `app`/`core`, so pure logic is unit-tested directly and integration tests can substitute fixtures for the adapters.

```rust
pub fn create_editor(config: EditorConfig) -> Editor {
    Editor::create_editor(config)
}
```

## Core skeleton

### Editable buffer

```rust
trait EditableBuffer {
    // Text reads and primitive edits, nothing else —
    // see the "Canonical buffer" section for the contract.
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

`CoreActions` targets the canonical editable buffer for the active language (for Typst, the `Source`-backed buffer described in the "Canonical buffer" section below) — never display state, per the canonical/derived split in Committed decisions.

### Cursor

Pure data with pure operations and no port access, so it lives in `core`, not `app`.

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

### Mode trait and modal state machine

The `Mode` trait *and* the concrete modes all live in `core`: `handle_input` maps an `Input` to an `EditorAction` — pure data, no port access — so the whole modal state machine is pure, generalizable logic. Actions whose *interpretation* requires side effects (paste reads the clipboard port, save writes through the document store) are returned as data and interpreted in `app`, which holds the ports.

Note that `handle_input` takes `&mut self`: multi-key sequences (`g g`, counts like `3 w`, pending operators like `d w`) mean modes carry transient state between keystrokes.

```rust
trait Mode {
    fn handle_input(&mut self, input: Input) -> EditorAction;
}
```

Modes are complex enough to deserve concrete mode structs. `core` keeps an enum wrapper for the active editor mode, while each concrete mode implements the `Mode` trait.

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
    fn handle_input(&mut self, input: Input) -> EditorAction { ... }
}

impl Mode for NormalMode {
    fn handle_input(&mut self, input: Input) -> EditorAction { ... }
}

impl Mode for VisualMode {
    fn handle_input(&mut self, input: Input) -> EditorAction { ... }
}

impl EditorMode {
    fn handle_input(&mut self, input: Input) -> EditorAction {
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

### Preview renderer trait

The renderer trait exists so the editor can ask for preview output without knowing Typst, Markdown, LaTeX, or other language-specific parsing/rendering details.

The renderer should return a language-neutral preview model. Different markup/rendering languages share common concepts such as headings, paragraphs, text spans, math blocks, code blocks, lists, and preview blocks. That common shape should influence `PreviewOutput` so `Editor::render()` can compose rendered preview output without caring which language produced it.

Rendering is a function of the document *plus the cursor positions*, not the text alone: in live preview, the block containing a cursor renders in raw-markup mode while every other block renders styled (see "Source ↔ display coordinate mapping" below).

`PreviewRenderer` is implemented by the same document-model type that implements `EditableBuffer` (see "Buffer/renderer pairing" below), so `render_preview` reads its own text and parse state rather than receiving a buffer parameter.

```rust
trait PreviewRenderer {
    fn render_preview(&self, cursor_positions: &[Position]) -> PreviewOutput;
}
```

```rust
struct PreviewOutput {
    blocks: Vec<PreviewBlock>,
}

// Every block carries its source range and a bidirectional source ↔ display
// offset map (see "Source ↔ display coordinate mapping" below). The enum shows
// only the per-variant payload.
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

This is still only a rough skeleton. The important design point is that Typst/Markdown/LaTeX-specific complexity is pushed into concrete document models, while the editor receives a shared preview model.

## App skeleton

### Editor

```rust
struct Editor<D: EditableBuffer + PreviewRenderer> {
    document: D, // canonical buffer + preview rendering
    cursors: Vec<Cursor>,
    selections: Vec<Selection>,
    mode: EditorMode,
}

impl<D: EditableBuffer + PreviewRenderer> Editor<D> {
    // How EditorConfig selects the document type is the open question
    // in "Buffer/renderer pairing" — enum vs dynamic dispatch.
    pub fn create_editor(config: EditorConfig) -> Editor<D> { ... }

    fn change_mode(&mut self, mode: EditorMode) { ... }

    fn handle_input(&mut self, input: Input) {
        let action = self.mode.handle_input(input);
        self.apply(action);
    }

    fn apply(&mut self, action: EditorAction) { ... }

    fn render(&self) -> RenderedEditor { ... }
}
```

### Concrete document models

One module per language owns both halves: the `EditableBuffer` implementation and the `PreviewRenderer` implementation, over shared parse state.

Document models stay in `app` even though parsing is pure. They fail the layering rule on the *generalizable* axis — they are special-purpose by design (Ousterhout principle 8: separate special-purpose from general-purpose code) — and the milestone-4 fragment compilation reintroduces side effects (font discovery, file and package access via `typst::World`).

```rust
struct TypstDocument { /* wraps typst_syntax::Source */ }

impl EditableBuffer for TypstDocument { ... }

impl PreviewRenderer for TypstDocument {
    fn render_preview(&self, cursor_positions: &[Position]) -> PreviewOutput { ... }
}
```

```rust
struct MarkdownDocument { /* text + Markdown parse state */ }

impl EditableBuffer for MarkdownDocument { ... }

impl PreviewRenderer for MarkdownDocument {
    fn render_preview(&self, cursor_positions: &[Position]) -> PreviewOutput { ... }
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

### Clipboard port

Yank/paste in the modes stays pure by returning actions as data; actually reading/writing the system clipboard is a side effect, so it is a port. The adapter is backed by GPUI's `ClipboardItem`.

```rust
trait Clipboard {
    fn read(&self) -> Text;
    fn write(&self, text: Text);
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

### GPUI clipboard adapter

```rust
struct GpuiClipboard;

impl Clipboard for GpuiClipboard {
    fn read(&self) -> Text { ... }   // via gpui ClipboardItem
    fn write(&self, text: Text) { ... }
}
```

## Committed decisions

- **Fidelity model: live preview** (Typora/Obsidian style), not paginated WYSIWYG. The editor displays the *source*, styled in place: heading lines render large, `*strong*` renders bold with its markers hidden until the cursor enters, math and code blocks are compiled as fragments and shown rendered. This is what the `typst-syntax` AST supports directly — it stops before evaluation and layout, so show rules, scripting (`#let`, loops), figure numbering, and page breaks do not exist at the AST level. True paginated WYSIWYG (rendering compiled `Frame`s and reverse-mapping clicks to source via `typst-ide`'s `jump_from_click`) is out of scope.
- **Rebuild, not evolve.** The existing flat `crates/editor` implementation (buffer/selection/element/input/position_map) is treated as a spike. The crate is rebuilt to the hexagonal shape in this document, porting spike pieces over as they prove useful.
- **Canonical buffer is source text.** Display state is derived from the canonical buffer plus cursor state; it is never stored or edited directly.
- **The editor model is language-neutral.** The editor targets Typst first, with Markdown and LaTeX to follow. Concrete language types (`TypstDocument`, syntax trees) are quarantined in `app/documents/` and selected at construction time — `Editor`, the modes, and render orchestration depend only on the `core` abstractions (`EditableBuffer`, `PreviewRenderer`, `Position`/`Range`). Anything else is information leakage of the document language into the editor core.

## Canonical buffer

`EditableBuffer` is language-neutral: it exposes text reads and primitive edits, nothing else. Each supported language provides its own implementation, and `Editor` holds the abstraction — never a concrete language type.

### Typst implementation

`typst_syntax::Source` owns the text, the parsed syntax tree, and incremental reparsing: `Source::edit(range, text)` applies a replacement and returns the invalidated span. `TypstDocument` wraps a `Source`, so every edit keeps the AST in sync for free — no separate parse step, no whole-document re-parse per keystroke.

Related API notes:

- The syntax tree is lossless: every `SyntaxNode` preserves all trivia, and its `Span` maps back to byte offsets in the source. Cursor ↔ node mapping is therefore cheap.
- The `typst_syntax::ast` module provides typed views over raw `SyntaxNode`s; the renderer walks these rather than matching on raw node kinds.
- `Position`/`Range` in `core` are byte offsets; line/column values are derived. Cursor movement must be grapheme-aware (`unicode-segmentation`).

### Buffer/renderer pairing (decision)

The Typst renderer needs the syntax tree that the Typst buffer maintains. If buffer and renderer were separate modules connected only through the language-neutral `EditableBuffer` interface, the tree could not cross that boundary: the renderer would either re-parse from text (discarding the incremental-reparse benefit) or reach into buffer internals (back-door information leakage). The repair is Ousterhout's "merge the modules": one document-model type per language (`app/documents/typst.rs`, `app/documents/markdown.rs`) owns both the parse state and the preview rendering, implementing both `EditableBuffer` and `PreviewRenderer`. Typst knowledge lives in exactly one module; Markdown and LaTeX follow the same shape with their own parsers. This is why `render_preview` takes no buffer parameter and why `Editor` is generic over `D: EditableBuffer + PreviewRenderer`.

Open question for milestone 1: selecting the document type at runtime (e.g. by file extension) needs either an enum over document models or dynamic dispatch — decide when the first second language lands, not before.

## Source ↔ display coordinate mapping

This is the hard problem of live preview, and it must be designed for from day one: **source coordinates ≠ display coordinates.** When `== Heading` renders without its `==` markers, or `*bold*` hides its stars, the cursor's byte offset in the source no longer matches its visual column — and the relationship changes *dynamically*, because the signature live-preview behavior is "reveal the markup when the cursor enters the block, hide it when it leaves."

Consequences:

- **Block, not line, is the render unit.** Typst constructs are block-shaped: a math block, code fence, or soft-wrapped paragraph spans multiple visual rows. A hard `\n`-line = screen-row mapping cannot hold. Each `PreviewBlock` carries its source `Range` and owns a bidirectional offset map (source offset ↔ display position) for its contents. The spike's `position_map.rs` is the seed of this, generalized per block.
- **Cursor position is an input to rendering.** The renderer must know which block each cursor is in, so that block renders in raw-markup mode while everything else renders styled. This is why `PreviewRenderer::render_preview` takes `cursor_positions`.
- All motions, selections, and mouse hit-testing on rendered text go through this mapping. Getting it right early is what makes those features fall out naturally; prior art is Zed's `DisplayMap` (layered coordinate transforms), worth reading even if our version is much simpler.

## Compiled fragments (`RenderedBlock`)

For content that cannot be styled as text — math blocks first, embedded code output later — the fragment is compiled with the `typst` crate and rasterized via `typst-render` into an image GPUI can draw. This is the `PreviewBlock::RenderedBlock { source_range }` variant.

- The real work is implementing the `typst::World` trait (font discovery, file access, package loading). This is the biggest external-integration lift in the project and is quarantined to its own milestone.
- Rendered fragments are cached, keyed by a hash of the block's source text, so unrelated edits never recompile a fragment.

## Dependencies

Grouped by role, with the milestone that first needs each.

### Core editing (milestones 1–2)

| Crate | Why |
|---|---|
| `typst-syntax` | `Source`, incremental reparse, spans, `ast` typed views. Usable standalone — the full `typst` crate is not needed until milestone 4. |
| `unicode-segmentation` | Grapheme-aware cursor motion (already a dep). |
| `thiserror` | Typed error enums in the library layers (`core`, `port`, `adapters`). |
| `bon` | Builders for `EditorConfig` and `imp`-layer wiring (light DI). |
| `gpui` | Already in the workspace. Clipboard is built in (`ClipboardItem`) and backs the `Clipboard` port's adapter, so yank/paste needs nothing extra. |

Error-handling split: `thiserror` and `anyhow` are not both "typed" — `thiserror` gives callers a typed enum they can match on; `anyhow` erases the type into a context-carrying trait object. The split that fits the hexagonal shape: **`thiserror` everywhere inside the `editor` crate** (ports return typed errors so `app` can handle them), **`anyhow` only in the `kioto` binary** where errors terminate in a message. Per Ousterhout Ch. 7, prefer defining buffer "errors" out of existence (clamp out-of-range positions, treat empty selections as normal) before adding an error variant.

### Typst compilation for fragments (milestone 4)

| Crate | Why |
|---|---|
| `typst` | The compiler — `typst::compile`, the `World` trait. |
| `typst-render` | Rasterizes compiled `Frame`s to a `tiny_skia::Pixmap` for GPUI. |
| `typst-kit` | Ready-made `World` building blocks — font searching, package download. What `typst-cli` itself uses; cuts the `World` milestone down substantially. |
| `typst-assets` | Embedded default fonts (math fonts especially), via typst-kit's `embed-fonts` feature, so math renders without depending on system fonts. |
| `comemo` | Typst's memoization layer; likely needed as a direct dep for the `Tracked`/caching plumbing around the `World` impl. |
| `image` | Pixmap → GPUI image conversion goes through the `image` crate's types. |

### Later, on purpose (do not add yet)

- `pulldown-cmark` — `MarkdownDocument`, when a second language actually lands.
- `typst-ide` — autocomplete, hover, click-to-source; not needed for live preview.
- `ropey` — skip: `typst_syntax::Source` owns its text as a `String`, so a rope cannot slot under it. Revisit only if profiling on large documents demands it (and that becomes a Typst-upstream conversation).
- `notify` — file watching for external edits; a new `port` when wanted.
- `proptest` — optional at milestone 1: property tests over `CoreActions` (random edit sequences preserve UTF-8 validity, offsets stay on grapheme boundaries, undo round-trips) catch buffer bugs example-based tests miss.

### Testing

Plain `#[test]` covers `core`/`app` (which the pure-logic split exists for); GPUI ships `#[gpui::test]` + `TestAppContext` for milestone-2 input dispatch (see `docs/gpui-testing.md`).

### Quarantine rule

Only `app/documents/typst.rs` and the milestone-4 fragment module may depend on `typst-*` crates — see "The editor model is language-neutral" under Committed decisions.

## Milestones

Each milestone is a self-contained chunk that compiles, runs, and exercises a distinct slice of the design.

1. **Core + buffer.** New crate structure (`api`/`imp`/`app`/`core`/`port`/`adapters`). `Position`/`Range` in byte offsets, the `EditableBuffer` trait, `CoreActions` (insert/replace/delete), and `TypstDocument`'s buffer half (the `Source`-backed `EditableBuffer`). Pure logic, fully unit-testable, no GPUI.
2. **Modal editing on plain text.** `Mode` trait, `Normal`/`Insert`/`Visual` structs, the keybind table above, and a GPUI element rendering the buffer as unstyled monospace text with a cursor. Port what proves useful from the spike's `element.rs` and `input.rs`. Checkpoint: a working modal text editor.
3. **Typst preview, text-only styling.** `TypstDocument` gains its `PreviewRenderer` half: walk the AST into `PreviewBlock`s — headings sized, emphasis/strong styled, list markers, inline code. Implement the per-block offset map and the "cursor inside → show raw markup" rule. No compilation and no `typst::World` yet — everything here styles text we already have.
4. **Compiled fragments.** Implement `typst::World`; compile and rasterize math blocks via `typst-render`; fragment cache keyed by source hash.
5. **Persistence and the rest of the keybind table.** `DocumentStore` port + filesystem adapter, yank/paste (`Clipboard` port + GPUI-backed adapter), undo (an edit log of inverse operations — pure and generalizable, so it lives in `core`, driven from `app`), search, then counts/operators.
