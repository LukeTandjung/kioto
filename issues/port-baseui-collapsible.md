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

### AccessKit accessibility follow-up

Written against the AccessKit surface in the pinned gpui revision (see `docs/accesskit-gpui-reference.md`). Base UI's Collapsible emits ARIA only on the trigger button (`aria-expanded`, `aria-controls` when open, `disabled`) — the root and panel are plain `div`s with no role, so only the trigger and (optionally) the panel need a11y-tree presence.

#### Per accessible part

- **`CollapsibleTrigger`** (`layers/collapsible_trigger.rs`): already has a stable `.id(id)` (default `"collapsible-trigger"`), so it enters the a11y tree once it gets a role.
  - Role: `.role(Role::Button)` — Base UI renders a native `<button>` disclosure trigger; there is no dedicated disclosure role in accesskit 0.24, `Role::Button` + expanded state is the standard mapping.
  - `.aria_expanded(state.open)` — from the `CollapsibleTriggerStyleState` already computed in `render` via `context.read(cx, |runtime, props| runtime.trigger_state(props))`. This mirrors Base UI's `aria-expanded: open`.
  - `.aria_label(...)` — see Labels below.
- **`CollapsibleRoot`** (`layers/collapsible_root.rs`): no role. Base UI's root is a bare `div`; assigning a role would add noise. Do not use `Role::GenericContainer` (filtered out / debug-asserts in gpui).
- **`CollapsiblePanel`** (`layers/collapsible_panel.rs`): optional. Base UI's panel has no ARIA role, only an `id` targeted by `aria-controls`. Since `aria-controls` has no gpui builder (see Gaps), the panel may stay out of the a11y tree; its text content is reachable as ordinary accessible text while `state.present` is true. If a container node proves useful for AT navigation, `Role::Group` + `.id(...)` is the only candidate — decide during implementation and note the choice. A kept-mounted closed panel is hidden via `.invisible()`; verify its text does not leak to AT while closed, and if it does, gate the children on `state.closed` in addition to `invisible()`.

#### Actions

- `Action::Click` — **already auto-registered** by the existing `.on_click(...)` on `CollapsibleTrigger`; do not re-add. AT "press" will route through the same `context.toggle(CollapsibleOpenChangeSource::Pointer, window, cx)` path as a mouse click — but note the current handler early-returns unless `matches!(event, ClickEvent::Mouse(_))`; confirm the synthesized a11y click still passes that guard, and if not, either relax the guard or add an explicit `.on_a11y_action(AccessibleAction::Click, ...)` that calls `context.toggle(...)` directly.
- `Action::Focus` — **already auto-registered** by the existing `.track_focus(&focus_handle)`; do not re-add.
- `.on_a11y_action(AccessibleAction::Expand, ...)` and `.on_a11y_action(AccessibleAction::Collapse, ...)` on the trigger — optional but cheap: route both into the same `CollapsibleContext::toggle(...)` transition (guarding on current `runtime.open()` so Expand is a no-op when already open and Collapse a no-op when closed). All disabled-gating stays in `CollapsibleRuntime::request_toggle(disabled)`, which already refuses toggles when disabled.

#### Labels

- Add a `.aria_label(impl Into<SharedString>)` builder to `CollapsibleTrigger` (stored as `Option<SharedString>`, applied in `render`). Callers with icon-only triggers must set it.
- When the trigger's visible label is text a caller passes as a child, callers should use `Text::new_inaccessible(...)` for that child whenever they also set `.aria_label(...)`, so the label is not announced twice (`text!(...)` is accessible by default). Document this on the builder. If no `aria_label` is set, leave child text accessible so it names the button.

#### Gaps (no gpui builder in this revision)

- **`aria-controls`** (trigger → panel id, Base UI `CollapsibleTrigger.tsx:55`): no relationship builders in gpui (`aria-controls`/`aria-labelledby`/`aria-owns` all absent). Fallback: omit. `aria-expanded` on the trigger plus the panel appearing/disappearing in the tree conveys the disclosure pattern; document that trigger→panel linking is blocked pending gpui upstream relationship support.
- **`disabled` / `aria-disabled`** (Base UI sets `disabled` on the trigger button): no `.aria_disabled(...)` builder and `write_a11y_info` never sets a disabled flag. Current behavior already removes a disabled trigger from tab order (`tab_stop(false)` / `tab_index(-1)`) and `CollapsibleRuntime::request_toggle` rejects toggles, so a disabled trigger is inert but may still be announced as an ordinary button. Fallback: keep the inert behavior, document the limitation, and track `set_disabled` as a candidate gpui upstream addition.
- No live-region needs: Collapsible announces nothing (`aria-live` not used by Base UI here), so the announcement gap does not apply.

#### Checklist

- [ ] Set `.role(Role::Button)` on `CollapsibleTrigger` in `layers/collapsible_trigger.rs`.
- [ ] Set `.aria_expanded(state.open)` on the trigger from `CollapsibleTriggerStyleState`.
- [ ] Add an `.aria_label(...)` builder to `CollapsibleTrigger` and document the `Text::new_inaccessible` pattern for visible label children.
- [ ] Verify AT-dispatched `Action::Click` reaches `context.toggle(...)` despite the `ClickEvent::Mouse` guard; add an explicit `on_a11y_action(AccessibleAction::Click, ...)` handler only if it does not.
- [ ] Optionally add `on_a11y_action` handlers for `AccessibleAction::Expand` / `AccessibleAction::Collapse` routed through `CollapsibleContext::toggle(...)`.
- [ ] Do not re-register `Action::Click` / `Action::Focus` where `.on_click` / `.track_focus` already exist.
- [ ] Decide whether `CollapsiblePanel` gets `Role::Group` + `.id(...)` or stays out of the a11y tree; record the decision.
- [ ] Verify a kept-mounted closed panel's content is not exposed to AT; gate children on closed state if `invisible()` alone leaks text.
- [ ] Document the `aria-controls` and disabled-state gaps as blocked pending gpui upstream support.
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
