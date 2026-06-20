# Port Base UI Form to GPUI

## Problem

Base UI Form provides one owner for form submission and consolidated field errors. It coordinates registered `Field.Root` instances, runs field validation before submit, focuses the first invalid control, exposes external/server errors by field name, and can produce a form-values object for successful submissions.

`base_gpui` now has most of the lower-level form-control stack: `Field`, public `Input`, `NumberField`, Checkbox, Switch, and Radio Group all expose or consume Field registration metadata. What is still missing is a GPUI-native `Form` component that can collect registered fields, trigger submit-time validation, inject form-level errors into fields, and provide a Rust-native submit callback.

The goal is behavioral parity with Base UI Form where it maps cleanly to GPUI. Do not port DOM `<form>` submission, `FormData`, React refs/context, browser events, or hidden inputs literally. Instead, use GPUI keyed runtime state, an ambient form context during render/layout, GPUI actions, typed field metadata, and Rust-native submit details.

## Scope

Port Base UI `Form` into GPUI-native components and form/field integration plumbing:

- `Form`
- `FormContext`
- `FormRuntime`
- `FormProps`
- `FormActions` / a GPUI-native actions handle equivalent
- `FormSubmitDetails`
- `FormValue` / form-values map type

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/form/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/form/Form.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/form/Form.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/form/Form.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/internals/form-context/FormContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/field/root/FieldRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/field/root/useFieldValidation.ts`
- `/home/luke/Projects/base-ui/packages/react/src/internals/field-register-control/useFieldControlRegistration.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/form/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/form/types.md`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/form/demos/hero/css-modules/index.tsx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/form/demos/form-action/css-modules/index.tsx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/form/demos/zod/css-modules/index.tsx`

Current GPUI prerequisites:

- `crates/base_gpui/src/field/`
- `crates/base_gpui/src/input/`
- `crates/base_gpui/src/number_field/`
- `crates/base_gpui/src/checkbox/`
- `crates/base_gpui/src/switch/`
- `crates/base_gpui/src/radio_group/`

Current GPUI implementation status:

- No `crates/base_gpui/src/form/` module exists yet.
- `FieldRoot::name(...)` and `FieldControlRegistration::name(...)` already preserve the metadata needed for future form value collection.
- `FieldRuntime` owns validity data and control registration, but it does not yet register field-level metadata with a surrounding `Form`.
- There is no GPUI-native `Fieldset` module yet, so Base UI fieldset-specific disabled behavior should be tracked as a follow-up unless Fieldset is ported first.

Expected GPUI implementation files:

```text
crates/base_gpui/src/form/mod.rs
crates/base_gpui/src/form/actions.rs
crates/base_gpui/src/form/context.rs
crates/base_gpui/src/form/props.rs
crates/base_gpui/src/form/render_state.rs
crates/base_gpui/src/form/runtime.rs
crates/base_gpui/src/form/value.rs
crates/base_gpui/src/form/layers/mod.rs
crates/base_gpui/src/form/layers/form.rs
crates/base_gpui/src/form/tests/
```

Additional Field integration edits are expected under:

```text
crates/base_gpui/src/field/context.rs
crates/base_gpui/src/field/layers/field_root.rs
crates/base_gpui/src/field/runtime.rs
crates/base_gpui/src/field/props.rs
```

## Initial design decisions

### Public API shape

Base UI exports a single `Form` component, not a compound `Form.Root`. Mirror that with:

```rust
Form::new()
    .validation_mode(FieldValidationMode::OnSubmit)
    .on_form_submit(|values, details, window, cx| {
        // values: FormValues
    })
```

Do not use `FormRoot` unless implementation work shows that Rust naming needs it. `Form::new()` should be the canonical public builder.

### Submit triggers

Base UI relies on native `<button type="submit">` and browser submit events. GPUI has no DOM form tree, so provide GPUI-native submit triggers instead:

- a `FormSubmit` action registered from `base_gpui::init(cx)`;
- a public form context/action method that a future `Button` component can call;
- optional builder/helper APIs only if they remain GPUI-native and do not imply hidden DOM behavior.

Do not block this issue on a Button component. Tests may trigger submit through a captured `FormContext` or action dispatch.

### Form values

Base UI `onFormSubmit` returns a JavaScript object keyed by field name. GPUI should use a Rust-native map:

```rust
pub type FormValues = BTreeMap<SharedString, FormValue>;
```

Use a value enum that can preserve the useful current control values without arbitrary JavaScript semantics, for example:

```rust
pub enum FormValue {
    Empty,
    Present,
    Bool(bool),
    Text(SharedString),
    Number(f64),
}
```

