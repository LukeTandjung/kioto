# Port Base UI Popover to GPUI

## Problem

Base UI's React Popover component provides a full anchored-popup behavior model: controlled and uncontrolled open state, multiple triggers, optional detached triggers through a handle, hover/click/focus/escape/outside dismissal reasons, modal outside-interaction behavior, portal mounting, positioning/collision handling, arrow positioning, close buttons, title/description metadata, and optional viewport state for trigger-to-trigger content transitions.

`base_gpui` currently has no Popover component family. Select introduced useful GPUI-native popup groundwork (`deferred(...)` / `anchored()`, collision logic, outside-dismiss behavior, `utils::overlay`, and `utils::presence`), but Popover should be its own component with its own runtime, context, typed children, style states, callbacks, and tests.

The goal is to port Popover behavior and contracts, not React or DOM internals. Web-specific implementation details such as React hooks, Floating UI DOM refs, render props, DOM data attributes, CSS variables, native button switches, and browser ARIA attributes should be dropped or translated into GPUI-native architecture.

Popover payloads are in scope and should be Rust-native typed values using a generic payload parameter such as `P: Clone + 'static`; do not port arbitrary JavaScript value semantics.

## Scope

Port the Popover component family from Base UI into GPUI-native components:

- `PopoverRoot<P>`
- `PopoverTrigger<P>`
- `PopoverPortal<P>`
- `PopoverBackdrop<P>`
- `PopoverPositioner<P>`
- `PopoverPopup<P>`
- `PopoverArrow<P>`
- `PopoverTitle<P>`
- `PopoverDescription<P>`
- `PopoverClose<P>`
- `PopoverViewport<P>`
- `PopoverHandle<P>` / `create_popover_handle(...)` or an equivalent GPUI-native constructor

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/popover/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/popover/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/popover/root/PopoverRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/root/PopoverRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/popover/root/PopoverRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/root/PopoverRoot.detached-triggers.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/root/PopoverRoot.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/trigger/PopoverTrigger.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/trigger/PopoverTrigger.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/trigger/PopoverTriggerDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/popover/portal/PopoverPortal.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/portal/PopoverPortalContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/popover/portal/PopoverPortal.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/backdrop/PopoverBackdrop.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/backdrop/PopoverBackdrop.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/backdrop/PopoverBackdropDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/popover/positioner/PopoverPositioner.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/positioner/PopoverPositionerContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/popover/positioner/PopoverPositioner.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/positioner/PopoverPositioner.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/positioner/PopoverPositionerDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/popover/positioner/PopoverPositionerCssVars.ts`
- `/home/luke/Projects/base-ui/packages/react/src/popover/popup/PopoverPopup.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/popup/PopoverPopup.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/popup/PopoverPopupDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/popover/popup/PopoverPopupCssVars.ts`
- `/home/luke/Projects/base-ui/packages/react/src/popover/arrow/PopoverArrow.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/arrow/PopoverArrow.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/arrow/PopoverArrowDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/popover/title/PopoverTitle.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/title/PopoverTitle.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/description/PopoverDescription.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/description/PopoverDescription.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/close/PopoverClose.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/close/PopoverClose.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/viewport/PopoverViewport.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/viewport/PopoverViewport.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/popover/viewport/PopoverViewportDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/popover/viewport/PopoverViewportCssVars.ts`
- `/home/luke/Projects/base-ui/packages/react/src/popover/store/PopoverStore.ts`
- `/home/luke/Projects/base-ui/packages/react/src/popover/store/PopoverHandle.ts`
- `/home/luke/Projects/base-ui/packages/react/src/popover/utils/constants.ts`

Base UI docs reference:

- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/popover/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/popover/types.md`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/popover/types.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/popover/demos/hero/`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/popover/demos/open-on-hover/`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/popover/demos/detached-triggers-simple/`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/popover/demos/detached-triggers-controlled/`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/popover/demos/detached-triggers-full/`

GPUI / Zed precedent reference:

- `/home/luke/Projects/zed/crates/gpui/examples/popover.rs`
  - Shows nested GPUI `deferred(...)` + `anchored()` popovers, explicit open state, and outside dismissal via `on_mouse_down_out(...)`.
- `/home/luke/Projects/gpui-component/crates/ui/src/popover.rs`
  - Shows a stateful GPUI popover using `window.use_keyed_state(...)`, a content builder closure, focus capture/restore, `DismissEvent`, trigger bounds measurement in `on_prepaint(...)`, and deferred anchored rendering.
- `/home/luke/Projects/gpui-component/crates/ui/src/global_state.rs`
  - Shows existing deferred-popover bookkeeping used by gpui-component.
- `/home/luke/Projects/zed/crates/ui/src/components/popover.rs`
  - This is only a styled visual popover container, not an interaction/controller implementation.
- `/home/luke/Projects/zed/crates/ui/src/components/popover_menu.rs`
  - Important handle precedent: `PopoverMenuHandle<M>` is an `Rc<RefCell<Option<...>>>` late-bound control handle with `show`, `hide`, `toggle`, `is_deployed`, and `refresh_menu`; the element installs the handle state during render/layout and uses a deferred anchored menu.
- `/home/luke/Projects/zed/crates/ui/src/components/dropdown_menu.rs`
  - Shows consumer-facing use of `PopoverMenuHandle` through dropdown-menu composition.

Current GPUI implementation:

- No `crates/base_gpui/src/popover/` implementation exists yet.

Expected GPUI implementation files:

```text
crates/base_gpui/src/popover/
  mod.rs            # barrel only
  actions.rs        # Escape / optional keyboard dispatch
  child.rs          # typed child enums
  child_wiring.rs   # private traversal/context attachment/trigger registration
  context.rs        # PopoverContext<P>
  props.rs          # root props, callbacks, modal/positioning config if shared
  runtime.rs        # PopoverRuntime<P> + metadata/outcomes/details
  style_state.rs    # Popover*StyleState structs
  layers/
    mod.rs          # barrel only
    popover_root.rs
    popover_trigger.rs
    popover_portal.rs
    popover_backdrop.rs
    popover_positioner.rs
    popover_popup.rs
    popover_arrow.rs
    popover_title.rs
    popover_description.rs
    popover_close.rs
    popover_viewport.rs
  tests/
