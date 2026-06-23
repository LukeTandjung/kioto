# Port Base UI Collapsible to GPUI

## Problem

Base UI's Collapsible provides a small disclosure component: a root owns open/closed state, a trigger toggles it, and a panel renders or hides the collapsible content. It supports controlled and uncontrolled open state, disabled interaction, cancelable open-change notifications, keyboard activation through button semantics, state-aware styling, and optional closed-panel mounting through `keepMounted`.

`crates/base_gpui` currently has no Collapsible component family. The goal is to port the behavior into GPUI-native components using the current `base_gpui` runtime/context/layers architecture, not to copy React, DOM, CSS, or browser search implementation details.

Collapsible state is boolean. Controlled state should use `Option<bool>` in the GPUI API, where `None` means uncontrolled and `Some(value)` means the caller owns the open value.

## Scope

Port the Collapsible component family from Base UI into GPUI-native components:

- `CollapsibleRoot`
- `CollapsibleTrigger`
- `CollapsiblePanel`

Base UI reference files:

- `/home/luke/Projects/base-ui/packages/react/src/collapsible/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/collapsible/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/collapsible/root/CollapsibleRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/collapsible/root/CollapsibleRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/collapsible/root/useCollapsibleRoot.ts`
- `/home/luke/Projects/base-ui/packages/react/src/collapsible/root/stateAttributesMapping.ts`
- `/home/luke/Projects/base-ui/packages/react/src/collapsible/root/CollapsibleRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/collapsible/root/CollapsibleRoot.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/collapsible/trigger/CollapsibleTrigger.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/collapsible/trigger/CollapsibleTriggerDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/collapsible/trigger/CollapsibleTrigger.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/collapsible/panel/CollapsiblePanel.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/collapsible/panel/useCollapsiblePanel.ts`
- `/home/luke/Projects/base-ui/packages/react/src/collapsible/panel/CollapsiblePanelCssVars.ts`
- `/home/luke/Projects/base-ui/packages/react/src/collapsible/panel/CollapsiblePanelDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/collapsible/panel/CollapsiblePanel.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/utils/collapsibleOpenStateMapping.ts`
- `/home/luke/Projects/base-ui/packages/react/src/internals/useTransitionStatus.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/collapsible/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/collapsible/types.md`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/collapsible/types.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/collapsible/demos/hero/css-modules/index.tsx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/collapsible/demos/hero/tailwind/index.tsx`

Current GPUI implementation:

- None. Add a new `crates/base_gpui/src/collapsible/` module.

Expected GPUI implementation files:

- `crates/base_gpui/src/collapsible/mod.rs`
- `crates/base_gpui/src/collapsible/actions.rs`
- `crates/base_gpui/src/collapsible/child.rs`
- `crates/base_gpui/src/collapsible/child_wiring.rs`
- `crates/base_gpui/src/collapsible/context.rs`
- `crates/base_gpui/src/collapsible/props.rs`
- `crates/base_gpui/src/collapsible/runtime.rs`
- `crates/base_gpui/src/collapsible/style_state.rs`
- `crates/base_gpui/src/collapsible/layers/mod.rs`
- `crates/base_gpui/src/collapsible/layers/collapsible_root.rs`
- `crates/base_gpui/src/collapsible/layers/collapsible_trigger.rs`
- `crates/base_gpui/src/collapsible/layers/collapsible_panel.rs`
- `crates/base_gpui/src/collapsible/tests/`

Use `crates/base_gpui/src/tabs/` as the closest precedent for panel `keep_mounted` behavior, and `crates/base_gpui/src/switch/` / `crates/base_gpui/src/checkbox/` as precedents for boolean controlled/uncontrolled state, cancelable change details, focus handling, and keyboard actions.

## Out of scope / drop from Base UI

- Do not port React context/hooks directly; use GPUI keyed state/entities plus a thin `CollapsibleContext` wrapper.
- Do not port `render` props.
- Do not port `className`.
- Do not port web `style` props; expose GPUI builder styling and state-aware styling APIs.
- Do not port `nativeButton`; GPUI does not currently expose a native button element in `crates/gpui/src/elements`.
- Do not port arbitrary HTML attributes or DOM event props.
- Do not port SSR, hydration, CSP nonce, or prehydration behavior.
- Do not port CSS variable APIs literally. If panel dimensions are useful, expose them as typed GPUI style-state fields.
- Do not port DOM data attributes as attributes; map them into typed style-state structs.
- Do not port arbitrary DOM event objects. Use Rust-native change details when cancellation/source information is needed.
- Do not port browser `hiddenUntilFound` / `beforematch`; GPUI has no browser find-in-page equivalent. Revisit only if GPUI gains a native searchable hidden-content mechanism.
- Do not write DOM ARIA attributes. Map accessibility through GPUI-native AccessKit APIs once the target GPUI revision supports the needed roles/states.

## Acceptance Criteria

### Module/API surface

- [x] Add a `collapsible` module and export it from `crates/base_gpui/src/lib.rs`.
- [x] Register Collapsible key bindings from `base_gpui::init(cx)`.
- [x] Add public `CollapsibleRoot`, `CollapsibleTrigger`, and `CollapsiblePanel` layer types.
- [x] Add a typed `CollapsibleChild` enum that routes trigger and panel children before `AnyElement` erasure.
- [x] Keep `CollapsibleRoot` children typed; add an `AnyElement` escape hatch only if Base UI examples require arbitrary root children.
- [x] Support uncontrolled construction with `.default_open(bool)`, defaulting to `false`.
- [x] Support controlled construction with `.open(Option<bool>)`; controlled state takes precedence over internal state.
- [x] Support `.disabled(bool)`, defaulting to `false`.
- [x] Support `.on_open_change(...)` with a Rust-native cancelable change-details API, e.g. `Fn(bool, &mut CollapsibleOpenChangeDetails, &mut Window, &mut App)`.
- [x] Add `CollapsibleOpenChangeDetails` with `reason()`, `source()`, `cancelable()`, `cancel()`, and `is_canceled()` APIs.
- [x] Add `CollapsibleOpenChangeReason::TriggerPress`, matching trigger activation from Base UI.
- [x] Do not add `CollapsibleOpenChangeReason::None`; no GPUI-native non-trigger open path is implemented in this pass.
- [x] Add a source enum such as `CollapsibleOpenChangeSource::{Pointer, Keyboard}` without exposing DOM event objects.
- [x] Add `.keep_mounted(bool)` on `CollapsiblePanel`, defaulting to `false`.
- [x] `collapsible/mod.rs` remains barrel-only and exposes ergonomic exports for component names, style states, context, props, runtime, actions, and child types.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui collapsible` passes.
- [x] The component compiles without adding web/React-specific concepts to public APIs.
- [x] The implementation follows `docs/base-gpui-component-architecture.md` and does not introduce the old `child/context/{props,runtime,state}` taxonomy.
- [x] `ast-grep scan crates/base_gpui/src` passes.
- [x] Add a small gallery demo in `crates/base_gpui/src/main.rs` that renders a trigger and panel.

