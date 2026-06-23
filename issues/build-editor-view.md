# Build Kioto Editor View

## Problem

Kioto now has a dedicated `editor` crate, but it does not yet have an actual editor view. We need a GPUI-native editor architecture that starts small, remains testable, and can grow toward code-editor features without copying Zed's full application/editor stack.

This issue is the living design document for that work. It should accumulate lessons from multiple editor codebases before implementation. The researched codebases so far are Zed, the old `polachok/helix-gpui` experiment, Zed's Helix mode, and `longbridge/gpui-component`'s editor-like input component.

## Current source-research status

| Codebase | Status | Notes |
|---|---:|---|
| Zed core editor | Added below | GPUI-native editor architecture and input/rendering model. |
| `polachok/helix-gpui` | Added below | Old GPUI + Helix experiment; useful design, stale API. |
| Zed Helix mode | Added below | How Zed layers Helix-style modal bindings over its editor. |
| `longbridge/gpui-component` editor/input | Added below | Practical GPUI component implementation of multiline/code-editor behavior. |
| Additional editor codebases | TODO | Add separate sections before locking the final architecture. |

## Zed source snapshot

Research is based on the Zed source already pinned by this workspace's `gpui` dependency:

- Repository: `zed-industries/zed`
- Commit: `f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b`

Primary files inspected:

- [`crates/editor/src/editor.rs`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/editor.rs)
- [`crates/editor/src/element.rs`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/element.rs)
- [`crates/editor/src/display_map.rs`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/display_map.rs)
- [`crates/editor/src/input.rs`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/input.rs)
- [`crates/editor/src/selection.rs`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/selection.rs)
- [`crates/editor/src/selections_collection.rs`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/selections_collection.rs)
- [`crates/multi_buffer/src/multi_buffer.rs`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/multi_buffer/src/multi_buffer.rs)
- [`crates/gpui/src/input.rs`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/gpui/src/input.rs)

## Zed learnings to carry forward

### 1. Split editor state from editor rendering

Zed's `Editor` owns the application/editor state, while `EditorElement` owns custom GPUI layout and painting. `Editor::render` is intentionally tiny: it creates an `EditorElement` from the current entity and style.

References:

- [`Editor` owns state](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/editor.rs#L880-L1123)
- [`Render for Editor`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/editor.rs#L11510-L11513)
- [`EditorElement`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/element.rs#L198-L219)

Kioto decision:

- Keep `Editor` as the GPUI entity/state owner.
- Keep custom measurement, hit testing, line layout, and painting in an `EditorElement`-style custom element.
- Do not let the rendering element become the owner of text, selections, undo, or editor behavior.

### 2. Keep text storage below the view layer

Zed's editor edits an `Entity<MultiBuffer>`, not a string stored in the element. `MultiBuffer` can represent one or more underlying buffers in one editor surface, but a normal file is represented by wrapping a single buffer as a singleton.

References:

- [`MultiBuffer` purpose and fields](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/multi_buffer/src/multi_buffer.rs#L70-L92)
- [`MultiBuffer::singleton`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/multi_buffer/src/multi_buffer.rs#L1230-L1248)

Kioto decision:

- Start with a simpler single-buffer model.
- Keep the buffer as an independent runtime/entity that can later be replaced by a rope-backed or multi-buffer model.
- Do not bake text storage directly into visual layout code.

### 3. Add a display projection layer before adding advanced features

Zed's `DisplayMap` turns buffer coordinates into display coordinates and owns transformations such as inlays, folds, tab expansion, soft wraps, custom blocks, and highlights.

References:

- [`display_map` module docs](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/display_map.rs#L1-L16)
- [`DisplayMap` fields](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/display_map.rs#L208-L241)
- [`DisplayMap::new`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/display_map.rs#L364-L412)

Kioto decision:

- Introduce a minimal display-map/display-snapshot layer early, even if it initially only maps buffer lines directly to display lines.
- Add soft wrap, folds, inlays, diagnostics, and virtual blocks as later transformations.
- Keep coordinate conversion APIs explicit: buffer offset/point ↔ display point ↔ pixels.

### 4. Render from immutable snapshots

Zed snapshots editor/display state before laying out and painting. This lets prepaint and paint operate over a consistent view of the buffer and display transforms.

References:

- [`Editor::snapshot`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/editor.rs#L2828-L2869)
- [`DisplayMap::snapshot`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/display_map.rs#L603-L665)

Kioto decision:

- Define an `EditorSnapshot` and a `DisplaySnapshot` before building complex rendering.
- Use snapshots for layout, hit testing, and painting rather than reading mutable editor state throughout paint.

### 5. Use GPUI's platform text input bridge, not raw printable key handlers

Zed implements `EntityInputHandler` on `Editor`, installs `ElementInputHandler` during paint, and lets the platform deliver text input/IME composition through UTF-16 ranges.

References:

- [`EntityInputHandler` trait](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/gpui/src/input.rs#L4-L78)
- [`ElementInputHandler`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/gpui/src/input.rs#L80-L99)
- [`window.handle_input` in editor paint](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/element.rs#L11324-L11335)
- [`EntityInputHandler for Editor`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/input.rs#L2706-L3045)

Kioto decision:

- Implement text insertion and IME through `EntityInputHandler`.
- Install `ElementInputHandler` only during the custom element paint phase.
- Treat UTF-16 offset conversion as first-class editor infrastructure.
- Use raw key actions for commands, not printable text insertion.

### 6. Keep selection behavior centralized

Zed's selection collection tracks disjoint selections plus pending selections. Selection mutation flows through `change_selections`, which can defer side effects and then apply focus, scroll, history, completion, and notification behavior once.

References:

- [`SelectionsCollection`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/selections_collection.rs#L19-L57)
- [`Editor::change_selections`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/selection.rs#L50-L88)
- [`selections_did_change`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/selection.rs#L1454-L1508)

Kioto decision:

- Start with a single cursor/selection if needed, but still isolate selection mutation behind editor commands.
- Do not let mouse handlers, key actions, and input handlers each update selection side effects independently.
- Add multiple selections later without changing the rendering/input architecture.

### 7. Apply edits transactionally, then derive new selections

Zed computes edits for all selections, applies them to the buffer, snapshots the display map, then resolves new selections from anchors after edits.

References:

- [`Editor::handle_input` start](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/input.rs#L66-L81)
- [`handle_input` edit application and selection update](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/input.rs#L409-L521)
- [`Editor::transact`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/editor.rs#L8022-L8033)

Kioto decision:

- Introduce an edit command path before adding many editing commands.
- Keep undo/redo and selection restoration in the same conceptual area as transactions.
- Prefer anchor-like stable positions once edits become more complex.

### 8. Cache pixel/coordinate mapping from prepaint

Zed builds a `PositionMap` during layout. The input handler later uses it for hit testing and character-index lookup.

References:

- [`PositionMap` fields](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/element.rs#L12121-L12139)
- [`PositionMap::point_for_position`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/element.rs#L12187-L12225)
- [`character_index_for_point`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/input.rs#L3030-L3045)

Kioto decision:

- Store a layout/position map after prepaint.
- Use it for mouse hit testing, text selection, cursor placement, IME candidate bounds, and future hover features.

### 9. Virtualize by visible display rows

Zed computes the visible display-row range from scroll position and content masks, then only lays out visible row info and lines.

References:

- [`visible row calculation`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/element.rs#L10035-L10060)
- [`layout_lines` for visible rows](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/element.rs#L10430-L10439)

Kioto decision:

- Do not build a first editor that shapes every line on every frame.
- Even a minimal editor should compute a visible row range and shape only visible lines.

### 10. Paint the editor as ordered layers

Zed paints background, guides, gutter, text, blocks, overlays, minimap, scrollbars, and popovers in a deliberate order.

Reference:

- [`EditorElement::paint`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/element.rs#L11373-L11414)

Kioto decision:

- Define a simple paint order up front:
  1. background/current line/selection backgrounds,
  2. gutter,
  3. text,
  4. cursor,
  5. overlays/popovers.
- Add diagnostics, inlays, and other blocks as explicit layers later.

### 11. Use GPUI actions and key contexts for editor commands

Zed registers many editor actions during paint and dispatches only during the bubble phase. This keeps command behavior in editor methods and lets keymaps target actions.

References:

- [`register_actions`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/element.rs#L229-L260)
- [`register_action` helper](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/element.rs#L12666-L12680)
- [`actions.rs` examples](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/actions.rs#L1-L120)

Kioto decision:

- Define editor actions for movement, deletion, newline, select-all, copy/cut/paste, and undo/redo.
- Bind keys through an `init(cx)` entry point instead of ad hoc raw key handlers.

### 12. Keep integrations optional and outside the core editing path

Zed's `Editor` has many integrations: LSP, completions, diagnostics, git blame, minimap, collaboration, runnables, code lens, and more. These are valuable, but they make the full editor very large.

References:

- [`Editor` integration-heavy fields](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/editor.rs#L913-L1123)
- [`new_internal` project subscriptions](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/editor/src/editor.rs#L1850-L2036)

Kioto decision:

- Do not start with LSP, completions, git, diagnostics, minimap, collaboration, or code actions.
- Design explicit extension points only after the core editor can edit, select, scroll, and render text reliably.

## helix-gpui source snapshot

Research is based on the archived/stale `polachok/helix-gpui` repository:

- Repository: `polachok/helix-gpui`
- Commit: `0d73f6cd2fe1e2f8a04b863dff77228adbb06046`

Primary files inspected:

- [`Cargo.toml`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/Cargo.toml)
- [`src/application.rs`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/application.rs)
- [`src/document.rs`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/document.rs)
- [`src/workspace.rs`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/workspace.rs)
- [`src/utils.rs`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/utils.rs)
- [`src/overlay.rs`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/overlay.rs)
- [`src/picker.rs`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/picker.rs)
- [`src/prompt.rs`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/prompt.rs)
- [`src/statusline.rs`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/statusline.rs)

## helix-gpui learnings to carry forward

This codebase is not a target to port directly. It uses an old GPUI API, depends on a pinned Helix fork, and carries terminal-UI copy/paste code. Its value is architectural: it shows a small GPUI shell around Helix's editor model and keymap semantics.

### 1. Treat Helix's editing model as a separate core, not as GPUI widget state

`helix-gpui` keeps an `Application` model with Helix's `Editor`, `Compositor`, `EditorView`, jobs, and LSP progress. GPUI views send input/update events into that model rather than owning editing behavior directly.

References:

- [`Application` fields](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/application.rs#L25-L31)
- [`InputEvent`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/application.rs#L34-L44)
- [`handle_input_event`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/application.rs#L86-L120)
- [`init_editor`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/application.rs#L712-L779)

Kioto decision:

- Keep the editor command/keybinding engine separate from GPUI rendering.
- Allow a future Helix-like modal layer to wrap or drive the core editor without becoming the core editor.
- Do not couple modal command state to `EditorElement` paint/layout state.

### 2. A cell-grid editor is a good early mental model for a Helix-like GUI

`DocumentElement::prepaint` computes rows/columns from font metrics, resizes the Helix editor to a terminal-like `Rect`, and `paint` renders visible text using the view's anchor, visible row count, and shaped GPUI text.

References:

- [`DocumentElement` fields](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/document.rs#L217-L225)
- [`Element for DocumentElement`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/document.rs#L482-L555)
- [`prepaint` cell metrics and `editor.resize`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/document.rs#L503-L548)
- [`paint` visible text extraction and shaping](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/document.rs#L554-L660)

Kioto decision:

- Start the editor with a monospaced grid-compatible layout model.
- Still use GPUI text shaping/painting, not a terminal buffer, for the actual editor body.
- Keep proportional-font/general text layout as a later concern.

### 3. Highlighting can be merged from independent sources

`helix-gpui` reuses Helix syntax highlights, selection/focused-view highlights, and diagnostic highlights, then merges them into GPUI `TextRun`s through a local `StyleIter`.

References:

- [`overlay_highlights`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/document.rs#L285-L337)
- [`highlight`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/document.rs#L339-L433)
- [`StyleIter`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/document.rs#L916-L958)

Kioto decision:

- Model syntax, selection, diagnostics, search matches, and modal overlays as separate highlight sources.
- Merge highlight sources into line/text runs at layout time.
- Avoid one monolithic `LineStyle` owner that all features mutate directly.

### 4. Render gutters as their own layer derived from editor/view state

The document element asks Helix's view for gutter types, computes gutter width, then renders line numbers/diagnostic markers separately from document text.

References:

- [`paint` gutter width and cursor/text setup](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/document.rs#L589-L728)
- [`Gutter::init_gutter`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/document.rs#L740-L806)
- [`GutterRenderer`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/document.rs#L830-L857)

Kioto decision:

- Keep gutter rendering separate from text rendering.
- Make gutter width/layout a derived fact from the visible rows and enabled gutter decorations.
- Do not start with Zed's full gutter feature set; start with line numbers and current-line indication.

### 5. Keymaps and modal commands should not depend on printable text insertion

The workspace translates GPUI `Keystroke`s into Helix `KeyEvent`s and routes them through Helix's keymap/compositor. This is close to Helix, but it is too raw for Kioto because it bypasses GPUI actions/key contexts and does not use platform text input/IME for insert mode.

References:

- [`Workspace::handle_key`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/workspace.rs#L155-L162)
- [`utils::translate_key`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/utils.rs#L25-L62)
- [`handle_key_result` sketch](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/utils.rs#L68-L120)

Kioto decision:

- Use GPUI platform input for text insertion and IME.
- Use GPUI actions/key contexts or a dedicated modal key dispatcher for command-mode keys.
- If we add Helix-like keymaps, they should dispatch editor commands, not mutate text directly.

### 6. Workspace split layout can mirror editor tree state

`Workspace` builds GPUI views for Helix view IDs, maps Helix's split tree into flex rows/columns, focuses the active document view, and removes stale document views when the editor tree changes.

References:

- [`Workspace` document map](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/workspace.rs#L15-L24)
- [`make_views`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/workspace.rs#L164-L216)
- [`render_tree`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/workspace.rs#L138-L152)
- [`Workspace::render`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/workspace.rs#L228-L352)

Kioto decision:

- Keep editor view implementation independent from workspace/pane layout.
- Let a later workspace layer own splits, documents, overlays, and focus orchestration.
- Do not make the editor crate responsible for pane trees.

### 7. Terminal UI buffers can be a bridge for secondary UI, not the editor body

Prompts and pickers are rendered through Helix terminal components into a TUI buffer, then converted into GPUI styled text. This is pragmatic for reuse but should not become the long-term architecture for core UI.

References:

- [`Prompt::make`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/prompt.rs#L6-L25)
- [`Picker::make`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/picker.rs#L6-L30)
- [`TextWithStyle::from_buffer`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/utils.rs#L123-L190)
- [`OverlayView`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/overlay.rs#L7-L68)

Kioto decision:

- Prefer native GPUI components for prompts, pickers, diagnostics, and status line.
- It is acceptable to use styled-text bridges for early experiments, but not for core editor rendering.

### 8. Avoid polling-style async integration

`helix-gpui` creates a `Crank` model that wakes every 50ms to step Helix jobs/events. This worked as an experiment, but Kioto should not build editor responsiveness around polling.

References:

- [`Crank` setup](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/main.rs#L184-L216)
- [`Application::step`](https://github.com/polachok/helix-gpui/blob/0d73f6cd2fe1e2f8a04b863dff77228adbb06046/src/application.rs#L201-L260)

Kioto decision:

- Use GPUI tasks/channels/subscriptions for async editor services.
- Do not add a periodic redraw crank for normal editing.

## Zed Helix mode source snapshot

Research is based on the same Zed commit as the core editor research:

- Repository: `zed-industries/zed`
- Commit: `f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b`

Primary files inspected:

- [`crates/vim/src/vim.rs`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/vim.rs)
- [`crates/vim/src/helix.rs`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/helix.rs)
- [`crates/vim/src/state.rs`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/state.rs)
- [`crates/vim/src/insert.rs`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/insert.rs)
- [`crates/vim_mode_setting/src/vim_mode_setting.rs`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim_mode_setting/src/vim_mode_setting.rs)
- [`assets/keymaps/vim.json`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/assets/keymaps/vim.json)

## Zed Helix mode learnings to carry forward

### 1. Helix mode is a modal editing add-on, not part of the core editor view

Zed keeps Vim/Helix enablement in a small setting crate so other crates can query whether modal behavior is enabled without depending on the whole Vim crate. The Vim crate then activates/deactivates an addon on `Editor` entities.

References:

- [`VimModeSetting` / `HelixModeSetting`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim_mode_setting/src/vim_mode_setting.rs#L1-L40)
- [`ToggleVimMode` / `ToggleHelixMode`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/vim.rs#L278-L310)
- [`Vim::register` activation flow](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/vim.rs#L612-L669)

Kioto decision:

- Implement core editing first, then add a separate modal/Helix layer.
- Do not bake Helix state into the base editor data model.
- The modal layer should drive editor commands and settings, not own the buffer/rendering pipeline.

### 2. Helix mode can share infrastructure with Vim while adding explicit modes

Zed's modal state includes `HelixNormal` and `HelixSelect` alongside Vim modes. `Mode::is_visual`, `Mode::is_helix`, and `Mode::is_normal` let shared code distinguish behavior without duplicating the whole modal engine.

References:

- [`Mode` variants and helpers](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/state.rs#L48-L85)
- [`Operator` Helix variants](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/state.rs#L145-L180)
- [`Vim::new` Helix initial mode mapping](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/vim.rs#L559-L578)

Kioto decision:

- Represent modal mode explicitly, including Helix normal/select concepts.
- Keep shared command plumbing separate from mode-specific selection semantics.
- Do not model Helix as only a keymap preset; its selection semantics differ from ordinary non-modal editing.

### 3. Keybindings are data mapping into actions, while semantics live in Rust commands

Zed's `assets/keymaps/vim.json` maps Helix contexts and keys to actions. `helix.rs` defines Helix-specific actions and registers their handlers against the editor/Vim addon.

References:

- [`helix_normal` / `helix_select` keymap contexts](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/assets/keymaps/vim.json#L421-L511)
- [`helix_m`, `helix_next`, `helix_previous` operator keymaps](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/assets/keymaps/vim.json#L654-L690)
- [`Helix` actions](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/helix.rs#L31-L68)
- [`helix::register`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/helix.rs#L70-L118)

Kioto decision:

- Store default Helix bindings as declarative keymap data when possible.
- Implement semantic commands as Rust actions/methods.
- Use key context state (`mode`, `operator`, `count`) to resolve multi-key bindings.

### 4. Helix motions are selection-oriented

Zed's Helix motions do not simply move a caret. In Helix normal/select modes, movement often creates or extends selections. Word and subword movement need special handling, and end-of-line means the cursor lands on the last character rather than after it.

References:

- [`helix_normal_motion` / `helix_select_motion`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/helix.rs#L121-L240)
- [`helix_new_selections`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/helix.rs#L240-L266)
- [`helix_move_cursor`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/helix.rs#L441-L565)

Kioto decision:

- Build the core selection model before attempting Helix mode.
- Make selection commands expressive enough for inclusive selections and cursor-as-selection behavior.
- Avoid implementing Helix by remapping keys to simple caret movement.

### 5. Insert/normal transitions must adapt editor input settings

Zed syncs modal state back to editor settings: cursor shape, clipping, input enabled, expected character input, autoindent, cursor offset on selection, line mode, and edit prediction visibility. In Helix mode, leaving insert switches to `HelixNormal` without Vim's normal-mode cursor-left adjustment.

References:

- [`insert::normal_before` Helix branch](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/insert.rs#L37-L60)
- [`switch_mode` Helix remapping](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/vim.rs#L1200-L1240)
- [`state_for_editor_settings` / `sync_vim_settings_to_editor`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/vim.rs#L2221-L2255)

Kioto decision:

- The modal layer should expose a compact `EditorModeSettings` payload to the core editor.
- The core editor should provide commands for enabling/disabling text input, setting cursor shape, and configuring selection rendering.

### 6. Helix-specific commands often compose existing editor primitives

Many Zed Helix commands are thin semantic wrappers: yank expands an empty selection to the current character, insert collapses to selection start, append collapses to selection end, line select selects full lines, substitute deletes the current selection and enters insert mode, and select-next/select-previous integrate with search.

References:

- [`helix_yank`, `helix_insert`, `helix_append`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/helix.rs#L566-L729)
- [`helix_select_lines`, `helix_substitute`, `helix_select_next`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/helix.rs#L802-L929)

Kioto decision:

- Implement normal editor commands first: select ranges, edit ranges, copy/yank, paste, line selection, search result selection.
- Build Helix commands as composition over those primitives.

### 7. Some Helix UI features are editor overlays over visible ranges

Zed's Helix jump-to-word feature collects candidates from the visible editor range, creates labels, and renders them through editor navigation overlays. The command keeps labels in the active operator state, then consumes subsequent character input.

References:

- [`helix_jump_to_word` / `start_helix_jump`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/helix.rs#L982-L1023)
- [`collect_helix_jump_data`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/helix.rs#L1037-L1190)
- [`HelixJumpUiData`](https://github.com/zed-industries/zed/blob/f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b/crates/vim/src/helix.rs#L1605-L1711)

Kioto decision:

- Design editor overlays as data attached to ranges, not ad hoc drawing from the modal layer.
- Ensure the editor can expose visible ranges and map range overlays into paint/layout data.

## gpui-component source snapshot

Research is based on the local `longbridge/gpui-component` checkout in `/home/luke/Projects/gpui-component`:

- Repository: `longbridge/gpui-component`
- Commit: `c36b0c6ae6d14c33473f6610a27c3abc584afdf9`

Primary files inspected:

- [`docs/docs/components/editor.md`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/docs/docs/components/editor.md)
- [`crates/story/examples/editor.rs`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/story/examples/editor.rs)
- [`crates/ui/src/input/mod.rs`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/mod.rs)
- [`crates/ui/src/input/input.rs`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/input.rs)
- [`crates/ui/src/input/state.rs`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/state.rs)
- [`crates/ui/src/input/mode.rs`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/mode.rs)
- [`crates/ui/src/input/element.rs`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/element.rs)
- [`crates/ui/src/input/display_map/mod.rs`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/display_map/mod.rs)
- [`crates/ui/src/input/display_map/display_map.rs`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/display_map/display_map.rs)
- [`crates/ui/src/input/display_map/wrap_map.rs`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/display_map/wrap_map.rs)
- [`crates/ui/src/input/display_map/text_wrapper.rs`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/display_map/text_wrapper.rs)
- [`crates/ui/src/input/movement.rs`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/movement.rs)
- [`crates/ui/src/input/selection.rs`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/selection.rs)
- [`crates/ui/src/input/indent.rs`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/indent.rs)
- [`crates/ui/src/input/search.rs`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/search.rs)

## gpui-component learnings to carry forward

This codebase calls the docs page `Editor`, but the implementation is not a standalone editor crate/component. It is an advanced mode of the general `Input` component: `InputState` owns the editing state, `Input` renders the styled container/wrapper, and `TextElement` is the custom GPUI element for multiline layout and painting.

### 1. A practical GPUI editor can start as `state + wrapper + custom element`

`gpui-component` splits responsibilities into three layers: `InputState` is the entity/state owner, `Input` is the styled component wrapper with prefix/suffix/clear/loading/context-menu behavior, and `TextElement` does custom layout, prepaint, paint, hit testing support, and input-handler installation.

References:

- [`InputState`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/state.rs#L340-L443)
- [`Input`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/input.rs#L35-L57)
- [`TextElement`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/element.rs#L282-L294)
- [`RenderOnce for Input`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/input.rs#L244-L443)
- [`Element for TextElement`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/element.rs#L1496-L2259)

Kioto decision:

- Keep the same high-level shape: an `Editor` entity/state owner plus an `EditorElement` custom element.
- Avoid putting full editor behavior into a generic input wrapper; Kioto already has a dedicated `editor` crate, so it should keep editor concepts explicit.
- Consider a thin public wrapper component later, but keep it separate from the core editor runtime.

### 2. Rope-backed text storage is already useful before full editor complexity

The docs explicitly call out `ropey` for large-file support, and `InputState` stores text as `Rope`. All edits go through byte ranges internally while platform input APIs use UTF-16 ranges.

References:

- [`editor.md` CodeEditor docs](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/docs/docs/components/editor.md#L56-L73)
- [`ropey` and `tree-sitter` docs note](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/docs/docs/components/editor.md#L59-L63)
- [`InputState::text: Rope`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/state.rs#L344-L345)
- [`range_to_utf16` / `range_from_utf16`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/state.rs#L2175-L2183)

Kioto decision:

- Prefer starting with `ropey::Rope` rather than a plain `String` if dependency weight is acceptable.
- Make offset units explicit in API names and tests: UTF-8 byte offsets internally, UTF-16 offsets at GPUI platform-input boundaries.
- Add rope/offset conversion tests early, before editing commands become numerous.

### 3. `InputMode` is useful as configuration, but too much editor behavior should not hide inside an input enum

`InputMode` supports `PlainText`, `AutoGrow`, and `CodeEditor`. The `CodeEditor` variant owns language, line-number setting, indent-guide setting, folding flag, syntax highlighter, diagnostics, and parse task state.

References:

- [`InputMode` variants](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/mode.rs#L23-L47)
- [`InputMode::code_editor`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/mode.rs#L70-L84)
- [`InputState::code_editor` builder](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/state.rs#L589-L594)

Kioto decision:

- Use explicit editor settings/config structs instead of one broad mode enum that owns parsing, diagnostics, and rendering toggles.
- Keep mode-like flags for public API ergonomics, but keep feature subsystems (syntax, diagnostics, folding, search) behind separate boundaries.
- Do not merge general text-field behavior and code-editor behavior into one large state struct.

### 4. The layered display map matches Kioto's intended direction

`gpui-component` documents and implements a display mapping stack: buffer positions map through `WrapMap` for soft wrapping, then through `FoldMap` for folding, with `DisplayMap` as the public facade.

References:

- [`display_map` module docs](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/display_map/mod.rs#L1-L17)
- [`DisplayMap`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/display_map/display_map.rs#L28-L31)
- [`buffer_pos_to_display_pos` / `display_pos_to_buffer_pos`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/display_map/display_map.rs#L44-L65)
- [`WrapMap` prefix-cache fields](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/display_map/wrap_map.rs#L18-L35)
- [`DisplayMap::rebuild_fold_projection`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/display_map/display_map.rs#L245-L256)

Kioto decision:

- Keep `DisplayMap` as an explicit public facade with internal projection layers.
- Start with identity mapping, then add `WrapMap`, then `FoldMap` when needed.
- Keep display-coordinate APIs small and testable instead of scattering line/wrap/fold calculations across movement, rendering, and hit testing.

### 5. Incremental text wrapping can be isolated behind a line layout/preparation layer

`TextWrapper` stores a `Rope`, per-buffer-line `LineItem`s, soft-wrapped byte ranges, total soft-line count, and longest-row metadata. Incremental updates replace only the affected line range and then recompute aggregate counts.

References:

- [`TextWrapper` fields](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/display_map/text_wrapper.rs#L49-L64)
- [`TextWrapper::_update`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/display_map/text_wrapper.rs#L162-L254)
- [`LineLayout`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/display_map/text_wrapper.rs#L346-L356)
- [`LineLayout::position_for_index`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/display_map/text_wrapper.rs#L423-L458)

Kioto decision:

- Put line shaping/wrap preparation behind a dedicated layout module rather than doing it inline in `EditorElement`.
- Track longest-line width for horizontal scrolling when soft wrap is disabled.
- Treat soft-wrap ranges as byte ranges into buffer lines, with tests around boundaries and Unicode.

### 6. Prepaint should calculate a compact visible layout cache

`TextElement::prepaint` prepares font/wrap state, computes visible buffer lines, computes visible byte ranges, collects highlight runs, shapes only visible lines, computes scroll size, cursor bounds, selections, search matches, hover highlights, gutter line numbers, fold icons, and then returns a `PrepaintState`.

References:

- [`calculate_visible_range`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/element.rs#L816-L875)
- [`layout_lines`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/element.rs#L1217-L1312)
- [`highlight_lines`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/element.rs#L1315-L1392)
- [`TextElement::prepaint`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/element.rs#L1538-L1929)
- [`PrepaintState`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/element.rs#L1396-L1430)

Kioto decision:

- Keep a compact `EditorLayout`/`PositionMap` generated in prepaint and persisted for input/hit testing.
- Shape only visible lines in phase 1; do not shape the whole document.
- Prefer immutable snapshots/layout data for paint. `gpui-component` stores mutable `LastLayout` on `InputState`; Kioto should make this boundary more explicit.

### 7. Text input and IME integration should go through `EntityInputHandler`

`TextElement::paint` installs `ElementInputHandler`, while `InputState` implements `EntityInputHandler` for text extraction, selected/marked ranges, replacement, IME marked text, candidate bounds, and point-to-character mapping.

References:

- [`window.handle_input`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/element.rs#L1956-L1960)
- [`EntityInputHandler for InputState`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/state.rs#L2713-L3042)
- [`replace_text_in_range`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/state.rs#L2755-L2857)
- [`replace_and_mark_text_in_range`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/state.rs#L2860-L2936)
- [`bounds_for_range`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/state.rs#L2939-L2988)

Kioto decision:

- Follow this pattern directly for phase 1.
- Keep IME marked range separate from committed selection state.
- Make `bounds_for_range` depend on the persisted position map from prepaint.

### 8. Use GPUI actions/key contexts for commands, not raw printable key handling

`gpui-component` declares actions and binds them in an `init(cx)` entry point. The rendered `Input` attaches action handlers for deletion, movement, selection, clipboard, undo/redo, search, indentation, code actions, mouse events, and scroll events.

References:

- [`input` actions and `init`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/state.rs#L54-L279)
- [`Input` action handlers](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/input.rs#L271-L356)
- [`movement` command methods](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/movement.rs#L34-L259)
- [`selection` double/triple click helpers](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/selection.rs#L7-L31)

Kioto decision:

- Keep `crates/editor/src/actions.rs` as a first-class module.
- Bind platform-like defaults through `editor::init(cx)`.
- Keep printable text out of raw key handlers; key handlers should pause cursor blink or dispatch commands only.

### 9. One edit pipeline is valuable, but feature hooks must not make it untestable

`replace_text_in_range` is the central editing choke point. It validates/masks text, records history, updates diagnostics, adjusts folds, updates the display map, updates the highlighter and optional background parser, updates LSP/search/completion state, moves the selection, emits events, and notifies GPUI.

References:

- [`replace_text_in_range`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/state.rs#L2755-L2857)
- [`push_history`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/state.rs#L2009-L2017)
- [`undo` / `redo`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/state.rs#L2019-L2039)

Kioto decision:

- Use a single edit transaction path for insertion, deletion, paste, IME commit, undo, and redo.
- Keep the phase-1 edit path small: buffer change, selection change, display-map update, history/event emission.
- Add syntax/LSP/search/completion hooks later through explicit extension points instead of growing one monolithic method.

### 10. Background parsing should be debounced and bounded

For code editor mode, synchronous tree-sitter parsing is given a short timeout. If it times out, a background task parses and applies a tree later; replacing the stored task naturally debounces rapid edits.

References:

- [`InputMode::update_highlighter`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/mode.rs#L225-L279)
- [`dispatch_background_parse`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/state.rs#L2613-L2702)

Kioto decision:

- Keep parsing out of phase 1.
- When syntax highlighting arrives, use a bounded sync parse budget plus debounced background parse tasks.
- Never block editor input/rendering on full-document syntax work.

### 11. Cursor-follow and scroll ergonomics deserve explicit settings

The docs and implementation expose `scroll_beyond_last_line` and `cursor_surrounding_lines`, compute scroll size from wrapped rows plus trailing space, use deferred scroll offsets during layout, and clamp persisted scroll offsets to safe ranges.

References:

- [`editor.md` scroll behavior docs](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/docs/docs/components/editor.md#L156-L177)
- [`empty_bottom_height` and cursor padding helpers](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/element.rs#L221-L276)
- [`update_scroll_offset`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/state.rs#L1837-L1865)
- [`scroll_to`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/state.rs#L1868-L1970)

Kioto decision:

- Add minimal scroll state in phase 1 or 2, but design it for cursor-follow behavior from the beginning.
- Keep scroll clamping in one module, not scattered across movement and paint.
- Add editor-style options for trailing empty rows and cursor surrounding lines after basic scrolling works.

### 12. Gutter and fold affordances should be fixed overlay/layers, not part of text lines

Line numbers and fold icons are calculated separately from text content. Fold icon hitboxes use the unscrolled origin so the gutter stays fixed while the text scrolls horizontally.

References:

- [`layout_line_numbers`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/element.rs#L878-L914)
- [`layout_fold_icons`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/element.rs#L1044-L1194)
- [`paint_fold_icons`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/element.rs#L1197-L1214)
- [`gutter background and line-number paint`](https://github.com/longbridge/gpui-component/blob/c36b0c6ae6d14c33473f6610a27c3abc584afdf9/crates/ui/src/input/element.rs#L2140-L2202)

Kioto decision:

- Make gutter a paint/layout layer derived from visible rows and editor settings.
- Keep gutter x-position independent from horizontal text scroll.
- Add fold icons only after fold ranges exist in the display map.

### 13. Caveat: do not copy the monolithic state surface

`gpui-component` is very useful because it is a working GPUI implementation, but it also shows the cost of letting one component absorb all text-field, code-editor, search, LSP, completion, context-menu, popover, mask, number-input, and scroll behavior.

Kioto decision:

- Copy the boundaries that worked: state/entity, custom element, display map, visible layout cache, action dispatch, input handler.
- Do not copy the all-in-one `InputState` shape.
- Keep the first Kioto editor focused on editing, selection, rendering, and scrolling; add integrations as separate subsystems.

## Proposed Kioto architecture, informed by current research

Initial crate shape target:

```text
crates/editor/src/
  lib.rs              # barrel exports only once modules exist
  actions.rs          # GPUI editor actions and key bindings
  buffer.rs           # initial single-buffer text model
  display_map/        # buffer -> display projection and snapshot
    mod.rs
    snapshot.rs
    wrap_map.rs       # later: soft-wrap projection
    fold_map.rs       # later: fold projection
  editor.rs           # Editor entity/state and public API
  element.rs          # custom GPUI Element implementation
  input.rs            # EntityInputHandler implementation
  layout.rs           # visible line shaping and layout cache
  position_map.rs     # byte/display/pixel hit testing from last layout
  selection.rs        # selection/cursor model and commands
  scroll.rs           # scroll state and visible row math
  gutter.rs           # later: line numbers/fold markers
  style.rs            # public style/config types
  tests/
```

A later modal layer can either live in sibling modules or a separate crate/addon:

```text
crates/editor/src/
  modal.rs            # modal state interface, if kept in this crate
  helix.rs            # Helix-style commands, if kept in this crate
  keymap.rs           # declarative keymap/context support, if kept in this crate
```

This layout can change after additional codebase research, but the core boundary should remain:

```text
Buffer -> DisplayMap/DisplaySnapshot -> EditorSnapshot -> EditorElement layout/paint
                         ^                         |
                         |                         v
                  Editor commands <- input/actions/mouse
```

## Initial implementation phases

### Phase 1: Minimal editable multiline view

- [ ] Add GPUI dependencies to `crates/editor` only when the first view is implemented.
- [ ] Create an `Editor` entity with focus handle, buffer handle, selection state, display map, and last position map.
- [ ] Create an `EditorElement` custom element with `request_layout`, `prepaint`, and `paint`.
- [ ] Render visible plain-text lines with a cursor and selection highlight.
- [ ] Install `ElementInputHandler` during paint.
- [ ] Support text insertion, backspace/delete, arrow movement, click-to-focus, click-to-place-cursor, and select-all.
- [ ] Keep all text insertion IME-compatible through `EntityInputHandler`.

### Phase 2: Editing correctness

- [ ] Add grapheme-aware cursor movement and deletion.
- [ ] Add UTF-16 ↔ internal offset conversion tests.
- [ ] Add undo/redo transaction grouping.
- [ ] Add clipboard copy/cut/paste.
- [ ] Add mouse drag selection.
- [ ] Add scroll support and visible-row virtualization.

### Phase 3: Editor-quality display model

- [ ] Replace direct line mapping with a more explicit display-map pipeline.
- [ ] Add soft wrapping.
- [ ] Add line numbers/gutter as optional paint layers.
- [ ] Add current-line highlighting.
- [ ] Add syntax-highlight-ready text runs without committing to a language system yet.

### Phase 4: Extension points

- [ ] Define how diagnostics render as highlights/blocks.
- [ ] Define how completions attach to cursor position.
- [ ] Define how hover/links use the position map.
- [ ] Define how project/LSP-specific behavior stays outside the core editor crate.

### Phase 5: Helix-style modal layer

- [ ] Add explicit modal modes for insert, normal, and Helix-select semantics.
- [ ] Add a declarative default Helix keymap that dispatches editor/modal actions.
- [ ] Add counts and multi-key operator state.
- [ ] Implement Helix motions as selection-producing commands, not caret-only movement.
- [ ] Add modal setting sync for input enabled, cursor shape, line mode, and selection rendering.
- [ ] Keep modal state outside `EditorElement` and preferably outside core buffer/display-map logic.

## Out of scope for the first editor view

- Multi-buffer/excerpt editing.
- Syntax parsing and semantic tokens.
- LSP completions, diagnostics, code actions, or inlay hints.
- Git diff gutter, blame, review comments, or hunks.
- Minimap.
- Collaboration / remote selections.
- Helix/Vim modal editing in phase 1.
- Notebook cells or embedded block widgets.
- Full Zed feature parity.

## Open questions for later research

- What buffer representation should Kioto use first: simple string, rope, or a custom line tree?
- Should anchors exist in phase 1, or should they wait until undo/multi-selection/display transforms require them?
- Which non-Zed codebases should influence the buffer and editing-command model?
- How should this editor crate relate to `base_gpui` primitives and the existing single-line input primitive?
- Should the public API expose a high-level `Editor::new()` only, or also lower-level buffer/display-map handles?
- Should Helix mode live inside `crates/editor` or in a separate addon/crate that depends on the editor core?