```

Also update:

- `crates/base_gpui/src/lib.rs`
- `crates/base_gpui/src/main.rs` with a Popover demo

Implementation notes from the initial GPUI port:

- Trigger IDs are root-scoped internally using the root `ElementId`; public trigger IDs passed to root/handle APIs are converted into that scoped ID space.
- Duplicate scoped trigger IDs are deterministic: the last registered trigger metadata wins for activation and payload lookup.
- `PopoverHandle::open(...)` / `toggle(...)` return `false` when the handle is not bound or the requested trigger ID is not registered/enabled.
- Hover opens/closes are immediate in this pass; delayed hover timers remain unchecked follow-up criteria.
- Accessibility audit for pinned GPUI: the pinned dependency at `/home/luke/.cargo/git/checkouts/zed-a70e2ad075855582/f7ca86e/crates/gpui/src/elements/div.rs` has `StatefulInteractiveElement` focus/click APIs but no public `Role`, `.role(...)`, `aria_expanded(...)`, controls, label relationship, dialog/modal, hidden/inert, or focus-trap APIs. A newer local Zed worktree contains richer AccessKit APIs, but they are not available to this crate's pinned GPUI. Popover therefore keeps title/description metadata in runtime for future mapping and intentionally does not emit literal DOM ARIA strings.

## Out of scope / drop from Base UI

- React context, hooks, stores, refs, `FloatingTree`, `FloatingNode`, and `FloatingPortal` implementation details.
- React children render-function API for payload-driven content. Payload storage and payload-driven content are in scope; only the React pattern of `children={({ payload }) => ...}` is dropped. Expose payload through a GPUI-native content builder, open-change details, runtime query, or another explicit Rust API.
- `className` and web `style` props.
- `render` prop support.
- `nativeButton` support. Use GPUI interactive elements and focus handles.
- DOM `container` portal prop. GPUI deferred/anchored rendering should use GPUI-native popup rendering.
- Browser event objects in change details. Use GPUI-native source/reason fields.
- DOM `id`, `aria-haspopup`, `aria-expanded`, `aria-controls`, `aria-labelledby`, and `aria-describedby` literals unless GPUI-native AccessKit support exists and is explicitly implemented.
- DOM data attributes. Map them into typed `*StyleState` structs.
- CSS variable APIs such as `--anchor-width`, `--available-height`, `--transform-origin`, `--popup-width`, and `--popup-height`. Expose typed style-state measurements instead.
- DOM `ResizeObserver`, `MutationObserver`, `getBoundingClientRect`, focus guards, `inert`, and scroll-lock implementation details. Translate to GPUI measurement/focus/outside-interaction behavior where possible.
- Exact CSS animation semantics. Preserve mounted/open/transition/instant facts in style state; real animation timing can be a follow-up if GPUI infrastructure supports it.
- Full browser touch-screen-reader escape behavior. Implement GPUI focus/pointer behavior first; audit accessibility separately.

## Acceptance Criteria

### Module/API surface

- [x] `crates/base_gpui/src/popover/` exists with the expected flat component architecture.
- [x] `popover/mod.rs` is a barrel-only file with module declarations and re-exports only.
- [x] `popover/layers/mod.rs` is a barrel-only file with module declarations and re-exports only.
- [x] `base_gpui::popover` is exported from `crates/base_gpui/src/lib.rs`.
- [x] `base_gpui::init(cx)` calls `popover::init(cx)`.
- [x] `PopoverRoot<P>` exists and is publicly exported.
- [x] `PopoverTrigger<P>` exists and is publicly exported.
- [x] `PopoverPortal<P>` exists and is publicly exported.
- [x] `PopoverBackdrop<P>` exists and is publicly exported.
- [x] `PopoverPositioner<P>` exists and is publicly exported.
- [x] `PopoverPopup<P>` exists and is publicly exported.
- [x] `PopoverArrow<P>` exists and is publicly exported.
- [x] `PopoverTitle<P>` exists and is publicly exported.
- [x] `PopoverDescription<P>` exists and is publicly exported.
- [x] `PopoverClose<P>` exists and is publicly exported.
- [x] `PopoverViewport<P>` exists and is publicly exported.
- [x] `PopoverHandle<P>` and a public constructor exist, using the Zed `PopoverMenuHandle` late-bound handle pattern where appropriate, or the issue documents why detached triggers are deferred.
- [x] Public payload APIs use Rust-native generics such as `P: Clone + 'static`.
- [x] All public style payloads use `*StyleState` names from `style_state.rs`; no `*RenderState` or `render_state.rs` files are introduced.
- [x] No Popover code uses Rust scoped visibility syntax such as `pub(...)`.