### Architecture / internal primitives

- [x] Add `CollapsibleRuntime` as the single owner of Collapsible business state: open value, mounted/present panel state, focused trigger state, and any derived transition/dimension state that is implemented.
- [x] Add `CollapsibleProps` for stable root props and callbacks.
- [x] Add `CollapsibleContext` as a thin injection/plumbing type with only `read(...)`, `update(...)`, and `toggle(...)`-style methods.
- [x] Keep controlled/uncontrolled resolution in `CollapsibleContext::toggle(...)`, not in layers.
- [x] Keep Collapsible behavior on `CollapsibleRuntime`; do not grow component vocabulary on `CollapsibleContext` beyond the value-changing method.
- [x] Add `CollapsibleRootStyleState`, `CollapsibleTriggerStyleState`, and `CollapsiblePanelStyleState` in `style_state.rs`.
- [x] Add renderable GPUI elements only under `collapsible/layers/`.
- [x] Add typed child routing in `collapsible/child.rs` and private context attachment in `collapsible/child_wiring.rs`.
- [x] Do not add a `utils/` folder for Collapsible.
- [x] Do not add new generic primitives unless they hide a repeated deep concept across components.

### Stateful/stateless behavior

- [x] Uncontrolled Collapsible initializes open state from `default_open`, defaulting to closed.
- [x] Uncontrolled Collapsible toggles internal open state on valid user activation.
- [x] Controlled Collapsible reflects the external `open` value.
- [x] Controlled Collapsible calls `on_open_change` on valid user activation without mutating internal open state as the source of truth.
- [x] External controlled value changes update root, trigger, and panel style state.
- [x] Disabled Collapsible ignores user activation and does not call `on_open_change`.
- [x] Re-rendering with changed props does not reset uncontrolled state except when the keyed root id changes.
- [x] Closed state is modeled as a normal component state, not as an error/special path spread across layers.

