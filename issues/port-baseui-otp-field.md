# Port Base UI OTP Field to GPUI

## Problem

Base UI OTP Field renders a group of one-character slots that together edit a single OTP string. The root owns the whole value (controlled/uncontrolled), normalizes it (whitespace stripping, `validationType` character filtering, optional custom normalization, clamping to `length` by Unicode code points), tracks the focused/active slot, fires `onValueChange` / `onValueComplete` / `onValueInvalid`, and integrates with Field and Form. Each slot input handles roving focus, arrow/Home/End navigation, Backspace/Delete editing, same-character overtype advancement, and forward distribution of typed and pasted characters across slots.

`crates/base_gpui` has no OTP Field. It does have everything needed to build one GPUI-natively: the generic `input()` primitive whose `InputRuntime` provides the platform text-input bridge (`EntityInputHandler`, IME marking, clipboard, UTF-16/UTF-8 range mapping, injectable `FocusHandle`, controlled value sync), the `field` family with `FieldControlRegistration`, the `form` family with programmatic `FormContext::submit(...)`, and the ported `separator`.

The goal is behavioral parity with Base UI OTP Field using GPUI-native architecture: one deep runtime owning the single OTP string and the virtual active slot, ONE group `FocusHandle` shared with `InputRuntime` (no per-slot focus handles, no roving tabindex), thin slot parts, typed style state, and no DOM/React leakage. Complexity: medium.

## Scope

Add an `otp_field` component family under `crates/base_gpui/src/otp_field/`:

- `OTPFieldRoot`
- `OTPFieldInput` (one per character slot)
- `Separator` re-exported from the already-ported `separator` component, matching Base UI's `index.parts.ts`

The first implementation should support:

- a single OTP string value, controlled/uncontrolled, with `on_value_change`, `on_value_complete`, and `on_value_invalid` callbacks;
- value normalization: whitespace stripping, `validation_type` filtering (`numeric` / `alpha` / `alphanumeric` / `none`), an optional `normalize_value` hook, and clamping to `length` by Unicode code points;
- a virtual active slot with pointer and keyboard navigation over one group focus handle;
- typed-character and paste distribution forward from the active slot;
- `mask` rendering, `disabled` / `read_only` / `required` state;
- Field integration through `FieldControlRegistration` registered once at the root;
- `auto_submit` through `FormContext::submit(...)` when the value becomes complete inside a `FormRoot`.

Text entry MUST reuse the existing `primitives/input` `InputRuntime` as the platform input bridge. Do NOT add a new primitive. The segmented display (N slot boxes, blinking caret in the active slot, per-slot bounds for IME candidate placement) is a component-local custom element under `otp_field/layers/`, following the precedent of `crates/base_gpui/src/number_field/layers/number_field_input.rs` composing the input primitive.

## Base UI references

Documentation:

- `https://base-ui.com/react/components/otp-field.md`

Source:

- `/home/luke/Projects/base-ui/packages/react/src/otp-field/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/otp-field/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/otp-field/root/OTPFieldRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/otp-field/root/OTPFieldRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/otp-field/root/OTPFieldRootDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/otp-field/root/OTPFieldRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/otp-field/root/OTPFieldRoot.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/otp-field/input/OTPFieldInput.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/otp-field/input/OTPFieldInputDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/otp-field/input/OTPFieldInput.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/otp-field/input/OTPFieldInput.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/otp-field/utils/otp.ts`
- `/home/luke/Projects/base-ui/packages/react/src/otp-field/utils/otp.test.ts`
- `/home/luke/Projects/base-ui/packages/react/src/otp-field/utils/stateAttributesMapping.ts`

Current GPUI implementation: none exists under `crates/base_gpui/src/otp_field/`; this issue creates it.

Local GPUI references:

- `crates/base_gpui/src/primitives/input/runtime.rs` (`InputRuntime`: `new_with_focus_handle`, `sync_props`, `EntityInputHandler` impl, UTF-16 mapping, clipboard, `set_last_layout`)
- `crates/base_gpui/src/primitives/input/layers/text_element.rs` (`window.handle_input(...)` + `ElementInputHandler` during paint; IME candidate bounds)
- `crates/base_gpui/src/number_field/layers/number_field_input.rs` (precedent: component wraps `input()` with injected focus handle, controlled value, and value-change interception)
- `crates/base_gpui/src/field/` (`layers/field_control.rs`, `context.rs`, `runtime.rs`, `validation.rs`)
- `crates/base_gpui/src/form/` (`context.rs::submit(...)`, `submit.rs`)
- `crates/base_gpui/src/separator/`
- `issues/add-gpui-input-primitive.md` (input primitive scope and out-of-scope decisions)
- `issues/port-baseui-number-field.md` (sibling issue shape)
- `docs/base-gpui-component-architecture.md`
- `/home/luke/Projects/gpui-component/crates/ui/src/input/otp_input.rs` (design reference for single-value + virtual-slot rendering only; do NOT copy its raw `on_key_down` character insertion — printable input must flow through the platform input handler per the input primitive's rules)

## Initial design decisions

### Placement and module layout

Put OTP Field under `crates/base_gpui/src/otp_field/` with the flat per-component layout from `docs/base-gpui-component-architecture.md`: `runtime.rs` (deep), `context.rs` (thin), `props.rs`, `style_state.rs`, `child.rs` (+ private `child_wiring.rs`), `actions.rs`, `layers/`, `tests/`. No nested `child/context/{props,runtime,state}/` taxonomy, no `utils/` folder; the pure normalization helpers live in `otp_field/otp.rs`.

### Single value, single focus handle, virtual active slot

Unlike Base UI's N DOM `<input>` elements with roving tabindex, the GPUI port uses:

- one OTP string value owned by `OTPFieldRuntime`;
- ONE group `FocusHandle` created by the root and injected into a keyed `InputRuntime` entity via `InputRuntime::new_with_focus_handle(...)`;
- a virtual `active_index` in `OTPFieldRuntime` replacing per-slot DOM focus. Base UI's `focusInput(i)` becomes the runtime command "set active index"; roving `tabIndex` disappears entirely — there is one tab stop for the whole OTP field.

Base UI's active-index derivation carries over: while focused, `active_index = min(focused_index, length - 1)`; while unfocused, the styling-active slot is `min(value_code_points, length - 1)`; attempting to activate a slot beyond the end of the value clamps to `min(value_code_points, length - 1)`.

### Reusing `InputRuntime` (no new primitive)

The root keeps a keyed `Entity<InputRuntime>` in controlled mode as the platform text-entry bridge:

- printable characters and IME composition arrive through `EntityInputHandler::replace_text_in_range` / `replace_and_mark_text_in_range` on `InputRuntime` — never raw `on_key_down` character handling;
- the OTP component observes edits through the input's value-change hook, normalizes/distributes into the OTP value with the `otp.rs` helpers, then re-syncs `InputRuntime` with the controlled normalized value via `sync_props`;
- character filtering, length clamping, and rejection detection are OTP-component logic on the controlled round-trip, NOT `InputRuntime` extensions.

Small `primitives/input` extensions ARE expected where `InputRuntime` lacks a hook; they are acceptance criteria of THIS issue (see "Input primitive extensions" below), not a separate primitive issue.

### Segmented display element

A component-local custom element (e.g. `otp_field/layers/otp_slot_element.rs`) draws each slot: the slot's character (or the mask character when `mask == true`), a blinking caret in the active slot while the group is focused, and records slot bounds. Exactly one platform input handler is installed per OTP field: during paint of the root/active-slot element, call `window.handle_input(&group_focus_handle, ElementInputHandler::new(bounds, input_runtime_entity), ...)` following `primitives/input/layers/text_element.rs`. IME candidate bounds must map to the active slot's bounds, not a single-line text layout.

### Keyboard dispatch

Define OTP actions in `otp_field/actions.rs` bound from `base_gpui::init(cx)` under an OTP-specific key context set on the root/group element (previous, next, first, last-of-value, backspace, clear-all, delete). Handlers translate into runtime commands via `OTPFieldContext`. Printable characters are NOT actions; they flow through the platform input handler.

### Value model and callbacks

- `value` / `default_value` are `SharedString`-like OTP strings; every incoming value (controlled, default, user edit) passes through `normalize_otp_value` before use, mirroring Base UI normalizing `valueUnwrapped` on every render.
- `on_value_change(value, details)` fires once per accepted normalized value change with a Rust-native reason enum (`InputChange`, `InputClear`, `InputPaste`, `Keyboard`); in controlled mode internal state is not the source of truth.
- `on_value_complete(value, details)` fires after the value update is applied when the normalized value reaches `length` code points and it previously was not complete, and ALSO when a paste produces a complete value equal to the already-complete current value (in that case `on_value_change` does not fire). Reasons: `InputChange`, `InputPaste`.
- `on_value_invalid(attempted_value, details)` fires when typed or pasted text loses characters to validation/normalization/clamping, with the raw attempted string. Reasons: `InputChange`, `InputPaste`.
- `normalize_value` is an optional `Fn(String) -> String` (documented as idempotent); its output is re-filtered by `validation_type` and clamped to `length`, and its rejections count toward `on_value_invalid`.
- Do not port Base UI's `isCanceled` event cancellation; callbacks are notifications, matching the existing `base_gpui` controlled/uncontrolled model.

### Field and Form integration

The root registers ONE `FieldControlRegistration` (stable key derived from root id) with `FieldValue::Text(value)`, name (with `FieldRoot::name(...)` as override), required, merged disabled, focused state, and the group focus handle so `FieldLabel` click focuses the OTP group — the GPUI translation of Base UI registering the control on the first slot. Filled/dirty/touched/focused and `OnChange`/`OnBlur` validation flow through the existing Field plumbing. `auto_submit` uses `FormContext::submit(FormSubmitReason::Programmatic, ...)` when a `FormContext` is present (programmatic submit exists in `crates/base_gpui/src/form/context.rs`, so `auto_submit` is in scope, not deferred).

### Input primitive extensions (specified here, not a separate issue)

Expected small extensions to `crates/base_gpui/src/primitives/input/`:

1. **Paste interception hook**: an optional `on_paste`-style handler on `InputRuntime` (shape like the existing `on_home` / `on_end` hooks: returns `bool` handled) receiving the raw clipboard text, so OTP can normalize, report rejections with reason `InputPaste`, and distribute across slots instead of the default single-line insertion.
2. **IME candidate bounds override**: a command such as `set_ime_candidate_bounds(Bounds<Pixels>)` that `EntityInputHandler::bounds_for_range` prefers over the single-shaped-line `last_layout` math, so the IME candidate window anchors to the active slot box.
3. **End-of-value cursor**: OTP forces the cursor to the end of the controlled value after every sync so `replace_text_in_range` inserts at the end. If the existing public surface (`sync_props` selection clamping plus the public `end(...)` command) suffices, no extension is needed; otherwise add a minimal public `move_cursor_to_end`-style command.

No char-filter or max-length extension: filtering and clamping are OTP-side via the controlled round-trip.

## Expected implementation files

```text
crates/base_gpui/src/otp_field/mod.rs
crates/base_gpui/src/otp_field/actions.rs
crates/base_gpui/src/otp_field/child.rs
crates/base_gpui/src/otp_field/child_wiring.rs
crates/base_gpui/src/otp_field/context.rs
crates/base_gpui/src/otp_field/props.rs
crates/base_gpui/src/otp_field/style_state.rs
crates/base_gpui/src/otp_field/runtime.rs
crates/base_gpui/src/otp_field/otp.rs
crates/base_gpui/src/otp_field/layers/mod.rs
crates/base_gpui/src/otp_field/layers/otp_field_root.rs
crates/base_gpui/src/otp_field/layers/otp_field_input.rs
crates/base_gpui/src/otp_field/layers/otp_slot_element.rs
crates/base_gpui/src/otp_field/tests/
```

Plus the small extensions inside `crates/base_gpui/src/primitives/input/runtime.rs` (and `props.rs` if a builder method is added). Alternative filenames are fine if they preserve the architecture: deep runtime, thin layers, typed child wiring, typed style state, isolated pure `otp.rs` helpers, component-local slot element.

## Out of scope / drop from Base UI

- Do not port React hooks/contexts, `useControlled`, `useIsoLayoutEffect`, `CompositeList`, or ref plumbing literally.
- Do not port `render` props, `className`, web `style`, DOM data attributes, or CSS variables; expose typed style-state structs and `style_with_state(...)`.
- Do not port the hidden validation `<input>`, `visuallyHidden` styles, browser form submission/`FormData`, or the `form` string-id attribute; Form integration goes through the GPUI `form` family.
- Do not port `autoComplete="one-time-code"` / OS autofill, password-manager `maxLength` tricks, `inputMode`, `enterKeyHint`, `autoCorrect`, or `spellCheck`.
- Do not port ARIA roles/attributes (`role="group"`, `aria-label(ledby)`, `aria-describedby`, `aria-invalid`, per-slot DOM id generation for labelling); revisit only via GPUI AccessKit when available.
- Do not port DOM event objects, `stopEvent`/propagation, or `isCanceled` cancellation semantics.
- Do not port roving `tabIndex` or per-slot DOM focus/`select()`; a single group focus handle with a virtual active slot replaces it.
- Do not implement RTL-aware arrow-key mirroring in phase 1; note it as a follow-up.
- Do not implement per-slot custom input `type` overrides beyond the boolean `mask`.
- Do not add a new text-entry primitive; reuse `InputRuntime` with the small extensions listed above.
- Do not handle printable character insertion with raw `on_key_down`; use the platform input handler / IME bridge.
- Do not use Rust scoped visibility syntax (`pub(...)`); `ast-grep scan` must remain clean.

## Acceptance Criteria

### Module/API surface

- [ ] Add an `otp_field` module and export it from `crates/base_gpui/src/lib.rs`.
- [ ] Register OTP Field key bindings from `base_gpui::init(cx)`.
- [ ] Add public `OTPFieldRoot` and `OTPFieldInput` layer types.
- [ ] Re-export the ported `Separator` from `otp_field/mod.rs`, matching Base UI's `index.parts.ts`.
- [ ] Add a typed `OTPFieldChild` enum routing `OTPFieldInput` and `Separator` root children.
- [ ] Support root `.id(...)` as the stable keyed identity; slot element ids derive from it.
- [ ] Support `.length(usize)` as a required prop.
- [ ] Support `.name(...)` metadata.
- [ ] Support uncontrolled `.default_value(...)`.
- [ ] Support controlled `.value(...)`.
- [ ] Support `.on_value_change(...)` with Rust-native change details (reason enum, no DOM events).
- [ ] Support `.on_value_complete(...)` with Rust-native complete details.
- [ ] Support `.on_value_invalid(...)` with Rust-native invalid details and the raw attempted string.
- [ ] Support `.validation_type(...)` with `Numeric` (default), `Alpha`, `Alphanumeric`, `None` variants.
- [ ] Support `.normalize_value(...)` as an optional `Fn(String) -> String` hook.
- [ ] Support `.mask(bool)`, defaulting to `false`.
- [ ] Support `.auto_submit(bool)`, defaulting to `false`.
- [ ] Support `.disabled(bool)`, `.read_only(bool)`, `.required(bool)`, each defaulting to `false`.
- [ ] Support `style_with_state(...)` on root and slot with typed style states.
- [ ] Expose ergonomic barrel exports from `otp_field/mod.rs`; `mod.rs` files contain only module declarations and re-exports.
- [ ] Do not expose `className`, web style props, CSS variables, or DOM data attributes.

### Correctness / compile readiness

- [ ] `cargo check -p base_gpui` passes.
- [ ] `cargo test -p base_gpui otp` passes.
- [ ] `cargo test -p base_gpui` passes.
- [ ] `cargo clippy -p base_gpui --all-targets` passes.
- [ ] `ast-grep scan` passes; no `pub(crate)`, `pub(super)`, or other `pub(...)` scoped visibility anywhere in the new code.
- [ ] `crates/base_gpui/src/main.rs` (or an example) renders an OTP Field demo, including a `FieldRoot` + `FieldLabel` + `OTPFieldRoot` + slots + `Separator` + `FieldError` composition.

### Architecture / internal model

- [ ] Add `OTPFieldRuntime` as the single deep module owning: the normalized OTP value (uncontrolled), `length`, active/focused slot index, focused/touched/dirty facts, per-slot bounds, and pending complete/focus intents; plain `&mut self` methods, unit-testable without a window.
- [ ] Runtime interface is commands + queries only: no getter/setter pairs; slot parts ask slot-shaped questions (e.g. `input_state(index)`) and emit event-shaped commands.
- [ ] Add `OTPFieldProps` for stable props/callbacks and `OTPFieldContext` as thin `read`/`update`/value-command plumbing; the controlled/uncontrolled rule lives only in the context's value-changing method and the `reconcile` input.
- [ ] The root owns ONE group `FocusHandle` and injects it into a keyed `Entity<InputRuntime>` via `InputRuntime::new_with_focus_handle(...)`; no per-slot focus handles, no roving tab stops.
- [ ] `InputRuntime` runs in controlled mode: OTP normalizes on the value-change hook and re-syncs the normalized value through `sync_props`; the cursor is kept at the end of the value so platform insertions append.
- [ ] Printable characters and IME composition flow exclusively through `InputRuntime`'s `EntityInputHandler` (`replace_text_in_range` / `replace_and_mark_text_in_range`); no raw `on_key_down` character insertion.
- [ ] Exactly one platform input handler is installed per OTP field, via `window.handle_input(...)` with `ElementInputHandler` during the segmented element's paint, gated on the group focus handle.
- [ ] The segmented display is a component-local custom element under `otp_field/layers/` that paints slot characters (or mask characters), a blinking caret in the active slot while focused, and records slot bounds; it is not a new primitive.
- [ ] Add the `primitives/input` paste-interception hook (`on_paste`-style, returns handled `bool`) and route OTP paste through it.
- [ ] Add the `primitives/input` IME-candidate-bounds override so `bounds_for_range` reports the active slot's bounds; the IME candidate window anchors to the active slot.
- [ ] Add a public end-of-value cursor command to `InputRuntime` only if the existing public surface cannot force the cursor to the end after controlled sync; document the decision in the code.
- [ ] Input primitive extensions stay OTP-agnostic: `primitives/input` must not import or know about `otp_field` or `field`.
- [ ] Child wiring assigns slot indices in one private `child_wiring.rs` pass; no index counters threaded through parts; emit a debug warning when the number of rendered `OTPFieldInput` children differs from `length`, and when `length` is zero.
- [ ] Keyboard behavior uses `actions.rs` + an OTP-specific key context + `on_action(...)`, not raw key matching in layers.
- [ ] Pure value helpers live in `otp_field/otp.rs` as plain functions (no GPUI types): whitespace stripping, validation-type filtering, normalize-with-rejection-details, replace-at-index, remove-at-index.

### Value normalization / validation type / clamping

- [ ] Whitespace (all Unicode whitespace) is stripped from every incoming value.
- [ ] `Numeric` filters to ASCII digits; `Alpha` filters to ASCII letters; `Alphanumeric` filters to ASCII letters and digits; `None` applies no character filter.
- [ ] The `normalize_value` hook runs after whitespace/validation filtering; its output is filtered by `validation_type` again, then clamped.
- [ ] Values are clamped to `length` by Unicode code points, never splitting a multi-byte character across the clamp boundary.
- [ ] Normalization reports whether characters were rejected at any stage (validation filter, custom hook shrinkage, post-hook re-filter, clamping) for `on_value_invalid`.
- [ ] Controlled `.value(...)` and `.default_value(...)` are normalized before display, exactly like user edits.
- [ ] Replace-at-index normalizes the inserted fragment, splices prefix/insert/suffix, and re-normalizes the final value so paste and multi-character edits stay contiguous and length-bounded.
- [ ] Remove-at-index removes exactly one code point and leaves the value unchanged for out-of-bounds indices.
- [ ] A non-positive or missing `length` renders no editable slots and warns in debug builds instead of panicking.

### Controlled/uncontrolled behavior

- [ ] Uncontrolled root initializes from normalized `.default_value(...)` and updates internal state on accepted edits.
- [ ] Controlled root reflects normalized `.value(...)` from props; user edits call `.on_value_change(...)` without treating internal state as the source of truth.
- [ ] Changing controlled `.value(...)` across renders updates the displayed slots and the unfocused active-slot derivation.
- [ ] Re-rendering with changed unrelated props does not reset uncontrolled value, active index, or focus unless the keyed `.id(...)` changes.
- [ ] `on_value_change` fires exactly once per accepted normalized value change and never when normalization yields the current value.

### Slot navigation, distribution, and paste

- [ ] Mouse down on a slot focuses the group and sets the active index to that slot, clamped to `min(value_code_points, length - 1)`; disabled fields ignore pointer activation.
- [ ] While focused, the active index is `min(focused_index, length - 1)`; while unfocused, the styling-active slot is `min(value_code_points, length - 1)`.
- [ ] Typing a character distributes it (and any multi-character IME commit) forward from the active index via replace-at-index, then advances the active index by the number of accepted characters, clamped to the last slot.
- [ ] Typing the same character a filled, fully-selected slot already contains advances to the next slot without changing the value and without firing `on_value_change`.
- [ ] Typing characters fully rejected by validation leaves the value and active index unchanged and fires `on_value_invalid` (reason `InputChange`).
- [ ] Paste normalizes clipboard text, fires `on_value_invalid` (reason `InputPaste`) when characters are rejected, distributes accepted characters forward from the active index, and advances the active index past the pasted run (clamped to the last slot); an all-rejected paste is a no-op.
- [ ] `on_value_complete` fires after the value update when the value becomes `length` code points long (reasons `InputChange` / `InputPaste`), and also when pasting a complete value identical to the current complete value (without `on_value_change`).
- [ ] `auto_submit == true` inside a `FormRoot` calls `FormContext::submit(...)` immediately after `on_value_complete`; outside a form it is a no-op.
- [ ] Disabled fields ignore all editing and navigation; read-only fields allow focus and navigation but ignore all value-changing interactions.

### Keyboard contract

All items via OTP actions bound in `actions.rs` (LTR only in phase 1):

- [ ] Left moves the active index back one slot, stopping at the first slot.
- [ ] Right moves the active index forward one slot, stopping at the last slot and never beyond `min(value_code_points, length - 1)`.
- [ ] Ctrl/Cmd+Left (without Alt) moves to the first slot; Ctrl/Cmd+Right moves to the end-of-value slot.
- [ ] Home and ArrowUp move to the first slot; End and ArrowDown move to the end-of-value slot.
- [ ] Backspace on a filled slot removes that slot's character and moves the active index back one slot.
- [ ] Backspace on an empty slot removes the previous slot's character and moves the active index back one slot.
- [ ] Ctrl/Cmd+Backspace clears the entire value and moves the active index to the first slot.
- [ ] Delete removes the character at the active index (later characters shift left) and keeps the active index in place.
- [ ] Keyboard-driven value changes fire `on_value_change` with reason `Keyboard`; clearing via text input uses reason `InputClear`.
- [ ] Navigation keys work when read-only; editing keys are ignored when read-only or disabled.

### Field / Form integration

- [ ] OTP Field rendered inside `FieldRoot` consumes root disabled state; rendering inside a disabled `FieldItem` disables it.
- [ ] The root registers exactly one `FieldControlRegistration` with a stable key, `FieldValue::Text(value)`, `name` metadata (with `FieldRoot::name(...)` as override), `required`, merged disabled, focused state, and the group focus handle.
- [ ] `FieldLabel` click focuses the OTP group.
- [ ] Field becomes filled when the value is non-empty and unfilled when empty.
- [ ] Field becomes dirty when the value differs from its initial registered value.
- [ ] Field becomes focused while the group focus handle is focused and touched when it blurs.
- [ ] `FieldValidationMode::OnChange` validates on value changes; `FieldValidationMode::OnBlur` validates when the group blurs.
- [ ] Required-only Field validation reports `value_missing` for an empty required OTP Field.

### Styling / state exposure

- [ ] Add `OTPFieldRootStyleState` exposing at least `value`, `length`, `complete`, `filled`, `focused`, `disabled`, `read_only`, `required`, `dirty`, `touched`, and field-derived `valid`/`invalid` when available.
- [ ] Add `OTPFieldInputStyleState` exposing at least the slot's `value` character (empty when unfilled), `index`, `filled` (this slot has a character), `active` (this slot is the active slot), `masked`, plus the inherited root facts (`complete`, `focused`, `disabled`, `read_only`, `required`, `valid`/`invalid`).
- [ ] `mask == true` renders a mask character in filled slots while the underlying value, clipboard, and callbacks keep real characters.
- [ ] The active slot renders a blinking caret while the group is focused; unfocused fields render no caret.
- [ ] `style_with_state(...)` receives the typed state for root and each slot; per-slot styling can vary by `filled`, `active`, and `index`.
- [ ] Base UI data attributes (`data-active`, `data-filled`, `data-complete`, ...) map into these style-state structs, not attribute APIs.

### Tests / verification

Add behavior tests under `crates/base_gpui/src/otp_field/tests/` and pure unit tests beside `otp.rs`.

- [ ] The pure `otp.rs` helpers are unit-tested without GPUI, porting the cases from `otp.test.ts`: whitespace stripping (including nullish/empty), numeric/alpha/alphanumeric filtering, `None` with custom normalization, custom normalization ordering and re-filtering, clamping after custom normalization, non-positive lengths, replace at first/middle/last slot, suffix preservation when normalization shrinks a middle replacement, remove at first/last/out-of-bounds index, code-point clamping of multi-byte characters, and rejection-details reporting.
- [ ] Uncontrolled default value renders across slots; controlled value updates slots across renders.
- [ ] Typing distributes characters forward and advances the active slot.
- [ ] Same-character overtype advances without a value change.
- [ ] Paste distributes from the active slot, clamps to `length`, and reports rejected characters.
- [ ] Backspace/Delete/Ctrl+Backspace behavior matches the keyboard contract.
- [ ] Left/Right/Home/End/ArrowUp/ArrowDown navigation matches the keyboard contract, including end-of-value clamping.
- [ ] `on_value_change`, `on_value_complete`, and `on_value_invalid` fire with the specified reasons and ordering, including complete-paste-over-complete-value.
- [ ] Disabled and read-only behavior matches the contract.
- [ ] Mask rendering hides characters without changing the underlying value.
- [ ] Field registration drives filled/dirty/focused/touched and required validation; label click focuses the group.
- [ ] `auto_submit` submits the surrounding form on completion and is a no-op outside a form.
- [ ] IME composition through `replace_and_mark_text_in_range` commits into slots, and IME candidate bounds resolve to the active slot's bounds.

## Follow-ups

- RTL-aware Left/Right arrow mirroring once `base_gpui` grows a direction concept.
- OS/autofill one-time-code integration if GPUI ever exposes a platform hook.
- Per-slot custom mask characters or input-type overrides beyond the boolean `mask`.
- AccessKit/accessibility wiring (group semantics, per-slot labels) once GPUI accessibility work is revisited.

## Uncertain items

- [ ] Whether the existing public `InputRuntime` surface (`sync_props` selection clamping + the public `end(...)` command) is enough to force the end-of-value cursor, or whether the small `move_cursor_to_end`-style extension is needed — decide during implementation.
- [ ] Exact shape of the IME-candidate-bounds override (`set_ime_candidate_bounds(Bounds<Pixels>)` vs. an alternative `set_last_layout` variant) — pick whichever keeps `EntityInputHandler` glue smallest.
- [ ] Whether ArrowUp/ArrowDown should be claimed by OTP actions on all platforms or left unbound where they conflict with other bindings registered in `base_gpui::init(cx)`.
- [ ] Whether the debug warning for slot-count/`length` mismatch should be a `log` warning or a `debug_assert!`-style check, matching whatever convention other `base_gpui` components use.
