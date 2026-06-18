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

- [ ] Add a `number_field` module and export it from `crates/base_gpui/src/lib.rs`.
- [ ] Register Number Field key bindings from `base_gpui::init(cx)`.
- [ ] Add public `NumberFieldRoot`, `NumberFieldInput`, `NumberFieldGroup`, `NumberFieldIncrement`, `NumberFieldDecrement`, `NumberFieldScrubArea`, and `NumberFieldScrubAreaCursor` layer types.
- [ ] Add a typed `NumberFieldChild` enum for root children.
- [ ] Add typed child routing for `NumberFieldGroup` children.
- [ ] Support root `.id(...)` as the stable keyed identity.
- [ ] Support `.name(...)` metadata.
- [ ] Support uncontrolled `.default_value(Option<f64>)`.
- [ ] Support controlled `.value(Option<f64>)`, including controlled empty value.
- [ ] Support `.on_value_change(...)` with Rust-native change details.
- [ ] Support `.on_value_committed(...)` with Rust-native commit details.
- [ ] Support `.min(f64)` and `.max(f64)`.
- [ ] Support `.step(f64)`, defaulting to `1.0`.
- [ ] Support a `step_any` equivalent where interactive stepping still uses `1.0` but step mismatch validation is disabled/deferred.
- [ ] Support `.small_step(f64)`, defaulting to `0.1`.
- [ ] Support `.large_step(f64)`, defaulting to `10.0`.
- [ ] Support `.snap_on_step(bool)`, defaulting to `false`.
- [ ] Support `.allow_out_of_range(bool)`, defaulting to `false`.
- [ ] Support `.allow_wheel_scrub(bool)`, defaulting to `false`, or explicitly defer wheel scrub with tests documenting no-op behavior.
- [ ] Support `.disabled(bool)`, defaulting to `false`.
- [ ] Support `.read_only(bool)`, defaulting to `false`.
- [ ] Support `.required(bool)`, defaulting to `false`.
- [ ] Preserve `.form(...)` only as metadata for future form integration if useful; do not implement DOM form ownership.
- [ ] Expose ergonomic barrel exports from `number_field/mod.rs`.
- [ ] Do not expose CSS class/data-attribute APIs.

### Runtime / architecture

- [ ] Add `NumberFieldRuntime` as the owner of numeric value, input text value, dirty/touched/focused facts, scrubbing state, and pending commit state.
- [ ] Add `NumberFieldProps` for stable root props and callbacks.
- [ ] Add `NumberFieldContext` as thin read/update/command plumbing.
- [ ] Keep parsing, validation, clamping, stepping, and value-change decisions in runtime/helpers, not in visual layers.
- [ ] Keep layers thin: user events translate into runtime commands, runtime state feeds styling/painting.
- [ ] Use the generic `input()` primitive inside `NumberFieldInput`.
- [ ] Do not make the generic input primitive know about Number Field.
- [ ] Do not introduce generic helpers unless they hide a repeated deep concept across components.
- [ ] Avoid large dependencies for parsing/formatting; prefer small Rust-native helpers first.
- [ ] Do not use `pub(crate)`, `pub(super)`, or any other `pub(...)` scoped visibility syntax.

### Numeric parsing / formatting / validation

- [ ] Empty text parses to `None` and marks the field unfilled.
- [ ] ASCII integer text parses to `Some(f64)`.
- [ ] ASCII decimal text parses to `Some(f64)`.
- [ ] Leading `+` and `-` are handled.
- [ ] Surrounding whitespace is ignored for parsing.
- [ ] Non-finite values are rejected.
- [ ] Intermediate invalid text can remain in `input_value` while focused without immediately clobbering user input.
- [ ] Blur/commit resolves invalid/intermediate text to the formatted current numeric value or empty text when `value == None`.
- [ ] Blur/commit clamps to `min`/`max` when `allow_out_of_range == false`.
- [ ] Direct text entry may remain outside `min`/`max` while focused when `allow_out_of_range == true`.
- [ ] Step-based interactions always clamp to `min`/`max`.
- [ ] `snap_on_step` snaps stepping values to the configured step grid.
- [ ] Floating-point stepping noise is cleaned enough for common cases such as `0.1 + 0.2`.
- [ ] Parser/formatter helpers have focused unit tests.

### Controlled/uncontrolled behavior

