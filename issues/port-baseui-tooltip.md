# Port Base UI Tooltip to GPUI

## Problem

Base UI's React Tooltip component provides a sighted-user hint popup that opens from hover or focus, supports controlled and uncontrolled open state, delayed hover timing, provider-level delay grouping, detached and multiple triggers with typed payloads, cursor tracking, hoverable popup behavior, anchored positioning, optional web-styled arrows, and viewport state for trigger-to-trigger content transitions.

`base_gpui` currently has no Tooltip component family. The Popover port now provides useful GPUI-native overlay groundwork, but Tooltip needs its own runtime because its core contract is different: tooltips are hover/focus-driven visual hints, not click-open disclosure popups.

The goal is to port Tooltip behavior and contracts, not React or DOM internals. Web-specific implementation details such as React hooks, Floating UI DOM refs, browser event objects, render props, DOM data attributes, CSS variables, native button switches, and literal ARIA attributes should be dropped or translated into GPUI-native architecture.

Tooltip payloads are in scope and should be Rust-native typed values using a generic payload parameter such as `P: Clone + 'static`; do not port arbitrary JavaScript value semantics.

## Scope

Port the Tooltip component family from Base UI into GPUI-native components:

- `TooltipProvider`
- `TooltipRoot<P>`
- `TooltipTrigger<P>`
- `TooltipPortal<P>`
- `TooltipPositioner<P>`
- `TooltipPopup<P>`
- `TooltipViewport<P>`
- `TooltipHandle<P>` / `create_tooltip_handle(...)` or an equivalent GPUI-native constructor

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/tooltip/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/provider/TooltipProvider.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/provider/TooltipProviderContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/provider/TooltipProvider.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/root/TooltipRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/root/TooltipRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/root/TooltipRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/root/TooltipRoot.detached-triggers.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/trigger/TooltipTrigger.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/trigger/TooltipTrigger.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/trigger/TooltipTrigger.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/trigger/TooltipTriggerDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/portal/TooltipPortal.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/portal/TooltipPortalContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/portal/TooltipPortal.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/positioner/TooltipPositioner.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/positioner/TooltipPositionerContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/positioner/TooltipPositioner.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/positioner/TooltipPositioner.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/positioner/TooltipPositionerDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/positioner/TooltipPositionerCssVars.ts`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/popup/TooltipPopup.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/popup/TooltipPopup.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/popup/TooltipPopupDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/arrow/TooltipArrow.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/arrow/TooltipArrow.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/arrow/TooltipArrowDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/viewport/TooltipViewport.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/viewport/TooltipViewport.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/viewport/TooltipViewportDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/viewport/TooltipViewportCssVars.ts`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/store/TooltipStore.ts`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/store/TooltipHandle.ts`
- `/home/luke/Projects/base-ui/packages/react/src/tooltip/utils/constants.ts`

Base UI docs reference:

- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/tooltip/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/tooltip/types.md`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/tooltip/types.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/tooltip/demos/hero/`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/tooltip/demos/detached-triggers-simple/`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/tooltip/demos/detached-triggers-controlled/`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/tooltip/demos/detached-triggers-full/`

Relevant local GPUI precedent:

- `crates/base_gpui/src/popover/`
  - Reusable architecture precedent for anchored overlay parts, typed payloads, late-bound handles, presence, hover timers, arrow placement, viewport activation direction, and root-scoped trigger IDs.
- `crates/base_gpui/src/select/`
  - Reusable precedent for popup positioning/collision behavior and modal/non-modal outside interaction.
- `crates/base_gpui/src/utils/overlay.rs`
- `crates/base_gpui/src/utils/presence.rs`
- `crates/base_gpui/src/utils/direction.rs`

Current GPUI implementation:

- `crates/base_gpui/src/tooltip/` now contains the first GPUI-native Tooltip pass with provider/root/trigger/portal/positioner/popup/viewport parts, typed payloads, late-bound handles, hover/focus/escape/outside/imperative open-change reasons, delayed hover timers, trigger bounds, popup measurement, cursor-axis anchoring, and runtime/rendered tests.
- `TooltipArrow<P>` is intentionally not ported. Base UI's arrow is a headless DOM `div` whose visual arrow is produced by user CSS/pseudo-elements; Zed and gpui-component tooltips do not expose arrow parts. The GPUI port follows gpui-component's trigger-bounds overlay model instead of emulating Floating UI arrow middleware.

Expected GPUI implementation files:

```text
crates/base_gpui/src/tooltip/
  mod.rs            # barrel only
  actions.rs        # Escape / optional keyboard dispatch
  child.rs          # typed child enums
  child_wiring.rs   # private traversal/context attachment/trigger registration
  context.rs        # TooltipContext<P>
  props.rs          # root/provider/trigger props and callbacks
  runtime.rs        # TooltipRuntime<P> + metadata/outcomes/details
  style_state.rs    # Tooltip*StyleState structs
  layers/
    mod.rs          # barrel only
    tooltip_provider.rs
    tooltip_root.rs
    tooltip_trigger.rs
    tooltip_portal.rs
    tooltip_positioner.rs
    tooltip_popup.rs
    tooltip_viewport.rs
  tests/
