# GPUI input primitive research

This note captures the code search for building a GPUI-native equivalent of an HTML `<input>` for `base_gpui`, with an emphasis on making `Field` useful for real text entry.

## TL;DR

A working GPUI text input is possible. GPUI does not expose a ready-made `input()` element, but it **does** expose the platform text-input bridge we need:

- implement `EntityInputHandler` for an entity that owns text/selection/IME state;
- render a custom `Element` that shapes/paints text, selection, cursor, and placeholder;
- call `window.handle_input(&focus_handle, ElementInputHandler::new(bounds, entity), cx)` during `paint`;
- use GPUI actions/key contexts for editing commands like Backspace, arrows, copy/paste, select-all;
- use a stable `FocusHandle` and track it on the focusable wrapper;
- register the input with `Field` by publishing `FieldControlRegistration { value: FieldValue::Text(...), focused, disabled, required, focus_handle, name }`.

The smallest useful version for `base_gpui` should be a **single-line `Input` primitive**, not the full `gpui-component` editor stack. Use Zed's `crates/gpui/examples/input.rs` as the base mental model, borrow production lessons from `gpui-component`, and keep the public API field-aware from the start.

## Code searched

Primary sources:

- `/home/luke/Projects/zed/crates/gpui/src/input.rs`
- `/home/luke/Projects/zed/crates/gpui/src/platform.rs`
- `/home/luke/Projects/zed/crates/gpui/src/window.rs`
- `/home/luke/Projects/zed/crates/gpui/examples/input.rs`
- `/home/luke/Projects/zed/crates/editor/src/input.rs`
- `/home/luke/Projects/zed/crates/editor/src/element.rs`
- `/home/luke/Projects/zed/crates/ui_input/src/input_field.rs`
- `/home/luke/Projects/zed/crates/settings_ui/src/components/input_field.rs`
- `/home/luke/Projects/gpui-component/crates/ui/src/input/`
- `/home/luke/Projects/gpui-component/crates/story/src/stories/input_story.rs`
- `/home/luke/Projects/base-ui/packages/react/src/field/control/FieldControl.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/internals/field-register-control/`
- `/home/luke/Projects/base-ui/packages/react/src/field/root/useFieldValidation.ts`
- current `base_gpui` field/control integrations under `crates/base_gpui/src/field`, `checkbox`, `switch`, and `radio_group`.

Useful anchors:

- GPUI text bridge: `EntityInputHandler` at `zed/crates/gpui/src/input.rs:10`, `ElementInputHandler` at `:82`, `Window::handle_input` at `zed/crates/gpui/src/window.rs:4339`, `InputHandler` / `UTF16Selection` at `zed/crates/gpui/src/platform.rs:1355`.
- Minimal text input example: `zed/crates/gpui/examples/input.rs` (`TextInput` at `:36`, `EntityInputHandler` at `:274`, custom `Element` at `:419`, `handle_input` at `:553`).
- `gpui-component` input: `InputState` at `gpui-component/crates/ui/src/input/state.rs:340`, `Input` builder at `input.rs:35`, `TextElement` element implementation at `element.rs:1496`, `handle_input` at `element.rs:1956`.
- Base UI FieldControl behavior: `FieldControl.tsx:31`, field registration at `:103`, dirty/filled/focus/blur handling at `:117-141`.

## What GPUI already gives us

GPUI's platform bridge is intentionally lower-level than HTML:

1. `EntityInputHandler` is the entity-facing trait. It asks for text ranges, selected range, marked/composing range, replacement, IME candidate bounds, point-to-character hit testing, and whether the view accepts text input.
2. `ElementInputHandler` adapts an `Entity<T: EntityInputHandler>` into GPUI's platform `InputHandler`.
3. `Window::handle_input` installs that handler for the current frame, but only if the given `FocusHandle` is focused. It must be called during the element `paint` phase.
4. Platform ranges are UTF-16 offsets. Our text buffer and layout code will probably use UTF-8 byte offsets, so conversion helpers are mandatory.

This is the GPUI equivalent of the browser's hidden text-input machinery. We do not need to intercept printable keys manually; the platform/IME path should perform insertion through `replace_text_in_range` and `replace_and_mark_text_in_range`.

## Lessons from Zed's minimal GPUI input example

`zed/crates/gpui/examples/input.rs` is the best small blueprint.

Key ideas to copy:

- State owns:
  - stable `FocusHandle`;
  - `SharedString` content and placeholder;
  - selected byte range and selection direction;
  - optional marked/IME byte range;
  - last shaped line and bounds for hit testing / IME positioning;
  - mouse selection state.
- Editing actions are GPUI actions (`Backspace`, `Delete`, `Left`, `Right`, `SelectAll`, `Home`, `End`, `Paste`, `Cut`, `Copy`), not raw printable key handlers.
- Grapheme boundaries use `unicode_segmentation` so Left/Right/Backspace do not split emoji or combined characters.
- The entity implements UTF-16 conversion helpers and `EntityInputHandler`.
- The custom `TextElement`:
  - requests a simple line-height layout;
  - shapes the current display text with `window.text_system().shape_line(...)`;
  - paints selection before text;
  - paints cursor only when focused;
  - stores the shaped line/bounds back into the entity;
  - calls `window.handle_input(...)` during `paint`.
