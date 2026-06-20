# Port Base UI Input to GPUI

## Problem

Base UI's `Input` is a small but important form-control component: it renders a native text input and automatically participates in `Field.Root` state. In Base UI, `Input` is intentionally implemented as a thin wrapper around `Field.Control`, so its behavior is mostly the shared text-entry and field-control behavior rather than a large standalone component family.

`base_gpui` already has the hard parts needed for this port:

- a reusable GPUI-native single-line text input primitive under `crates/base_gpui/src/primitives/input/`;
- `FieldControl` under `crates/base_gpui/src/field/layers/field_control.rs`, which composes the primitive input and registers with `FieldRoot`;
- `FieldRoot` / `FieldLabel` / `FieldDescription` / `FieldError` / `FieldValidity` infrastructure.

What is still missing is a Base UI-style public `Input` component/module that users can import directly without reaching for `FieldControl` or the low-level primitive. The goal is to port the Base UI `Input` contract as a GPUI-native, Field-aware text input wrapper while preserving the dependency boundary: generic primitive input stays field-agnostic; the public Base UI `Input` may compose `FieldControl`.

## Scope

Port Base UI `Input` into GPUI-native API surface:

- `Input`
- `InputStyleState` or an equivalent state payload
- `InputChangeReason`
- `InputChangeDetails` if richer change details are added beyond the existing simple value callback

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/input/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/input/Input.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/input/Input.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/input/Input.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/input/InputDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/field/control/FieldControl.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/field/control/FieldControl.test.tsx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/input/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/input/types.md`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/input/demos/hero/css-modules/index.tsx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/input/demos/hero/css-modules/index.module.css`

Current GPUI implementation / prerequisites:

- `crates/base_gpui/src/primitives/input/mod.rs`
- `crates/base_gpui/src/primitives/input/actions.rs`
- `crates/base_gpui/src/primitives/input/props.rs`
- `crates/base_gpui/src/primitives/input/style_state.rs`
- `crates/base_gpui/src/primitives/input/runtime.rs`
- `crates/base_gpui/src/primitives/input/layers/input.rs`
- `crates/base_gpui/src/primitives/input/layers/text_element.rs`
- `crates/base_gpui/src/primitives/input/tests/`
- `crates/base_gpui/src/field/layers/field_control.rs`
- `crates/base_gpui/src/field/tests/`

Expected GPUI implementation files:

```text
crates/base_gpui/src/input/mod.rs
crates/base_gpui/src/input/props.rs                 # optional if wrapper props need named types
crates/base_gpui/src/input/style_state.rs          # optional if not reusing primitive/FieldControl state directly
crates/base_gpui/src/input/layers/mod.rs            # optional for consistency with other component modules
crates/base_gpui/src/input/layers/input.rs          # optional thin wrapper around FieldControl
crates/base_gpui/src/input/tests/
```

If the implementation is only a thin `FieldControl` wrapper, keep it thin. Do not invent a second text-editing runtime.

## Initial design decisions

### Placement

Create a top-level Base UI component module:

```text
crates/base_gpui/src/input/
```

This should be distinct from the low-level primitive:

```text
crates/base_gpui/src/primitives/input/
```

The primitive owns text editing behavior. The Base UI `Input` component owns the public Field-aware component contract.

### Public API shape

Prefer a builder-shaped component:

```rust
Input::new()
    .id("email")
    .name("email")
    .placeholder("hello@example.com")
    .default_value("")
    .required(true)
```

The implementation may also add an `input()` constructor under `base_gpui::input` if the naming is not confusing with `base_gpui::primitives::input::input()`. `Input::new()` should remain the canonical unambiguous API.

### Dependency direction

`Input` should compose `FieldControl`, which composes `primitives::input::Input`.

Do not make `primitives::input` import or know about `field` or the public `input` module.

### State model

Base UI `Input.State` is the `FieldControlState`:

- `disabled`
- `touched`
- `dirty`
- `valid: bool | null`
- `filled`
- `focused`

GPUI `Input` should expose these as typed style-state fields rather than DOM data attributes. It may also expose useful GPUI-native input facts already available from the primitive, such as:

- `value`
- `read_only`
- `required`
- `empty`
- `controlled`

### Change details

Base UI exposes `onValueChange(value, eventDetails)` where the only reason is `none` and the details object wraps DOM event behavior. GPUI should not port DOM event objects literally.

Acceptable GPUI options:

1. keep the current simple callback shape for phase 1:

   ```rust
   .on_value_change(|value: SharedString| { ... })
   ```

2. or add Rust-native details:

   ```rust
   .on_value_change(|value: SharedString, details: &mut InputChangeDetails, window, cx| { ... })
   ```

If details are added, use a Rust-native `InputChangeReason::None`; do not expose browser `Event`, propagation APIs, or DOM cancellation directly.