### Root API and open state

- [x] `PopoverRoot<P>::new()` exists.
- [x] `PopoverRoot<P>::id(...)` or equivalent stable root identity exists for keyed runtime state.
- [x] `PopoverRoot<P>::default_open(bool)` initializes uncontrolled open state.
- [x] `PopoverRoot<P>::open(bool)` supports controlled open state.
- [x] `PopoverRoot<P>::on_open_change(...)` supports open-change callbacks.
- [x] `PopoverRoot<P>::on_open_change_complete(...)` supports completion callbacks, firing immediately when no transition infrastructure is active.
- [x] `PopoverRoot<P>::modal(...)` supports at least non-modal and modal behavior.
- [x] `PopoverRoot<P>::modal(false)` is the default.
- [x] `PopoverRoot<P>::trigger_id(...)` supports controlled active-trigger selection when multiple triggers exist, or a documented GPUI-native equivalent exists.
- [x] `PopoverRoot<P>::default_trigger_id(...)` supports initially open uncontrolled popovers associated with a trigger, or a documented GPUI-native equivalent exists.
- [x] `PopoverRoot<P>::handle(...)` can associate the root with detached triggers using an `Rc<RefCell<...>>` late-bound handle pattern similar to Zed `PopoverMenuHandle`, or detached-trigger support is explicitly deferred in the issue before implementation starts.
- [x] Controlled `open(...)` takes precedence over `default_open(...)`.
- [x] Controlled roots call `on_open_change(...)` without mutating internal open state.
- [x] Uncontrolled roots call `on_open_change(...)` and mutate internal open state unless canceled.
- [x] Closing can keep the popup mounted when details request `prevent_unmount_on_close()`.
- [x] A later normal close after a prevented-unmount cycle unmounts as expected.