### Change event behavior

- [x] User activation computes the next open value as `!current_open`.
- [x] `on_open_change` is called exactly once per accepted user activation.
- [x] `on_open_change` is not called for disabled activation attempts.
- [x] User-initiated change details use `CollapsibleOpenChangeReason::TriggerPress`.
- [x] User-initiated change details expose pointer vs keyboard source without exposing DOM event objects.
- [x] User-initiated change details are cancelable.
- [x] Uncontrolled mode calls the callback before mutating internal state and skips mutation when canceled.
- [x] Controlled mode calls the callback but never mutates internal state; cancellation only confirms that Base GPUI should perform no internal state change.
- [x] Do not expose Base UI's native `event`, `trigger`, `allowPropagation`, or propagation APIs literally.

### Pointer interaction behavior

- [x] Clicking an enabled trigger toggles closed to open.
- [x] Clicking an enabled trigger toggles open to closed.
- [x] Clicking a disabled trigger does not toggle and does not call the change handler.
- [x] Pointer activation and keyboard activation share the same runtime toggle command.
- [x] Pointer activation uses GPUI click events only as an input source; no browser event object leaks into the public API.

### Keyboard/focus behavior

- [x] `CollapsibleTrigger` owns a stable keyed `FocusHandle`.
- [x] `CollapsibleTrigger` is focusable when enabled.
- [x] Trigger focused state is synced into `CollapsibleRuntime` and exposed through trigger style state.
- [x] Collapsible uses GPUI actions/key dispatch and a Collapsible-specific key context instead of raw DOM-style key handlers.
- [x] Space toggles an enabled Collapsible when the trigger is focused.
- [x] Enter toggles an enabled Collapsible when the trigger is focused.
- [x] Disabled Collapsible does not toggle from keyboard activation.
- [x] Disabled triggers are removed from tab order initially, matching current GPUI control behavior.

### Panel/content behavior

- [x] `CollapsiblePanel` renders its children when the root is open.
- [x] `CollapsiblePanel` returns `gpui::Empty` when the root is closed and `keep_mounted(false)`.
- [x] `CollapsiblePanel` remains in the element tree when the root is closed and `keep_mounted(true)`.
- [x] A kept-mounted closed panel is visually hidden using a GPUI-native mechanism such as `invisible()`, not by DOM `hidden` attributes.
- [x] A kept-mounted closed panel exposes closed state to `style_with_state(...)`.
- [x] Opening a kept-mounted panel makes it visible again and exposes open state to `style_with_state(...)`.
- [x] Closing a non-kept-mounted panel removes it after any implemented GPUI-native transition state has completed; if transitions are not implemented initially, removal can be immediate.
- [x] The panel can be omitted without breaking trigger behavior.
- [x] Multiple panels are not a target behavior for Collapsible; if multiple panels are provided, either all observe the same root state or the issue should explicitly reject unsupported composition.

