# Port Base UI Number Field to GPUI

## Problem

Base UI Number Field provides a numeric text input with controlled/uncontrolled numeric value, parse/format behavior, increment/decrement controls, keyboard stepping, optional scrubbing, disabled/read-only/required state, and Field integration.

`crates/base_gpui` now has a generic GPUI-native `input()` primitive and a Field component family. Number Field should be the first higher-level text-entry component built on top of `input()`, without making `input()` special-purpose. The goal is to port the useful Base UI Number Field behavior into GPUI-native components while preserving the local architecture: deep runtime/state module, thin layers, typed render state, and no DOM/React leakage.

## Scope

Add a `number_field` component family under `crates/base_gpui/src/number_field/`:

- `NumberFieldRoot`
- `NumberFieldInput`
- `NumberFieldGroup`
- `NumberFieldIncrement`
- `NumberFieldDecrement`
- `NumberFieldScrubArea`
- `NumberFieldScrubAreaCursor`

The first implementation should support a practical ASCII/`f64` Number Field:

- controlled/uncontrolled numeric value, including empty/null value;
- text editing through the existing generic `input()` primitive;
- parseable text entry and empty clears;
- parse/format/clamp on blur/commit;
- increment/decrement buttons;
- keyboard stepping;
- basic pointer-drag scrubbing;
- min/max/step/small-step/large-step/snap-on-step behavior;
- disabled/read-only/required state;
- Field integration using `FieldControlRegistration` from the Number Field component, not from the generic input primitive.

## Base UI references

Documentation:

- `https://base-ui.com/react/components/number-field.md`

Source:

- `/home/luke/Projects/base-ui/packages/react/src/number-field/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/root/NumberFieldRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/root/NumberFieldRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/root/NumberFieldRootDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/root/NumberFieldRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/root/NumberFieldRoot.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/root/useNumberFieldButton.ts`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/root/useNumberFieldStepperButton.ts`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/input/NumberFieldInput.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/input/NumberFieldInput.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/group/NumberFieldGroup.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/group/NumberFieldGroup.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/increment/NumberFieldIncrement.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/increment/NumberFieldIncrement.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/decrement/NumberFieldDecrement.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/decrement/NumberFieldDecrement.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/scrub-area/NumberFieldScrubArea.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/scrub-area/NumberFieldScrubArea.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/scrub-area-cursor/NumberFieldScrubAreaCursor.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/scrub-area-cursor/NumberFieldScrubAreaCursor.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/utils/parse.ts`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/utils/parse.test.ts`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/utils/validate.ts`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/utils/validate.test.ts`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/utils/stateAttributesMapping.ts`
- `/home/luke/Projects/base-ui/packages/react/src/number-field/utils/types.ts`

Current GPUI implementation: none exists under `crates/base_gpui/src/number_field/`; this issue creates it.

Local GPUI references:

- `crates/base_gpui/src/primitives/input/`
- `crates/base_gpui/src/field/`
- `crates/base_gpui/src/checkbox/`
- `crates/base_gpui/src/switch/`
- `crates/base_gpui/src/radio_group/`
- `docs/base-gpui-component-architecture.md`

## Initial design decisions

### Placement

Put Number Field under:

```text
crates/base_gpui/src/number_field/
```

Do not put it under `primitives/`: Number Field is a higher-level component with number-specific parsing, stepping, validation, and child parts. The generic `input()` primitive remains the low-level text entry primitive.

### Public API shape

Expose Base UI-style component parts as Rust builder types:

```rust
NumberFieldRoot::new()
    .id("amount")
    .default_value(Some(100.0))
    .min(0.0)
    .max(1_000.0)
    .step(1.0)
    .child(NumberFieldScrubArea::new().child("Amount"))
    .child(
        NumberFieldGroup::new()
            .child(NumberFieldDecrement::new().child("-"))
            .child(NumberFieldInput::new())
            .child(NumberFieldIncrement::new().child("+")),
    )
```

Use `NumberFieldRoot::value(Option<f64>)` for controlled mode. Internally this should distinguish absent controlled props from controlled empty value, e.g. `Option<Option<f64>>`.

### Dependency direction

`NumberFieldInput` should compose the generic `input()` primitive.

`input()` must not import or know about Number Field or Field. Number Field may integrate with `FieldRoot`, because Number Field is a form-control component, not a generic primitive. Keep this integration in `NumberFieldRoot`/runtime registration code rather than in `input()`.

