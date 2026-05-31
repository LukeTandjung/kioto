# Port Base UI Checkbox to GPUI

## Problem

Base UI Checkbox provides a two-state checkbox with an optional indeterminate visual state, controlled/uncontrolled checked state, disabled/read-only handling, keyboard and pointer activation, an optional indicator child, and field/form integration.

`crates/base_gpui` currently has no Checkbox component family. The goal is to port Checkbox behavior into GPUI-native components using the existing `base_gpui` compound component architecture, not to copy React/DOM implementation details.

## Scope

Port the Checkbox component family from Base UI into GPUI-native components:

- `CheckboxRoot`
- `CheckboxIndicator`

Base UI reference files:

- `/home/luke/Projects/base-ui/packages/react/src/checkbox/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/checkbox/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/checkbox/root/CheckboxRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/checkbox/root/CheckboxRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/checkbox/root/CheckboxRootDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/checkbox/root/CheckboxRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/checkbox/indicator/CheckboxIndicator.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/checkbox/indicator/CheckboxIndicatorDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/checkbox/indicator/CheckboxIndicator.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/checkbox/utils/useStateAttributesMapping.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/checkbox/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/checkbox/types.md`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/checkbox/types.ts`

Expected GPUI implementation files:

- `crates/base_gpui/src/checkbox/mod.rs`
- `crates/base_gpui/src/checkbox/actions.rs`
- `crates/base_gpui/src/checkbox/child/checkbox_child.rs`
- `crates/base_gpui/src/checkbox/child/context/checkbox_context.rs`
- `crates/base_gpui/src/checkbox/child/context/props/checkbox_props.rs`
- `crates/base_gpui/src/checkbox/child/context/runtime/checkbox_runtime.rs`
- `crates/base_gpui/src/checkbox/child/context/state/checkbox_state.rs`
- `crates/base_gpui/src/checkbox/layers/checkbox_root.rs`
- `crates/base_gpui/src/checkbox/layers/checkbox_indicator.rs`
- `crates/base_gpui/src/checkbox/tests/`

## Out of scope / drop from Base UI

- Do not port React context/hooks directly; use GPUI keyed state/entities plus a `CheckboxContext` wrapper.
- Do not port `render` props.
- Do not port `className`.
- Do not port web `style` props; expose GPUI builder styling and state-aware styling APIs.
- Do not port `nativeButton`; GPUI does not currently expose a native button element in `crates/gpui/src/elements`.
- Do not port hidden DOM input behavior literally.
- Do not port browser form submission details literally; only add GPUI-native field/form integration when those primitives exist in `base_gpui`.
- Do not port SSR/hydration/prehydration APIs.
- Do not port CSS variable APIs.
- Do not port DOM data attributes as attributes; map them into typed render-state structs.
- Do not port arbitrary DOM event objects. Use Rust-native change details if needed.
- Do not write DOM ARIA attributes. Map accessibility through GPUI-native AccessKit APIs once available.
- Do not include Checkbox Group parent/child aggregation in this issue except where the standalone Checkbox API must not block future Checkbox Group support.

## Acceptance Criteria

### Module/API surface

- [x] Add `checkbox` module and export it from `crates/base_gpui/src/main.rs` or the crate's public module surface.
- [x] Add public `CheckboxRoot` and `CheckboxIndicator` layer types.
- [x] Add a typed `CheckboxChild` enum that can route `CheckboxIndicator` children before `AnyElement` erasure.
- [x] Support uncontrolled construction with `default_checked: bool`, defaulting to `false`.
- [x] Support controlled construction with `checked: Option<bool>` where controlled state takes precedence over internal state.
- [x] Support `indeterminate: bool`, defaulting to `false`.
- [x] Support `disabled: bool`, defaulting to `false`.
- [x] Support `read_only: bool`, defaulting to `false`.
- [x] Support `required: bool`, defaulting to `false`, even if validation is initially only exposed in render state.
- [x] Support an `on_checked_change` callback for user-requested checked changes.
- [x] Decide whether the first GPUI API should include Rust-native change details and cancellation, or initially use `Fn(bool)` and track richer event details as a follow-up.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui checkbox` passes.
- [x] The component compiles without adding web/React-specific concepts to public APIs.
- [x] The implementation does not add component-specific inherent methods to `GenericContext`.

### Architecture / internal primitives

- [x] Store primary checked state in `CheckboxState` implementing `GenericState`.
- [x] Store stable configuration/callbacks in `CheckboxProps`.
- [x] Store derived runtime metadata in `CheckboxRuntime`.
- [x] Implement component behavior on `CheckboxContext`, not directly on generic context types.
- [x] Use `GenericChild<CheckboxContext>` only for context injection.
- [x] Keep renderable GPUI elements under `layers/`.
- [x] Keep child-routing enums under `child/`.
- [x] Do not add a `utils/` folder for new primitives.

