# Port Base UI Radio Group to GPUI

## Problem

Base UI Radio Group provides mutually exclusive selection across a set of radio buttons. It combines a shared group value, individual radio roots, optional radio indicators, controlled/uncontrolled value state, cancelable value-change notifications, disabled/read-only/required behavior, roving focus, arrow-key navigation, Space activation, styling state, and field/form integration.

`crates/base_gpui` currently has no Radio Group component family. The goal is to port Radio Group behavior into GPUI-native components using the current `base_gpui` runtime/context/layers architecture, not to copy React, DOM, hidden-input, composite, or ARIA implementation details.

Radio values should be generic, using a Rust type parameter constrained around clone/equality semantics, e.g. `T: Clone + Eq + 'static`. No selection should be represented as `None`. If callers need a Base UI-style `null` radio value, they can use `T = Option<U>`, allowing `Some(None)` to mean the selected radio's value is null while `None` still means no radio is selected.

## Scope

Port the Radio Group component family from Base UI into GPUI-native components:

- `RadioGroupRoot<T>`
- `RadioGroupRadio<T>`
- `RadioGroupIndicator`

Base UI reference files:

- `/home/luke/Projects/base-ui/packages/react/src/radio-group/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/radio-group/RadioGroup.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/radio-group/RadioGroupContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/radio-group/RadioGroupDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/radio-group/RadioGroup.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/radio-group/RadioGroup.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/radio/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/radio/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/radio/root/RadioRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/radio/root/RadioRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/radio/root/RadioRootDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/radio/root/RadioRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/radio/root/RadioRoot.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/radio/indicator/RadioIndicator.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/radio/indicator/RadioIndicatorDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/radio/indicator/RadioIndicator.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/radio/utils/stateAttributesMapping.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/radio/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/radio/types.md`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/radio/types.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/radio-group/types.md`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/radio-group/types.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/radio/demos/hero/css-modules/index.tsx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/radio/demos/hero/tailwind/index.tsx`

Current GPUI implementation:

- No `crates/base_gpui/src/radio_group/` implementation exists yet.

Expected GPUI implementation files:

- `crates/base_gpui/src/radio_group/mod.rs`
- `crates/base_gpui/src/radio_group/actions.rs`
- `crates/base_gpui/src/radio_group/child.rs`
- `crates/base_gpui/src/radio_group/child_wiring.rs`
- `crates/base_gpui/src/radio_group/context.rs`
- `crates/base_gpui/src/radio_group/props.rs`
- `crates/base_gpui/src/radio_group/render_state.rs`
- `crates/base_gpui/src/radio_group/runtime.rs`
- `crates/base_gpui/src/radio_group/layers/mod.rs`
- `crates/base_gpui/src/radio_group/layers/radio_group_root.rs`
- `crates/base_gpui/src/radio_group/layers/radio_group_radio.rs`
- `crates/base_gpui/src/radio_group/layers/radio_group_indicator.rs`
- `crates/base_gpui/src/radio_group/tests/`

Use `crates/base_gpui/src/tabs/` for registration, roving-focus, child-wiring, and generic value precedents. Use `crates/base_gpui/src/switch/` for cancelable Rust-native change details. Use `issues/port-baseui-direction-provider.md` for shared ambient LTR/RTL direction behavior.

## Out of scope / drop from Base UI

- Do not port React context/hooks directly; use GPUI keyed state/entities plus a thin `RadioGroupContext<T>` wrapper.
- Do not port Base UI's internal `CompositeRoot` / `CompositeItem` implementation literally; translate it into GPUI runtime registration, focus handles, and key-dispatch actions.
- Do not port `render` props.
- Do not port `className`.
- Do not port web `style` props; expose GPUI builder styling and state-aware styling APIs.
- Do not port `nativeButton`; GPUI does not currently expose a native button element in `crates/gpui/src/elements`.
- Do not port hidden DOM input behavior literally.
- Do not port browser form submission details literally; only add GPUI-native field/form integration when those primitives exist in `base_gpui`.
- Do not port DOM label association behavior literally; future `Field`, `Fieldset`, and `Label` primitives should provide GPUI-native labeling.
- Do not port arbitrary HTML attributes or DOM event props.
- Do not port SSR/hydration/prehydration APIs.
- Do not port CSS variable APIs.
- Do not port DOM data attributes as attributes; map them into typed render-state structs.
- Do not port arbitrary DOM event objects. Use Rust-native change details for cancellation, reason, and source information.
- Do not write DOM ARIA attributes. Map accessibility through GPUI-native AccessKit APIs once available.

## Acceptance Criteria

### Module/API surface