### Parsing and formatting phase

Phase 1 should implement a small Rust-native parser/formatter:

- ASCII digits;
- optional leading `+`/`-`;
- decimal point;
- optional exponent if straightforward with `f64::from_str`;
- trim surrounding whitespace;
- reject non-finite values (`NaN`, `inf`, `-inf`) as invalid text.

Locale-aware parsing, Unicode numerals, currency, percent/per-mille, units, grouping separators, and `Intl.NumberFormatOptions` parity are follow-ups. Do not pull in ICU or a large formatting library in this issue unless explicitly justified.

### Text vs numeric value model

The runtime should track both:

- `value: Option<f64>` — committed/parseable numeric value;
- `input_value: SharedString` — text currently displayed in the input.

Typing may update `input_value` even when the text is an intermediate invalid number. Numeric `value` changes only when the text is empty or parseable. Blur/commit should resolve invalid/intermediate text by formatting the current numeric value or clearing if value is `None`.

### Field integration

When rendered inside `FieldRoot`, Number Field should register a control with:

- `FieldValue::Text(input_value)` for current Field infrastructure, or add a numeric `FieldValue` variant only if it stays shallow and does not disrupt existing controls;
- name metadata, with `FieldRoot::name(...)` continuing to act as the future Form override;
- required metadata;
- merged disabled metadata;
- focused state;
- stable focus handle for label-click focus.

Required-only Field validation should treat `None` / empty input as missing. Full numeric validity flags (`rangeUnderflow`, `rangeOverflow`, `stepMismatch`, `badInput`) can be represented in Number Field render state first and wired into Field validity only if the existing Field model supports it cleanly.

## Expected implementation files

```text
crates/base_gpui/src/number_field/mod.rs
crates/base_gpui/src/number_field/actions.rs
crates/base_gpui/src/number_field/child.rs
crates/base_gpui/src/number_field/child_wiring.rs
crates/base_gpui/src/number_field/context.rs
crates/base_gpui/src/number_field/props.rs
crates/base_gpui/src/number_field/render_state.rs
crates/base_gpui/src/number_field/runtime.rs
crates/base_gpui/src/number_field/number.rs
crates/base_gpui/src/number_field/layers/mod.rs
crates/base_gpui/src/number_field/layers/number_field_root.rs
crates/base_gpui/src/number_field/layers/number_field_input.rs
crates/base_gpui/src/number_field/layers/number_field_group.rs
crates/base_gpui/src/number_field/layers/number_field_increment.rs
crates/base_gpui/src/number_field/layers/number_field_decrement.rs
crates/base_gpui/src/number_field/layers/number_field_scrub_area.rs
crates/base_gpui/src/number_field/layers/number_field_scrub_area_cursor.rs
crates/base_gpui/src/number_field/tests/
```

Alternative filenames are fine if they preserve the same architecture: deep runtime/state, thin layers, typed child wiring, typed render state, and isolated number parsing/validation helpers.

## Out of scope / drop from Base UI

- Do not port React hooks/context directly.
- Do not port `className`, web `style`, `render` props, or DOM data attributes; expose typed render-state structs and `style_with_state(...)`.
- Do not port hidden DOM inputs, browser form submission, or `FormData`.
- Do not port DOM event objects, propagation APIs, or cancellation APIs literally.
- Do not implement locale-aware `Intl.NumberFormat` parity in phase 1.
- Do not implement Unicode numeral/currency/unit/percent/per-mille parsing in phase 1.
- Do not implement native Pointer Lock API semantics; GPUI scrub behavior can use normal pointer drag first.
- Do not implement scrub cursor teleport/wrap behavior in phase 1.
- Do not implement AccessKit/accessibility wiring until GPUI accessibility work is explicitly revisited.
- Do not make the generic `input()` primitive depend on Number Field or Field.
- Do not use Rust scoped visibility syntax (`pub(...)`); `ast-grep scan` must remain clean.

## Acceptance Criteria

### Module/API surface