### Open change details

- [x] Add `PopoverOpenChangeReason` with at least `TriggerHover`, `TriggerFocus`, `TriggerPress`, `OutsidePress`, `EscapeKey`, `ClosePress`, `FocusOut`, `ImperativeAction`, and `None`.
- [x] Add `PopoverOpenChangeSource` or equivalent GPUI-native source metadata for pointer, keyboard, touch, focus, imperative, and unknown sources where GPUI can distinguish them.
- [x] Add `PopoverOpenChangeDetails<P>` with `reason`, `source`, `trigger_id`, optional typed payload, `cancelable`, `cancel()`, `is_canceled()`, and `prevent_unmount_on_close()` APIs.
- [x] Open from trigger click/keyboard activation reports `TriggerPress`.
- [x] Open from trigger hover reports `TriggerHover`.
- [x] Open from trigger focus reports `TriggerFocus` if focus-open behavior is implemented.
- [x] Close from outside interaction reports `OutsidePress`.
- [x] Close from Escape reports `EscapeKey`.
- [x] Close from `PopoverClose` reports `ClosePress`.
- [x] Close from focus leaving the popover reports `FocusOut`.

Implementation note: trigger focus by itself does not currently open the popover, so `TriggerFocus` remains available as a reason variant for a future `open_on_focus` API but has no active behavior to report yet.
- [x] Handle/action-driven close reports `ImperativeAction`.
- [x] Canceled uncontrolled open does not open.
- [x] Canceled uncontrolled close does not close and preserves the active trigger.
- [x] Callback details do not expose DOM/browser event objects.

### Trigger behavior

- [x] `PopoverTrigger<P>::new()` exists.
- [x] `PopoverTrigger<P>::id(...)` assigns a stable trigger id within its root/handle scope.
- [x] Trigger ids are scoped by the popover root/handle so independent popovers can reuse child ids safely.
- [x] Duplicate trigger-id behavior is deterministic and documented.
- [x] `PopoverTrigger<P>::disabled(bool)` prevents pointer, keyboard, focus, and hover opening.
- [x] `PopoverTrigger<P>::payload(P)` stores typed payload metadata for the active trigger.
- [x] `PopoverTrigger<P>::handle(...)` associates a detached trigger with a root handle, or detached-trigger support is explicitly deferred.
- [x] `PopoverTrigger<P>::open_on_hover(bool)` enables hover opening.
- [x] `PopoverTrigger<P>::delay(Duration)` or an equivalent GPUI duration API controls hover-open delay.
- [x] `PopoverTrigger<P>::close_delay(Duration)` controls hover-close delay.
- [x] Clicking a closed trigger opens the popover.
- [x] Clicking the active trigger again closes the popover.
- [x] Clicking a different registered trigger while open changes the active trigger and keeps/reuses the popup tree.
- [x] Keyboard activation on the focused trigger toggles the popover using GPUI actions/key contexts.
- [x] Disabled triggers remain non-activating and expose disabled state.
- [x] Trigger style state exposes `disabled`, `open`, `pressed` if tracked, active-trigger status, and payload-present status if useful.
- [x] Hover-opened popovers do not immediately become click-sticky until the patient-click threshold behavior is implemented or explicitly deferred.
- [x] Re-hovering during a hover-close path reopens deterministically if transition/presence support exists.