- [ ] Uncontrolled root initializes from `.default_value(...)`.
- [ ] Uncontrolled text edits update internal `input_value`.
- [ ] Uncontrolled parseable text edits update internal numeric `value`.
- [ ] Uncontrolled empty text sets internal numeric `value` to `None`.
- [ ] Controlled root reflects `.value(...)` from props.
- [ ] Controlled user edits call `.on_value_change(...)` without treating internal numeric state as the source of truth.
- [ ] Controlled empty value displays empty text.
- [ ] Changing controlled `.value(...)` across renders updates displayed input text when allowed by controlled sync rules.
- [ ] Re-rendering with changed unrelated props does not reset uncontrolled numeric value, input text, selection, or focus unless keyed `.id(...)` changes.
- [ ] Programmatic/default value changes do not clobber focused uncontrolled edits unless explicitly controlled.

### Change / commit details

- [ ] Add `NumberFieldChangeReason` with at least `InputChange`, `InputClear`, `InputBlur`, `Keyboard`, `IncrementPress`, `DecrementPress`, `Wheel`, `Scrub`, and `None`.
- [ ] Add `NumberFieldCommitReason` with at least `InputBlur`, `InputClear`, `Keyboard`, `IncrementPress`, `DecrementPress`, `Wheel`, `Scrub`, and `None`.
- [ ] Add Rust-native change details that expose reason and optional direction without DOM event objects.
- [ ] Add Rust-native commit details that expose reason and optional direction without DOM event objects.
- [ ] Call `on_value_change` exactly once per accepted numeric value change.
- [ ] Do not call `on_value_change` when invalid intermediate text does not change the numeric value.
- [ ] Call `on_value_committed` on blur after typing.
- [ ] Call `on_value_committed` when pointer is released after increment/decrement press or scrub.
- [ ] Keyboard and wheel stepping may commit immediately, matching Base UI semantics where practical.

### Input behavior

- [ ] `NumberFieldInput` renders the generic `input()` primitive.
- [ ] Text insertion remains handled by the generic input's platform input bridge, not raw printable key handlers.
- [ ] Number-specific text changes are observed through `input()` value changes.
- [ ] Input displays `input_value` from Number Field runtime.
- [ ] Input receives disabled/read-only/required state from root.
- [ ] Input focus updates root/input render state.
- [ ] Input blur commits/parses/formats/clamps as specified.
- [ ] Input supports `style_with_state(...)` with `NumberFieldInputRenderState`.

### Button / keyboard / wheel behavior

- [ ] Clicking `NumberFieldIncrement` increments by `step`.
- [ ] Clicking `NumberFieldDecrement` decrements by `step`.
- [ ] Disabled increment/decrement controls do not change value.
- [ ] Read-only increment/decrement controls do not change value.
- [ ] ArrowUp increments by `step` while the input/root is focused.
- [ ] ArrowDown decrements by `step` while the input/root is focused.
- [ ] Shift+ArrowUp/ArrowDown uses `large_step`.
- [ ] Alt+ArrowUp/ArrowDown uses `small_step`.
- [ ] Home moves to `min` when `min` exists.
- [ ] End moves to `max` when `max` exists.
- [ ] Wheel stepping works only when `allow_wheel_scrub == true`, if implemented in phase 1.
- [ ] Button and keyboard stepping share runtime stepping commands.

### Scrub behavior

- [ ] `NumberFieldScrubArea` renders arbitrary children.
- [ ] Dragging the scrub area changes value by step units according to pointer movement.
- [ ] Horizontal scrubbing is supported.
- [ ] Vertical scrubbing is either supported or explicitly deferred.
- [ ] `pixel_sensitivity` controls how many pixels produce one step.
- [ ] Scrubbing sets `scrubbing == true` in render state while active.
- [ ] Releasing pointer after scrubbing commits the value.
- [ ] `NumberFieldScrubAreaCursor` renders only while scrubbing, or exposes state so users can style it as hidden/visible.
- [ ] Pointer lock and cursor teleport behavior are not required in phase 1.

### Field integration

- [ ] Number Field rendered inside `FieldRoot` consumes root disabled state.
- [ ] Number Field rendered inside disabled `FieldItem` is disabled.
- [ ] Number Field registers a `FieldControlRegistration` with a stable key.
- [ ] Registration includes current value metadata compatible with Field validation.
- [ ] Registration includes `name` metadata, with `FieldRoot::name(...)` continuing to act as future Form override.
- [ ] Registration includes `required` metadata.
- [ ] Registration includes disabled metadata after root/item/root-prop merging.
- [ ] Registration includes focused state.
- [ ] Registration includes a focus handle so `FieldLabel` click focuses the Number Field input.
- [ ] Field becomes filled when numeric value is `Some(_)`.
- [ ] Field becomes unfilled when numeric value is `None`.
- [ ] Field becomes dirty when numeric value differs from its initial registered value.
- [ ] Field becomes focused when the Number Field input is focused.
- [ ] Field becomes touched when the Number Field input loses focus.
- [ ] `FieldValidationMode::OnChange` validates when numeric value changes.
- [ ] `FieldValidationMode::OnBlur` validates when the input blurs.
- [ ] Required-only Field validation reports `value_missing` when a required Number Field is empty.

