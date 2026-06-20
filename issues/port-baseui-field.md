# Port Base UI Field to GPUI

## Problem

Base UI Field provides shared form-field infrastructure: disabled state, label/control/message association, field item grouping, dirty/touched/filled/focused state, validation state, error visibility, and validity render callbacks. It is the primitive that lets controls such as Checkbox, Switch, Radio Group, Select, Number Field, and text inputs participate in one field-level state model.

`crates/base_gpui` currently has Checkbox, Switch, Tabs, Radio Group, and shared Direction Provider work, but no Field primitive. Checkbox, Switch, and Radio Group still have unchecked follow-ups for field/form integration. The goal is to port Field as GPUI-native field infrastructure that existing and future controls can consume, not to copy React context, DOM inputs, browser validity, hidden inputs, or ARIA attributes literally.

This should be implemented as a deep `field` component module following `docs/base-gpui-component-architecture.md`.

## Scope

Port the Field component family from Base UI into GPUI-native components and shared field registration plumbing:

- `FieldRoot`
- `FieldItem`
- `FieldLabel`
- `FieldDescription`
- `FieldError`
- `FieldValidity`
- field-aware control registration APIs for existing/future controls

Base UI also has `FieldControl`, which renders an HTML `<input>`. GPUI currently does not expose a built-in text input element in `crates/gpui/src/elements`, so this issue should **not** fake a DOM input. Instead, port the control-registration contract first. Add a GPUI `FieldControl` only if a GPUI-native text input primitive exists or is introduced as part of a separate input component issue.

Base UI reference files:

- `/home/luke/Projects/base-ui/packages/react/src/field/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/field/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/field/root/FieldRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/field/root/FieldRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/field/root/useFieldValidation.ts`
- `/home/luke/Projects/base-ui/packages/react/src/field/root/FieldRootDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/field/control/FieldControl.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/field/control/FieldControl.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/field/control/FieldControlDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/field/label/FieldLabel.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/field/label/FieldLabel.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/field/label/FieldLabelDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/field/description/FieldDescription.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/field/description/FieldDescription.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/field/description/FieldDescriptionDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/field/error/FieldError.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/field/error/FieldError.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/field/error/FieldErrorDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/field/validity/FieldValidity.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/field/validity/FieldValidity.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/field/item/FieldItem.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/field/item/FieldItem.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/field/item/FieldItemContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/field/item/FieldItemDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/field/utils/getCombinedFieldValidityData.ts`
- `/home/luke/Projects/base-ui/packages/react/src/internals/field-root-context/FieldRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/internals/field-constants/constants.ts`
- `/home/luke/Projects/base-ui/packages/react/src/internals/field-register-control/useFieldControlRegistration.ts`
- `/home/luke/Projects/base-ui/packages/react/src/internals/field-register-control/useRegisterFieldControl.ts`
- `/home/luke/Projects/base-ui/packages/react/src/internals/labelable-provider/LabelableProvider.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/internals/labelable-provider/LabelableContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/internals/labelable-provider/useLabel.ts`
- `/home/luke/Projects/base-ui/packages/react/src/internals/labelable-provider/useLabelableId.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/field/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/field/types.md`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/field/demos/hero/css-modules/index.tsx`

Current GPUI implementation:

- No `crates/base_gpui/src/field/` module exists yet.
- Checkbox, Switch, and Radio Group expose state but do not consume field context.
- There is no GPUI-native Form primitive in `base_gpui` yet.
- There is no GPUI-native text input component in `base_gpui` yet.

Expected GPUI implementation files:

- `crates/base_gpui/src/field/mod.rs`
- `crates/base_gpui/src/field/child.rs`
- `crates/base_gpui/src/field/child_wiring.rs`
- `crates/base_gpui/src/field/context.rs`
- `crates/base_gpui/src/field/item_context.rs`
- `crates/base_gpui/src/field/props.rs`
- `crates/base_gpui/src/field/style_state.rs`
- `crates/base_gpui/src/field/runtime.rs`
- `crates/base_gpui/src/field/validation.rs`
- `crates/base_gpui/src/field/layers/mod.rs`
- `crates/base_gpui/src/field/layers/field_root.rs`
- `crates/base_gpui/src/field/layers/field_item.rs`
- `crates/base_gpui/src/field/layers/field_label.rs`
- `crates/base_gpui/src/field/layers/field_description.rs`
- `crates/base_gpui/src/field/layers/field_error.rs`
- `crates/base_gpui/src/field/layers/field_validity.rs`
- `crates/base_gpui/src/field/tests/`

Use the runtime/context/layers pattern from `docs/base-gpui-component-architecture.md`. Field is cross-control infrastructure, but it should still be its own component module, not a generic `utils` dump.

## Out of scope / drop from Base UI

- Do not port React context/hooks directly.
- Do not port DOM `<input>` behavior literally.
- Do not port browser `ValidityState` APIs as browser objects; use Rust-native validity flags/data.
- Do not port hidden inputs, browser form submission, or `FormData` behavior until a GPUI-native `Form` primitive exists.
- Do not port `render` props, `className`, CSS style props, or DOM attributes.
- Do not port DOM ARIA attributes literally. Map labels/descriptions/errors/accessibility through GPUI-native AccessKit APIs once available.
- Do not port SSR/hydration/prehydration logic.
- Do not port native label tag warnings (`nativeLabel`) literally.
- Do not port HTML `aria-describedby`, `aria-labelledby`, `htmlFor`, or generated DOM IDs as public API.
- Do not port transition DOM data attributes literally; represent transition/presence as typed style state if transition support exists, otherwise track as a follow-up.
- Do not implement a fake text input just to satisfy `Field.Control` anatomy.

## Acceptance Criteria

### Module/API surface

- [x] Add a `field` module and export it from `crates/base_gpui/src/lib.rs`.
- [x] Add public `FieldRoot`, `FieldItem`, `FieldLabel`, `FieldDescription`, `FieldError`, and `FieldValidity` layer types.
- [x] Add typed child routing so `FieldRoot` accepts Field parts before `AnyElement` erasure.
- [x] Add typed child routing so `FieldItem` accepts label/description/control-like descendants when needed.
- [x] Support `FieldRoot::new()`.
- [x] Support `FieldRoot::disabled(bool)`, defaulting to `false`.
- [x] Support `FieldRoot::name(...)` as metadata for future Form integration and field-aware controls.
- [x] Support controlled field state builders where Base UI exposes them: `.invalid(bool)`, `.dirty(bool)`, and `.touched(bool)`.
- [x] Support validation configuration with Rust-native types: `.validation_mode(FieldValidationMode)` and `.validation_debounce(...)` or a documented GPUI equivalent.
- [x] Support a custom validation callback with a Rust-native value/validity API.
- [x] Support a manual validation action/handle equivalent to Base UI `actionsRef.validate` if GPUI has a suitable API; otherwise expose a documented command on context/runtime for tests and future Form integration.
- [x] Support `FieldItem::disabled(bool)`, defaulting to `false`.
- [x] Support `FieldError::match_(...)` or equivalent because `match` is a Rust keyword.
- [x] Support `FieldError` default matching behavior: show when the field is invalid and no specific validity key is requested.
- [x] Support `FieldError::match_always(true)` or `FieldErrorMatch::Always` for Base UI `match={true}` behavior.
- [x] Support `FieldValidity` as a GPUI builder that exposes the current validity data through a callback/style-state API.
- [x] Add a field-aware control registration API for compound controls, without requiring a text input.
- [x] Do not add `FieldControl` as a DOM/text input until a GPUI-native text input primitive exists.
- [x] Re-export ergonomic Field names from `field/mod.rs`.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui field` passes.
- [x] `cargo test -p base_gpui` passes.
- [x] The implementation compiles without adding web/React-specific concepts to public APIs.
- [x] The implementation follows `docs/base-gpui-component-architecture.md` and does not introduce the old `child/context/{props,runtime,state}` taxonomy.
- [x] Add a small example/demo in `crates/base_gpui/src/main.rs` or a dedicated example showing a labeled field with description and error styling.