```

Also update:

- `crates/base_gpui/src/lib.rs`
- `crates/base_gpui/src/main.rs` with Tooltip smoke scenarios

Implementation notes from the first GPUI port:

- `TooltipProvider<P>` is a typed provider wrapper for same-payload tooltip roots. It passes delay and close-delay defaults to descendant typed roots. The `timeout(...)` configuration is stored, surfaced in provider style state, and used for adjacent-tooltip instant handoff while the delay-group remains recently visible.
- `TooltipRoot<P>::track_cursor_axis(...)` uses GPUI mouse coordinates from trigger `on_mouse_move(...)`; `None`, `X`, `Y`, and `Both` are represented by virtual anchor bounds in `TooltipRuntime<P>`.
- Hoverable popup persistence is implemented with trigger/popup hover state and close-delay timers. `disable_hoverable_popup(true)` closes when the popup itself is hovered instead of altering GPUI pointer-event routing. Tooltip positioning now follows gpui-component's native trigger-bounds overlay model: the popup body is measured, placed against the trigger edge, flipped above/below where necessary, and clamped to the viewport. Moving through an explicit trigger/popup gap still gets a bounded GPUI-native safe-gap grace using measured trigger/popup bounds plus `Window::mouse_position()`, while the default gallery path has no body gap.
- Nested-tooltip ancestor suppression uses GPUI hitbox occlusion on triggers so frontmost nested trigger hitboxes prevent ancestor trigger hitboxes from being considered hovered. Provider handoff now preserves focus-opened and controlled-open ancestors while closing hover-opened ancestors. Rendered tests cover sibling nested triggers, third-level nesting, parent delayed reopen/cancel, focus-open preservation, controlled-open preservation, and nested focus traversal.
- Tooltip viewport exposes activation direction, previous/current popup sizes, and a GPUI-native `payload_content(...)` builder for rendering the current active payload. Full previous/current content morph containers are deferred until GPUI animation/layout infrastructure makes that worthwhile.
- Sticky positioning and arbitrary external anchor elements are intentionally not ported in this pass; Tooltip anchors to registered triggers or cursor-derived virtual bounds.
- Accessibility semantics are intentionally limited to GPUI-native behavior. The first pass emits no literal DOM ARIA/data attributes; tooltips remain visual hints and should not be used as a replacement for accessible trigger labels. The pinned GPUI checkout at `/home/luke/.cargo/git/checkouts/zed-a70e2ad075855582/f7ca86e/crates/gpui/src` does not expose AccessKit role/name/description element APIs, so the conditional native semantic mapping is not applicable until GPUI is upgraded.
- Follow-up passes added rendered/runtime coverage for uncontrolled cancellation, provider delay/close-delay/timeout facts, active-trigger unmount close requests, multiple detached triggers sharing one handle, current viewport payload content, controlled programmatic payload switching, cursor-coordinate anchoring, detached focus runtime handling, and an expanded gallery with basic, delay-group, hoverable, disabled-hoverable, payload, detached, and cursor-tracking smoke scenarios.
- Tooltip positioner now resolves effective side using measured trigger/popup bounds in a deferred GPUI element. The default placement mirrors gpui-component behavior: centered against the trigger edge, flipped when the preferred vertical side does not fit, and clamped to viewport margins.
- Tooltip viewport style state now includes previous/current trigger identity in addition to activation direction and popup sizes, so payload/content transitions can distinguish current and previous content without React render-prop containers.
- Provider delay groups now track the currently active root. Opening a sibling root asks any previously open sibling root in the provider group to close with an instant close reason, avoiding stale transition facts during adjacent handoff.
- Detached trigger registrations carry a root-render generation so active detached triggers close after they stop re-registering, while still-mounted detached triggers are not closed before their later-in-render registration runs.
- Closed `keep_mounted(true)` portal/positioner/popup subtrees are opacity-hidden and `visibility: hidden` via GPUI's `invisible()` style helper, so they remain rendered for style-state/prepaint while GPUI skips paint/event binding. Tooltip-owned popup/positioner event handlers are also omitted while closed; rendered coverage verifies the closed kept-mounted portal does not receive mouse-down callbacks.

## Out of scope / drop from Base UI

- React context, hooks, stores, refs, `FloatingDelayGroup`, `FloatingTree`, `FloatingNode`, and `FloatingPortal` implementation details.
- React children render-function API for payload-driven content. Payload storage and payload-driven content are in scope; only the React pattern of `children={({ payload }) => ...}` is dropped. Expose payload through a GPUI-native content builder, runtime query, open-change details, viewport child routing, or another explicit Rust API.
- `className` and web `style` props.
- `render` prop support.
- Native DOM button switching. Use GPUI interactive elements and focus handles.
- DOM `container` portal prop. GPUI deferred/anchored rendering should use GPUI-native popup rendering.
- Browser event objects in change details. Use GPUI-native source/reason fields.
- Literal DOM `id`, role, `aria-*`, tooltip relationship attributes, or `aria-hidden` unless GPUI-native AccessKit support exists and is explicitly implemented.
- DOM data attributes. Map them into typed `*StyleState` structs.
- CSS variable APIs such as `--anchor-width`, `--available-height`, `--transform-origin`, `--popup-width`, and `--popup-height`. Expose typed style-state measurements instead.
- DOM `ResizeObserver`, `MutationObserver`, `getBoundingClientRect`, `composedPath`, shadow-root traversal, and `safePolygon` implementation details. Translate to GPUI measurement, focus, hit-testing, and pointer behavior where possible.
- Exact CSS animation semantics. Preserve mounted/open/transition/instant facts in style state; real animation timing can be a follow-up if GPUI infrastructure supports it.
- Touch-device browser behavior unless GPUI exposes pointer type information needed to distinguish mouse-like hover from touch.

## Acceptance Criteria

### Module/API surface

- [x] `crates/base_gpui/src/tooltip/` exists with the expected flat component architecture.
- [x] `tooltip/mod.rs` is a barrel-only file with module declarations and re-exports only.
- [x] `tooltip/layers/mod.rs` is a barrel-only file with module declarations and re-exports only.
- [x] `base_gpui::tooltip` is exported from `crates/base_gpui/src/lib.rs`.
- [x] `base_gpui::init(cx)` calls `tooltip::init(cx)` if Tooltip registers actions/key bindings.
- [x] `TooltipProvider` exists and is publicly exported.
- [x] `TooltipRoot<P>` exists and is publicly exported.
- [x] `TooltipTrigger<P>` exists and is publicly exported.
- [x] `TooltipPortal<P>` exists and is publicly exported.
- [x] `TooltipPositioner<P>` exists and is publicly exported.
- [x] `TooltipPopup<P>` exists and is publicly exported.
- [x] `TooltipArrow<P>` is intentionally omitted from the GPUI API; Zed/gpui-component do not expose tooltip arrows and Base UI's DOM arrow depends on user CSS/pseudo-elements.
- [x] `TooltipViewport<P>` exists and is publicly exported.
- [x] `TooltipHandle<P>` and a public constructor exist, using the Zed/Popover late-bound handle pattern where appropriate, or detached-trigger support is explicitly deferred before implementation starts.
- [x] Public payload APIs use Rust-native generics such as `P: Clone + 'static`.
- [x] All public style payloads use `*StyleState` names from `style_state.rs`; no `*RenderState` or `render_state.rs` files are introduced.
- [x] No Tooltip code uses Rust scoped visibility syntax such as `pub(...)`.