Implementation note: Base UI's patient-click hover stickiness depends on DOM pointer timing and hoverable floating-tree behavior. The GPUI port does not currently make hover-opened popovers click-sticky; delayed hover open/close and re-hover cancellation are implemented, while patient-click stickiness is explicitly deferred.

### Detached handles and multiple triggers

- [x] Multiple `PopoverTrigger<P>` children inside one `PopoverRoot<P>` can open the same popover.
- [x] A shared `PopoverHandle<P>` can connect detached triggers to a root outside the root's child tree, or this behavior is explicitly deferred.
- [x] A handle exposes `open(trigger_id)`, `close()`, and `is_open()` or documented GPUI-native equivalents.
- [x] Opening a handle with a missing trigger id fails deterministically and documents the failure mode.
- [x] Active trigger metadata includes trigger id, focus handle, bounds, disabled state, hover config, and typed payload.
- [x] Switching active triggers preserves the same popup/positioner runtime rather than creating a separate popover per trigger.
- [x] Controlled open state with multiple triggers can identify the active trigger through `trigger_id(...)` or callback details.
- [x] If the active trigger unmounts while open, final focus and positioning fall back deterministically.
- [x] Active payload is observable through open-change details and context/runtime queries.
- [x] Payload-driven popup content is supported through a GPUI-native content builder or another explicit Rust API.

### Portal / mounting / presence

- [x] `PopoverPortal<P>::new()` exists.
- [x] `PopoverPortal<P>::keep_mounted(bool)` preserves mounted popup children while closed.
- [x] Portal children are omitted when closed and not keep-mounted.
- [x] Portal children remain mounted but report closed/inactive style state when `keep_mounted(true)`.
- [x] Runtime distinguishes `open`, `mounted`, and `transitioning` facts.
- [x] `utils::PresenceState` is reused where appropriate instead of adding another presence module.
- [x] `on_open_change_complete(...)` fires after open/close presence settles.
- [x] Force/prevent-unmount close behavior does not leak into later closes.

### Positioner / popup positioning

- [x] `PopoverPositioner<P>::new()` exists.
- [x] Positioner uses GPUI `deferred(...)` / `anchored()` or a GPUI-native equivalent; it does not rely on DOM portals or Floating UI DOM refs.
- [x] Positioner defaults to side `Bottom` and align `Center`.
- [x] Positioner supports side values `Top`, `Bottom`, `Left`, `Right`, and logical `InlineStart` / `InlineEnd` if direction-aware positioning is implemented.
- [x] Positioner supports align values `Start`, `Center`, and `End`.
- [x] Positioner supports `side_offset(...)`.
- [x] Positioner supports `align_offset(...)`.
- [x] Positioner supports configurable collision padding.
- [x] Positioner supports side-axis collision behavior: keep, flip, shift, or a documented subset matching Select's practical collision behavior.
- [x] Positioner supports alignment-axis collision behavior: keep, flip, shift, or a documented subset.
- [x] Positioner supports fallback-axis behavior where practical.
- [x] Positioner accounts for LTR/RTL logical sides using `utils::direction`.
- [x] Positioner measures trigger/anchor bounds.
- [x] Positioner measures popup/positioner bounds.
- [x] Positioner exposes available width/height and anchor width/height in `PopoverPositionerStyleState`.
- [x] Positioner exposes effective side, align, anchor-hidden, transform-origin, and instant-transition facts in style state.
- [x] Positioner keeps the popup inside the window with configured collision padding.
- [x] Positioner updates when the active trigger changes.
- [x] Positioner supports an explicit anchor override only if there is a GPUI-native representation; otherwise document it as out of scope.
- [x] Positioner does not depend on Select-specific public types; extract flat shared positioning helpers under `utils/` only if the abstraction is genuinely repeated and useful.