## Out of scope / drop from Base UI

- Do not port React refs, hooks, or context implementation details.
- Do not port the `render` prop. In Base UI this allows replacing `<input>` with `<textarea>`; in GPUI, multiline text input should be a separate `textarea`/multiline primitive issue.
- Do not port `className`.
- Do not port web `style` props.
- Do not port DOM data attributes as attributes. Map them into typed style-state fields.
- Do not port DOM event objects, `preventDefault`, propagation flags, or browser cancellation APIs literally.
- Do not port browser `ValidityState` objects. Use `FieldValidityData` / Rust-native validity flags.
- Do not implement hidden DOM inputs, browser form submission, or `FormData`.
- Do not implement exhaustive HTML `<input type=...>` parity here. Type-specific validation and masking remain input-primitive follow-ups unless a specific component needs them.
- Do not make the generic primitive input Field-aware.
- Do not use Rust scoped visibility syntax (`pub(...)`); `ast-grep scan` must remain clean.

## Acceptance Criteria

### Prerequisites already satisfied

- [x] A GPUI-native single-line text input primitive exists under `crates/base_gpui/src/primitives/input/`.
- [x] The primitive uses GPUI platform text input / IME plumbing rather than raw printable key handlers.
- [x] The primitive supports controlled and uncontrolled string values.
- [x] The primitive supports placeholder text.
- [x] The primitive supports disabled, read-only, required, auto-focus, and tab index metadata.
- [x] The primitive supports `style_with_state(...)` with typed style state.
- [x] The primitive exposes key bindings from `base_gpui::init(cx)`.
- [x] `FieldControl` exists and composes the primitive input.
- [x] `FieldControl` registers `FieldValue::Text(...)`, name, required, disabled, focused state, and focus handle metadata with `FieldRoot`.
- [x] `FieldLabel` can focus a registered `FieldControl`.
- [x] Required-only Field validation works for empty `FieldControl` values.

### Module/API surface

- [x] Add a top-level `input` module and export it from `crates/base_gpui/src/lib.rs`.
- [x] Add a public `Input` builder/element type that represents the Base UI `Input` component.
- [x] Re-export ergonomic names from `crates/base_gpui/src/input/mod.rs`.
- [x] Support `Input::new()`.
- [x] Decide whether to add a `base_gpui::input::input()` constructor or avoid it to prevent confusion with `base_gpui::primitives::input::input()`. Decision: use `Input::new()` only.
- [x] Support `.id(...)` as the stable keyed identity.
- [x] Support `.name(...)` metadata.
- [x] Support uncontrolled `.default_value(...)`.
- [x] Support controlled `.value(...)`.
- [x] Support `.on_value_change(...)` using either the existing simple value callback or a documented Rust-native change-details API.
- [x] Support `.placeholder(...)`.
- [x] Support `.disabled(bool)`, defaulting to `false`.
- [x] Support `.read_only(bool)`, defaulting to `false`.
- [x] Support `.required(bool)`, defaulting to `false`.
- [x] Support `.auto_focus(bool)`, defaulting to `false`.
- [x] Support `.tab_index(isize)` or the existing GPUI equivalent.
- [x] Support `.style_with_state(...)` with a Base UI Input-shaped style state.
- [x] Do not expose CSS class/data-attribute APIs.

### Architecture / implementation model

- [x] Implement `Input` as a thin wrapper around `FieldControl` unless a concrete missing behavior requires a small wrapper state type.
- [x] Do not duplicate `InputRuntime` or text editing logic from `primitives::input`.
- [x] Keep the generic primitive input field-agnostic.
- [x] Keep Field integration in `FieldControl` / public `Input`, not in the primitive.
- [x] Keep layers thin: builders forward configuration, runtime behavior stays in the primitive and Field runtime.
- [x] Follow `docs/base-gpui-component-architecture.md`; for this thin single-part component, do not create unnecessary runtime/context ontology.
- [x] Do not introduce `utils/` for Input-specific behavior.
- [x] Do not use `pub(crate)`, `pub(super)`, or other `pub(...)` scoped visibility syntax.

### Text editing behavior preserved through `Input`

- [x] Uncontrolled `Input` initializes from `.default_value(...)`.
- [x] Uncontrolled user edits update internal text.
- [x] Controlled `Input` displays `.value(...)` from props.
- [x] Controlled user edits call `.on_value_change(...)` without treating internal text as the source of truth.
- [x] Changing controlled `.value(...)` across renders updates displayed text.
- [x] Re-rendering changed props does not reset uncontrolled text, selection, or focus unless the keyed `.id(...)` changes.
- [x] Backspace/delete/cursor movement/selection/copy/cut/paste behavior from the primitive still works through public `Input`.
- [x] IME composition behavior from the primitive still works through public `Input`.
- [x] Disabled `Input` cannot be focused through normal tab navigation and cannot be edited.
- [x] Read-only `Input` can be focused/selected but cannot be edited.
- [x] Enter does not insert a newline and can trigger the existing enter/commit callback if that API is forwarded.

