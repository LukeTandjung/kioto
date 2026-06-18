# Add GPUI-native Input Primitive

## Problem

`Field` currently has control-registration plumbing, but no GPUI-native text input primitive to register a real text value. Base UI ports will keep running into this because many components depend on the browser's `<input>` primitive for text entry, focus, selection, IME, form value metadata, and native-like validation events.

GPUI does not expose a ready-made `input()` element, but it does expose the lower-level bridge needed to build one:

- `EntityInputHandler` / `InputHandler` for platform text input and IME;
- `ElementInputHandler` for connecting an entity to the platform handler;
- `Window::handle_input(...)` for installing the handler during paint;
- GPUI actions/key contexts for editing commands.

The goal is to add a reusable `input()` primitive, similar in spirit to `div()`, with HTML input attributes translated into Rust builder methods. This primitive should be shared by Field, future Number Field, Search, Combobox, Select-like filtering, and other Base UI ports.

## Scope

Add a reusable single-line text input primitive under `base_gpui` primitives:

- public `input()` constructor function;
- public `Input` builder/element type;
- input runtime/state for text, cursor, selection, IME composition, focus, dirty/filled facts, and layout cache;
- a custom GPUI text element that shapes/paints placeholder, text, selection, cursor, and marked IME text;
- platform text input wiring with `EntityInputHandler` + `ElementInputHandler`;
- key bindings/actions for common single-line editing commands;
- Field integration through a Field-owned `FieldControl` wrapper that composes `input()` and registers `FieldControlRegistration` when rendered inside a `FieldRoot`.

Primary references:

- `/home/luke/Projects/kioto/docs/gpui-input-primitive-research.md`
- `/home/luke/Projects/zed/crates/gpui/src/input.rs`
- `/home/luke/Projects/zed/crates/gpui/src/platform.rs`
- `/home/luke/Projects/zed/crates/gpui/src/window.rs`
- `/home/luke/Projects/zed/crates/gpui/examples/input.rs`
- `/home/luke/Projects/gpui-component/crates/ui/src/input/`
- `/home/luke/Projects/base-ui/packages/react/src/field/control/FieldControl.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/internals/field-register-control/`
- `/home/luke/Projects/base-ui/packages/react/src/field/root/useFieldValidation.ts`

## Initial design decisions

### Placement

Put primitives under:

```text
crates/base_gpui/src/primitives/
  mod.rs
  input/
```

Do not put this in `field/`: text input is shared infrastructure, not Field-specific behavior. Do not put it in `utils/`: an input owns deep editing behavior and state, so it is a primitive, not a helper. Do not move it into `crates/ui` yet; incubate it where the Base UI ports need it, then split/move later only if multiple crates need the same primitive boundary.

### Public constructor shape

Expose an ergonomic primitive constructor:

```rust
input()
    .id("email")
    .name("email")
    .placeholder("Email")
    .default_value("")
```

The public type can be `Input`, but consumers should usually reach it through `input()` the same way they reach GPUI primitives through functions like `div()`.

### State ownership

Hide the editing entity behind keyed state for the first API. Consumers should not need to create an `Entity<InputState>` just to render a normal input.

Use `.id(...)` as the required stable identity for preserving uncontrolled text, selection, and focus across renders. A future state handle can be added if programmatic editing APIs become necessary.

### Field integration

Keep `input()` generic and field-agnostic. Field integration lives in a Field-owned `FieldControl` component that composes `input()` and registers with the current `FieldContext` using `FieldControlRegistration` with:

- `FieldValue::Text(current_value)`;
- merged disabled state;
- focused state;
- required metadata;
- name metadata;
- stable focus handle.

Keep the dependency direction one-way: `field` may import and compose the generic input primitive, but the input primitive must not import or know about Field. If Field registration ordering causes stale state, fix it through the shared registration pass/refresh behavior or document the remaining one-frame limitation with tests.

### Attribute mapping

Start with the HTML attributes that are needed for Base UI Field and common text input usage:

| HTML concept | GPUI builder |
|---|---|
| `id` | `.id(...)` |
| `name` | `.name(...)` |
| `value` | `.value(...)` |
| `defaultValue` | `.default_value(...)` |
| `placeholder` | `.placeholder(...)` |
| `disabled` | `.disabled(bool)` |
| `readonly` | `.read_only(bool)` |
| `required` | `.required(bool)` |
| `autofocus` | `.auto_focus(bool)` |
| `tabIndex` | `.tab_index(isize)` |

Do not attempt exhaustive HTML input parity in the first issue. Add input types, pattern/min/max validation, number stepping, password masking, etc. as follow-ups once the primitive works.

### Validity model

Phase 1 should support Field's required-only validation by registering `required` plus `FieldValue::Text`. Full browser-like validity flags (`typeMismatch`, `tooShort`, `patternMismatch`, etc.) require Rust-native validation logic and can be a follow-up.

### Accessibility