It is acceptable to start by converting from existing `FieldValue` and add `Number(f64)` only if Number Field can provide numeric value cleanly. If Number Field still registers textual `FieldValue`, document the numeric-value follow-up in the issue checklist before checking off full Number Field value parity.

### Field naming rules

- `FieldRoot::name(...)` is the form field name override.
- If `FieldRoot::name(...)` is absent, Form can fall back to `FieldControlRegistration::name(...)`.
- Unnamed fields still participate in validation and can block submit, but they are omitted from submitted values.
- Multiple fields with the same name remain separate validation units; value collection should be deterministic and documented. Base UI's object collection means later same-name values overwrite earlier ones in the submitted object, but validity must remain scoped per mounted field.

### Validation mode inheritance

Base UI's `Form::validationMode` is inherited by `Field.Root`, but an explicit `Field.Root::validationMode` takes precedence.

In GPUI, `FieldRoot` may need to distinguish an explicitly configured validation mode from an inherited default. Do not accidentally make every field ignore the Form mode because `FieldRoot` internally defaults to `OnSubmit`.

### External errors

Base UI `Form::errors` maps field names to one or many messages. GPUI should represent this as Rust-native data, e.g.:

```rust
BTreeMap<SharedString, Vec<SharedString>>
```

External errors should feed the corresponding Field validity/error render state, not mutate browser validity APIs.

### Submit behavior

On submit:

1. mark submit attempted;
2. synchronously validate all mounted, enabled fields;
3. refresh the field registry after validation;
4. if any enabled field is invalid, do not call submit callbacks and focus the first invalid control with an available `FocusHandle`;
5. if all enabled fields are valid, collect named enabled field values and call the Rust-native submit callback.

Async validation can be supported later if needed. Base UI does not let pending async validators block native submit; if GPUI async validation is not implemented in this port, document it as a follow-up and keep submit validation synchronous.

## Out of scope / drop from Base UI

- Do not port React refs, hooks, context implementation, or `useImperativeHandle` literally.
- Do not port DOM `<form>` submission behavior, browser navigation, hidden inputs, or `FormData`.
- Do not port native DOM submit/reset events or event propagation APIs literally.
- Do not port `onSubmit` as a raw browser event API. Prefer `on_form_submit(values, details, window, cx)`.
- Do not port `render`, `className`, or web `style` props.
- Do not port DOM `noValidate`; GPUI has no browser native validation to disable.
- Do not port arbitrary JavaScript object semantics. Use Rust-native value enums/maps.
- Do not port DOM `aria-invalid`, `aria-describedby`, or focus/select browser methods literally. Use Field state and GPUI `FocusHandle`.
- Do not implement full browser constraint validation. Use Field/Control Rust-native validity data.
- Do not implement server functions, React `useActionState`, or framework action attributes. External errors can be represented by updating `Form::errors(...)` props.
- Do not block the first Form port on `Fieldset`; add Fieldset integration as a follow-up unless Fieldset is ported first.
- Do not use Rust scoped visibility syntax (`pub(...)`); `ast-grep scan` must remain clean.

## Acceptance Criteria

### Prerequisites already satisfied

- [x] `FieldRoot` exists and owns field validity/dirty/touched/filled/focused state.
- [x] `FieldControlRegistration` preserves stable key, name, value, required, disabled, focused, and focus handle metadata.
- [x] Public `Input` exists and registers with Field through `FieldControl`.
- [x] Number Field exists and registers with Field.
- [x] Checkbox, Switch, and Radio Group can consume Field disabled state and report Field value/focus metadata.
- [x] `issues/port-baseui-field.md` tracks Field-specific follow-ups.
- [x] `issues/port-baseui-input.md` tracks public Input-specific follow-ups.

### Module/API surface