### Root API and open state

- [x] `TooltipRoot<P>::new()` exists.
- [x] `TooltipRoot<P>::id(...)` or equivalent stable root identity exists for keyed runtime state.
- [x] `TooltipRoot<P>::default_open(bool)` initializes uncontrolled open state.
- [x] `TooltipRoot<P>::open(bool)` supports controlled open state.
- [x] `TooltipRoot<P>::on_open_change(...)` supports open-change callbacks.
- [x] `TooltipRoot<P>::on_open_change_complete(...)` supports completion callbacks, firing immediately when no transition infrastructure is active.
- [x] `TooltipRoot<P>::disabled(bool)` prevents opening and closes an already-open tooltip with a disabled reason.
- [x] `TooltipRoot<P>::disable_hoverable_popup(bool)` controls whether hovering the popup keeps the tooltip open.
- [x] `TooltipRoot<P>::track_cursor_axis(...)` supports `None`, `X`, `Y`, and `Both` where GPUI exposes cursor coordinates, or the issue documents what is blocked.
- [x] `TooltipRoot<P>::trigger_id(...)` supports controlled active-trigger selection when multiple triggers exist, or a documented GPUI-native equivalent exists.
- [x] `TooltipRoot<P>::default_trigger_id(...)` supports initially open uncontrolled tooltips associated with a trigger, or a documented GPUI-native equivalent exists.
- [x] `TooltipRoot<P>::handle(...)` can associate the root with detached triggers using an `Rc<RefCell<...>>` late-bound handle pattern.
- [x] Controlled `open(...)` takes precedence over `default_open(...)`.
- [x] Controlled roots call `on_open_change(...)` without mutating internal open state.
- [x] Uncontrolled roots call `on_open_change(...)` and mutate internal open state unless canceled.
- [x] An uncontrolled tooltip with `default_open(true)` and `disabled(true)` renders closed without panicking.
- [x] An uncontrolled tooltip with `default_open(true)` and `default_trigger_id(...)` resolves the active trigger payload when that trigger is registered.
- [x] When the active trigger unmounts, the uncontrolled tooltip closes unless the close is canceled.
- [x] If the active trigger becomes unavailable while controlled, runtime state does not invent a replacement trigger.
- [x] Closing can keep the popup mounted when details request `prevent_unmount_on_close()`.
- [x] A later normal close after a prevented-unmount cycle unmounts as expected.