Accessibility is intentionally out of scope for this issue. Do not add AccessKit roles, synthetic text runs, labels, or invalid/required accessibility state until the GPUI dependency is updated and that work is explicitly revisited.

## Expected implementation files

```text
crates/base_gpui/src/primitives/mod.rs
crates/base_gpui/src/primitives/input/mod.rs
crates/base_gpui/src/primitives/input/actions.rs
crates/base_gpui/src/primitives/input/props.rs
crates/base_gpui/src/primitives/input/render_state.rs
crates/base_gpui/src/primitives/input/runtime.rs
crates/base_gpui/src/primitives/input/layers/mod.rs
crates/base_gpui/src/primitives/input/layers/input.rs
crates/base_gpui/src/primitives/input/layers/text_element.rs
crates/base_gpui/src/primitives/input/tests/
crates/base_gpui/src/field/layers/field_control.rs
```

Alternative filenames are fine if they preserve the same architecture: deep runtime/state, thin builder/layer, custom text element, and action bindings.

## Out of scope

- Do not implement multiline textarea behavior.
- Do not implement code editor behavior, syntax highlighting, LSP, diagnostics, folding, search, or scrollbars.
- Do not depend on Zed's full `editor` crate.
- Do not depend on `gpui-component`'s input implementation directly.
- Do not port DOM hidden inputs, browser form submission, or `FormData`.
- Do not port arbitrary DOM event objects or cancellation APIs.
- Do not implement full browser `ValidityState` parity beyond required metadata.
- Do not implement AccessKit/accessibility wiring in this issue.
- Do not support arbitrary child elements inside the primitive; HTML `<input>` is void-like. Prefix/suffix/chrome should be wrapper components built around `input()`.
- Do not handle printable character insertion with raw `on_key_down`; use GPUI's platform input handler / IME bridge.

## Acceptance Criteria

### Module/API surface

- [x] Add `crates/base_gpui/src/primitives/` and export it from `crates/base_gpui/src/lib.rs`.
- [x] Add `base_gpui::primitives::input` module exports.
- [x] Add a public `input()` constructor function.
- [x] Add a public `Input` builder/element type.
- [x] Register input key bindings from `base_gpui::init(cx)`.
- [x] Support `.id(...)` as the stable identity for keyed input state.
- [x] Support `.name(...)` metadata.
- [x] Support uncontrolled `.default_value(...)`.
- [x] Support controlled `.value(...)`.
- [x] Support `.on_value_change(...)` for user-driven text changes.
- [x] Support `.placeholder(...)`.
- [x] Support `.disabled(bool)`, defaulting to `false`.
- [x] Support `.read_only(bool)`, defaulting to `false`.
- [x] Support `.required(bool)`, defaulting to `false`.
- [x] Support `.auto_focus(bool)`, defaulting to `false`.
- [x] Support `.tab_index(isize)` or a documented GPUI equivalent.
- [x] Support `style_with_state(...)` with a typed render state.
- [x] Re-export ergonomic names from `primitives/input/mod.rs` and `primitives/mod.rs`.

### Text input behavior

- [x] Text insertion is handled through `EntityInputHandler::replace_text_in_range`, not raw printable key handlers.
- [x] IME composition is handled through `replace_and_mark_text_in_range` and `marked_text_range`.
- [x] Platform ranges are converted correctly between UTF-16 offsets and UTF-8 byte offsets.
- [x] Cursor movement and deletion are grapheme-aware and do not split combined characters or emoji.
- [x] Backspace deletes the selected text or previous grapheme.
- [x] Delete deletes the selected text or next grapheme.
- [x] Left/Right move the cursor or collapse the selection.
- [x] Shift-Left/Shift-Right extend the selection.
- [x] Home/End move to the start/end of the single-line input.
- [x] Select All selects the full input value.
- [x] Copy writes selected text to the clipboard.
- [x] Cut writes selected text and removes it when editable.
- [x] Paste inserts clipboard text, normalizing line breaks for single-line input.
- [x] Enter does not insert a newline and can trigger a Rust-native enter/submit callback or action.
- [x] Clicking the input focuses it and moves the cursor to the clicked character.
- [x] Dragging selects text.
- [x] Placeholder text renders when the value is empty.
- [x] Selection highlight renders when text is selected.
- [x] Cursor renders when focused and the selection is collapsed.
- [x] IME marked text renders with a visible underline or equivalent marker.
- [x] IME candidate bounds are computed from the current text layout.
- [x] Disabled input cannot be focused through normal tab navigation and cannot be edited.
- [x] Read-only input can be focused and selected but cannot be edited.

### Controlled/uncontrolled behavior

- [x] Uncontrolled input initializes from `.default_value(...)`.
- [x] Uncontrolled user edits update internal state.
- [x] Controlled input reflects `.value(...)` from props.
- [x] Controlled user edits call `.on_value_change(...)` without treating internal state as the source of truth.
- [x] Changing controlled `.value(...)` across renders updates displayed text.
- [x] Re-rendering with changed props does not reset uncontrolled text, selection, or focus unless the keyed `.id(...)` changes.
- [x] Programmatic/default value changes do not clobber focused uncontrolled edits unless explicitly controlled.