Implementation note: alignment-axis collision currently uses GPUI anchored placement plus the configured window collision padding rather than Base UI/Floating UI's full alignment-axis middleware. Explicit anchor overrides are out of scope until GPUI exposes a native, stable way to pass an arbitrary anchor separate from the trigger/view element.

### Popup behavior and focus

- [x] `PopoverPopup<P>::new()` exists.
- [x] Popup renders arbitrary GPUI children.
- [x] Popup style state exposes `open`, `mounted`, `side`, `align`, `transitioning`, and `instant` facts.
- [x] Popup records its bounds in the runtime for positioning, arrow, viewport, and style state.
- [x] Opening by pointer/keyboard can move focus into the popup according to GPUI-native focus rules.
- [x] Opening by hover does not steal focus.
- [x] Closing by trigger/close/outside/Escape returns focus to the active trigger when available.
- [x] If the active trigger is gone, closing returns focus to a deterministic fallback or leaves focus unchanged as documented.
- [x] `PopoverPopup<P>::initial_focus(...)` is implemented with a GPUI-native API or explicitly deferred.
- [x] `PopoverPopup<P>::final_focus(...)` is implemented with a GPUI-native API or explicitly deferred.
- [x] Escape closes the popover when focus is inside the popup.
- [x] Focus leaving the popup closes non-modal popovers when Base UI-equivalent focus-out behavior is possible.
- [x] Nested popup interactions are audited so child popups do not accidentally close parents on internal presses.

Implementation note: GPUI exposes focus handles and blur/focus-next primitives but not a Base UI-equivalent initial/final focus callback API, trap-focus-only layer, or touch scroll-lock primitive in the pinned version. Custom `initial_focus(...)` / `final_focus(...)`, modal focus trapping, and touch scroll locking are therefore explicitly deferred. When the active trigger metadata is unavailable at close time, the implementation leaves focus unchanged rather than guessing an unrelated fallback. Nested popup dismissal was audited against the current single-root outside-press wiring: parent popovers close on GPUI `on_mouse_down_out`, while active-trigger presses are classified separately; a future shared popup stack can add child-popup containment once GPUI/global popup ancestry APIs are available.

### Backdrop / modal / outside interactions

- [x] `PopoverBackdrop<P>::new()` exists.
- [x] Backdrop style state exposes `open`, `mounted`, and transition/presence facts.
- [x] User-rendered backdrop appears below the popup when mounted.
- [x] Clicking/pressing outside the popup closes non-modal popovers.
- [x] Clicking/pressing a rendered backdrop closes the popover through `OutsidePress` unless canceled.
- [x] Hover-opened popovers make the backdrop non-interactive or otherwise avoid backdrop pointer capture, matching Base UI intent.
- [x] `PopoverRoot::modal(true)` blocks outside pointer interaction with GPUI-native occlusion/backdrop behavior.
- [x] `PopoverRoot::modal(true)` uses `utils::overlay::modal_backdrop` where appropriate; do not create a new overlay module folder.
- [x] `PopoverRoot::modal(true)` closes on outside/backdrop interaction unless canceled.
- [x] `PopoverRoot::modal(false)` allows normal outside interaction except for dismissal handling.
- [x] `PopoverRoot::modal_trap_focus()` or equivalent supports trap-focus-only behavior if GPUI focus APIs make it practical; otherwise document it as deferred.
- [x] Touch-driven modal scroll locking is either implemented GPUI-natively or documented as deferred.

### Close / title / description

- [x] `PopoverClose<P>::new()` exists.
- [x] Clicking `PopoverClose<P>` closes the popover with reason `ClosePress`.
- [x] Keyboard activation on `PopoverClose<P>` closes the popover through GPUI actions/focus behavior.
- [x] Disabled close buttons do not close and expose disabled state if a disabled prop exists.
- [x] `PopoverTitle<P>::new()` exists and renders children.
- [x] `PopoverDescription<P>::new()` exists and renders children.
- [x] Title and description metadata can be registered in the runtime for future AccessKit labeling when GPUI supports the needed relationship APIs.
- [x] Title and description style states exist even if initially empty, preserving `style_with_state(...)` extensibility.