### Open change details

- [x] Add `TooltipOpenChangeReason` with at least `TriggerHover`, `TriggerFocus`, `TriggerPress`, `OutsidePress`, `EscapeKey`, `Disabled`, `ImperativeAction`, and `None`.
- [x] Add `TooltipOpenChangeSource` or equivalent GPUI-native source metadata for pointer, keyboard, touch, focus, imperative, and unknown sources where GPUI can distinguish them.
- [x] Add `TooltipOpenChangeDetails<P>` with `reason`, `source`, `trigger_id`, optional typed payload, `cancelable`, `cancel()`, `is_canceled()`, and `prevent_unmount_on_close()` APIs.
- [x] Open from trigger hover reports `TriggerHover`.
- [x] Close from trigger unhover reports `TriggerHover` or a documented GPUI-native equivalent.
- [x] Open from trigger focus reports `TriggerFocus`.
- [x] Close from trigger blur reports `TriggerFocus` or a documented GPUI-native equivalent.
- [x] Trigger click / pointer-down cancellation and click-to-close report `TriggerPress` when they produce an open-state callback.
- [x] Close from outside interaction reports `OutsidePress` if GPUI outside interaction can be observed for Tooltip.
- [x] Close from Escape reports `EscapeKey`.
- [x] Root becoming disabled reports `Disabled`.
- [x] Handle/action-driven open and close report `ImperativeAction`.
- [x] Internal sibling-tooltip handoff can report `None` when matching Base UI's instant transition behavior.
- [x] Canceling an uncontrolled opening prevents internal mutation.
- [x] Canceling an uncontrolled close prevents internal mutation where the close reason is cancelable.
- [x] Controlled mode still emits details but never mutates internal open state.
- [x] Callback details expose the active trigger id and typed payload when available.
- [x] Browser event propagation controls such as `allowPropagation()` are not ported unless a GPUI-native equivalent exists.

### Provider / delay-group behavior

- [x] `TooltipProvider::new()` exists.
- [x] `TooltipProvider::delay(Duration)` supplies a shared open delay for descendant tooltips.
- [x] `TooltipProvider::close_delay(Duration)` supplies a shared close delay for descendant tooltips.
- [x] `TooltipProvider::timeout(Duration)` controls the adjacent-tooltip instant-open window and defaults to Base UI's `400ms` behavior where practical.
- [x] A trigger-level `delay(...)` overrides provider delay for that trigger.
- [x] A trigger-level `close_delay(...)` overrides provider close delay for that trigger.
- [x] Provider delay `0ms` opens descendant tooltips immediately on hover.
- [x] When one tooltip was recently visible, moving to an adjacent provider-group tooltip opens it immediately during the timeout window.
- [x] Instant handoff state is exposed in style state as `TooltipInstant::Delay` or an equivalent typed value.
- [x] Closing because a sibling tooltip opened uses an instant/dismiss state that avoids stale transition facts.
- [x] Provider state is scoped to its subtree and does not leak across unrelated tooltip groups.

### Trigger behavior