- The focusable wrapper sets `.key_context("TextInput")`, `.track_focus(&focus_handle)`, cursor style, actions, and mouse handlers.

What to avoid copying literally:

- global/unscoped key bindings from the example; `base_gpui` should bind under a component-specific context in `input/actions.rs`;
- example-only styling;
- assuming a one-line `ShapedLine` is enough for future textarea/code-editor behavior.

## Lessons from `gpui-component`'s input

`gpui-component` has a production-grade `Input` family. It is much larger than we need, but it validates the architecture.

Important pieces:

- `InputState` is the editing entity and implements `EntityInputHandler`.
- `Input` is a renderable builder bound to an `Entity<InputState>`.
- `TextElement` is the custom element that shapes and paints text, selection, cursor, line numbers, scroll state, placeholders, masks, and inline completion.
- It registers the platform handler in `TextElement::paint` with `window.handle_input(...)`.
- It emits `InputEvent::{Change, PressEnter, Focus, Blur}`.
- It separates text input from wrapper chrome: prefix/suffix, clear button, loading, mask toggle, borders, sizes, disabled state.
- It has a much richer model than a primitive needs: multi-line, auto-grow, code editor mode, folding, LSP completions, diagnostics, search panel, scrollbars, syntax highlighting, document colors.

Good ideas for `base_gpui`:

- Have an explicit `InputEvent` concept or equivalent callbacks for `change`, `enter`, `focus`, and `blur`.
- Keep `InputState`/runtime separate from visual chrome.
- Support a disabled gate inside `replace_text_in_range` and `replace_and_mark_text_in_range`, not only in the wrapper.
- Store `last_layout` and `last_bounds` for IME candidate positioning and mouse hit testing.
- Treat masks/patterns as optional future constraints, not part of the first primitive.

Do **not** import this whole design into `base_gpui` initially. For an HTML `<input>` equivalent, we only need the single-line subset.

## Lessons from Zed's `Editor` and `ui_input`

Zed often wraps its full editor for input fields:

- `ui_input::InputField` wraps a single-line `Editor` behind an `ErasedEditor` trait.
- `SettingsInputField` creates `Editor::single_line(...)` and tracks its focus handle on an outer chrome element.

This is a strong option for an application, but less ideal for `base_gpui` because:

- `base_gpui` currently depends only on `gpui` / `gpui_platform`, not Zed's `editor` crate;
- pulling the full editor into a base component library would be a very large dependency and abstraction leak;
- we need a primitive, not a code editor.

Accessibility is intentionally out of scope for the first `base_gpui` input primitive. Revisit AccessKit wiring after updating GPUI.

## Base UI `FieldControl` contract to preserve

Base UI's `Field.Control` does not implement text editing; the browser does. Its real job is field integration:

- render an `<input>`;
- derive disabled/name from the surrounding field;
- register a control with the field root;
- on change: notify value callback, set dirty, set filled, clear form errors, run change validation;
- on focus: set focused;
- on blur: set touched, clear focused, run blur validation;
- on Enter: mark touched and commit validation.

For `base_gpui`, our text input must provide both halves:

1. text editing / IME / selection / painting, because GPUI has no DOM input;
2. field registration and state reporting, because `Field` cannot infer text values from an erased `AnyElement`.

## Current `base_gpui` Field implications

Current `Field` has the right metadata shape for a text input:

- `FieldControlRegistration::value(FieldValue::Text(...))` exists;
- registrations can carry disabled, focused, required, name, and a `FocusHandle`;
- labels can call `focus_control(...)`;
- validation modes already include `OnSubmit`, `OnBlur`, and `OnChange`.

However, text input exposes a current limitation more sharply than checkbox/switch/radio:

- `FieldRoot` derives root render state before descendant controls register for the frame.
- Descendant parts can query field state before a later sibling control has registered.
- For text input, this can create stale/one-frame-late `filled`, `focused`, `dirty`, and error state.

When adding a field-aware `Input`, prefer one of these fixes:

1. make `FieldRoot`'s child wiring able to collect field-aware control metadata before any part queries state; or
2. make `Input` a typed `FieldChild`/`FieldItemChild` so the root can attach context and pre-register it before rendering labels/errors; or
3. keep registration during render/layout but explicitly accept a refresh pass and document the one-frame behavior.

Option 1 or 2 is cleaner for a form primitive.

## Recommended `base_gpui` shape

Add a new `input` module, following `docs/base-gpui-component-architecture.md`:

```text
crates/base_gpui/src/input/
  mod.rs
  actions.rs
  runtime.rs        # text, selection, IME range, dirty/focused metadata, layout cache
  context.rs        # thin wrapper around Entity<InputRuntime> + props
  props.rs          # value/default_value/name/disabled/read_only/required/callbacks
  render_state.rs   # InputRootRenderState / InputTextRenderState if needed
  layers/
    input_root.rs   # public builder/chrome + field registration
    text_element.rs # custom Element for text layout/painting/platform input
  tests/
```