- [x] Add a `number_field` module and export it from `crates/base_gpui/src/lib.rs`.
- [x] Register Number Field key bindings from `base_gpui::init(cx)`.
- [x] Add public `NumberFieldRoot`, `NumberFieldInput`, `NumberFieldGroup`, `NumberFieldIncrement`, `NumberFieldDecrement`, `NumberFieldScrubArea`, and `NumberFieldScrubAreaCursor` layer types.
- [x] Add a typed `NumberFieldChild` enum for root children.
- [x] Add typed child routing for `NumberFieldGroup` children.
- [x] Support root `.id(...)` as the stable keyed identity.
- [x] Support `.name(...)` metadata.
- [x] Support uncontrolled `.default_value(Option<f64>)`.
- [x] Support controlled `.value(Option<f64>)`, including controlled empty value.
- [x] Support `.on_value_change(...)` with Rust-native change details.
- [x] Support `.on_value_committed(...)` with Rust-native commit details.
- [x] Support `.min(f64)` and `.max(f64)`.
- [x] Support `.step(f64)`, defaulting to `1.0`.
- [x] Support a `step_any` equivalent where interactive stepping still uses `1.0` but step mismatch validation is disabled/deferred.
- [x] Support `.small_step(f64)`, defaulting to `0.1`.
- [x] Support `.large_step(f64)`, defaulting to `10.0`.
- [x] Support `.snap_on_step(bool)`, defaulting to `false`.
- [x] Support `.allow_out_of_range(bool)`, defaulting to `false`.
- [x] Support `.allow_wheel_scrub(bool)`, defaulting to `false`, or explicitly defer wheel scrub with tests documenting no-op behavior.
- [x] Support `.disabled(bool)`, defaulting to `false`.
- [x] Support `.read_only(bool)`, defaulting to `false`.
- [x] Support `.required(bool)`, defaulting to `false`.
- [x] Preserve `.form(...)` only as metadata for future form integration if useful; do not implement DOM form ownership.
- [x] Expose ergonomic barrel exports from `number_field/mod.rs`.
- [x] Do not expose CSS class/data-attribute APIs.

### Runtime / architecture

- [x] Add `NumberFieldRuntime` as the owner of numeric value, input text value, dirty/touched/focused facts, scrubbing state, and pending commit state.
- [x] Add `NumberFieldProps` for stable root props and callbacks.
- [x] Add `NumberFieldContext` as thin read/update/command plumbing.
- [x] Keep parsing, validation, clamping, stepping, and value-change decisions in runtime/helpers, not in visual layers.
- [x] Keep layers thin: user events translate into runtime commands, runtime state feeds styling/painting.
- [x] Use the generic `input()` primitive inside `NumberFieldInput`.
- [x] Do not make the generic input primitive know about Number Field.
- [x] Do not introduce generic helpers unless they hide a repeated deep concept across components.
- [x] Avoid large dependencies for parsing/formatting; prefer small Rust-native helpers first.
- [x] Do not use `pub(crate)`, `pub(super)`, or any other `pub(...)` scoped visibility syntax.

### Numeric parsing / formatting / validation

- [x] Empty text parses to `None` and marks the field unfilled.
- [x] ASCII integer text parses to `Some(f64)`.
- [x] ASCII decimal text parses to `Some(f64)`.
- [x] Leading `+` and `-` are handled.
- [x] Surrounding whitespace is ignored for parsing.
- [x] Non-finite values are rejected.
- [x] Intermediate invalid text can remain in `input_value` while focused without immediately clobbering user input.
- [x] Blur/commit resolves invalid/intermediate text to the formatted current numeric value or empty text when `value == None`.
- [x] Blur/commit clamps to `min`/`max` when `allow_out_of_range == false`.
- [x] Direct text entry may remain outside `min`/`max` while focused when `allow_out_of_range == true`.
- [x] Step-based interactions always clamp to `min`/`max`.
- [x] `snap_on_step` snaps stepping values to the configured step grid.
- [x] Floating-point stepping noise is cleaned enough for common cases such as `0.1 + 0.2`.
- [x] Parser/formatter helpers have focused unit tests.

### Controlled/uncontrolled behavior

- [x] Uncontrolled root initializes from `.default_value(...)`.
- [x] Uncontrolled text edits update internal `input_value`.
- [x] Uncontrolled parseable text edits update internal numeric `value`.
- [x] Uncontrolled empty text sets internal numeric `value` to `None`.
- [x] Controlled root reflects `.value(...)` from props.
- [x] Controlled user edits call `.on_value_change(...)` without treating internal numeric state as the source of truth.
- [x] Controlled empty value displays empty text.
- [x] Changing controlled `.value(...)` across renders updates displayed input text when allowed by controlled sync rules.
- [x] Re-rendering with changed unrelated props does not reset uncontrolled numeric value, input text, selection, or focus unless keyed `.id(...)` changes.
- [x] Programmatic/default value changes do not clobber focused uncontrolled edits unless explicitly controlled.