### Architecture / internal primitives

- [x] Add `FieldRuntime` as the single owner of Field business state: disabled, invalid, touched, dirty, filled, focused, validity data, registered controls, registered label/description/error messages, and error presence.
- [x] Add `FieldProps` for stable root props and callbacks.
- [x] Add `FieldContext` as the injection/plumbing type with `read(...)`, `update(...)`, and small registration/validation forwarding methods only.
- [x] Keep validation and field-state transitions on `FieldRuntime`, not in layers.
- [x] Keep controlled/uncontrolled dirty/touched/invalid resolution in root/context, not in leaf parts.
- [x] Add `FieldItemContext` for item-local disabled state.
- [x] Add `FieldRootStyleState`, `FieldItemStyleState`, `FieldLabelStyleState`, `FieldDescriptionStyleState`, `FieldErrorStyleState`, and `FieldValidityStyleState` or equivalent validity payload type.
- [x] Add renderable GPUI elements only under `field/layers/`.
- [x] Add typed child routing in `field/child.rs` and private context/registration wiring in `field/child_wiring.rs`.
- [ ] Register field-aware control metadata before descendant state queries so initial filled/focused/disabled/valid states are correct.
- [x] Do not add a generic labelable provider in `utils` unless another component needs the same API immediately; keep it in `field` at first.
- [x] Do not add generic form primitives inside Field; Form should be a separate issue/module.

### Field state model

- [x] `FieldRootStyleState` exposes `disabled`, `touched`, `dirty`, `valid: Option<bool>` or equivalent tri-state, `filled`, and `focused`.
- [x] Default root state is `disabled=false`, `touched=false`, `dirty=false`, `valid=None`, `filled=false`, `focused=false`.
- [x] `disabled=true` on `FieldRoot` marks root and descendants disabled.
- [x] Root disabled state takes precedence over item/control disabled state.
- [x] Controlled `.dirty(...)` overrides internal dirty state.
- [x] Controlled `.touched(...)` overrides internal touched state.
- [x] Controlled `.invalid(true)` makes field validity invalid.
- [x] Disabled fields expose `valid=None`, matching Base UI's disabled-valid behavior.
- [x] Field becomes filled when the registered control reports a non-empty value/selection.
- [x] Field becomes focused when the registered control focus handle is focused.
- [x] Field becomes touched when the registered control loses focus or an equivalent control-specific blur/commit event occurs.
- [x] Field becomes dirty when the registered control value differs from its initial registered value.
- [x] Re-rendering with changed props does not reset uncontrolled touched/dirty/filled/focused state except when the keyed root id changes.

### Field-aware control registration

- [x] Add a `FieldControlRegistration` or equivalent metadata type that can be used by Checkbox, Switch, Radio Group, and future text inputs.
- [x] Registration includes a stable control id/key or focus handle where available.
- [x] Registration includes a control name fallback so `FieldRoot::name(...)` can override it later for Form integration.
- [x] Registration includes enough value/filled metadata for dirty and filled state.
- [x] Registration includes enough disabled metadata for FieldRoot/FieldItem disabled cascading.
- [x] Registration supports replacing one control with another without stale state leaks.
- [x] Registration supports unregistering removed controls.
- [x] If multiple controls register under one field, define deterministic precedence or aggregation rules.
- [x] Existing controls should be able to consume field disabled state without adding DOM-specific APIs.

### Label / description / error behavior

