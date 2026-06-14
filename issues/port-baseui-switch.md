# Port Base UI Switch to GPUI

## Problem

Base UI Switch provides a binary on/off control with controlled and uncontrolled checked state, pointer and keyboard activation, disabled/read-only/required handling, a thumb subpart that shares root state for styling, cancelable `onCheckedChange` notifications, and form/field integration through a hidden input.

`crates/base_gpui` currently has no Switch component family. The goal is to port Switch behavior into GPUI-native components using the current `base_gpui` runtime/context/layers architecture, not to copy React, DOM, hidden-input, or ARIA implementation details.

Switch state is boolean. Controlled state should use `Option<bool>` in the GPUI API, where `None` means uncontrolled and `Some(value)` means the caller owns the checked value.

## Scope

Port the Switch component family from Base UI into GPUI-native components:

- `SwitchRoot`
- `SwitchThumb`

Base UI reference files:

- `/home/luke/Projects/base-ui/packages/react/src/switch/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/switch/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/switch/root/SwitchRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/switch/root/SwitchRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/switch/root/SwitchRootDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/switch/root/SwitchRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/switch/thumb/SwitchThumb.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/switch/thumb/SwitchThumbDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/switch/thumb/SwitchThumb.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/switch/stateAttributesMapping.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/switch/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/switch/types.md`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/switch/types.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/switch/demos/hero/css-modules/index.tsx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/switch/demos/hero/tailwind/index.tsx`

Current GPUI implementation:

- `crates/base_gpui/src/switch/mod.rs`
- `crates/base_gpui/src/switch/actions.rs`
- `crates/base_gpui/src/switch/child.rs`
- `crates/base_gpui/src/switch/child_wiring.rs`
- `crates/base_gpui/src/switch/context.rs`
- `crates/base_gpui/src/switch/props.rs`
- `crates/base_gpui/src/switch/render_state.rs`
- `crates/base_gpui/src/switch/runtime.rs`
- `crates/base_gpui/src/switch/layers/mod.rs`
- `crates/base_gpui/src/switch/layers/switch_root.rs`
- `crates/base_gpui/src/switch/layers/switch_thumb.rs`
- `crates/base_gpui/src/switch/tests/`

Use `crates/base_gpui/src/checkbox/` as the closest implementation precedent, but do not copy Checkbox-only behavior such as `indeterminate` state or indicator conditional mounting.

## Out of scope / drop from Base UI

- Do not port React context/hooks directly; use GPUI keyed state/entities plus a thin `SwitchContext` wrapper.
- Do not port `render` props.
- Do not port `className`.
- Do not port web `style` props; expose GPUI builder styling and state-aware styling APIs.
- Do not port `nativeButton`; GPUI does not currently expose a native button element in `crates/gpui/src/elements`.
- Do not port hidden DOM input behavior literally.
- Do not port browser form submission details literally; only add GPUI-native field/form integration when those primitives exist in `base_gpui`.
- Do not port DOM label association behavior literally; future `Field`/`Label` primitives should provide GPUI-native labeling.
- Do not port arbitrary HTML attributes or DOM event props such as browser `onClick` objects.
- Do not port SSR/hydration/prehydration APIs.
- Do not port CSS variable APIs.
- Do not port DOM data attributes as attributes; map them into typed render-state structs.
- Do not port arbitrary DOM event objects. Use Rust-native change details when cancellation/source information is needed.
- Do not write DOM ARIA attributes. Map accessibility through GPUI-native AccessKit APIs once available.

## Acceptance Criteria

### Module/API surface

- [x] Add a `switch` module and export it from `crates/base_gpui/src/lib.rs`.
- [x] Register Switch key bindings from `base_gpui::init(cx)`.
- [x] Add public `SwitchRoot` and `SwitchThumb` layer types.
- [x] Add a typed `SwitchChild` enum that routes `SwitchThumb` children before `AnyElement` erasure.
- [x] Keep `SwitchRoot` children typed; do not add an `AnyElement` escape hatch unless examples require arbitrary Switch root children.
- [x] Support uncontrolled construction with `.default_checked(bool)`, defaulting to `false`.
- [x] Support controlled construction with `.checked(Option<bool>)`; controlled state takes precedence over internal state.
- [x] Support `.disabled(bool)`, defaulting to `false`.
- [x] Support `.read_only(bool)`, defaulting to `false`.
- [x] Support `.required(bool)`, defaulting to `false`.
- [x] Support `.on_checked_change(...)` with a Rust-native cancelable change-details API, e.g. `Fn(bool, &mut SwitchCheckedChangeDetails, &mut Window, &mut App)`.
- [x] Add `SwitchCheckedChangeDetails` with `reason()`, `source()`, `cancelable()`, `cancel()`, and `is_canceled()` APIs.
- [x] Add `SwitchCheckedChangeReason::None`, matching Base UI's current single change reason.
- [x] Add a source enum such as `SwitchCheckedChangeSource::{Pointer, Keyboard}` without exposing DOM event objects.
- [x] Preserve form-related builder props only where useful for future GPUI field/form integration: `.name(...)`, `.value(...)`, `.form(...)`, and `.unchecked_value(...)`.
- [x] `switch/mod.rs` exposes ergonomic barrel exports for component names, render states, context, props, runtime, actions, and child types.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui switch` passes.
- [x] The component compiles without adding web/React-specific concepts to public APIs.
- [x] The implementation follows `docs/base-gpui-component-architecture.md` and does not introduce the old `child/context/{props,runtime,state}` taxonomy.
- [x] Add a small example/demo in `crates/base_gpui/src/main.rs` or a dedicated example that renders a Switch with a thumb.