- [x] Add a top-level `form` module and export it from `crates/base_gpui/src/lib.rs`.
- [x] Register Form actions from `base_gpui::init(cx)` if submit/validate actions use GPUI key dispatch.
- [x] Add public `Form` builder/element type.
- [x] Support `Form::new()`.
- [x] Support `.child(...)`, `.children(...)`, and `.child_any(...)` for arbitrary form contents.
- [x] Support `.validation_mode(FieldValidationMode)` defaulting to `OnSubmit`.
- [x] Support `.errors(...)` with a Rust-native external error map keyed by field name.
- [x] Support `.on_form_submit(...)` with `FormValues`, `FormSubmitDetails`, `&mut Window`, and `&mut App`.
- [x] Add a Rust-native `FormSubmitDetails` with `reason = FormSubmitReason::None` or equivalent; do not expose DOM events.
- [x] Add a `FormActions`/handle/context equivalent for `validate()` and `validate_field(name)`.
- [x] Add a GPUI-native submit command/action equivalent.
- [x] Defer reset behavior to a later issue; do not imply browser reset semantics unless implemented.
- [x] Re-export ergonomic names from `form/mod.rs`.
- [x] Do not expose CSS class/data-attribute APIs.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui form` passes.
- [x] `cargo test -p base_gpui` passes.
- [x] `ast-grep scan crates/base_gpui/src/form crates/base_gpui/src/field` passes or produces no scoped-visibility violations.
- [x] Add a small Form demo in `crates/base_gpui/src/main.rs` or a dedicated example.
- [x] Existing Field/Input/NumberField tests continue passing after Form integration changes.

### Architecture / internals

- [x] Implement Form as a component module first; do not create separate lower-level Form/Fieldset primitives unless repeated implementation complexity clearly warrants it.
- [x] Add `FormRuntime` as the owner of mounted field registry, external errors, submit-attempted state, pending focus target, and form submission outcomes.
- [x] Add `FormProps` for stable form config and callbacks.
- [x] Add `FormContext` as thin injection/plumbing with `read(...)`, `update(...)`, and small form commands only.
- [x] Keep validation orchestration, field registry reconciliation, submit blocking, and values collection in `FormRuntime` or `FormContext` methods, not visual layers.
- [x] Add ambient form context scoped to `Form` descendants during layout/prepaint/paint, mirroring the existing GPUI-native Field context pattern.
- [x] Keep `Form` as a component module under `crates/base_gpui/src/form/`, not a `utils` dump.
- [x] Do not create a second generic form-control abstraction outside Field/Form.
- [x] Keep `primitives::input` field-agnostic and form-agnostic.
- [x] Do not port React context, refs, or effects literally.
- [x] Do not use old `child/context/{props,runtime,state}` taxonomy; follow `docs/base-gpui-component-architecture.md`.
- [x] Keep renderable GPUI elements only under `form/layers/`.

### Field registration and lifecycle

- [x] FieldRoot rendered inside Form registers one field metadata entry with the current Form.
- [x] Field registration includes stable field id/key independent of field name so same-name fields remain validity-scoped.
- [x] Field registration includes effective field name: `FieldRoot::name(...)` first, then registered control name fallback.
- [x] Field registration includes current representative value.
- [x] Field registration includes disabled state so disabled fields are skipped for submit validation and value collection.
- [x] Field registration includes current validity data after synchronous validation.
- [x] Field registration includes a `validate` command/callback for Form submit and actions.
- [x] Field registration includes a focus handle for first-invalid-field focus.
- [x] Removed/unmounted fields are pruned from the Form registry.
- [x] Replacing a control inside one field updates the field registry without stale control ids or stale values.
- [x] Same-name fields keep independent validity/error state even if their collected form-value key collides.

### Validation mode behavior

- [x] Form validation mode defaults to `OnSubmit`.
- [x] FieldRoot inherits Form validation mode when its own validation mode is not explicitly set.
- [x] Explicit `FieldRoot::validation_mode(...)` takes precedence over the surrounding Form mode.
- [x] `OnSubmit` validates fields when Form is submitted.
- [x] After an `OnSubmit` submit attempt, changed fields revalidate on change, matching Base UI's submit-attempted behavior.
- [x] `OnBlur` fields validate on blur even when inside Form.
- [x] `OnChange` fields validate on every value change even when inside Form.
- [x] Manual Form validation validates all mounted enabled fields.
- [x] Manual Form validation with a field name validates matching mounted field(s) by effective name.
- [x] Async validation is explicitly deferred; Form submit validation remains synchronous and does not model pending async validators.
- [x] Debounced validation remains Field-owned; Form submit forces immediate synchronous validation through FieldContext.

### Submit behavior

- [x] Submit validates all mounted enabled fields before invoking `on_form_submit`.
- [x] Submit does not call `on_form_submit` when any enabled field is invalid.
- [x] Submit focuses the first invalid enabled field with a registered focus handle.
- [x] Submit does not focus a later invalid field immediately when earlier invalid field changes unless another submit/validate action requests it.
- [x] Submit skips disabled fields for validation and value collection.
- [x] Submit includes unnamed fields in validation blocking but omits unnamed fields from `FormValues`.
- [x] Submit collects values from named fields in stable render/registration order.
- [x] If multiple enabled fields share the same name, define and test deterministic value collection behavior.
- [x] Successful submit calls `on_form_submit` exactly once.
- [x] `FormSubmitDetails` exposes a Rust-native reason/source without browser event objects.
- [x] Submit can be triggered by a GPUI-native command/action without a DOM button.

### External/server error behavior

- [x] `Form::errors(...)` marks matching named fields invalid.
- [x] `Form::errors(...)` populates matching `FieldError` and `FieldValidity` data with one or multiple messages.
- [x] Fields with no matching external error are not marked invalid by Form.
- [x] Updating the external errors map updates Field error presence.
- [x] Editing a field clears that field's external error before or as validation reruns, matching Base UI's `clearErrors(name)` behavior.
- [x] Clearing one field's external error does not clear sibling field errors.
- [x] External errors and custom Field validation errors combine deterministically.
- [x] If a field has both a controlled `invalid(true)` prop and Form external error state, controlled invalid continues to make the field invalid until the user clears/changes the controlling prop.
- [x] Same-name field external errors use deterministic name-keyed scoping: every mounted field with the matching effective name receives the external error.

### Form value model

- [x] Add `FormValue` or equivalent Rust-native value enum.
- [x] Add `FormValues` map type keyed by `SharedString` or equivalent.
- [x] Text Input / FieldControl values collect as `FormValue::Text(...)` or equivalent.
- [x] Checkbox and Switch values collect as bool values in a documented way.
- [x] Radio Group values collect as `Present`/`Empty` without arbitrary JavaScript value semantics.
- [x] Number Field values currently collect as formatted text; numeric `FormValue::Number` collection remains a follow-up until Number Field exposes a numeric form value cleanly.
- [x] Empty values are represented explicitly and consistently.
- [x] Disabled named fields are omitted from collected values.
- [x] Unnamed fields are omitted from collected values.
- [x] Form values are computed from current registered field values at submit time, not cached stale values.

### Fieldset follow-up

- [x] Create or reference a separate `Fieldset` issue before implementing Base UI Fieldset-specific disabled propagation (`issues/port-baseui-fieldset.md`).
- [ ] Once Fieldset exists, disabled fieldsets should cause descendant fields to skip validation and value collection.
- [ ] Once Fieldset exists, invalid UI should clear when a field becomes disabled through Fieldset.
- [x] Do not fake DOM `<fieldset disabled>` behavior inside Form before a GPUI-native Fieldset exists.

### Styling/state exposure

- [x] Add `FormRenderState` even if initially empty, matching Base UI's empty `Form.State` while preserving `style_with_state(...)` extensibility.
- [x] `Form::style_with_state(...)` works with `FormRenderState`.
- [x] Do not expose DOM data attributes as the styling API.
- [x] Do not expose CSS variable names as the styling API.
- [x] The docs hero pattern can be recreated with GPUI builder methods: form, fields, labels, inputs, errors, and submit trigger layout.

### Accessibility follow-up

- [ ] Once GPUI AccessKit APIs support form semantics, expose relevant form/field invalid/required/focused state through GPUI-native accessibility APIs.
- [ ] Once available, ensure focusing first invalid field is represented correctly in the accessibility tree.
- [ ] Add accessibility tests if GPUI exposes AccessKit tree test helpers.
- [x] Do not port DOM `aria-invalid`, `aria-describedby`, generated HTML ids, or browser form roles literally.

### Tests / verification

Add behavior tests under `crates/base_gpui/src/form/tests/` where practical.

- [x] `Form::new()` renders arbitrary children.
- [x] Submit with an empty required Input does not call `on_form_submit` and shows a Field error.
- [x] Submit with valid Input calls `on_form_submit` with collected values.
- [x] Submit with an unnamed invalid Switch blocks submit but omits unnamed values.
- [x] Changing an invalid required Switch clears invalid state and allows later submit.
- [x] Same-name fields keep validity scoped on submit.
- [x] Removed fields are pruned and no longer block submit.
- [x] Disabled field/control is skipped for validation and values.
- [x] Re-enabled field/control can register again and block submit if invalid.
- [x] External errors mark matching fields invalid and populate FieldError.
- [x] External errors clear on field change without clearing other fields' errors.
- [x] External error focus behavior focuses first invalid field only after submit/validate.
- [x] Form validation mode is inherited by fields.
- [x] Field validation mode overrides Form validation mode.
- [x] OnSubmit fields revalidate on change after a failed submit attempt.
- [x] Manual validate-all action validates all fields.
- [x] Manual validate-by-name action validates only matching named fields.
- [x] `on_form_submit` does not run when the form is invalid.
- [x] Number Field contributes its value to `FormValues` according to the implemented value model.
- [x] Checkbox/Switch/Radio Group contribute values according to the implemented value model.
- [x] `Form::style_with_state(...)` receives render state.
- [x] Form demo compiles.
- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui form` passes.
- [x] `cargo test -p base_gpui` passes.