### Styling/state exposure

- [ ] Add `NumberFieldRootRenderState`.
- [ ] Add `NumberFieldInputRenderState`.
- [ ] Add `NumberFieldGroupRenderState`.
- [ ] Add `NumberFieldIncrementRenderState`.
- [ ] Add `NumberFieldDecrementRenderState`.
- [ ] Add `NumberFieldScrubAreaRenderState`.
- [ ] Add `NumberFieldScrubAreaCursorRenderState`.
- [ ] Render states expose `value`, `input_value`, `disabled`, `read_only`, `required`, `scrubbing`, `touched`, `dirty`, `valid`, `invalid`, `filled`, and `focused` where relevant.
- [ ] Stepper render states expose whether the control can increment/decrement at the current boundary.
- [ ] `style_with_state(...)` receives typed state for every public layer.
- [ ] Do not expose DOM data attributes as styling API.
- [ ] Do not expose CSS variable names as styling API.

### Docs/demo

- [ ] Update `crates/base_gpui/src/main.rs` or add an example rendering a Number Field with input and increment/decrement controls.
- [ ] Add a demo of `FieldRoot` + `FieldLabel` + `NumberFieldRoot` + `NumberFieldInput` + `FieldError`.
- [ ] Document the supported phase-1 prop subset and explicitly list deferred Base UI props/features.
- [ ] Document the ASCII parser/formatter limitations and locale follow-ups.

### Tests / verification

Add behavior tests under `crates/base_gpui/src/number_field/tests/` where practical.

- [ ] Default value is empty/`None`.
- [ ] `default_value(Some(...))` initializes numeric value and input text.
- [ ] Controlled `value(Some(...))` is displayed.
- [ ] Controlled `value(None)` displays empty text.
- [ ] User typing a parseable integer updates uncontrolled numeric value.
- [ ] User typing a parseable decimal updates uncontrolled numeric value.
- [ ] User clearing the input sets uncontrolled numeric value to `None`.
- [ ] Controlled user edits call `on_value_change` without mutating source of truth.
- [ ] Invalid intermediate text remains visible while focused.
- [ ] Blur resolves invalid intermediate text.
- [ ] Blur clamps to min/max when `allow_out_of_range == false`.
- [ ] Direct text entry outside min/max is allowed while focused when `allow_out_of_range == true`.
- [ ] Increment button increments by step.
- [ ] Decrement button decrements by step.
- [ ] Increment/decrement clamp at min/max.
- [ ] Shift+Arrow uses large step.
- [ ] Alt+Arrow uses small step.
- [ ] Home moves to min when min is set.
- [ ] End moves to max when max is set.
- [ ] `snap_on_step` snaps to the step grid.
- [ ] Disabled Number Field ignores typing, buttons, keyboard stepping, wheel, and scrub.
- [ ] Read-only Number Field can focus/select text but ignores value-changing interactions.
- [ ] `on_value_committed` fires on blur after typing.
- [ ] Scrub drag changes value.
- [ ] Scrub release commits value.
- [ ] Render states expose disabled/read-only/required/focused/dirty/touched/filled/scrubbing.
- [ ] Field label click focuses the Number Field input.
- [ ] Field filled/dirty/focused/touched states update from Number Field registration.
- [ ] Field required validation reports missing value for an empty required Number Field.
- [ ] `cargo check -p base_gpui` passes.
- [ ] `cargo test -p base_gpui number_field` passes.
- [ ] `cargo test -p base_gpui` passes.
- [ ] `cargo clippy -p base_gpui --all-targets` passes without `pub_with_shorthand` violations.
- [ ] `ast-grep scan` passes.

## Follow-ups

- Locale-aware formatting and parsing.
- Unicode numerals, fullwidth punctuation, Arabic/Persian/Han numerals.
- Currency, unit, percent, per-mille, grouping separators, and notation options.
- Full browser-like numeric validity parity: `badInput`, `rangeUnderflow`, `rangeOverflow`, `stepMismatch`.
- Pointer lock and cursor teleport behavior for scrub area.
- Native form submission / hidden input equivalents if a GPUI Form primitive is added.
- AccessKit/accessibility wiring once the project revisits GPUI accessibility.