### Architecture / internal primitives

- [x] Add `SwitchRuntime` as the single owner of Switch business state: checked value, focused state, and any derived field-style state that is implemented.
- [x] Add `SwitchProps` for stable root props and callbacks.
- [x] Add `SwitchContext` as a thin injection/plumbing type with only `read(...)`, `update(...)`, and `toggle(...)`-style methods.
- [x] Keep controlled/uncontrolled resolution in `SwitchContext::toggle(...)`, not in layers.
- [x] Keep Switch behavior on `SwitchRuntime`; do not grow component vocabulary on `SwitchContext` beyond the value-changing method.
- [x] Add `SwitchRootRenderState` and `SwitchThumbRenderState` in `render_state.rs`.
- [x] Add renderable GPUI elements only under `switch/layers/`.
- [x] Add typed child routing in `switch/child.rs` and private context attachment in `switch/child_wiring.rs`.
- [x] Do not add a `utils/` folder for Switch.
- [x] Do not add new generic primitives unless they hide a repeated deep concept across components.

### Stateful/stateless behavior

- [x] Uncontrolled Switch initializes checked state from `default_checked`, defaulting to unchecked.
- [x] Uncontrolled Switch toggles internal checked state on valid user activation.
- [x] Controlled Switch reflects the external `checked` value.
- [x] Controlled Switch calls `on_checked_change` on valid user activation without mutating internal checked state as the source of truth.
- [x] External controlled value changes update root and thumb render state.
- [x] Disabled Switch ignores user activation and does not call `on_checked_change`.
- [x] Read-only Switch ignores user activation and does not call `on_checked_change`.
- [x] Required Switch exposes `required` in render state even if validation is a future field/form concern.
- [x] Re-rendering with changed props does not reset uncontrolled state except when the keyed root id changes.

### Change event behavior

- [x] User activation computes the next checked value as `!current_checked`.
- [x] `on_checked_change` is called exactly once per accepted user activation.
- [x] `on_checked_change` is not called for disabled or read-only activation attempts.
- [x] Expose `SwitchCheckedChangeReason::None`, matching Base UI's current single change reason.
- [x] Expose pointer vs keyboard source without exposing DOM event objects.
- [x] User-initiated change details are cancelable.
- [x] Uncontrolled mode calls the callback before mutating internal state and skips mutation when canceled.
- [x] Controlled mode calls the callback but never mutates internal state; cancellation only confirms that Base GPUI should perform no internal state change.
- [x] Do not expose Base UI's native `event`, `trigger`, or propagation APIs literally.

### Pointer interaction behavior

- [x] Clicking an enabled, non-read-only root toggles unchecked to checked.
- [x] Clicking an enabled, non-read-only root toggles checked to unchecked.
- [x] Clicking a disabled root does not toggle and does not call the change callback.
- [x] Clicking a read-only root does not toggle and does not call the change callback.
- [x] Pointer activation and keyboard activation share the same runtime toggle command.
- [x] Pointer activation uses GPUI click events only as an input source; no browser event object leaks into the public API.

### Keyboard/focus behavior

