# Port Base UI Checkbox Group to GPUI

## Problem

Base UI's Checkbox Group provides shared multi-selection state for a series of `Checkbox.Root` controls. It supports controlled and uncontrolled value arrays, parent checkboxes that toggle child values, cancellation-aware value changes, disabled propagation, field/form integration, and required validation across multiple checkboxes.

`crates/base_gpui` currently has a standalone `CheckboxRoot` / `CheckboxIndicator`, but no `CheckboxGroup` module. The existing `CheckboxRoot::parent(...)` and `CheckboxRoot::value(...)` APIs are not yet wired to any group context, and field/form values only model scalar controls.

The goal is to port Checkbox Group behavior using GPUI-native component architecture. Do not copy React hooks, DOM refs, hidden inputs, ARIA attributes, or browser form submission mechanics literally. Checkbox group values should use Rust-native ordered string values, e.g. `Vec<SharedString>`, because the existing GPUI Checkbox value API is `SharedString`-based and Base UI Checkbox Group is string-valued.

## Decisions

- Add general list-valued field/form variants for Checkbox Group values:
  - `FieldValue::List(Vec<SharedString>)`
  - `FormValue::List(Vec<SharedString>)`
- `FieldValue::List` should be considered filled when non-empty.
- Checkbox Group validation and Form submission should use the full ordered selected-value list, not representative scalar child checkbox values.
- Upgrade Checkbox change callbacks to Rust-native cancelable change details so Checkbox Group can preserve Base UI's cancellation order:
  - child checkbox callback can cancel before group changes,
  - parent checkbox callback can cancel before group changes,
  - group value callback can cancel uncontrolled group mutation.
- Prefer updating `CheckboxRoot::on_checked_change` to the richer details model rather than adding a parallel callback API.

## Scope

Port the Checkbox Group component family from Base UI into GPUI-native components:

- `CheckboxGroup`

Update existing related GPUI components where required:

- `CheckboxRoot`
- `CheckboxIndicator` only if parent/indeterminate style-state wiring needs adjustment
- `FieldRoot` / `FieldRuntime` validation and value model to support multi-checkbox required validation
- `FieldValue::List(Vec<SharedString>)` and `FormValue::List(Vec<SharedString>)` for ordered checkbox group values

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/checkbox-group/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/checkbox-group/CheckboxGroup.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/checkbox-group/CheckboxGroupContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/checkbox-group/CheckboxGroupDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/checkbox-group/useCheckboxGroupParent.ts`
- `/home/luke/Projects/base-ui/packages/react/src/checkbox-group/CheckboxGroup.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/checkbox-group/useCheckboxGroupParent.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/checkbox/root/CheckboxRoot.tsx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/checkbox-group/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/checkbox-group/types.md`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/checkbox-group/types.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/checkbox-group/demos/hero/css-modules/index.tsx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/checkbox-group/demos/parent/css-modules/index.tsx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/checkbox-group/demos/nested/css-modules/index.tsx`

Current GPUI implementation:

- No `crates/base_gpui/src/checkbox_group/` module exists yet.
- `crates/base_gpui/src/checkbox/` exists and should be reused.
- `CheckboxRoot` already has `value`, `name`, `parent`, `disabled`, `read_only`, `required`, and `indeterminate` builder APIs, but it does not currently consume Checkbox Group context.
- `CheckboxRoot::on_checked_change` has been upgraded to support Rust-native cancellation for child, parent, and group changes.

Expected GPUI implementation files:

- `crates/base_gpui/src/checkbox_group/mod.rs`
- `crates/base_gpui/src/checkbox_group/context.rs`
- `crates/base_gpui/src/checkbox_group/props.rs`
- `crates/base_gpui/src/checkbox_group/style_state.rs`
- `crates/base_gpui/src/checkbox_group/runtime.rs`
- `crates/base_gpui/src/checkbox_group/child.rs`
- `crates/base_gpui/src/checkbox_group/child_wiring.rs` if typed child/context wiring is needed
- `crates/base_gpui/src/checkbox_group/layers/checkbox_group.rs`
- `crates/base_gpui/src/checkbox_group/tests/`

## Out of scope / drop from Base UI