- [x] `TooltipTrigger<P>::new()` exists.
- [x] `TooltipTrigger<P>::id(...)` supports stable trigger identity.
- [x] `TooltipTrigger<P>::payload(P)` associates Rust-native payload data with the trigger.
- [x] `TooltipTrigger<P>::disabled(bool)` prevents opening from hover/focus and reflects disabled state in `TooltipTriggerStyleState`.
- [x] `TooltipTrigger<P>::delay(Duration)` configures delayed hover opening and defaults to `600ms` when no provider override applies.
- [x] `TooltipTrigger<P>::close_delay(Duration)` configures delayed hover closing and defaults to `0ms` when no provider override applies.
- [x] `TooltipTrigger<P>::close_on_click(bool)` defaults to `true`.
- [x] Hovering an enabled trigger opens the tooltip after the effective delay.
- [x] Unhovering the active trigger closes after the effective close delay unless the hoverable popup keeps it open.
- [x] Re-hovering before close delay expires cancels the pending close.
- [x] Moving from one enabled trigger to another updates the active trigger, payload, anchor, and trigger style state.
- [x] Focusing an enabled trigger opens the tooltip without waiting for hover delay.
- [x] Blurring the active focused trigger closes the tooltip unless focus moves into an interaction area that GPUI considers part of the same tooltip interaction.
- [x] Focusing or hovering a disabled trigger while another trigger owns the tooltip closes or deactivates the previous tooltip according to Base UI behavior.
- [x] Clicking a trigger before a pending hover delay completes cancels the pending open when `close_on_click(true)`.
- [x] Pointer-down before a pending hover delay completes cancels the pending open when `close_on_click(true)`.
- [x] Clicking an open active trigger closes it when `close_on_click(true)`.
- [x] Clicking an open active trigger leaves it open when `close_on_click(false)`.
- [x] Clicking before the pending delay does not cancel the hover open when `close_on_click(false)`.
- [x] After click-driven close, hover can open the tooltip again on a later hover.
- [x] Trigger style state reports whether this trigger currently owns the open tooltip.
- [x] Trigger style state reports disabled and focused facts where GPUI exposes them.
- [x] Trigger IDs are root-scoped internally so multiple tooltip roots can reuse public trigger ids safely.
- [x] Duplicate scoped trigger ids are deterministic and documented.

### Hoverable popup and nested trigger behavior

- [x] By default, moving the pointer from the trigger into the tooltip popup keeps the tooltip open.
- [x] When `disable_hoverable_popup(true)`, moving into the popup does not keep the tooltip open.
- [x] `disable_hoverable_popup(true)` makes the positioner/popup non-interactive where GPUI exposes pointer event control, or documents the closest GPUI-native behavior.
- [x] Popup hover uses the active trigger's effective `close_delay`.
- [x] Moving from popup back to trigger keeps or reopens the tooltip without flicker where practical.
- [x] Moving across a small gap between trigger and popup preserves the tooltip when GPUI pointer geometry is sufficient to approximate Base UI's safe-polygon behavior.
- [x] If safe-polygon behavior cannot be implemented with current GPUI APIs, the issue records the API gap and keeps simpler trigger/popup hover persistence.
- [x] Nested tooltip triggers inside another trigger do not open the ancestor tooltip when the nested trigger is hovered.
- [x] Moving between sibling nested triggers does not accidentally open the ancestor tooltip.
- [x] A third-level nested trigger does not open ancestor tooltips.
- [x] Moving from a nested trigger back to the parent trigger area can reopen the parent tooltip after the parent delay.
- [x] Pending parent reopen is canceled when the pointer leaves the parent trigger.
- [x] Pending parent reopen is canceled when the pointer moves onto a nested trigger before the delay expires.
- [x] Hovering a nested trigger closes only hover-opened ancestors, not focus-opened or controlled-open ancestors.
- [x] Focusing a nested tooltip trigger does not open the ancestor tooltip.
- [x] If GPUI cannot observe nested popup ancestry globally, nested behavior is documented as a follow-up with a clear API gap.

### Detached handles, multiple triggers, and payloads

- [x] `create_tooltip_handle<P>()` creates a late-bound handle that can connect detached triggers to a root.
- [x] Detached `TooltipTrigger<P>::handle(...)` can open the associated root on hover.
- [x] Detached `TooltipTrigger<P>::handle(...)` can open the associated root on focus.
- [x] `TooltipHandle<P>::open(trigger_id)` opens the tooltip for a registered trigger id.
- [x] `TooltipHandle<P>::close()` closes the tooltip.
- [x] `TooltipHandle<P>::is_open()` reports current open state or `false` when unbound.
- [x] Handle open returns a recoverable result instead of panicking when the trigger id is missing, or the public API documents why it panics.
- [x] Multiple triggers inside one root can open the same tooltip.
- [x] Multiple detached triggers sharing one handle can open the same tooltip.
- [x] Switching triggers updates the active payload.
- [x] Switching triggers reuses the same popup/positioner runtime identity rather than remounting the whole overlay unless GPUI requires remounting.
- [x] Trigger style state updates so only the active trigger reports open.
- [x] Active trigger unmount closes an uncontrolled tooltip.
- [x] Active detached trigger unmount closes an uncontrolled tooltip.
- [x] Programmatic controlled examples can open trigger 1, open trigger 2, and close the tooltip while preserving payload correctness.

### Portal, mounting, and presence