### Stateful/stateless behavior

- [x] Uncontrolled Checkbox initializes from `default_checked`.
- [x] Uncontrolled Checkbox toggles internal checked state on valid user activation.
- [x] Controlled Checkbox reflects external `checked` value.
- [x] Controlled Checkbox calls `on_checked_change` on valid user activation without mutating internal checked state as the source of truth.
- [x] External controlled value changes update root and indicator render state.
- [x] `indeterminate` is visual/semantic state that takes precedence over checked in render state.
- [x] Activating an indeterminate Checkbox does not silently clear indeterminate unless the public API explicitly chooses that behavior.

### Pointer interaction behavior

- [x] Clicking an enabled, non-read-only root requests a checked-state toggle.
- [x] Clicking a disabled root does not request a checked-state toggle.
- [x] Clicking a read-only root does not request a checked-state toggle.
- [x] Pointer activation invokes `on_checked_change` exactly once per accepted toggle.
- [x] Pointer activation does not fire when interaction is blocked by disabled/read-only state.

### Keyboard/focus behavior

- [x] `CheckboxRoot` is focusable with a GPUI `FocusHandle`.
- [x] Space toggles an enabled, non-read-only Checkbox when focused.
- [x] Enter does not toggle the Checkbox.
- [x] Disabled Checkbox does not toggle from keyboard activation.
- [x] Read-only Checkbox does not toggle from keyboard activation.
- [x] Focused state is exposed in render state if tracked by the component.
- [x] Use GPUI actions/key contexts where practical instead of raw DOM-style key handlers.

### Indicator behavior

- [x] `CheckboxIndicator` renders when the root is checked.
- [x] `CheckboxIndicator` renders when the root is indeterminate.
- [x] `CheckboxIndicator` does not render when unchecked and not indeterminate by default.
- [x] `CheckboxIndicator` supports `keep_mounted` so it remains rendered when unchecked.
- [x] Indicator render state includes root checkbox state.
- [x] If transition support exists in `base_gpui`, expose a GPUI-native transition status; otherwise track transition behavior as a follow-up rather than porting DOM animation attributes.

### Styling/state exposure

- [x] Add `CheckboxRootState` render-state struct with at least `checked`, `disabled`, `read_only`, `required`, and `indeterminate`.
- [x] Add `CheckboxIndicatorState` render-state struct including root state and any GPUI-native transition status.
- [x] Expose state-aware styling through `style_with_state(...)` on root and indicator.
- [x] Map Base UI state/data attributes (`checked`, `unchecked`, `indeterminate`, `disabled`, `readonly`, `required`, field validity states) into typed render-state fields, not DOM attributes.
- [x] Do not expose CSS variable names as the styling API.

### Accessibility follow-up

- [ ] Once the target GPUI revision supports AccessKit roles/states, map root semantics to `Role::CheckBox`.
- [ ] Once available, expose checked/unchecked/mixed state through GPUI-native accessibility APIs.
- [ ] Once available, expose disabled/read-only/required state through GPUI-native accessibility APIs.
- [ ] Add accessibility tests if GPUI exposes test helpers for AccessKit state.

### Field/form integration follow-up

- [x] Decide how Checkbox should compose with a future GPUI `Field` component.
- [x] Decide whether `name`, `value`, `form`, and `unchecked_value` have GPUI-native meaning or should wait for form primitives.
- [x] Preserve enough state (`required`, dirty/touched/focused/valid/invalid if available) to integrate cleanly with future field validation.
- [x] Do not implement hidden DOM input submission behavior in GPUI.

### Tests / verification

Add one behavior per file under `crates/base_gpui/src/checkbox/tests/`.

- [x] Uncontrolled initial unchecked state.
- [x] Uncontrolled `default_checked` initial checked state.
- [x] Controlled checked state reflects external state.
- [x] Click toggles unchecked to checked.
- [x] Click toggles checked to unchecked.
- [x] `on_checked_change` is called with the next checked value.
- [x] Disabled click does not toggle and does not call change handler.
- [x] Read-only click does not toggle and does not call change handler.
- [x] Space toggles when focused.
- [x] Enter does not toggle when focused.
- [x] Disabled/read-only keyboard activation does not toggle.
- [x] Indeterminate render state takes precedence over checked render state.
- [x] Indeterminate activation behavior is covered once the intended GPUI semantics are finalized.
- [x] Indicator is absent by default when unchecked.
- [x] Indicator renders when checked.
- [x] Indicator renders when indeterminate.
- [x] Indicator remains rendered with `keep_mounted`.
- [x] `style_with_state(...)` receives correct root state.
- [x] `style_with_state(...)` receives correct indicator state.
