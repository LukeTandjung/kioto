# Port Base UI Number Field to GPUI

## Problem

Base UI Number Field provides a numeric text input with controlled/uncontrolled numeric value, parse/format behavior, increment/decrement controls, keyboard stepping, optional scrubbing, disabled/read-only/required state, and Field integration.

`crates/base_gpui` now has a generic GPUI-native `input()` primitive and a Field component family. Number Field should be the first higher-level text-entry component built on top of `input()`, without making `input()` special-purpose. The goal is to port the useful Base UI Number Field behavior into GPUI-native components while preserving the local architecture: deep runtime/state module, thin layers, typed style state, and no DOM/React leakage.

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

Required-only Field validation should treat `None` / empty input as missing. Full numeric validity flags (`rangeUnderflow`, `rangeOverflow`, `stepMismatch`, `badInput`) can be represented in Number Field style state first and wired into Field validity only if the existing Field model supports it cleanly.

## Expected implementation files

```text
crates/base_gpui/src/number_field/mod.rs
crates/base_gpui/src/number_field/actions.rs
crates/base_gpui/src/number_field/child.rs
crates/base_gpui/src/number_field/child_wiring.rs
crates/base_gpui/src/number_field/context.rs
crates/base_gpui/src/number_field/props.rs
crates/base_gpui/src/number_field/style_state.rs
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

Alternative filenames are fine if they preserve the same architecture: deep runtime/state, thin layers, typed child wiring, typed style state, and isolated number parsing/validation helpers.

## Out of scope / drop from Base UI

- Do not port React hooks/context directly.
- Do not port `className`, web `style`, `render` props, or DOM data attributes; expose typed style-state structs and `style_with_state(...)`.
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
- [x] Input focus updates root/input style state.
- [x] Input blur commits/parses/formats/clamps as specified.
- [x] Input supports `style_with_state(...)` with `NumberFieldInputStyleState`.

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
- [x] Scrubbing sets `scrubbing == true` in style state while active.
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

- [x] Add `NumberFieldRootStyleState`.
- [x] Add `NumberFieldInputStyleState`.
- [x] Add `NumberFieldGroupStyleState`.
- [x] Add `NumberFieldIncrementStyleState`.
- [x] Add `NumberFieldDecrementStyleState`.
- [x] Add `NumberFieldScrubAreaStyleState`.
- [x] Add `NumberFieldScrubAreaCursorStyleState`.
- [x] Style states expose `value`, `input_value`, `disabled`, `read_only`, `required`, `scrubbing`, `touched`, `dirty`, `valid`, `invalid`, `filled`, and `focused` where relevant.
- [x] Stepper style states expose whether the control can increment/decrement at the current boundary.
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
- [x] Style states expose disabled/read-only/required/focused/dirty/touched/filled/scrubbing.
- [x] Field label click focuses the Number Field input.
- [x] Field filled/dirty/focused/touched states update from Number Field registration.
- [x] Field required validation reports missing value for an empty required Number Field.
- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui number_field` passes.
- [x] `cargo test -p base_gpui` passes.
- [x] `cargo clippy -p base_gpui --all-targets` passes without `pub_with_shorthand` violations.
- [x] `ast-grep scan` passes.

## AccessKit accessibility follow-up

Scope: wire the existing `number_field` layers into GPUI's AccessKit tree using only the APIs available in the pinned gpui revision (see `docs/accesskit-gpui-reference.md`). A node is reported only when it has both `.id(...)` and `.role(...)`; all layers already have stable keyed ids via `context.child_id(...)` / user `.id(...)`, so this work is additive on the existing render paths.

### Per accessible part

- `NumberFieldInput` (`layers/number_field_input.rs`): the accessible control. Base UI renders a text `<input inputMode="numeric">`; the closest AccessKit mapping in this revision is `Role::SpinButton` (matching `gpui/examples/a11y.rs`). On the input's rendered root element set:
  - `.role(Role::SpinButton)`;
  - `.aria_numeric_value(v)` from `NumberFieldRootStyleState::value` when it is `Some(v)` (the style state is already read in `render` via `context.read(cx, |runtime, props| runtime.input_state(props))`);
  - `.aria_min_numeric_value(...)` / `.aria_max_numeric_value(...)` from `NumberFieldProps::min()` / `NumberFieldProps::max()` when set;
  - `.aria_label(...)` from a new `NumberFieldInput::aria_label(...)` builder prop (see Labels below).
  - The generic `input()` primitive currently exposes no role/aria pass-through, so `Input` needs small forwarding builders (`role`, `aria_label`, numeric-value setters) applied to its stateful root element. Keep them generic on `Input`; Number Field remains the caller that decides the role.
- `NumberFieldIncrement` / `NumberFieldDecrement` (`layers/number_field_increment.rs`, `layers/number_field_decrement.rs`): `.role(Role::Button)` on the existing `base.id(id)` element, plus `.aria_label("Increase")` / `.aria_label("Decrease")` defaults (mirroring `useNumberFieldButton.ts`), overridable via a new `.aria_label(...)` builder prop.
- `NumberFieldGroup` (`layers/number_field_group.rs`): `.role(Role::Group)` on its root element, matching Base UI's `role="group"`.
- `NumberFieldRoot`, `NumberFieldScrubArea`, `NumberFieldScrubAreaCursor`: **no roles**. Base UI's root is a plain `div`, and the scrub parts are `role="presentation"` / `aria-hidden`. In gpui, simply not assigning a role keeps them out of the a11y tree, which is the correct equivalent — no work needed beyond a code comment.

### Actions