- [x] `TooltipPortal<P>::new()` exists.
- [x] `TooltipPortal<P>::keep_mounted(bool)` keeps portal children mounted while closed.
- [x] Portal children are omitted when closed and `keep_mounted(false)`.
- [x] Portal children remain rendered but hidden/non-interactive when closed and `keep_mounted(true)`.
- [x] Portal mounted/open facts flow into `TooltipPortalStyleState` if the portal exposes styling.
- [x] `TooltipPopup<P>` can report open/closed transition status using the existing presence helper or a Tooltip-specific presence model.
- [x] `on_open_change_complete(true)` fires after open completion or immediately when no animation tracking exists.
- [x] `on_open_change_complete(false)` fires after close completion or immediately when no animation tracking exists.
- [x] `on_open_change_complete` does not fire on initial closed mount.
- [x] Prevented unmount on one close does not permanently force future closes to stay mounted.

### Positioner and cursor tracking

- [x] `TooltipPositioner<P>::new()` exists.
- [x] Positioner defaults to side `Top` and align `Center`.
- [x] Positioner supports side `Top`, `Bottom`, `Left`, `Right`, `InlineStart`, and `InlineEnd` using the shared direction helper.
- [x] Positioner supports align `Start`, `Center`, and `End`.
- [x] Positioner supports numeric `side_offset(...)`.
- [x] Positioner supports numeric `align_offset(...)`.
- [x] Function-valued offset APIs are either replaced by GPUI-native typed configuration or explicitly dropped as web/JS-specific.
- [x] Positioner supports `collision_padding(...)`.
- [x] Positioner intentionally does not expose `arrow_padding(...)` because `TooltipArrow<P>` is not ported.
- [x] Positioner supports side-axis collision flip or shift using GPUI anchored placement where practical.
- [x] Positioner supports alignment-axis flip or shift where practical.
- [x] Positioner records the effective side/align after collision handling.
- [x] Positioner style state exposes open, side, align, anchor-hidden/anchor-missing, instant, measured anchor bounds, popup bounds, available size, and transform-origin facts where available.
- [x] Positioner bounds update when the active trigger changes.
- [x] Positioner bounds update when trigger or popup layout changes.
- [x] Positioner anchors to the active trigger by default.
- [x] Positioner can anchor to cursor coordinates for configured `track_cursor_axis(...)` values where GPUI exposes cursor coordinates.
- [x] `track_cursor_axis(X)` tracks cursor x while preserving trigger y/side behavior.
- [x] `track_cursor_axis(Y)` tracks cursor y while preserving trigger x/side behavior.
- [x] `track_cursor_axis(Both)` tracks both axes and avoids the popup capturing hover in a way that breaks tracking.
- [x] Disabling cursor tracking while closed returns future opens to trigger-centered anchoring.
- [x] Repeated delayed hovers use the latest cursor coordinate rather than stale coordinates.
- [x] Sticky positioning and explicit arbitrary anchor elements are either implemented through GPUI-native APIs or documented as deferred.

### Popup and viewport behavior

- [x] `TooltipPopup<P>::new()` exists.
- [x] Popup renders arbitrary GPUI child content.
- [x] Popup style state exposes open, side, align, instant, and transition status.
- [x] Popup registers its focus/pointer bounds so hoverable popup behavior can keep the tooltip open.
- [x] Popup does not implement Popover-style close buttons or dialog semantics.
- [x] Tooltip arrow behavior is deliberately omitted in favor of gpui-component/Zed-style body-only tooltips.
- [x] `TooltipViewport<P>::new()` exists.
- [x] Viewport renders the current payload-driven content by default.
- [x] Viewport records previous and current content identity when active trigger/payload changes.
- [x] Viewport style state exposes activation direction, transitioning, instant, previous popup size, and current popup size where available.
- [x] Activation direction reports horizontal and vertical movement between previous and current trigger bounds.
- [x] Rapid trigger changes settle on the latest trigger/payload without stale previous content.
- [x] If full morphing previous/current containers are not practical in GPUI, expose activation-direction facts and document full morphing as a follow-up.

### Styling/state exposure

- [x] `TooltipProviderStyleState` is added only if the provider renders a styleable element; otherwise no empty style state is exposed unnecessarily.
- [x] `TooltipRootStyleState` is added only if the root renders a styleable element; otherwise root remains an injection-only component.
- [x] `TooltipTriggerStyleState` exposes open, active, disabled, focused, hovered, trigger id, and payload-present facts as appropriate.
- [x] `TooltipPortalStyleState` exposes mounted/open facts if useful for styling.
- [x] `TooltipPositionerStyleState` exposes open, side, align, anchor-hidden, instant, transform origin, anchor size, popup size, and available size facts.
- [x] `TooltipPopupStyleState` exposes open, side, align, instant, and transition status.
- [x] `TooltipArrowStyleState` is intentionally absent because `TooltipArrow<P>` is not part of the GPUI Tooltip API.
- [x] `TooltipViewportStyleState` exposes activation direction, transitioning, instant, and measured size facts.
- [x] All parts that draw expose `style_with_state(...)`.
- [x] State formerly expressed through Base UI data attributes is available through typed style state instead of DOM attributes.
- [x] CSS variable facts are represented as typed measurements in style state where useful.
- [x] No `className`, web `style`, render-prop, or DOM data-attribute public API is introduced.