### Arrow behavior

- [x] `PopoverArrow<P>::new()` exists.
- [x] Arrow style state exposes `open`, `side`, `align`, and `uncentered`.
- [x] Arrow bounds are measured and stored in runtime if needed for positioning.
- [x] Arrow placement follows the active trigger side/align.
- [x] Arrow padding prevents the arrow from exceeding popup edges where practical.
- [x] Arrow updates when side flips due to collision handling.
- [x] Arrow is decorative; no DOM `aria-hidden` literal is added unless GPUI has a native accessibility equivalent.

### Viewport / trigger-change content transitions

- [x] `PopoverViewport<P>::new()` exists.
- [x] Viewport renders children normally by default.
- [x] Runtime tracks previous and current active trigger metadata needed to derive activation direction.
- [x] Viewport style state exposes `activation_direction`, `transitioning`, `instant`, and measured popup size facts where useful.
- [x] Switching triggers updates viewport activation direction based on previous vs next trigger position.
- [x] Viewport supports current/previous content containers only if GPUI-native composition can expose payload-driven content clearly; otherwise document the simplified behavior.

Implementation note: `PopoverViewport` currently exposes activation direction and renders a single GPUI child tree. Base UI's DOM-oriented current/previous container choreography is documented as out of scope for now; payload-driven content should use `PopoverPopup::payload_content(...)` and style off `PopoverViewportStyleState`.
- [x] Rapid trigger changes are deterministic and do not panic.
- [x] Closed keep-mounted popovers reopen with sane viewport state.

### Styling/state exposure

- [x] `PopoverRootStyleState<P>` exists, even if initially small.
- [x] `PopoverTriggerStyleState<P>` exists with disabled/open/pressed/active-trigger facts.
- [x] `PopoverPortalStyleState` exists, even if initially small.
- [x] `PopoverBackdropStyleState` exists with open/mounted/presence facts.
- [x] `PopoverPositionerStyleState` exists with open/mounted/side/align/anchor-hidden/measurement/instant facts.
- [x] `PopoverPopupStyleState` exists with open/mounted/side/align/presence/instant facts.
- [x] `PopoverArrowStyleState` exists with open/side/align/uncentered facts.
- [x] `PopoverTitleStyleState` exists, even if initially empty.
- [x] `PopoverDescriptionStyleState` exists, even if initially empty.
- [x] `PopoverCloseStyleState` exists, even if initially small.
- [x] `PopoverViewportStyleState` exists with activation direction/transition/instant facts.
- [x] Every visible Popover part that supports state-aware styling has `.style_with_state(...)`.
- [x] Base UI data attributes are represented only as typed style-state fields, not DOM-like attributes.
- [x] CSS variable concepts are represented as typed measurement fields in style state where useful.

### Architecture / internals

- [x] Follow `docs/base-gpui-component-architecture.md`: one deep `PopoverRuntime<P>`, thin `PopoverContext<P>`, thin render layers.
- [x] `PopoverRuntime<P>` owns open/mounted state, active trigger id, trigger metadata, payload metadata, hover timers/state, focus handles, popup/trigger/arrow bounds, modal facts, presence facts, and positioning-derived facts.
- [x] `PopoverRuntime<P>` uses domain commands such as `sync_triggers`, `request_open`, `request_close`, `toggle_trigger`, `activate_trigger`, `dismiss_outside`, `close_from_escape`, `register_title`, `register_description`, and `set_bounds`.
- [x] `PopoverRuntime<P>` exposes part-shaped queries returning style-state structs.
- [x] `PopoverRuntime<P>` is unit-testable without a GPUI window.
- [x] `PopoverContext<P>` owns entity/props/controlled-state mediation and exposes only `read`, `update`, and high-level open/close/toggle commands needed by layers.
- [x] `PopoverContext<P>` does not accumulate child-indexing, positioning, hover, or trigger-registration vocabulary that belongs in the runtime.
- [x] `PopoverProps<P>` owns stable root props and callbacks only.
- [x] Typed child enums are used before `AnyElement` erasure.
- [x] `child_wiring.rs` is the only place that walks typed children, scopes child ids, registers trigger metadata, and attaches context.
- [x] Render layers do not inspect runtime internals directly; they use context commands and style-state queries.
- [x] Implementation avoids new generic abstractions unless they hide a deep repeated concept.
- [x] Shared helpers, if needed beyond Popover and Select, live as flat named files under `crates/base_gpui/src/utils/` and are re-exported by `utils/mod.rs`.