### Field integration

- [x] `FieldControl` rendered inside `FieldRoot` consumes root disabled state.
- [x] `FieldControl` rendered inside disabled `FieldItem` is disabled.
- [x] `FieldControl` registers a `FieldControlRegistration` with a stable key.
- [x] Registration includes `FieldValue::Text(current_value)`.
- [x] Registration includes `name` metadata, with `FieldRoot::name(...)` continuing to act as the future Form override.
- [x] Registration includes `required` metadata.
- [x] Registration includes disabled metadata after root/item/input disabled merging.
- [x] Registration includes focused state.
- [x] Registration includes a focus handle so `FieldLabel` click focuses the input.
- [x] Field becomes filled when the input value is non-empty.
- [x] Field becomes unfilled when the input value is empty.
- [x] Field becomes dirty when the input value differs from its initial registered value.
- [x] Field becomes focused when the input is focused.
- [x] Field becomes touched when the input loses focus.
- [x] `FieldValidationMode::OnChange` validates when the input value changes.
- [x] `FieldValidationMode::OnBlur` validates when the input blurs.
- [x] Required-only Field validation reports `value_missing` when a required input is empty.
- [x] Field error presence updates when text changes or blur validation runs.

### Architecture / implementation model

- [x] Use GPUI `EntityInputHandler` and `ElementInputHandler` for platform text input.
- [x] Call `window.handle_input(...)` only during the custom text element's `paint` phase.
- [x] Use a stable keyed `FocusHandle`.
- [x] Attach `.track_focus(...)` and a scoped key context on the focusable wrapper.
- [x] Use GPUI actions/key bindings for editing commands.
- [x] Keep text editing state and rules in the primitive runtime/state, not in the visual layer.
- [x] Keep the visual layer thin: events/actions translate into runtime commands, runtime queries feed styling/painting.
- [x] Keep Field integration one-way from `field` to `input`; do not make the primitive input module depend on `field`.
- [x] Avoid adding generic helpers unless they represent a repeated deep primitive concept.
- [x] Add only small dependencies when justified, e.g. `unicode-segmentation`; do not pull in full editor/input libraries.

### Styling/state exposure

- [x] Add an `InputRenderState` or equivalent typed render-state struct.
- [x] Render state exposes `disabled`, `read_only`, `required`, `focused`, `empty`/`filled`, `dirty`, and whether the input is controlled if useful.
- [x] Render state includes field-derived invalid/valid facts if available without introducing stale/cyclic dependencies.
- [x] `style_with_state(...)` receives the current render state.
- [x] Do not expose DOM data attributes as the styling API.
- [x] Do not expose CSS variable names as the styling API.

### Docs/demo

- [x] Update `crates/base_gpui/src/main.rs` or add an example rendering a plain input.
- [x] Add a demo of `FieldRoot` + `FieldLabel` + `FieldControl` + `FieldError`, where `FieldControl` composes `input()`.
- [x] Document the first supported builder-method attribute subset.
- [x] Document follow-ups for input type/pattern/min/max validation.

### Tests / verification

Add behavior tests under `crates/base_gpui/src/primitives/input/tests/` where practical.

- [x] Default input value is empty.
- [x] `default_value` initializes displayed text.
- [x] Controlled `value` is displayed.
- [x] User text insertion updates uncontrolled value.
- [x] User text insertion calls `on_value_change` in controlled mode.
- [x] Backspace removes previous grapheme.
- [x] Delete removes next grapheme.
- [x] Replacing a selection inserts new text at the selection.
- [x] Left/Right cursor movement respects grapheme boundaries.
- [x] Select All selects the whole value.
- [x] Copy copies selected text.
- [x] Cut removes selected text when editable.
- [x] Paste inserts clipboard text.
- [x] Disabled input ignores editing.
- [x] Read-only input ignores editing but can focus/select.
- [x] Placeholder renders when empty.
- [x] Cursor renders when focused.
- [x] Selection renders when non-empty.
- [x] IME marked range is stored and reported.
- [x] Mouse click moves the cursor.
- [x] Mouse drag selects text.
- [x] Field label click focuses the input.
- [x] Field filled/dirty/focused/touched states update from input registration.
- [x] Field required validation reports missing value for an empty required input.
- [x] OnChange Field validation runs after value edits.
- [x] OnBlur Field validation runs after blur.
- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui input` passes.
- [x] `cargo test -p base_gpui` passes.

## Follow-ups

- Add input type variants such as search, password, email, url, number.
- Add pattern/min/max/minLength/maxLength validity flags.
- Add Number Field on top of `input()`.
- Add password masking and reveal controls.
- Add multiline `textarea()` primitive.
- Revisit whether a `FieldControl` alias/wrapper is useful for exact Base UI anatomy.