### Architecture / internal primitives

- [x] `TooltipRuntime<P>` owns all Tooltip state: open state, active trigger id, payloads, trigger metadata, hover timers, provider delay-group facts, bounds, cursor anchor, instant state, presence, and viewport transition facts.
- [x] `TooltipRuntime<P>` exposes domain commands such as `sync_children`, `request_open`, `request_close`, `hover_trigger`, `unhover_trigger`, `focus_trigger`, `blur_trigger`, `press_trigger`, `dismiss_outside`, `close_from_escape`, `activate_trigger`, `set_bounds`, and `reconcile`.
- [x] `TooltipRuntime<P>` exposes part-shaped queries returning `Tooltip*StyleState` values.
- [x] `TooltipContext<P>` remains a thin injection vehicle with `read`, `update`, and one or two value-changing methods for controlled/uncontrolled open resolution.
- [x] `TooltipContext<P>` does not grow vocabulary for child indexing, trigger registration, positioning, hover timing, or provider delay grouping; those concepts live in the runtime.
- [x] Child routing uses typed enums before `AnyElement` erasure.
- [x] Child-wiring owns traversal, context attachment, trigger metadata collection, focus-handle collection, and trigger indexing.
- [x] Public layer parts do not expose helper methods solely for child traversal or registration.
- [x] Runtime methods are unit-testable without a GPUI window where possible.
- [x] Timer tasks use generation checks so stale delayed hover opens/closes cannot mutate current state.
- [x] Shared helpers stay flat under `crates/base_gpui/src/utils/` only when they are genuinely shared beyond Tooltip.
- [x] No new `runtime_control.rs` or equivalent trait-boundary file is introduced.
- [x] `mod.rs` files contain only module declarations, re-exports, and test declarations.

### Accessibility / GPUI-native semantics

- [x] Tooltip docs clearly state that tooltips are visual hints and not a substitute for accessible labels.
- [x] Current pinned GPUI accessibility APIs are audited before adding any role/semantic behavior.
- [x] No literal DOM ARIA attributes are emitted.
- [x] If pinned GPUI exposes AccessKit role/name/description APIs, map Tooltip trigger/popup semantics through those GPUI-native APIs.
- [x] If pinned GPUI lacks needed AccessKit support, document the missing APIs and keep semantics as a follow-up.
- [x] Tooltip popup remains non-modal and does not trap focus.
- [x] Tooltip content is not treated as a replacement for trigger accessible names unless GPUI exposes a native relationship that can express this correctly.

### Tests / verification