### Transition and measurement behavior

- [x] Expose a typed transition-like state only if GPUI can make it meaningful, e.g. `CollapsibleTransitionStatus::{Starting, Ending, Idle}` or a simpler presence state.
- [x] Do not copy Base UI DOM transition attributes (`data-starting-style`, `data-ending-style`) as attributes.
- [x] No GPUI-native transition sequencing is implemented in this pass, so there is no transition sequencing outside `CollapsibleRuntime`.
- [x] No panel dimensions are implemented in this pass, so there is no DOM-style measurement to replace.
- [x] No panel dimensions are implemented in this pass, so no CSS variable API is exposed.
- [x] Do not port the Base UI `hiddenUntilFound` instant-open and animation-suppression paths.

### Styling/state exposure

- [x] `CollapsibleRootStyleState` includes at least `open`, `closed`, and `disabled`.
- [x] `CollapsibleTriggerStyleState` includes at least `open`, `closed`, `disabled`, and `focused`.
- [x] `CollapsiblePanelStyleState` includes at least `open`, `closed`, `mounted`/`present`, and any implemented transition/dimension fields.
- [x] Expose state-aware styling through `style_with_state(...)` on root, trigger, and panel.
- [x] Map Base UI state/data attributes (`data-panel-open`, `data-open`, `data-closed`, and transition status if implemented) into typed style-state fields, not DOM attributes.
- [x] Do not expose CSS variable names as the styling API.
- [x] The docs hero styling pattern can be recreated with GPUI builder methods: trigger styling changes when open, panel visibility changes when open/closed, and focus styling can use trigger focused style state.

### Accessibility follow-up

- [ ] Once the target GPUI revision supports the needed AccessKit APIs, map trigger semantics to a GPUI/AccessKit button or disclosure trigger role.
- [ ] Once available, expose expanded/collapsed state through GPUI-native accessibility APIs.
- [ ] Once available, expose disabled state through GPUI-native accessibility APIs.
- [ ] Decide how GPUI Collapsible links a trigger to its panel through AccessKit relationships if GPUI supports relationships; do not port DOM `aria-controls` / `id` linking literally.
- [ ] Add accessibility tests if GPUI exposes test helpers for AccessKit state.

### Tests / verification

Add behavior-level tests under `crates/base_gpui/src/collapsible/tests/`.

- [x] Uncontrolled initial closed state.
- [x] Uncontrolled `default_open` initial open state.
- [x] Controlled open state reflects external state.
- [x] External controlled value changes update root, trigger, and panel style state.
- [x] Click toggles closed to open.
- [x] Click toggles open to closed.
- [x] `on_open_change` is called with the next open value.
- [x] Disabled click does not toggle and does not call the change handler.
- [x] Space toggles when the trigger is focused.
- [x] Enter toggles when the trigger is focused.
- [x] Disabled keyboard activation does not toggle.
- [x] Trigger focused state appears in `CollapsibleTriggerStyleState` when focused and clears on blur.
- [x] Panel renders children while open.
- [x] Panel is omitted while closed by default.
- [x] Panel remains rendered while closed when `keep_mounted(true)`.
- [x] Kept-mounted closed panel receives closed style state.
- [x] `style_with_state(...)` receives correct root state.
- [x] `style_with_state(...)` receives correct trigger state.
- [x] `style_with_state(...)` receives correct panel state.
- [x] Canceled uncontrolled pointer activation does not mutate open state.
- [x] Canceled uncontrolled keyboard activation does not mutate open state.
- [x] Canceled controlled activation still calls the handler but does not mutate internal open state.
- [x] Transition/dimension tests are not applicable in this pass because GPUI-native transition and measurement behavior were not implemented.