- [x] Add a `radio_group` module and export it from `crates/base_gpui/src/lib.rs`.
- [x] Register Radio Group key bindings from `base_gpui::init(cx)`.
- [x] Add public `RadioGroupRoot<T>`, `RadioGroupRadio<T>`, and `RadioGroupIndicator` layer types.
- [x] Add a typed `RadioGroupChild<T>` enum that routes radio and indicator children before `AnyElement` erasure.
- [x] Add any nested child enum needed so `RadioGroupRadio<T>` can route `RadioGroupIndicator` children before `AnyElement` erasure.
- [x] Keep `RadioGroupRoot<T>` children typed; do not add an `AnyElement` escape hatch unless examples require arbitrary Radio Group root children.
- [x] Support uncontrolled construction with `.default_value(Option<T>)`, defaulting to `None`.
- [x] Support controlled construction with `.value(Option<T>)`; calling the builder marks the root controlled even when the supplied value is `None`.
- [x] Support `.disabled(bool)`, defaulting to `false`.
- [x] Support `.read_only(bool)`, defaulting to `false`.
- [x] Support `.required(bool)`, defaulting to `false`.
- [x] Support `.on_value_change(...)` with a Rust-native cancelable change-details API, e.g. `Fn(Option<&T>, &mut RadioGroupValueChangeDetails, &mut Window, &mut App)`.
- [x] Add `RadioGroupValueChangeDetails` with `reason()`, `source()`, `cancelable()`, `cancel()`, and `is_canceled()` APIs.
- [x] Add `RadioGroupValueChangeReason::None`, matching Base UI's current single change reason.
- [x] Add a source enum such as `RadioGroupValueChangeSource::{Pointer, Keyboard}` without exposing DOM event objects.
- [x] Add `RadioGroupRadio<T>::value(T)`; values are generic and constrained as `T: Clone + Eq + 'static`.
- [x] Add `RadioGroupRadio<T>::disabled(bool)`, defaulting to `false`.
- [x] Add `RadioGroupRadio<T>::read_only(bool)`, defaulting to `false`.
- [x] Add `RadioGroupRadio<T>::required(bool)`, defaulting to `false`.
- [x] Add `RadioGroupIndicator::keep_mounted(bool)`, defaulting to `false`.
- [x] Preserve form-related builder props only where useful for future GPUI field/form integration: `.name(...)` and `.form(...)`.
- [x] Consume the shared ambient direction primitive from `issues/port-baseui-direction-provider.md` instead of adding a one-off Radio Group `.direction(...)` builder.
- [x] `radio_group/mod.rs` exposes ergonomic barrel exports for component names, render states, context, props, runtime, actions, and child types.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui radio_group` passes.
- [x] The component compiles without adding web/React-specific concepts to public APIs.
- [x] The implementation follows `docs/base-gpui-component-architecture.md` and does not introduce the old `child/context/{props,runtime,state}` taxonomy.
- [x] Add a small example/demo in `crates/base_gpui/src/main.rs` or a dedicated example that renders a Radio Group with indicators.

### Architecture / internal primitives

- [x] Add `RadioGroupRuntime<T>` as the single owner of Radio Group business state: selected value, registered radios, highlighted/focused radio index, focus handles, touched/focused facts if implemented, and any derived field-style state.
- [x] Add `RadioGroupProps<T>` for stable root props and callbacks.
- [x] Add `RadioGroupContext<T>` as a thin injection/plumbing type with only `read(...)`, `update(...)`, and `select(...)`-style methods.
- [x] Keep controlled/uncontrolled resolution in `RadioGroupContext<T>::select(...)`, not in layers.
- [x] Keep Radio Group behavior on `RadioGroupRuntime<T>`; do not grow component vocabulary on `RadioGroupContext<T>` beyond value-changing methods.
- [x] Add `RadioGroupRootRenderState`, `RadioGroupRadioRenderState`, and `RadioGroupIndicatorRenderState` in `render_state.rs`.
- [x] Add renderable GPUI elements only under `radio_group/layers/`.
- [x] Add typed child routing in `radio_group/child.rs` and private context/registration wiring in `radio_group/child_wiring.rs`.
- [x] Pre-register radio metadata before root reconciliation/render state queries so initial selected/focused states are correct.
- [x] Store radio metadata in source order: value, disabled, read-only, required, index, and focus handle.
- [x] Do not add a `utils/` folder for Radio Group.
- [x] Do not add new generic primitives unless they hide a repeated deep concept across components.

### Stateful/stateless behavior

- [x] Uncontrolled Radio Group initializes selected value from `default_value`.
- [x] Uncontrolled Radio Group defaults to no selected value when `default_value` is `None`.
- [x] Uncontrolled Radio Group mutates internal selected value on accepted user selection.
- [x] Controlled Radio Group reflects the external `value`.
- [x] Controlled Radio Group supports externally controlled `None` as no selected radio.
- [x] Controlled Radio Group calls `on_value_change` on valid user selection without mutating internal selected value as the source of truth.
- [x] External controlled value changes update group, radio, and indicator render state.
- [x] `None` means no radio is selected.
- [x] A radio value of `T = Option<U>` supports Base UI-style null values as `Some(None)`, distinct from no selection.
- [x] If the selected value does not match any mounted radio, no radio is checked and the selected value is otherwise preserved.
- [x] If the selected radio becomes disabled, it may remain selected visually, but disabled radios cannot be newly selected by user interaction.
- [x] Radio Group does not automatically fall back to a different selected value when the selected radio is removed, missing, or disabled.
- [x] Re-rendering with changed props does not reset uncontrolled state except when the keyed root id changes.

### Change event behavior

- [x] User selection computes the next selected value from the target radio's `value`.
- [x] `on_value_change` is called exactly once per accepted user selection.
- [x] Selecting the already selected radio is a no-op and does not call `on_value_change`.
- [x] `on_value_change` is not called for disabled or read-only selection attempts.
- [x] Expose `RadioGroupValueChangeReason::None`, matching Base UI's current single change reason.
- [x] Expose pointer vs keyboard source without exposing DOM event objects.
- [x] User-initiated change details are cancelable.
- [x] Uncontrolled mode calls the callback before mutating internal state and skips mutation when canceled.
- [x] Controlled mode calls the callback but never mutates internal selected state; cancellation only confirms that Base GPUI should perform no internal state change.
- [x] Canceled arrow-key selection still moves focus/highlight to the navigated radio, matching Base UI's cancellation behavior.
- [x] Do not expose Base UI's native `event`, `trigger`, or propagation APIs literally.

### Pointer interaction behavior

- [x] Clicking an enabled, non-read-only, unchecked radio selects it.
- [x] Clicking the already checked radio is a no-op.
- [x] Clicking a disabled group radio does not select and does not call the change callback.
- [x] Clicking a disabled radio does not select and does not call the change callback.
- [x] Clicking a read-only group radio does not select and does not call the change callback.
- [x] Clicking a read-only radio does not select and does not call the change callback.
- [x] Pointer selection and keyboard selection share the same runtime selection command.
- [x] Pointer activation uses GPUI click events only as an input source; no browser event object leaks into the public API.

### Keyboard/focus behavior

- [x] `RadioGroupRadio<T>` owns a stable keyed `FocusHandle`.
- [x] Radio Group uses GPUI actions/key dispatch and a Radio Group key context instead of raw DOM-style key handlers.
- [x] Initial tab stop is the checked enabled radio when one exists.
- [x] Initial tab stop is the first enabled radio when no checked enabled radio exists.
- [x] Disabled radios are skipped by roving focus.
- [x] ArrowDown moves focus/highlight to the next enabled radio and requests selection.
- [x] ArrowUp moves focus/highlight to the previous enabled radio and requests selection.
- [x] In ambient LTR direction, ArrowRight moves to the next enabled radio and ArrowLeft moves to the previous enabled radio.
- [x] In ambient RTL direction, ArrowLeft moves to the next enabled radio and ArrowRight moves to the previous enabled radio.
- [x] Arrow navigation wraps at the ends, matching Base UI behavior.
- [x] Shift-modified arrow keys still move focus like unmodified arrow keys.
- [x] Home and End do not navigate radios because Base UI disables Home/End for Radio Group.
- [x] Space selects the focused enabled, non-read-only radio exactly once.
- [x] Space should not double-fire if GPUI exposes both key-down and key-up style dispatch.
- [x] Enter does not select a radio.
- [x] Keyboard navigation marks the group as touched/focused if those render-state fields are tracked.
- [x] After tabbing away and back, focus returns to the current roving tab stop rather than always resetting to the first radio.

### Radio behavior

- [x] `RadioGroupRadio<T>` computes checked state by comparing its value with the group's selected value.
- [x] `RadioGroupRadio<T>` merges group-level and radio-level disabled state.
- [x] `RadioGroupRadio<T>` merges group-level and radio-level read-only state.
- [x] `RadioGroupRadio<T>` merges group-level and radio-level required state.
- [x] `RadioGroupRadio<T>` exposes checked, unchecked, disabled, read-only, required, focused, and highlighted/tab-stop state for styling.
- [x] `RadioGroupRadio<T>` can be used with no indicator child.
- [x] Do not forward the radio value into any generic GPUI element attribute-like API.

### Indicator behavior

- [x] `RadioGroupIndicator` renders when its parent radio is checked.
- [x] `RadioGroupIndicator` does not render when its parent radio is unchecked by default.
- [x] `RadioGroupIndicator` supports `keep_mounted` so it remains rendered when unchecked.
- [x] Indicator render state includes its parent radio state.
- [x] Indicator render state exposes whether it is present.
- [x] If transition support exists in `base_gpui`, expose a GPUI-native transition status; otherwise track transition behavior as a follow-up rather than porting DOM animation attributes.

### Styling/state exposure

- [x] `RadioGroupRootRenderState` includes at least `disabled`, `read_only`, `required`, and focused/touched/dirty/filled/valid fields when those concepts exist in `base_gpui`.
- [x] `RadioGroupRadioRenderState` includes at least `checked`, `unchecked`, `disabled`, `read_only`, `required`, `focused`, and tab-stop/highlighted facts.
- [x] `RadioGroupIndicatorRenderState` includes the parent radio render state and indicator presence.
- [x] Expose state-aware styling through `style_with_state(...)` on root, radio, and indicator.
- [x] Map Base UI state/data attributes (`checked`, `unchecked`, `disabled`, `readonly`, `required`, transition status, and field validity states when available) into typed render-state fields, not DOM attributes.
- [x] Do not expose CSS variable names as the styling API.
- [x] The docs hero styling pattern can be recreated with GPUI builder methods: checked radio changes background/text color and indicator visibility follows checked state.

### Accessibility follow-up

- [ ] Once the target GPUI revision supports the needed AccessKit APIs, map root semantics to `Role::RadioGroup`.
- [ ] Once available, map radio semantics to `Role::RadioButton` or the closest GPUI/AccessKit role.
- [ ] Once available, expose checked/unchecked state through GPUI-native accessibility APIs.
- [ ] Once available, expose disabled/read-only/required state through GPUI-native accessibility APIs.
- [ ] Decide how GPUI Radio Group gets an accessible name through future label/fieldset APIs; do not port DOM `aria-labelledby`/`id` linking literally.
- [ ] Add accessibility tests if GPUI exposes test helpers for AccessKit state.

### Field/form integration follow-up

- [ ] Decide how Radio Group should compose with future GPUI `Field`, `FieldItem`, `Fieldset`, `Label`, `Description`, and `Form` components.
- [ ] Decide whether `name`, `form`, and input-ref concepts have GPUI-native meaning before implementing submission behavior.
- [ ] Preserve enough state to integrate with future field validation: required, focused, touched, dirty, filled, and valid/invalid when those concepts exist.
- [ ] Required validation should eventually match Base UI behavior: no selected radio is invalid, selected radio is valid, and disabled selected radios are excluded from submitted form value.
- [ ] Do not implement hidden DOM radio input submission behavior in GPUI.

### Tests / verification

Add one behavior per file under `crates/base_gpui/src/radio_group/tests/`.

- [x] Uncontrolled initial no-selection state.
- [x] Uncontrolled `default_value` initial selected state.
- [x] Controlled selected state reflects external value.
- [x] Controlled `None` value clears selection.
- [x] External controlled value changes update radio and indicator render state.
- [x] `T = Option<U>` can distinguish no selection from a selected null-like value.
- [x] Click selects an unchecked radio.
- [x] Clicking the already selected radio is a no-op.
- [x] `on_value_change` is called with the next value.
- [x] Disabled group click does not select and does not call the change handler.
- [x] Disabled radio click does not select and does not call the change handler.
- [x] Read-only group click does not select and does not call the change handler.
- [x] Read-only radio click does not select and does not call the change handler.
- [x] Canceled uncontrolled pointer activation does not mutate selected value.
- [x] Canceled uncontrolled keyboard activation does not mutate selected value.
- [x] Canceled controlled activation still calls the handler but does not mutate internal selected value.
- [x] Canceled arrow-key selection moves focus/highlight but does not check the navigated radio.
- [x] ArrowDown/ArrowUp navigation moves focus and selects.
- [x] DirectionProvider-wrapped LTR ArrowRight/ArrowLeft navigation moves focus and selects.
- [x] DirectionProvider-wrapped RTL ArrowLeft/ArrowRight navigation moves focus and selects.
- [x] Arrow navigation wraps at the ends.
- [x] Shift-modified arrow navigation still moves focus.
- [x] Home and End do not navigate/select.
- [x] Space selects focused radio exactly once.
- [x] Enter does not select focused radio.
- [x] Disabled radios are skipped by roving focus.
- [x] Initial tab stop is checked radio when one exists.
- [x] Initial tab stop is first enabled radio when no checked radio exists.
- [x] Radio state exposes checked/unchecked.
- [x] Radio state exposes disabled/read-only/required.
- [x] Root `style_with_state(...)` receives correct group state.
- [x] Radio `style_with_state(...)` receives correct radio state.
- [x] Indicator `style_with_state(...)` receives correct indicator state.
- [x] Indicator is absent by default when unchecked.
- [x] Indicator renders when checked.
- [x] Indicator remains rendered with `keep_mounted`.
- [ ] Form/field integration tests are added later when GPUI-native `Field`/`Form` primitives exist.