- [x] `SwitchRoot` owns a stable keyed `FocusHandle`.
- [x] `SwitchRoot` is focusable when enabled.
- [x] Focused state is synced into `SwitchRuntime` and exposed through render state.
- [x] Switch uses GPUI actions/key dispatch and a `SWITCH_ROOT_KEY_CONTEXT` instead of raw DOM-style key handlers.
- [x] Space toggles an enabled, non-read-only Switch when focused.
- [x] Enter toggles an enabled, non-read-only Switch when focused.
- [x] Disabled Switch does not toggle from keyboard activation.
- [x] Read-only Switch does not toggle from keyboard activation.
- [x] Decide whether disabled Switch should be removed from tab order initially, matching current Checkbox behavior, or revisited with AccessKit accessibility work.

### Thumb behavior

- [x] `SwitchThumb` renders inside `SwitchRoot` and receives root Switch context.
- [x] `SwitchThumb` always renders when present; do not add Checkbox-style indicator presence or `keep_mounted` semantics.
- [x] `SwitchThumb` exposes the same checked/unchecked, disabled, read-only, required, and focused state needed for state-aware styling.
- [x] `SwitchThumb` supports `style_with_state(...)`.
- [x] `SwitchThumb` can be omitted without breaking root behavior.

### Styling/state exposure

- [x] `SwitchRootRenderState` includes at least `checked`, `unchecked`, `disabled`, `read_only`, `required`, and `focused`.
- [x] `SwitchThumbRenderState` includes the root render state or equivalent fields.
- [x] Expose state-aware styling through `style_with_state(...)` on root and thumb.
- [x] Map Base UI state/data attributes (`checked`, `unchecked`, `disabled`, `readonly`, `required`, and focused/field states when available) into typed render-state fields, not DOM attributes.
- [x] Do not expose CSS variable names as the styling API.
- [x] The docs hero styling pattern can be recreated with GPUI builder methods: root background changes when checked, thumb position/color changes when checked, and focus styling can use the focused render state.

### Accessibility follow-up

- [ ] Once the target GPUI revision supports the needed AccessKit APIs, map root semantics to the GPUI/AccessKit switch role.
- [ ] Once available, expose checked/unchecked state through GPUI-native accessibility APIs.
- [ ] Once available, expose disabled/read-only/required state through GPUI-native accessibility APIs.
- [ ] Decide how GPUI Switch gets an accessible name through future label/field APIs; do not port DOM `aria-labelledby`/`id` linking literally.
- [ ] Add accessibility tests if GPUI exposes test helpers for AccessKit state.

### Field/form integration follow-up

- [ ] Decide how Switch should compose with a future GPUI `Field` component.
- [ ] Decide whether `name`, `value`, `form`, and `unchecked_value` have GPUI-native meaning before implementing submission behavior.
- [ ] Preserve enough state to integrate with future field validation: required, focused, touched, dirty, filled, and valid/invalid when those concepts exist.
- [ ] Required validation should eventually match Base UI's behavior: unchecked required switch is invalid, checked required switch is valid.
- [x] Do not implement hidden DOM input submission behavior in GPUI.

### Tests / verification

Add one behavior per file under `crates/base_gpui/src/switch/tests/`.

- [x] Uncontrolled initial unchecked state.
- [x] Uncontrolled `default_checked` initial checked state.
- [x] Controlled checked state reflects external state.
- [x] External controlled value changes update root and thumb render state.
- [x] Click toggles unchecked to checked.
- [x] Click toggles checked to unchecked.
- [x] `on_checked_change` is called with the next checked value.
- [x] Disabled click does not toggle and does not call the change handler.
- [x] Read-only click does not toggle and does not call the change handler.
- [x] Space toggles when focused.
- [x] Enter toggles when focused.
- [x] Disabled/read-only keyboard activation does not toggle.
- [x] Focused state appears in `SwitchRootRenderState` when focused and clears on blur.
- [x] Thumb receives checked state for styling.
- [x] Thumb receives unchecked state for styling.
- [x] Thumb receives disabled/read-only/required state for styling.
- [x] `style_with_state(...)` receives correct root state.
- [x] `style_with_state(...)` receives correct thumb state.
- [x] Canceled uncontrolled pointer activation does not mutate checked state.
- [x] Canceled uncontrolled keyboard activation does not mutate checked state.
- [x] Canceled controlled activation still calls the handler but does not mutate internal checked state.
- [ ] Form/field integration tests are added later when GPUI-native `Field`/`Form` primitives exist.