- `NumberFieldInput`: add
  - `.on_a11y_action(AccessibleAction::Increment, ...)` → `context.step(NumberFieldStepDirection::Up, NumberFieldStepAmount::Normal, NumberFieldChangeReason::Keyboard, NumberFieldCommitReason::Keyboard, window, cx)`;
  - `.on_a11y_action(AccessibleAction::Decrement, ...)` → same with `NumberFieldStepDirection::Down`.
  These route into the exact runtime command the ArrowUp/ArrowDown keyboard path already uses; do not add a parallel value path. `Action::SetValue` is optional; if added, route through `context.input_changed(...)` + `context.commit(NumberFieldCommitReason::Keyboard, ...)` so parse/clamp/commit rules are reused.
- `NumberFieldIncrement` / `NumberFieldDecrement`: **nothing to add** — `Action::Click` is auto-registered by the existing `.on_click(...)` handlers, which already call `context.step(...)`. Note: those handlers early-return unless `matches!(event, ClickEvent::Mouse(_))`; verify AT-dispatched clicks are not filtered out by that guard, and relax it if they are.
- Focus: `Action::Focus` is auto-registered wherever the focus handle is tracked (the input primitive already wires `context.focus_handle()`); do not re-add.

### Labels

- The input's accessible name comes from a literal `.aria_label(...)` string (new builder prop on `NumberFieldInput`, forwarded through `Input`). There is no `aria-labelledby`, so `FieldLabel` cannot name the control by id-reference; callers must pass the label text explicitly. Document this on the builder.
- Stepper button children are caller-supplied (e.g. `.child("-")` / `.child("+")`); since the buttons carry `.aria_label(...)`, document that visible glyph children should use `Text::new_inaccessible(...)` (not `text!(...)`) to avoid double-announcing, and use it in the gallery/demo.
- `NumberFieldScrubArea` label text (e.g. `.child("Amount")` in the demo) is presentational in Base UI; keep it inaccessible or leave it as plain text outside the a11y tree (scrub area has no role, so its subtree only leaks if children carry ids + roles).

### Gaps (no gpui builder in this revision)

- `aria-roledescription: "Number field"` (`NumberFieldInput.tsx`): no builder. Fallback: omit; `Role::SpinButton` already conveys the numeric-stepper semantics.
- `aria-invalid` (input, when Field validation fails): no builder. Fallback: omit + document; track as blocked pending gpui upstream.
- `aria-labelledby: labelId` (input ← `FieldLabel`): no id-reference wiring. Fallback: literal `.aria_label(...)` as above.
- `aria-controls: id` (increment/decrement → input, `useNumberFieldButton.ts`): no builder. Fallback: omit + document; group + labels make the relationship inferable.
- `disabled` / `aria-disabled` and `required`: no `.aria_disabled(...)` / `.aria_required(...)` builders and `write_a11y_info` never sets a disabled flag. Fallback: the runtime already no-ops stepping/typing when `NumberFieldProps::disabled()` / `read_only()` is true, so AT actions are inert; document that disabled state is not *announced*, blocked pending gpui upstream.
- `aria-readonly`: not applicable — Base UI itself deliberately omits it on the input (`useNumberFieldStepperButton.ts` comment); nothing to do.
- `aria-hidden` on the root's scrub cursor markup: no builder needed — omit role/id instead.

### Checklist

- [ ] Add generic `role` / `aria_label` / numeric-value forwarding builders to the `input()` primitive root element (no Number Field knowledge in `Input`).
- [ ] `NumberFieldInput` sets `Role::SpinButton`, `.aria_numeric_value` from runtime `value`, `.aria_min_numeric_value` / `.aria_max_numeric_value` from `NumberFieldProps::min()` / `max()`.
- [ ] `NumberFieldInput` exposes `.aria_label(...)` and documents the missing `aria-labelledby` wiring.
- [ ] `NumberFieldInput` registers `AccessibleAction::Increment` / `Decrement` handlers routed through `context.step(...)`.
- [ ] `NumberFieldIncrement` / `NumberFieldDecrement` set `Role::Button` with default `"Increase"` / `"Decrease"` `.aria_label(...)`, overridable.
- [ ] Verify AT-dispatched `Action::Click` on the stepper buttons is not dropped by the `ClickEvent::Mouse(_)` guard in their `on_click` handlers.
- [ ] `NumberFieldGroup` sets `Role::Group`.
- [ ] `NumberFieldRoot` / `NumberFieldScrubArea` / `NumberFieldScrubAreaCursor` stay roleless, with a comment mapping this to Base UI's `role="presentation"` / `aria-hidden`.
- [ ] Demo/gallery stepper glyphs use `Text::new_inaccessible(...)`.
- [ ] Do not re-register `Action::Click` / `Action::Focus` where `on_click` / focus-handle tracking already exists.
- [ ] Document the gaps above (`aria-invalid`, `aria-controls`, `aria-labelledby`, disabled/required announcement, `aria-roledescription`) in the module docs as blocked pending gpui upstream.

## Follow-ups

- Locale-aware formatting and parsing.
- Unicode numerals, fullwidth punctuation, Arabic/Persian/Han numerals.
- Currency, unit, percent, per-mille, grouping separators, and notation options.
- Full browser-like numeric validity parity: `badInput`, `rangeUnderflow`, `rangeOverflow`, `stepMismatch`.
- Pointer lock and cursor teleport behavior for scrub area.
- Native form submission / hidden input equivalents if a GPUI Form primitive is added.
- AccessKit/accessibility wiring: see the `## AccessKit accessibility follow-up` section above.