- [x] `FieldLabel` receives field style state and supports `style_with_state(...)`.
- [x] `FieldDescription` receives field style state and supports `style_with_state(...)`.
- [x] `FieldError` receives field style state plus error presence/transition state and supports `style_with_state(...)`.
- [x] `FieldLabel` can be clicked to focus the registered field control when a focus handle is available.
- [x] `FieldDescription` registers as descriptive text/message metadata for future accessibility integration.
- [x] `FieldError` registers as descriptive text/message metadata only while present.
- [x] Label/description/error metadata updates when the corresponding part is added, removed, or replaced.
- [x] Label/description/error metadata does not leak outside the current `FieldRoot`.
- [x] If GPUI-native AccessKit APIs are unavailable, keep metadata internal and expose follow-up tasks rather than writing DOM ARIA attributes.

### Validation behavior

- [x] Add `FieldValidationMode::{OnSubmit, OnBlur, OnChange}` or equivalent.
- [x] Default validation mode is `OnSubmit`, matching Base UI/Form default.
- [x] Without a GPUI-native `Form`, `OnSubmit` validation is driven only by manual validation commands or future Form integration.
- [x] `OnBlur` validates when the registered control commits blur/touched state.
- [x] `OnChange` validates when the registered control value changes.
- [x] Validation callbacks receive the current field value and known form values if a Form context exists; otherwise pass an empty form-values map or omit the argument with a documented Rust-native API.
- [x] Validation can return no error, one error, or multiple errors.
- [x] Validation result updates `FieldValidityData` / validity style state.
- [x] Validation result updates `FieldError` presence.
- [x] Async validation is either supported with stale-result cancellation or explicitly left as a follow-up.
- [x] Debounced validation is either supported through GPUI timers/tasks or explicitly left as a follow-up.
- [x] Value-missing/required validation for Checkbox, Switch, and Radio Group is implemented only when those controls integrate with Field and expose required/filled metadata.

Follow-up: async validation and actual timer-backed debounce behavior are intentionally deferred until Field has a GPUI-native async validation task design with stale-result cancellation. The public `.validation_debounce(...)` metadata is preserved for that follow-up but is not yet executed as a timer.
- [x] Do not depend on browser native `validationMessage` or `ValidityState`.

### Error and validity behavior

- [x] `FieldValidityData` includes validity flags equivalent to Base UI's keys where meaningful: `bad_input`, `custom_error`, `pattern_mismatch`, `range_overflow`, `range_underflow`, `step_mismatch`, `too_long`, `too_short`, `type_mismatch`, `value_missing`, and `valid`.
- [x] Initial validity state has all flags false and `valid=None`.
- [x] `FieldError` is absent by default when the field has no error.
- [x] `FieldError` renders when the field is invalid and no specific match is requested.
- [x] `FieldError` with a specific match renders only when that validity flag is true.
- [x] `FieldError` with match always renders regardless of current field validity.
- [x] Disabled fields do not render validation errors by default.
- [x] `FieldErrorStyleState` exposes `present` and the current error message(s) in a GPUI-native way.
- [x] `FieldValidity` exposes validity data, current error, errors list, value, initial value, and transition/presence status if supported.
- [x] If transition support exists in `base_gpui`, expose a GPUI-native transition status for errors/validity; otherwise track transition behavior as a follow-up.

Follow-up: `base_gpui` does not currently have shared transition/presence infrastructure for Field errors, so transition status is intentionally deferred rather than represented as DOM data attributes.

### FieldItem behavior

- [x] `FieldItem` merges root disabled state with item-local disabled state.
- [x] `FieldItem` exposes the merged disabled state through style state.
- [x] `FieldItem` can wrap a field-aware Checkbox and disable that checkbox through field context.
- [x] `FieldItem` can wrap a field-aware RadioGroup radio and disable that radio through field context.
- [x] `FieldItem` label/description descendants receive item-disabled state.
- [x] `FieldItem` does not create an independent field root; it shares the surrounding `FieldRoot` state.

### Existing component integration