### Field integration behavior

- [x] `Input` rendered inside `FieldRoot` consumes root disabled state.
- [x] `Input` rendered inside disabled `FieldItem` is disabled.
- [x] `Input` registers a stable field control key.
- [x] `Input` registers `FieldValue::Text(current_value)`.
- [x] `Input` registers `name` metadata.
- [x] `Input` registers `required` metadata.
- [x] `Input` registers disabled metadata after root/item/input disabled merging.
- [x] `Input` registers focused state.
- [x] `Input` registers a focus handle so `FieldLabel` click focuses the input.
- [x] Field becomes filled when the `Input` value is non-empty.
- [x] Field becomes unfilled when the `Input` value is empty.
- [x] Field becomes dirty when the `Input` value differs from its initial registered value.
- [x] Field becomes focused when the `Input` is focused.
- [x] Field becomes touched when the `Input` loses focus.
- [x] `FieldValidationMode::OnChange` validates after `Input` value edits.
- [x] `FieldValidationMode::OnBlur` validates when `Input` blurs.
- [x] Required-only Field validation reports `value_missing` for an empty required `Input`.
- [x] Field error presence updates when text changes or blur validation runs.

### Styling/state exposure

- [x] Add or re-export an `InputStyleState` that includes Base UI Input state: `disabled`, `touched`, `dirty`, `valid`, `filled`, and `focused`.
- [x] Style state exposes `invalid` as `valid == Some(false)` or equivalent.
- [x] Style state exposes the current input value or documents why value is intentionally omitted.
- [x] Style state exposes `read_only`, `required`, `empty`, and `controlled` if those remain part of the underlying primitive state.
- [x] `style_with_state(...)` receives Field-derived state when rendered inside `FieldRoot`.
- [x] `style_with_state(...)` still works outside `FieldRoot` with default field state values.
- [x] Map Base UI Input data attributes (`disabled`, `valid`, `invalid`, `dirty`, `touched`, `filled`, `focused`) into typed style-state fields, not DOM attributes.
- [x] The docs hero styling pattern can be recreated with GPUI builder methods: label text plus a bordered input with placeholder/focus styling.

### Accessibility follow-up

- [ ] Once GPUI AccessKit input/label APIs are available, expose text input role/value/focused/disabled/required/invalid state through GPUI-native accessibility APIs.
- [ ] Once available, connect `FieldLabel` / accessible name to public `Input` through GPUI-native mechanisms.
- [ ] Add accessibility tests if GPUI exposes AccessKit tree test helpers.
- [x] Do not port DOM `aria-labelledby`, generated HTML ids, or browser label mechanics literally.

### Form integration follow-up

- [x] Keep `Input::name(...)` metadata compatible with future `issues/port-baseui-form.md` value collection.
- [x] Do not implement hidden DOM input submission behavior.
- [x] Do not implement browser `FormData`.

### Docs/demo

- [x] Add a small plain `Input` demo to `crates/base_gpui/src/main.rs` or a dedicated example.
- [x] Add a `FieldRoot` + `FieldLabel` + `Input` + `FieldError` demo showing required validation and state-aware styling.
- [x] Document that public `Input` is Field-aware, while `primitives::input::input()` is the low-level field-agnostic primitive.
- [x] Document follow-ups for input types, pattern/min/max/minLength/maxLength validation, password masking, and multiline textarea.

### Tests / verification

Add tests under `crates/base_gpui/src/input/tests/` where practical. It is fine for these to reuse primitive input test helpers where that avoids duplication, but public `Input` should have its own smoke/integration coverage.

- [x] Public `Input` default value is empty.
- [x] Public `Input::default_value(...)` initializes displayed text.
- [x] Public controlled `Input::value(...)` is displayed.
- [x] Public uncontrolled user edits update displayed text.
- [x] Public controlled user edits call `on_value_change`.
- [x] Public `Input` preserves primitive keyboard editing behavior through a smoke test.
- [x] Public `Input` placeholder renders when empty.
- [x] Public `Input` disabled state prevents focus/editing.
- [x] Public `Input` read-only state prevents editing but allows focus/selection.
- [x] Public `Input` style state receives disabled/focused/filled/dirty values.
- [x] Public `Input` inside `FieldRoot` consumes root disabled state.
- [x] Public `Input` inside disabled `FieldItem` is disabled.
- [x] Public `Input` label click focuses the input.
- [x] Public `Input` updates Field filled/dirty/focused/touched state.
- [x] Public required `Input` reports Field `value_missing` when empty.
- [x] Public `Input` updates Field error presence on change/blur validation.
- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui input` passes.
- [x] `cargo test -p base_gpui` passes.