- [x] Add runtime tests for uncontrolled default closed/open behavior.
- [x] Add runtime tests for controlled open precedence over default open.
- [x] Add runtime tests for `on_open_change` cancellation in uncontrolled mode.
- [x] Add runtime tests for disabled root closing an open tooltip.
- [x] Add runtime tests for trigger hover open delay and close delay.
- [x] Add runtime tests for re-hover canceling a pending close.
- [x] Add runtime tests for click/pointer-down canceling pending hover opens when `close_on_click(true)`.
- [x] Add runtime tests for `close_on_click(false)` preserving pending/open hover behavior.
- [x] Add runtime tests for provider delay, close delay, and instant handoff timeout.
- [x] Add runtime tests for multiple triggers and typed payload switching.
- [x] Add runtime tests for detached handle open/close and missing trigger behavior.
- [x] Add runtime tests for active trigger unmount close and canceled close.
- [x] Add runtime tests for effective placement, transform origin, trigger-bounds placement, and viewport clamping.
- [x] Add runtime tests for viewport activation direction and rapid trigger switching.
- [x] Add rendered behavior tests for hover open/close.
- [x] Add rendered behavior tests for focus open/blur close.
- [x] Add rendered behavior tests for Escape close.
- [x] Add rendered behavior tests for hoverable popup persistence.
- [x] Add rendered behavior tests for `disable_hoverable_popup(true)`.
- [x] Add rendered behavior tests for multiple triggers inside one root.
- [x] Add rendered behavior tests for multiple detached triggers with a shared handle.
- [x] Add rendered behavior tests for controlled root callbacks without internal mutation.
- [x] Add rendered behavior tests for portal `keep_mounted(true)`.
- [x] Add rendered behavior tests for provider adjacent-tooltip instant open.
- [x] Add rendered behavior tests for cursor tracking if GPUI exposes enough cursor coordinates.
- [x] Add rendered behavior tests for nested tooltip trigger suppression where GPUI can express nested hit-testing.
- [x] Add rendered style-state tests for trigger, positioner, popup, portal, and viewport.
- [x] Add a `crates/base_gpui/src/main.rs` gallery section with a minimal Tooltip smoke scenario.
- [x] `cargo fmt --check` passes.
- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui tooltip` passes.
- [x] `cargo test -p base_gpui` passes.
- [x] `cargo clippy -p base_gpui --all-targets` passes or only reports pre-existing warnings.
- [x] `ast-grep scan crates/base_gpui/src` passes.
- [x] Grep confirms no `pub(...)` scoped visibility under `crates/base_gpui/src/tooltip/`.
- [x] Grep confirms no `RenderState` / `render_state` under `crates/base_gpui/src/tooltip/`.

## Research notes

### GPUI cursor, hover, and tooltip support

Local GPUI source audit:

- `/home/luke/Projects/zed/crates/gpui/src/interactive.rs`
  - `MouseMoveEvent` exposes `position: Point<Pixels>`.
  - Mouse down/up, scroll, pinch, and click events also expose positions where relevant.
  - No general pointer-type field was found for distinguishing mouse vs touch/stylus on hover-like movement events.
- `/home/luke/Projects/zed/crates/gpui/src/window.rs`
  - `Window::mouse_position()` is public and returns the latest mouse position.
  - Window input dispatch updates this position on mouse move/down/up, scroll, pinch, and file-drag events.
  - `TooltipId::is_hovered(window)` can test whether GPUI's current tooltip bounds contain the current mouse position.
- `/home/luke/Projects/zed/crates/gpui/src/elements/div.rs`
  - `InteractiveElement::on_mouse_move(...)` is available and only fires when the element hitbox is hovered.
  - `StatefulInteractiveElement::on_hover(...)` reports hover start/end.
  - GPUI already has built-in `tooltip(...)`, `hoverable_tooltip(...)`, and `tooltip_show_delay(...)` element APIs.
  - Built-in hoverable tooltip behavior uses a hard-coded hoverable-hide delay and tooltip-bounds hit testing; it is useful precedent, but it is not enough for Base UI parity because it is mouse-position anchored and does not expose Tooltip compound parts, payloads, controlled open state, provider grouping, trigger focus behavior, or Base UI positioning/arrow/viewport APIs.

Local `gpui-component` source audit:

- `/home/luke/Projects/gpui-component/crates/ui/src/tooltip.rs`
  - Implements a per-window `TooltipOverlay` with delayed show, recent-tooltip grace period, trigger bounds measured via `on_prepaint(...)`, above/below placement, viewport clamping, and immediate switching during the grace period.
  - This is strong precedent for provider/delay-group behavior, but it does not implement Base UI-style controlled state, payloads, cursor tracking, focus open, hoverable popup persistence, arrows, or compound parts.
- `/home/luke/Projects/gpui-component/crates/ui/src/hover_card.rs`
  - Implements trigger/content hover persistence with `is_hovering_trigger`, `is_hovering_content`, open/close timers, and stale-timer epochs.
  - This is the closest local precedent for Tooltip hoverable popup behavior.

Conclusion for the cursor/pointer uncertainty:

- Cursor coordinates are available enough for `track_cursor_axis(...)`: store the latest `MouseMoveEvent.position` / `Window::mouse_position()` in the runtime and derive a virtual anchor for `None`, `X`, `Y`, or `Both` tracking modes.
- Mouse-only hover parity is only partially confirmable: GPUI's public hover/move events are mouse-oriented, but no public pointer-type API was found to distinguish mouse from touch/stylus hover. An external crate cannot solve that unless GPUI/platform exposes the raw pointer-source data.

## Uncertain items to confirm during implementation

- Whether GPUI exposes enough pointer-type information to distinguish mouse from touch/stylus for strict mouse-only hover parity.
- Whether GPUI exposes enough global popup ancestry / hit-testing to implement Base UI's full nested-tooltip and safe-polygon behavior.
- Whether provider delay grouping should be implemented as a rendered `TooltipProvider` context component, a keyed runtime registry, or a smaller first-pass context wrapper.
- Whether full viewport previous/current morphing is worth implementing immediately, or whether activation-direction style state is enough for the initial port.
- Whether `TooltipHandle::open(trigger_id)` should return `bool`/`Result` like the Popover GPUI port or panic like Base UI when a trigger is missing.