- Do not port React context/hooks directly; use GPUI keyed state/entities plus a `CheckboxGroupContext` wrapper or ambient GPUI context scope.
- Do not port `render` props.
- Do not port `className`.
- Do not port web `style` props; expose GPUI builder styling and `style_with_state(...)`.
- Do not port `nativeButton` or DOM element replacement behavior.
- Do not port hidden DOM input behavior literally.
- Do not port browser `FormData` mechanics literally; use GPUI `FormValues` / `FieldValue` instead.
- Do not port DOM refs or `HTMLButtonElement` registration; use GPUI focus handles and component metadata registration.
- Do not port DOM `aria-controls`, `aria-labelledby`, `aria-describedby`, `role="group"`, or generated DOM ids literally.
- Do not port DOM data attributes as attributes; map `data-disabled` and field validity state into typed style-state fields.
- Do not port arbitrary browser `Event` objects. Use Rust-native change detail structs with source/reason/cancel state where needed.
- Do not port CSS variable APIs.
- Do not add a generic shared primitive unless Checkbox Group genuinely reveals repeated deep knowledge.

## Acceptance Criteria

### Module/API surface

- [x] Add `checkbox_group` module and export/init it from `crates/base_gpui/src/lib.rs`.
- [x] Add public `CheckboxGroup` layer type with `CheckboxGroup::new()`.
- [x] `CheckboxGroup` supports `.child(...)`, `.children(...)`, and `.child_any(...)` for label/item/nested composition.
- [x] `CheckboxGroup` supports uncontrolled `.default_value(...)`, defaulting to an empty ordered list.
- [x] `CheckboxGroup` supports controlled `.value(...)`, where the external value is the source of truth.
- [x] `CheckboxGroup` supports `.on_value_change(...)` with Rust-native cancelable change details.
- [x] `CheckboxGroup` supports `.all_values(...)` for parent-checkbox behavior.
- [x] `CheckboxGroup` supports `.disabled(bool)`, defaulting to `false`.
- [x] `CheckboxGroup` supports `.style_with_state(...)`.
- [x] `CheckboxGroup` value APIs use ordered `SharedString` values, e.g. `Vec<SharedString>` or builder methods accepting `IntoIterator<Item = impl Into<SharedString>>`.
- [x] `checkbox_group/mod.rs` exposes ergonomic barrel exports.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui checkbox_group` passes.
- [x] `cargo test -p base_gpui checkbox` still passes after CheckboxRoot group integration.
- [x] `cargo test -p base_gpui field form` still passes after list-valued field/form changes.
- [x] The implementation compiles without adding React/DOM concepts to public APIs.
- [x] Add a small demo using Checkbox Group, including more than one child checkbox.

### Architecture / internal primitives

- [x] Follow `docs/base-gpui-component-architecture.md`: one deep `CheckboxGroupRuntime`, thin `CheckboxGroupContext`, thin render layer.
- [x] Keep all selection, parent-toggle, child metadata, disabled-child, and controlled/uncontrolled rules in `CheckboxGroupRuntime`.
- [x] Keep `CheckboxGroupContext` as injection/plumbing plus the value-changing method; do not leak runtime internals to render layers.
- [x] `CheckboxGroup` root is the single non-event mutation site for reconciliation and child registration pass setup/finish.
- [x] Use typed child routing where practical before `AnyElement` erasure, but preserve arbitrary children because Base UI examples place checkboxes inside labels/divs/Field items and nest Checkbox Groups.
- [x] Nested Checkbox Groups override the surrounding group context for their own descendants and do not leak state to siblings.
- [x] Do not add a `utils/` folder for component-local helpers.

### CheckboxRoot group integration

- [x] `CheckboxRoot` reads the current Checkbox Group context when present.
- [x] Non-parent grouped checkboxes derive checked state from group value membership instead of local uncontrolled state.
- [x] Non-parent grouped checkboxes require a `value` to participate in group value updates.
- [x] A grouped checkbox without `value` behaves deterministically and does not corrupt the group value.
- [x] `CheckboxRoot::disabled` combines root disabled, field disabled, field item disabled, fieldset disabled, and group disabled.
- [x] Group disabled state takes precedence over individual checkbox `disabled(false)`.
- [x] Child checkbox `read_only` still blocks user toggles without mutating the group.
- [x] Parent checkboxes are excluded from field/form value collection.
- [x] Grouped child checkboxes do not independently register scalar values with the surrounding `Field`; the group registers the representative list value.
- [x] `CheckboxRoot` reports child metadata to the group: value, disabled, required, parent flag, checked state, and focus handle.
- [x] Existing standalone Checkbox behavior remains unchanged outside a Checkbox Group.

### Change details / cancellation behavior

- [x] Upgrade `CheckboxRoot::on_checked_change` to receive Rust-native cancelable change details so a child or parent toggle can be canceled before the group mutates.
- [x] Add `CheckboxGroupValueChangeDetails` with at least reason `None`, cancelability, `cancel()`, and `is_canceled()`.
- [x] Child checkbox activation calls child `on_checked_change` before group `on_value_change`.
- [x] If child `on_checked_change` cancels, group `on_value_change` is not called and group value does not change.
- [x] If group `on_value_change` cancels, uncontrolled group state does not mutate.
- [x] Controlled groups call `on_value_change` but never mutate internal value as the source of truth.
- [x] Parent checkbox activation follows the same cancellation order: parent callback first, then group callback, then uncontrolled mutation only if accepted.
- [x] Do not expose browser event objects in change details.

### Stateful/stateless value behavior

- [x] Uncontrolled Checkbox Group initializes from `default_value`.
- [x] Omitted `default_value` initializes to an empty list.
- [x] Controlled Checkbox Group reflects external `value`.
- [x] External controlled value changes update child and parent style states.
- [x] Toggling an unchecked child appends its value to the current ordered group value.
- [x] Toggling a checked child removes its value while preserving the order of remaining values.
- [x] Re-selecting an already-present value does not create duplicate entries.
- [x] Removing a missing value is a no-op.
- [x] Values that no longer have mounted child checkboxes may remain in controlled or uncontrolled group value unless explicitly removed by user interaction.
- [x] Duplicate child values are treated deterministically as one logical selected value.

### Parent checkbox behavior

- [x] A `CheckboxRoot::parent(true)` inside a group uses group parent state instead of local checked state when `all_values` is present.
- [x] Parent style state is checked when every value in `all_values` is selected.
- [x] Parent style state is indeterminate when some but not all values in `all_values` are selected.
- [x] Parent style state is unchecked when no values in `all_values` are selected.
- [x] Clicking parent from all-off selects all enabled values from `all_values`.
- [x] Clicking parent from all-on clears all enabled values.
- [x] Clicking parent from mixed state first selects all enabled values.
- [x] Repeated parent clicks from an initial mixed snapshot cycle through all-on, all-off, and back to the original mixed snapshot, matching Base UI parent behavior.
- [x] An unchecked disabled child is not selected by the parent.
- [x] A checked disabled child remains selected when parent toggles all-on or all-off.
- [x] Parent checkbox does not call child checkbox change handlers for each child it affects; only the parent/group callbacks fire.
- [x] Parent behavior is a no-op or normal standalone checkbox behavior when no group/all-values context is available; it must not panic.

### Disabled/read-only behavior

- [x] `CheckboxGroup::disabled(true)` disables all descendant grouped checkboxes.
- [x] A disabled group ignores child and parent pointer activation.
- [x] A disabled group ignores child and parent keyboard activation.
- [x] Individual disabled children are omitted from parent all-on/all-off mutations except when preserving already-checked disabled values.
- [x] Disabled required child checkboxes are ignored by group required validation.
- [x] Read-only child checkboxes do not toggle and do not change group value.
- [x] Read-only parent checkboxes do not toggle group value.

### Field integration and validation behavior

- [x] A Checkbox Group inside `FieldRoot` registers a representative field control with list value.
- [x] Add `FieldValue::List(Vec<SharedString>)` to represent an ordered list of selected checkbox values.
- [x] `FieldValue::List` preserves selected value order and permits an empty list.
- [x] `FieldValue::filled()` returns true for a non-empty checkbox group list.
- [x] `FieldRuntime` dirty state compares the current group list against the initial group list.
- [x] `FieldRoot` `validation_mode` applies to Checkbox Group value changes.
- [x] `FieldRoot` `validate` callbacks receive the full selected value list.
- [x] `validation_mode = OnChange` revalidates after accepted child and parent group changes.
- [x] `validation_mode = OnBlur` revalidates when focus leaves the representative/active grouped checkbox.
- [x] `validation_mode = OnSubmit` validates the full group value on form submit.
- [x] Required validation stays invalid until every enabled required child checkbox in the group is checked.
- [x] Checking only one of multiple required children does not clear `value_missing` while another enabled required child is unchecked.
- [x] Disabled required children are ignored for required validation.
- [x] Required validation remains correct when a checked child unmounts and another required child remains unchecked.
- [x] Custom validation errors clear when the group becomes valid and return when the group becomes invalid again.
- [x] Group style state reflects field state: `touched`, `dirty`, `valid`, `invalid`, `filled`, `focused`, and `disabled`.
- [x] Group changes clear matching external form/server errors for the owning field without clearing sibling errors.
- [x] Form submit focuses the first enabled non-parent checkbox when the group field is invalid.

### Form value behavior

- [x] Add `FormValue::List(Vec<SharedString>)` to represent an ordered list of selected checkbox values.
- [x] Named Checkbox Group fields contribute the selected list to `FormValues`.
- [x] Empty selected lists are represented deterministically, not as stale scalar checkbox values.
- [x] Parent checkboxes are excluded from submitted values.
- [x] Disabled Checkbox Groups are skipped by Form validation and value collection.
- [x] Checkbox Groups disabled through Fieldset are skipped by Form validation and value collection.
- [x] Same-name behavior remains deterministic when a Checkbox Group shares a name with another field.

### Styling/state exposure

- [x] Add `CheckboxGroupStyleState` with disabled and field-derived state.
- [x] Expose state-aware styling through `CheckboxGroup::style_with_state(...)`.
- [x] Map Base UI `data-disabled` into `CheckboxGroupStyleState::disabled`.
- [x] Do not expose DOM data attributes, CSS class names, CSS variables, or web `style` APIs.
- [x] Parent checkbox indeterminate/checked state is exposed through existing `CheckboxRootStyleState`.
- [x] Child checkbox checked/disabled/read-only/required/indeterminate/focused state remains available through existing Checkbox style-state APIs.

### Keyboard/focus behavior

- [x] Child checkboxes keep standalone Checkbox keyboard semantics inside a group: Space toggles, Enter does not toggle.
- [x] Parent checkbox uses the same keyboard semantics as child checkboxes.
- [x] Disabled/read-only grouped checkboxes do not toggle from keyboard activation.
- [x] Focused state contributes to both child checkbox style state and group/field focused state.
- [x] Label-driven focus/click behavior already provided by `FieldLabel` continues to work for grouped checkboxes.
- [x] Use GPUI actions/key contexts; do not add DOM-style keydown logic.

### Nested Checkbox Groups

- [x] Nested Checkbox Groups maintain independent selected values.
- [x] Nested Checkbox Groups can be controlled independently.
- [x] A parent group's context does not accidentally toggle inner-group children unless the app explicitly bridges values through callbacks.
- [x] Nested parent checkboxes compute indeterminate state from their own `all_values`.
- [x] Demo coverage includes either parent-checkbox or nested-checkbox behavior.

### Accessibility follow-up

- [ ] Once the target GPUI revision supports the needed AccessKit APIs, map Checkbox Group semantics to a GPUI-native group role/accessible name plan.
- [ ] Once available, map CheckboxRoot semantics to AccessKit checkbox checked/mixed/disabled states in grouped and standalone modes.
- [ ] Add accessibility tests once GPUI exposes test helpers for AccessKit state.
- [x] Do not write DOM ARIA attributes while AccessKit support is unavailable.

### Tests / verification

Add behavior tests under `crates/base_gpui/src/checkbox_group/tests/`.

- [x] Uncontrolled default value initializes checked children.
- [x] Omitted default value initializes all children unchecked.
- [x] Controlled value reflects external state.
- [x] External controlled value changes update child states.
- [x] Child click appends/removes values in deterministic order.
- [x] Child click calls `on_value_change` with the next list.
- [x] Group cancellation prevents uncontrolled mutation.
- [x] Child callback cancellation prevents group callback and group mutation.
- [x] Parent callback cancellation prevents group callback and group mutation.
- [x] Disabled group disables all grouped checkboxes.
- [x] Disabled group activation does not call change handlers.
- [x] Individual disabled child is not toggled by parent all-on/all-off.
- [x] Checked disabled child remains checked through parent toggles.
- [x] Parent checked/unchecked/indeterminate style state is correct.
- [x] Parent all-on/all-off/mixed snapshot cycle is correct.
- [x] Parent changes do not call child change handlers.
- [x] Nested groups isolate state.
- [x] Group inside Field exposes filled/dirty/focused/touched state.
- [x] Required group validation requires every enabled required child.
- [x] Disabled required children are ignored.
- [x] Unmounted required children are pruned from validation metadata.
- [x] OnChange, OnBlur, and OnSubmit validation modes use the full group value.
- [x] Custom validation receives the full selected list.
- [x] External Form errors mark the group field invalid and clear on group change.
- [x] Form submit collects selected list values.
- [x] Parent checkbox is excluded from Form values.
- [x] Disabled/fieldset-disabled groups are skipped by Form validation and values.
- [x] `style_with_state(...)` receives correct Checkbox Group style state.
- [x] Existing `cargo test -p base_gpui checkbox` coverage remains green.
- [x] Existing `cargo test -p base_gpui field form` coverage remains green.