A practical API direction:

- `InputRoot::new()` / `Input::new()` with a stable `.id(...)`;
- `.value(...)`, `.default_value(...)`, `.on_value_change(...)` for controlled/uncontrolled behavior;
- `.placeholder(...)`;
- `.disabled(...)`, `.read_only(...)`, `.required(...)`, `.name(...)`;
- `.auto_focus(...)` if needed;
- `.on_enter(...)` or `InputEvent::PressEnter` equivalent;
- `.style_with_state(...)` with `disabled`, `read_only`, `focused`, `empty/filled`, `dirty`, `invalid` if field state is available.

Internally:

- `InputRuntime` owns text and selection state.
- `InputTextElement` implements `Element` and installs `ElementInputHandler` in `paint`.
- The runtime implements `EntityInputHandler`.
- The focus handle is stable and keyed by the input id.
- Field integration happens in the root render path:
  - compute merged disabled from input props + `FieldRoot` + `FieldItem`;
  - register `FieldValue::Text(current_value)`;
  - include `focused`, `required`, and `focus_handle`;
  - call `FieldContext::mark_touched` on blur or equivalent focus-out subscription;
  - trigger `OnChange` through `register_control` value changes.

## Implementation phases

### Phase 1: minimal single-line text input

Goal: a usable text box that works with `Field` required validation and label focus.

Include:

- text insertion via platform `EntityInputHandler`;
- Backspace/Delete/Left/Right/Home/End/SelectAll;
- copy/cut/paste;
- mouse click and drag selection;
- placeholder;
- focus ring/style state;
- disabled/read-only gates;
- UTF-16 <-> UTF-8 conversion;
- grapheme-aware movement/deletion;
- IME marked text underline and candidate bounds;
- `FieldControlRegistration` with `FieldValue::Text`.

Defer:

- multiline textarea;
- syntax highlighting / LSP / scrollbars;
- masks and number stepping;
- full browser-like constraint validation beyond required/custom validation;
- all AccessKit/accessibility wiring until the GPUI dependency is updated.

### Phase 2: field validity constraints

To support `FieldError::match_(FieldValidityKey::TypeMismatch)` and friends for text inputs, extend the field registration contract or validation path with text-input validity facts:

- `required` -> `value_missing`;
- `min_length` -> `too_short`;
- `max_length` -> `too_long`;
- `pattern` -> `pattern_mismatch`;
- `kind=email/url` -> `type_mismatch`;
- numeric constraints later -> range/step flags.

Base UI gets these from the browser `ValidityState`. In GPUI we need to compute them ourselves.

### Phase 3: specialized inputs

Build these on top of the primitive, not inside phase 1:

- `NumberInput` with step buttons and numeric masks;
- `PasswordInput` / mask toggle;
- `OtpInput`-style segmented input;
- multiline `TextArea`.

## Open design decisions

1. Should consumers manage an `Entity<InputState>` like `gpui-component`, or should `InputRoot` hide the entity behind keyed state like existing `base_gpui` components?
   - Recommendation: hide it for the common builder API, but expose a state handle later only if programmatic control needs it.
2. Should text input live as `InputRoot` or `FieldControl`?
   - Recommendation: create `InputRoot` first. Add `FieldControl` later as a thin alias/wrapper if we want Base UI's exact anatomy.
3. Should `FieldControlRegistration` carry full validity data?
   - Recommendation: yes eventually, but phase 1 can use `required` + custom `FieldRoot::validate`.
4. How should registration ordering be fixed?
   - Recommendation: update Field child wiring or add typed control children before relying heavily on field-derived styles.

## Pitfalls to avoid

- Do not handle printable character insertion through `on_key_down`; let platform input / IME call `replace_text_in_range`.
- Do not use raw byte offsets from platform callbacks; platform offsets are UTF-16.
- Do not split grapheme clusters on cursor movement or deletion.
- Do not call `window.handle_input` outside `paint`.
- Do not call it with an unfocused or unstable `FocusHandle`.
- Do not make the first version depend on Zed's full editor crate.
- Do not copy `gpui-component`'s code-editor/LSP/display-map complexity into a primitive.
- Do not rely on DOM concepts like hidden inputs, `htmlFor`, or browser `ValidityState` objects.

## Suggested next issue

Create a focused implementation issue: **Add GPUI-native single-line `Input` primitive and integrate it with `Field`**.

Acceptance highlights:

- typed `Input` module with actions, runtime/state, custom text element, tests;
- `cargo check -p base_gpui` and input tests pass;
- text insertion works through IME/platform input;
- placeholder, cursor, selection, mouse selection, copy/paste work;
- label click focuses the input inside `FieldRoot`;
- `FieldRoot` sees `filled`, `dirty`, `focused`, `touched`, and `FieldValue::Text` correctly;
- `OnChange` and `OnBlur` validation work for text values;
- disabled/read-only state prevents editing and is reflected in render state.