- [x] Checkbox consumes `FieldRoot` disabled state.
- [x] Checkbox inside `FieldItem disabled` is disabled.
- [x] Checkbox reports focused, touched, dirty, and filled facts to Field when registered.
- [x] Switch consumes `FieldRoot` disabled state.
- [x] Switch reports focused, touched, dirty, and filled facts to Field when registered.
- [x] Radio Group consumes `FieldRoot` disabled state.
- [x] Radio Group radio inside `FieldItem disabled` is disabled.
- [x] Radio Group reports focused, touched, dirty, and filled facts to Field when registered.
- [x] Required Checkbox/Switch/Radio Group validation is either implemented or explicitly left as a follow-up with enough metadata preserved.

### Styling/state exposure

- [x] All Field parts expose state-aware styling through `style_with_state(...)`.
- [x] Map Base UI Field data attributes (`disabled`, `valid`, `invalid`, `dirty`, `touched`, `filled`, `focused`, and error transition status) into typed style-state fields, not DOM attributes.
- [x] Do not expose CSS variable names as the styling API.
- [x] The docs hero styling pattern can be recreated with GPUI builder methods: label, control-like component, error, and description reflect field state.

### Accessibility follow-up

- [ ] Once the target GPUI revision supports needed AccessKit APIs, connect `FieldLabel` to the registered control accessible name.
- [ ] Once available, connect `FieldDescription` and present `FieldError` to the registered control accessible description.
- [ ] Once available, expose invalid/required/disabled state through GPUI-native accessibility APIs.
- [ ] Add accessibility tests if GPUI exposes test helpers for AccessKit state.
- [x] Do not port DOM `aria-labelledby`, `aria-describedby`, `htmlFor`, or generated HTML ids literally.

### Form integration follow-up

- [x] Create or reference a separate GPUI-native `Form` issue before implementing submit-level field validation. See `issues/port-baseui-form.md`.
- [x] Decide how `FieldRoot::name(...)` participates in future form value collection.
- [x] Decide how external form errors are represented in GPUI.
- [x] Decide how field arrays/multiple registered controls map to future form values.
- [x] Do not implement hidden DOM input submission behavior in GPUI.

### Tests / verification

Add one behavior per file under `crates/base_gpui/src/field/tests/` where practical.

- [x] Default root state is disabled=false, touched=false, dirty=false, valid=None, filled=false, focused=false.
- [x] Root disabled state is exposed on root, label, description, error, validity, and item.
- [x] Root disabled state takes precedence over item/control disabled state.
- [x] Controlled dirty state is reflected.
- [x] Controlled touched state is reflected.
- [x] Controlled invalid state makes valid false and error present.
- [x] Field becomes filled when a registered control reports a non-empty value.
- [x] Field becomes unfilled when a registered control reports an empty value.
- [x] Field becomes focused when a registered control focus handle is focused.
- [x] Field becomes touched when a registered control loses focus.
- [x] Field becomes dirty when registered value differs from initial value.
- [x] Registration replacement does not leak stale control state.
- [x] Unregistered controls are removed from Field runtime metadata.
- [x] `FieldLabel` style state receives current field state.
- [x] Clicking `FieldLabel` focuses the registered control when a focus handle is available.
- [x] `FieldDescription` style state receives current field state.
- [x] `FieldError` is absent by default.
- [x] `FieldError` renders when invalid by default.
- [x] `FieldError` specific match renders only for matching validity flags.
- [x] `FieldError` match-always renders regardless of validity.
- [x] `FieldValidity` receives validity data.
- [x] Validation mode `OnChange` validates on registered value change.
- [x] Validation mode `OnBlur` validates on registered blur/touched event.
- [x] Manual validation validates the current registered control value.
- [x] Validation string error updates `error` and one-item `errors`.
- [x] Validation multiple errors updates first `error` and full `errors`.
- [x] FieldItem disabled state is exposed.
- [x] FieldItem disables wrapped Checkbox.
- [x] FieldItem disables wrapped Radio Group radio.
- [x] Checkbox reports filled/focused/touched/dirty to Field.
- [x] Switch reports filled/focused/touched/dirty to Field.
- [x] Radio Group reports filled/focused/touched/dirty to Field.
- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui field` passes.
- [x] `cargo test -p base_gpui` passes.