### Change / commit details

- [x] Add `NumberFieldChangeReason` with at least `InputChange`, `InputClear`, `InputBlur`, `Keyboard`, `IncrementPress`, `DecrementPress`, `Wheel`, `Scrub`, and `None`.
- [x] Add `NumberFieldCommitReason` with at least `InputBlur`, `InputClear`, `Keyboard`, `IncrementPress`, `DecrementPress`, `Wheel`, `Scrub`, and `None`.
- [x] Add Rust-native change details that expose reason and optional direction without DOM event objects.
- [x] Add Rust-native commit details that expose reason and optional direction without DOM event objects.
- [x] Call `on_value_change` exactly once per accepted numeric value change.
- [x] Do not call `on_value_change` when invalid intermediate text does not change the numeric value.
- [x] Call `on_value_committed` on blur after typing.
- [x] Call `on_value_committed` when pointer is released after increment/decrement press or scrub.
- [x] Keyboard and wheel stepping may commit immediately, matching Base UI semantics where practical.

### Input behavior

- [x] `NumberFieldInput` renders the generic `input()` primitive.
- [x] Text insertion remains handled by the generic input's platform input bridge, not raw printable key handlers.
- [x] Number-specific text changes are observed through `input()` value changes.
- [x] Input displays `input_value` from Number Field runtime.
- [x] Input receives disabled/read-only/required state from root.
- [x] Input focus updates root/input render state.
- [x] Input blur commits/parses/formats/clamps as specified.
- [x] Input supports `style_with_state(...)` with `NumberFieldInputRenderState`.

### Button / keyboard / wheel behavior

- [x] Clicking `NumberFieldIncrement` increments by `step`.
- [x] Clicking `NumberFieldDecrement` decrements by `step`.
- [x] Disabled increment/decrement controls do not change value.
- [x] Read-only increment/decrement controls do not change value.
- [x] ArrowUp increments by `step` while the input/root is focused.
- [x] ArrowDown decrements by `step` while the input/root is focused.
- [x] Shift+ArrowUp/ArrowDown uses `large_step`.
- [x] Alt+ArrowUp/ArrowDown uses `small_step`.
- [x] Home moves to `min` when `min` exists.
- [x] End moves to `max` when `max` exists.
- [x] Wheel stepping works only when `allow_wheel_scrub == true`, if implemented in phase 1.
- [x] Button and keyboard stepping share runtime stepping commands.

### Scrub behavior

- [x] `NumberFieldScrubArea` renders arbitrary children.
- [x] Dragging the scrub area changes value by step units according to pointer movement.
- [x] Horizontal scrubbing is supported.
- [x] Vertical scrubbing is either supported or explicitly deferred.
- [x] `pixel_sensitivity` controls how many pixels produce one step.
- [x] Scrubbing sets `scrubbing == true` in render state while active.
- [x] Releasing pointer after scrubbing commits the value.
- [x] `NumberFieldScrubAreaCursor` renders only while scrubbing, or exposes state so users can style it as hidden/visible.
- [x] Pointer lock and cursor teleport behavior are not required in phase 1.

### Field integration

- [x] Number Field rendered inside `FieldRoot` consumes root disabled state.
- [x] Number Field rendered inside disabled `FieldItem` is disabled.
- [x] Number Field registers a `FieldControlRegistration` with a stable key.
- [x] Registration includes current value metadata compatible with Field validation.
- [x] Registration includes `name` metadata, with `FieldRoot::name(...)` continuing to act as future Form override.
- [x] Registration includes `required` metadata.
- [x] Registration includes disabled metadata after root/item/root-prop merging.
- [x] Registration includes focused state.
- [x] Registration includes a focus handle so `FieldLabel` click focuses the Number Field input.
- [x] Field becomes filled when numeric value is `Some(_)`.
- [x] Field becomes unfilled when numeric value is `None`.
- [x] Field becomes dirty when numeric value differs from its initial registered value.
- [x] Field becomes focused when the Number Field input is focused.
- [x] Field becomes touched when the Number Field input loses focus.
- [x] `FieldValidationMode::OnChange` validates when numeric value changes.
- [x] `FieldValidationMode::OnBlur` validates when the input blurs.
- [x] Required-only Field validation reports `value_missing` when a required Number Field is empty.