### Accessibility / AccessKit audit

- [x] Audit the pinned GPUI version for native AccessKit APIs relevant to button, dialog, modal, label, description, expanded, controls, hidden/inert, and focus-trap semantics.
- [x] Do not add literal DOM ARIA strings such as `aria-haspopup`, `aria-expanded`, `aria-controls`, `aria-labelledby`, or `aria-describedby`.
- [x] If GPUI supports native roles, map trigger to a button-like role and popup to a dialog-like role.
- [x] If GPUI supports native expanded/controls/label relationships, map trigger/popup/title/description semantics through those APIs.
- [x] If GPUI lacks the needed AccessKit APIs, document the accessibility gap in the issue before checking the audit item complete.

### Demos

- [x] Add a basic Popover demo to `crates/base_gpui/src/main.rs`.
- [x] Demo includes trigger, portal, positioner, popup, arrow, title, description, and close.
- [x] Demo starts closed by default.
- [x] Demo shows hover-open behavior if implemented.
- [x] Demo shows multiple triggers or detached handles if implemented.
- [x] Component gallery render test passes without initially showing closed popover panels.

### Tests / verification

- [x] Runtime tests cover uncontrolled default closed state.
- [x] Runtime tests cover `default_open(true)`.
- [x] Runtime tests cover controlled open reconciliation.
- [x] Runtime tests cover controlled open callbacks without internal mutation.
- [x] Runtime tests cover canceled open and canceled close.
- [x] Runtime tests cover `prevent_unmount_on_close()`.
- [x] Runtime tests cover active trigger registration and switching.
- [x] Runtime tests cover typed payload storage and active payload updates.
- [x] Runtime tests cover duplicate/missing trigger-id behavior.
- [x] Runtime tests cover multiple trigger activation.
- [x] Runtime tests cover detached handle open/close if handle support is implemented.
- [x] Runtime tests cover modal flag behavior.
- [x] Runtime tests cover positioner collision/flip/shift decisions.
- [x] Rendered tests cover clicking trigger opens and clicking it again closes.
- [x] Rendered tests cover Escape closing.
- [x] Rendered tests cover `PopoverClose` closing.
- [x] Rendered tests cover outside click dismissal.
- [x] Rendered tests cover disabled trigger not opening.
- [x] Rendered tests cover hover open delay and close delay if timers are implemented.
- [x] Rendered tests cover keep-mounted portal behavior.
- [x] Rendered tests cover focus returning to trigger on close when practical.
- [x] Rendered tests cover modal backdrop occluding outside clicks.
- [x] Rendered tests cover title/description registration style state or metadata.
- [x] Rendered tests cover arrow style state and side updates.
- [x] Rendered tests cover viewport activation direction if viewport support is implemented.
- [x] `cargo fmt --check` passes.
- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui popover` passes.
- [x] `cargo test -p base_gpui` passes.
- [x] `cargo clippy -p base_gpui --all-targets` exits successfully, allowing only existing warnings.
- [x] `ast-grep scan crates/base_gpui/src` passes, including the barrel-only `mod.rs` rule.
- [x] `rg -n "pub\\(" crates/base_gpui/src/popover` returns no scoped visibility.
- [x] `rg -n "RenderState|render_state" crates/base_gpui/src/popover` returns no results.