### Styling/state exposure

- [x] Add `NumberFieldRootRenderState`.
- [x] Add `NumberFieldInputRenderState`.
- [x] Add `NumberFieldGroupRenderState`.
- [x] Add `NumberFieldIncrementRenderState`.
- [x] Add `NumberFieldDecrementRenderState`.
- [x] Add `NumberFieldScrubAreaRenderState`.
- [x] Add `NumberFieldScrubAreaCursorRenderState`.
- [x] Render states expose `value`, `input_value`, `disabled`, `read_only`, `required`, `scrubbing`, `touched`, `dirty`, `valid`, `invalid`, `filled`, and `focused` where relevant.
- [x] Stepper render states expose whether the control can increment/decrement at the current boundary.
- [x] `style_with_state(...)` receives typed state for every public layer.
- [x] Do not expose DOM data attributes as styling API.
- [x] Do not expose CSS variable names as styling API.

### Docs/demo

- [x] Update `crates/base_gpui/src/main.rs` or add an example rendering a Number Field with input and increment/decrement controls.
- [x] Add a demo of `FieldRoot` + `FieldLabel` + `NumberFieldRoot` + `NumberFieldInput` + `FieldError`.
- [x] Document the supported phase-1 prop subset and explicitly list deferred Base UI props/features.
- [x] Document the ASCII parser/formatter limitations and locale follow-ups.

### Tests / verification

Add behavior tests under `crates/base_gpui/src/number_field/tests/` where practical.

- [x] Default value is empty/`None`.
- [x] `default_value(Some(...))` initializes numeric value and input text.
- [x] Controlled `value(Some(...))` is displayed.
- [x] Controlled `value(None)` displays empty text.
- [x] User typing a parseable integer updates uncontrolled numeric value.
- [x] User typing a parseable decimal updates uncontrolled numeric value.
- [x] User clearing the input sets uncontrolled numeric value to `None`.
- [x] Controlled user edits call `on_value_change` without mutating source of truth.
- [x] Invalid intermediate text remains visible while focused.
- [x] Blur resolves invalid intermediate text.
- [x] Blur clamps to min/max when `allow_out_of_range == false`.
- [x] Direct text entry outside min/max is allowed while focused when `allow_out_of_range == true`.
- [x] Increment button increments by step.
- [x] Decrement button decrements by step.
- [x] Increment/decrement clamp at min/max.
- [x] Shift+Arrow uses large step.
- [x] Alt+Arrow uses small step.
- [x] Home moves to min when min is set.
- [x] End moves to max when max is set.
- [x] `snap_on_step` snaps to the step grid.
- [x] Disabled Number Field ignores typing, buttons, keyboard stepping, wheel, and scrub.
- [x] Read-only Number Field can focus/select text but ignores value-changing interactions.
- [x] `on_value_committed` fires on blur after typing.
- [x] Scrub drag changes value.
- [x] Scrub release commits value.
- [x] Render states expose disabled/read-only/required/focused/dirty/touched/filled/scrubbing.
- [x] Field label click focuses the Number Field input.
- [x] Field filled/dirty/focused/touched states update from Number Field registration.
- [x] Field required validation reports missing value for an empty required Number Field.
- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui number_field` passes.
- [x] `cargo test -p base_gpui` passes.
- [x] `cargo clippy -p base_gpui --all-targets` passes without `pub_with_shorthand` violations.
- [x] `ast-grep scan` passes.

## Follow-ups

- Locale-aware formatting and parsing.
- Unicode numerals, fullwidth punctuation, Arabic/Persian/Han numerals.
- Currency, unit, percent, per-mille, grouping separators, and notation options.
- Full browser-like numeric validity parity: `badInput`, `rangeUnderflow`, `rangeOverflow`, `stepMismatch`.
- Pointer lock and cursor teleport behavior for scrub area.
- Native form submission / hidden input equivalents if a GPUI Form primitive is added.
- AccessKit/accessibility wiring once the project revisits GPUI accessibility.
