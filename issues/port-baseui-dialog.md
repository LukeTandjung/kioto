# Port Base UI Dialog to GPUI

## Problem

Base UI's React Dialog component provides a modal/non-modal popup opened by one or more triggers, with controlled/uncontrolled open state, close/dismiss semantics, focus management, backdrop/portal mounting, nested-dialog behavior, detached trigger handles, and state-aware styling for each part.

`base_gpui` does not currently have a `dialog` module. The goal is to port Dialog behavior and public contracts into GPUI-native components, not to translate React hooks, DOM portals, ARIA attributes, CSS variables, or Floating UI internals literally.

Dialog payloads should use Rust generics instead of JavaScript values, e.g. `P: Clone + 'static`. Dialog does not select among values, so `Eq` should only be required where a specific API needs equality.

## Scope

Port the Dialog component family from Base UI into GPUI-native components:

- `DialogRoot<P = ()>`
- `DialogTrigger<P = ()>`
- `DialogPortal<P = ()>`
- `DialogBackdrop<P = ()>`
- `DialogViewport<P = ()>`
- `DialogPopup<P = ()>`
- `DialogTitle<P = ()>`
- `DialogDescription<P = ()>`
- `DialogClose<P = ()>`
- `DialogHandle<P = ()>` and a `create_dialog_handle()` convenience if it improves ergonomics

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/dialog/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/root/DialogRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/root/useRenderDialogRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/root/useDialogRoot.ts`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/root/DialogRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/root/DialogRoot.detached-triggers.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/store/DialogStore.ts`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/store/DialogHandle.ts`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/trigger/DialogTrigger.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/trigger/DialogTrigger.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/portal/DialogPortal.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/portal/DialogPortal.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/backdrop/DialogBackdrop.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/backdrop/DialogBackdrop.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/viewport/DialogViewport.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/viewport/DialogViewport.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/popup/DialogPopup.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/popup/DialogPopup.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/title/DialogTitle.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/title/DialogTitle.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/description/DialogDescription.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/description/DialogDescription.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/close/DialogClose.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/close/DialogClose.test.tsx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/dialog/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/dialog/types.md`

Expected GPUI implementation files:

- `crates/base_gpui/src/dialog/mod.rs`
- `crates/base_gpui/src/dialog/actions.rs`
- `crates/base_gpui/src/dialog/runtime.rs`
- `crates/base_gpui/src/dialog/context.rs`
- `crates/base_gpui/src/dialog/props.rs`
- `crates/base_gpui/src/dialog/style_state.rs`
- `crates/base_gpui/src/dialog/child.rs`
- `crates/base_gpui/src/dialog/child_wiring.rs`
- `crates/base_gpui/src/dialog/layers/dialog_root.rs`
- `crates/base_gpui/src/dialog/layers/dialog_trigger.rs`
- `crates/base_gpui/src/dialog/layers/dialog_portal.rs`
- `crates/base_gpui/src/dialog/layers/dialog_backdrop.rs`
- `crates/base_gpui/src/dialog/layers/dialog_viewport.rs`
- `crates/base_gpui/src/dialog/layers/dialog_popup.rs`
- `crates/base_gpui/src/dialog/layers/dialog_title.rs`
- `crates/base_gpui/src/dialog/layers/dialog_description.rs`
- `crates/base_gpui/src/dialog/layers/dialog_close.rs`
- `crates/base_gpui/src/dialog/layers/mod.rs`
- `crates/base_gpui/src/dialog/tests/`

Current GPUI implementation:

- No `crates/base_gpui/src/dialog/` module exists yet.
- Use `crates/base_gpui/src/popover/` as a close implementation precedent for open state, handles, trigger metadata, portal/backdrop/popup layers, outside press, Escape close, focus return, and `prevent_unmount_on_close` semantics.
- Do not blindly reuse or generalize Popover internals unless a shared helper hides a repeated deep concept without adding generic ontology.

## GPUI capability audit for open questions

Current pinned GPUI dependency is `github.com/zed-industries/zed#f7ca86e6eeabd135645c4f25aa1ae83f5cf0231b`, with source at `/home/luke/.cargo/git/checkouts/zed-a70e2ad075855582/f7ca86e/crates/gpui`.

Focus and trapping:

- GPUI has `FocusHandle`, `.track_focus(...)`, `.focusable()`, `.tab_index(...)`, `.tab_stop(...)`, `.tab_group()`, `Window::focused(...)`, `Window::focus(...)`, `Window::focus_next(...)`, `Window::focus_prev(...)`, `Window::on_focus_in(...)`, and `Window::on_focus_out(...)`.
- `focus_next` / `focus_prev` operate on the window's global tab-stop map and wrap across the whole window. GPUI does not expose a built-in scoped focus trap or first-focusable-descendant query.
- Dialog should therefore own popup focus metadata in `DialogRuntime<P>` and implement trap-focus with Dialog-specific key actions plus registered popup focus handles, rather than relying on a GPUI-native focus manager.
- Default initial focus can target a registered popup/descendant focus handle if Dialog child wiring records one. If no descendant is registered, focus the popup's own `FocusHandle` as the first-pass fallback.
- Default final focus can use `Window::focused(...)` before opening plus active-trigger focus handles for return focus. There is no separate browser-style restore-focus manager.

Overlay and portal behavior:

- GPUI has `deferred(...)` / `Window::defer_draw(...)` for drawing an element above its ancestors, with numeric priority ordering. Existing Popover uses this for overlay-like popup rendering.
- GPUI has `InteractiveElement::occlude()` / `block_mouse_except_scroll()` and hitbox behaviors that block mouse interaction behind an overlay.
- GPUI has `on_mouse_down_out(...)` and capture/bubble mouse dispatch phases for outside-press handling.
- GPUI does not have a DOM-like portal container API. DialogPortal should be a GPUI-native mounting/deferred-overlay boundary, not a literal container target.

Measurement and layout:

- GPUI has `Div::on_children_prepainted(...)` for child bounds and `Window::defer_draw(...)` / `Window::paint_layer(...)` for prepaint/paint layering. Use these instead of DOM measurement APIs.

Accessibility:

- The pinned `gpui` crate has no `accesskit` dependency and no public `.role(...)`, `aria_*`, `on_a11y_action`, disabled/expanded/modal/label relationship APIs found by local source search.
- Keep Dialog accessibility as follow-up only; do not implement literal DOM ARIA attributes.

Payload-specific content:

- GPUI composition supports `AnyElement`, `IntoElement`, `RenderOnce`, and builder closures that receive `&mut Window` / `&mut App`. If Dialog needs payload-dependent content, prefer a typed GPUI builder such as `DialogRoot::payload_content(|payload, window, cx| -> AnyElement { ... })` or the existing child tree plus payload in style state/change details; do not port React function-as-children literally.

## Out of scope / drop from Base UI

- React context/hooks/store implementation details.
- React `render` props and function-as-children rendering. If payload-specific content is needed, expose a GPUI-native typed payload/content API instead of porting React render props literally.
- `className` and web `style` props.
- `nativeButton` options; GPUI controls should use `div()` plus GPUI focus/click/action/accessibility behavior.
- DOM `container` portal targets. Use GPUI-native portal/window/layer mechanisms instead.
- SSR, hydration, CSP nonce, and prehydration APIs.
- DOM event objects in change details. Use Rust-native reasons/sources and GPUI data such as `ElementId`.
- DOM `id`, `aria-controls`, `aria-labelledby`, `aria-describedby`, `aria-expanded`, `aria-haspopup`, `role`, `hidden`, and `inert` attributes as literal public API.
- DOM CSS variables such as `--nested-dialogs`; expose typed style-state fields instead.
- DOM data attributes such as `data-open`, `data-closed`, `data-nested`, `data-nested-dialog-open`, `data-popup-open`, `data-disabled`, `data-starting-style`, and `data-ending-style`; expose typed style-state fields instead.
- DOM `ResizeObserver`, `MutationObserver`, `getBoundingClientRect`, and Floating UI internals.
- Browser-only touch-reader guidance as literal behavior. Keep the useful escape/focus implications in GPUI terms.
- Drawer and AlertDialog-specific behavior. Dialog should leave room for those components to reuse parts later, but this issue is for Dialog.

## Acceptance Criteria

### Module/API surface

- [x] Add a `dialog` module and export it from `crates/base_gpui/src/lib.rs`.
- [x] Register Dialog key bindings from `base_gpui::init(cx)`.
- [x] Add public `DialogRoot<P = ()>`, `DialogTrigger<P = ()>`, `DialogPortal<P = ()>`, `DialogBackdrop<P = ()>`, `DialogViewport<P = ()>`, `DialogPopup<P = ()>`, `DialogTitle<P = ()>`, `DialogDescription<P = ()>`, and `DialogClose<P = ()>` layer types.
- [x] Add a public `DialogHandle<P = ()>` and a `create_dialog_handle()` convenience if the handle API needs construction symmetry with Base UI.
- [x] Constrain payloads as `P: Clone + 'static` unless a specific API proves equality is required.
- [x] Support uncontrolled construction with `.default_open(bool)`, defaulting to `false`.
- [x] Support controlled construction with `.open(bool)`; calling the builder marks the root controlled even when `false`.
- [x] Support `.on_open_change(...)` on `DialogRoot<P>` with a Rust-native cancelable change-details API, e.g. `Fn(bool, &mut DialogOpenChangeDetails<P>, &mut Window, &mut App)`.
- [x] Support `.on_open_change_complete(...)` on `DialogRoot<P>` with a Rust-native API called after immediate close/open completion or after any implemented GPUI-native transition completes.
- [x] Support `.modal(DialogModalMode)` or equivalent, defaulting to modal behavior.
- [x] Represent Base UI `modal=true`, `modal=false`, and `modal='trap-focus'` explicitly, e.g. `DialogModalMode::{Modal, NonModal, TrapFocus}`.
- [x] Support `.disable_pointer_dismissal(bool)`, defaulting to `false`.
- [x] Support `.default_trigger_id(...)` for uncontrolled initially open dialogs.
- [x] Support `.trigger_id(...)` for controlled dialogs; setting it marks trigger id controlled even when `None`.
- [x] Support `.handle(DialogHandle<P>)` on `DialogRoot<P>`.
- [x] Support `.handle(DialogHandle<P>)`, `.payload(P)`, `.id(...)`, and `.disabled(bool)` on `DialogTrigger<P>`.
- [x] Support `.keep_mounted(bool)` on `DialogPortal<P>`, defaulting to `false`.
- [x] Support `.force_render(bool)` on `DialogBackdrop<P>`, defaulting to `false`.
- [ ] Support `.initial_focus(...)` and `.final_focus(...)` on `DialogPopup<P>` using GPUI-native focus handles or focus policy enums.
- [x] Support `.disabled(bool)` on `DialogClose<P>`, defaulting to `false`.
- [x] `dialog/mod.rs` remains barrel-only and exposes ergonomic exports for component names, style states, context, props, runtime, actions, handles, and child types.
- [x] `DialogRoot<P>` does not impose visible layout by default; if it renders a GPUI wrapper for context propagation, it is visually neutral.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui dialog` passes.
- [x] `cargo test -p base_gpui` passes.
- [x] `ast-grep scan crates/base_gpui/src` passes.
- [x] The component compiles without adding web/React-specific concepts to public APIs.
- [x] The implementation follows `docs/base-gpui-component-architecture.md` and uses the flat `runtime.rs` / `context.rs` / `props.rs` / `style_state.rs` / `child.rs` / `child_wiring.rs` / `layers/` shape.
- [x] Add a small gallery demo in `crates/base_gpui/src/main.rs` with a basic modal dialog.
- [ ] Add a compact gallery example for either non-modal mode, nested dialogs, or detached triggers.

### Architecture / internal primitives

- [x] Add `DialogRuntime<P>` as the single owner of Dialog business state: open/mounted state, controlled observed open value, trigger metadata, active trigger id, active payload, modal mode, dismiss flags, nested dialog counts, title/description metadata, focus handles, and implemented transition state.
- [x] Add `DialogProps<P>` for stable root props and callbacks.
- [x] Add `DialogContext<P>` as thin entity/props/controlled-state injection with only `read(...)`, `update(...)`, and value-changing methods such as `set_open(...)`, `open_trigger(...)`, and `close(...)`.
- [x] Keep controlled/uncontrolled resolution in `DialogContext<P>`, not in layers.
- [x] Keep Dialog behavior on `DialogRuntime<P>`; do not grow component vocabulary on `DialogContext<P>` beyond value-changing mediation.
- [x] Add `DialogTriggerMetadata<P>` for trigger id, source id, payload, disabled state, focus handle, order, and detached/owned status.
- [x] Add runtime-owned metadata for popup, portal, viewport, title, description, backdrop, and close button state only where those parts affect behavior.
- [x] Add typed child routing in `dialog/child.rs` and private traversal/context attachment in `dialog/child_wiring.rs`.
- [x] Child indexing/ordering for triggers lives only in `child_wiring.rs`; do not recompute trigger order in layer render paths.
- [x] Add renderable GPUI elements only under `dialog/layers/`.
- [x] Do not add a `utils/` folder for Dialog.
- [x] Do not extract a new shared overlay primitive unless Dialog and existing Popover/Tooltip code clearly share a repeated deep concept that remains simpler after extraction.
- [x] If code is shared with Popover, keep the public Dialog API independent and behaviorally named in Dialog terms.

### Stateful/stateless behavior

- [x] Uncontrolled Dialog initializes from `default_open`, defaulting to closed.
- [x] Uncontrolled Dialog commits accepted user, outside, Escape, close, and imperative open changes to internal state.
- [x] Controlled Dialog reflects external `open`.
- [x] Controlled Dialog calls `on_open_change` on accepted open/close requests without mutating internal open state as source of truth.
- [ ] External controlled open changes update root, trigger, portal, backdrop, viewport, popup, and close style state.
- [ ] Re-rendering with changed unrelated props does not reset uncontrolled open state except when the keyed root id changes.
- [x] Open/closed/mounted state is modeled as normal runtime state, not as special cases spread across layers.
- [ ] Controlled `trigger_id` identifies the active trigger when controlled open is true.
- [x] Missing controlled trigger ids do not panic, do not invent fake triggers, and still allow the dialog to render open without trigger association.
- [ ] `default_trigger_id` associates an initially open uncontrolled dialog with a registered trigger when available.
- [ ] Trigger metadata updates when triggers are inserted, removed, disabled, or reordered.
- [ ] Trigger ownership is preserved when another trigger mounts while the dialog is open.

### Open change details and cancellation

- [x] Add `DialogOpenChangeReason::{TriggerPress, OutsidePress, EscapeKey, ClosePress, FocusOut, ImperativeAction, None}`.
- [x] Add a source enum such as `DialogOpenChangeSource::{Pointer, Touch, Keyboard, Focus, Imperative, Unknown, None}` without exposing DOM event objects.
- [x] Add `DialogOpenChangeDetails<P>` with `reason()`, `source()`, `trigger_id()`, `payload()`, `cancelable()`, `cancel()`, `is_canceled()`, `prevent_unmount_on_close()`, and `prevents_unmount_on_close()` APIs.
- [x] Trigger clicks/keyboard activation use `DialogOpenChangeReason::TriggerPress`.
- [x] Close button activation uses `DialogOpenChangeReason::ClosePress`.
- [x] Escape close uses `DialogOpenChangeReason::EscapeKey`.
- [x] Outside press close uses `DialogOpenChangeReason::OutsidePress`.
- [ ] Non-modal focus-out close, if implemented, uses `DialogOpenChangeReason::FocusOut`.
- [x] Handle/actions close/open uses `DialogOpenChangeReason::ImperativeAction`.
- [x] `on_open_change` receives the next open bool.
- [x] `on_open_change` receives the active trigger id and payload when the change is trigger-related.
- [x] Canceling an uncontrolled open request prevents internal open mutation.
- [x] Canceling an uncontrolled close request leaves the dialog open.
- [x] Canceling a controlled request still calls the handler but does not mutate internal open state.
- [x] Disabled trigger activation calls neither `on_open_change` nor internal open-change notifications.
- [x] Disabled close activation calls neither `on_open_change` nor internal open-change notifications.
- [x] `prevent_unmount_on_close()` keeps closing content mounted until an explicit unmount/completion path is invoked.
- [x] Do not expose Base UI's native `event`, `trigger` element object, `allowPropagation`, or propagation APIs literally.

### Trigger and handle behavior

- [x] Clicking an enabled in-root trigger opens the dialog.
- [x] Keyboard-activating an enabled in-root trigger opens the dialog.
- [x] A disabled trigger is not activatable and exposes disabled style state.
- [x] Multiple in-root triggers can open the same dialog.
- [x] The active trigger id updates to the trigger that most recently opened the dialog.
- [x] Trigger style state marks only the active trigger as open when the dialog is open.
- [x] Trigger style state exposes `disabled`, `open`, `active_trigger`, `focused`, `payload_present`, and payload when available.
- [x] Detached triggers can open a dialog through a shared `DialogHandle<P>`.
- [x] Detached triggers can be rendered outside `DialogRoot<P>` without panicking when a handle is supplied.
- [x] A `DialogTrigger<P>` rendered without a root context and without a handle reports a clear error or is a no-op by intentional design.
- [x] `DialogHandle<P>::open(trigger_id, window, cx)` opens the dialog and associates the matching trigger when found.
- [x] `DialogHandle<P>::open_with_payload(payload, window, cx)` opens without requiring trigger association.
- [x] `DialogHandle<P>::close(window, cx)` closes the dialog.
- [x] `DialogHandle<P>::is_open(cx)` or equivalent reports current open state.
- [x] Missing handle trigger ids open deterministically without association and do not panic.
- [x] Payload from the active trigger is available to style state and change details.
- [ ] Switching active triggers while open updates payload without remounting popup content unnecessarily.
- [ ] Handle recreation/reparenting cases remain deterministic or are explicitly unsupported in the public API.

### Portal, mounting, and presence behavior

- [x] `DialogPortal<P>` renders children only while the dialog is mounted by default.
- [x] `DialogPortal<P>::keep_mounted(true)` keeps portal children in the element tree while closed.
- [ ] Kept-mounted closed portal children are visually hidden or inert through GPUI-native mechanisms, not DOM `hidden` / `inert` attributes.
- [x] Portal style state exposes at least `open` and `mounted`.
- [x] Popup, viewport, and backdrop derive mounted/present state from the root runtime, not independent local booleans.
- [x] Closing a non-kept-mounted dialog removes portal contents immediately unless GPUI-native transition state or `prevent_unmount_on_close` is active.
- [x] `prevent_unmount_on_close` keeps mounted state true after a close request until an explicit unmount/completion command.
- [x] `on_open_change_complete` is called after open/close completion, and after transition completion if transition sequencing is implemented.
- [ ] Unmounting/removing an open dialog cleans up handle/runtime state deterministically.

### Popup, focus, and keyboard behavior

- [x] `DialogPopup<P>` owns a stable keyed `FocusHandle`.
- [ ] Opening a modal or trap-focus dialog moves focus according to the configured initial focus policy.
- [x] Default initial focus moves to the first registered popup descendant focus handle when available; otherwise it focuses the popup's own `FocusHandle`.
- [ ] Initial focus can target the popup itself through a GPUI-native policy.
- [ ] `initial_focus(false)` or equivalent suppresses focus movement.
- [ ] Closing returns focus according to the configured final focus policy.
- [x] Default final focus returns to the active trigger when available; otherwise it returns to the previously focused `FocusHandle` captured before opening when still valid.
- [ ] `final_focus(false)` or equivalent suppresses focus return.
- [x] Modal and trap-focus modes trap Tab/Shift+Tab focus within the popup using Dialog-owned registered focus handles and key actions.
- [x] Non-modal mode does not trap focus.
- [ ] Escape closes only the topmost open dialog.
- [ ] Escape close is cancelable through `on_open_change`.
- [ ] Composite/descendant keyboard handlers inside popup do not accidentally trigger parent composite navigation if GPUI equivalents exist.
- [x] Dialog uses GPUI actions/key dispatch and Dialog-specific key contexts instead of raw DOM-style key handlers.
- [ ] Focus state relevant to popup/trigger styling is synced into `DialogRuntime<P>`.

### Pointer dismissal and modal behavior

- [x] `DialogModalMode::Modal` blocks pointer interaction with outside content while open using GPUI-native overlay/backdrop mechanisms.
- [ ] `DialogModalMode::TrapFocus` traps focus without blocking outside pointer interactions or scroll-locking semantics.
- [ ] `DialogModalMode::NonModal` allows outside interaction by default.
- [x] Outside press closes modal dialogs only when the press targets the owning backdrop/internal backdrop or an otherwise valid outside target.
- [ ] Outside press closes non-modal dialogs when pointer dismissal is enabled.
- [x] `.disable_pointer_dismissal(true)` prevents pointer outside dismissal.
- [ ] For non-modal dialogs, `.disable_pointer_dismissal(true)` also prevents focus-out dismissal if focus-out close is implemented.
- [x] Non-main mouse button presses do not dismiss the dialog.
- [ ] Outside press dismissal does not close a parent dialog when interacting with a nested dialog, nested menu, nested select, or other known popup child.
- [x] Modal scroll-lock behavior is documented as unsupported for the initial GPUI port unless a project-local GPUI-native mechanism is introduced; do not add web scroll-lock APIs.

### Backdrop behavior

- [x] `DialogBackdrop<P>` renders for a non-nested dialog while mounted.
- [ ] Nested dialog backdrops are suppressed by default.
- [ ] `.force_render(true)` renders nested backdrops.
- [x] Backdrop style state exposes at least `open`, `closed`, `mounted`, `transitioning`, and whether it is nested/force-rendered if useful.
- [x] Clicking the user-visible backdrop requests close with `DialogOpenChangeReason::OutsidePress`.
- [x] Backdrop is non-selectable/non-interactive except for dismissal behavior.
- [x] Backdrop can be omitted without breaking popup behavior.

### Viewport behavior

- [x] `DialogViewport<P>` can wrap `DialogPopup<P>` as a positioning/scroll container.
- [x] Viewport renders only while mounted by default.
- [x] Viewport remains rendered while closed when the containing portal is kept mounted.
- [ ] Closed kept-mounted viewport is visually hidden or non-interactive using GPUI-native mechanisms.
- [x] Viewport style state exposes at least `open`, `closed`, `mounted`, `transitioning`, `nested`, `nested_dialog_open`, and nested dialog count.
- [x] Viewport can be omitted without breaking popup behavior.

### Popup/content behavior

- [x] `DialogPopup<P>` renders its children when mounted/open.
- [x] Popup style state exposes at least `open`, `closed`, `mounted`, `transitioning`, `nested`, `nested_dialog_open`, nested dialog count, active trigger id, and active payload.
- [x] Popup can be rendered directly inside `DialogPortal<P>` or inside `DialogViewport<P>` following Base UI anatomy.
- [x] Popup can include elements outside an inner styled container without breaking focus order or outside-press behavior.
- [x] Popup registers title/description metadata for future accessibility relationships.
- [x] Popup can be omitted without panicking; omitted popup means focus/portal behavior is limited but root/trigger state remains deterministic.

### Title, description, and close behavior

- [x] `DialogTitle<P>` renders arbitrary GPUI children.
- [x] `DialogDescription<P>` renders arbitrary GPUI children.
- [x] Title and description register their presence/ids or logical metadata with `DialogRuntime<P>` for future accessibility mapping.
- [x] Title style state is exposed through `style_with_state(...)`, even if initially empty or root-derived.
- [x] Description style state is exposed through `style_with_state(...)`, even if initially empty or root-derived.
- [x] `DialogClose<P>` closes the dialog when enabled and the dialog is open.
- [x] `DialogClose<P>` is a no-op when disabled.
- [x] Close style state exposes at least `disabled` and whether the owning dialog is open if useful.
- [x] Close can be omitted without preventing outside/Escape dismissal.
- [x] Close can be rendered inside `DialogPopup<P>` following Base UI anatomy.

### Nested dialog behavior

- [ ] A dialog rendered inside another open dialog is marked nested in runtime/style state.
- [ ] Parent dialog style state exposes `nested_dialog_open` and nested dialog count.
- [ ] Nested dialog open/close updates parent nested counts deterministically.
- [ ] Backdrops of child dialogs are suppressed by default, matching Base UI behavior.
- [ ] Escape closes only the deepest/topmost open dialog.
- [ ] Outside press dismisses non-nested sibling dialogs one by one, not all at once.
- [ ] Unmounting an open nested dialog decrements parent nested counts.
- [ ] Unmounting a closed nested dialog does not perturb parent nested counts.
- [ ] Nested Dialog should not close parent Menu/Select/Popover-like components unless that behavior is explicitly introduced through a shared overlay stack.

### Transition and measurement behavior

- [ ] Expose a typed transition-like state only if GPUI can make it meaningful, e.g. `DialogTransitionStatus::{Starting, Ending, Idle}` or the existing simpler presence state.
- [x] Do not copy Base UI DOM transition attributes (`data-starting-style`, `data-ending-style`) as attributes.
- [ ] If transition state is implemented, keep transition sequencing inside `DialogRuntime<P>`; layers should only query style state.
- [ ] If popup/viewport dimensions are implemented, measure with GPUI-native layout/prepaint mechanisms, not DOM `getBoundingClientRect` or `ResizeObserver`.
- [ ] If dimensions are implemented, expose them as typed fields on `DialogPopupStyleState<P>` / `DialogViewportStyleState<P>`, not CSS variables.
- [x] Expose nested dialog count as a typed numeric field, not as `--nested-dialogs`.

### Styling/state exposure

- [x] Add `DialogRootStyleState<P>`, `DialogTriggerStyleState<P>`, `DialogPortalStyleState`, `DialogBackdropStyleState`, `DialogViewportStyleState<P>`, `DialogPopupStyleState<P>`, `DialogTitleStyleState<P>`, `DialogDescriptionStyleState<P>`, and `DialogCloseStyleState` in `style_state.rs`.
- [x] Expose state-aware styling through `style_with_state(...)` on root, trigger, portal, backdrop, viewport, popup, title, description, and close.
- [x] `DialogRootStyleState<P>` includes at least open, mounted, modal mode, disable-pointer-dismissal, active trigger id, active payload, payload presence, nested, nested dialog count, and trigger availability.
- [x] `DialogTriggerStyleState<P>` includes at least disabled, open, active-trigger, focused, payload presence, and payload.
- [x] `DialogPortalStyleState` includes at least open and mounted.
- [x] `DialogBackdropStyleState` includes at least open, closed, mounted/present, transitioning, nested, and force-rendered/effective-rendered information.
- [x] `DialogViewportStyleState<P>` includes at least open, closed, mounted/present, transitioning, nested, nested-dialog-open, nested dialog count, active trigger id, and active payload.
- [x] `DialogPopupStyleState<P>` includes at least open, closed, mounted/present, transitioning, nested, nested-dialog-open, nested dialog count, active trigger id, active payload, and modal mode.
- [x] `DialogTitleStyleState<P>` and `DialogDescriptionStyleState<P>` include root-derived fields only if useful; otherwise they may be empty but still typed.
- [x] `DialogCloseStyleState` includes at least disabled and open if useful.
- [x] Map Base UI state/data attributes into typed style-state fields, not DOM attributes.
- [x] Do not expose CSS variable names as the styling API.
- [ ] The docs hero styling pattern can be recreated with GPUI builder methods: backdrop state styling, popup open/closed styling, nested styling, and focus-visible styling on controls.

### Accessibility follow-up

See `## AccessKit accessibility follow-up` at the end of this issue for the concrete per-part plan against the pinned gpui AccessKit surface.

### Tests / verification

Add behavior-level tests under `crates/base_gpui/src/dialog/tests/`.

- [x] Uncontrolled initial state defaults to closed.
- [x] Uncontrolled `default_open(true)` opens and mounts the dialog.
- [ ] Controlled `open(true)` opens and mounts the dialog.
- [ ] Controlled `open(false)` closes without using internal state as source of truth.
- [ ] External controlled open changes update root, trigger, portal, backdrop, viewport, popup, and close style state.
- [ ] Re-rendering unrelated props does not reset uncontrolled open state.
- [ ] Root keyed id changes reset uncontrolled open state.
- [x] Click trigger opens a closed dialog.
- [x] Disabled trigger click does not open and does not call handlers.
- [ ] Keyboard trigger activation opens a closed dialog.
- [ ] Multiple in-root triggers can open the same dialog.
- [ ] Active trigger state follows the trigger that opened the dialog.
- [x] Detached trigger opens a dialog through a handle.
- [x] Handle `open` and `close` work.
- [ ] Handle `open_with_payload` exposes payload in style state and details.
- [ ] Missing handle trigger id opens without panic and without fake trigger association.
- [ ] Controlled `trigger_id` marks the matching trigger active while open.
- [ ] Missing controlled `trigger_id` does not panic.
- [x] Payload from trigger is reflected in root/popup/trigger style state.
- [x] Close button closes an open dialog.
- [ ] Disabled close button does not close and does not call handlers.
- [x] Escape closes an open dialog.
- [ ] Escape close is cancelable.
- [x] Outside press closes a modal dialog through the owning backdrop.
- [ ] Outside press closes a non-modal dialog.
- [ ] Non-main mouse button outside press does not close.
- [ ] `disable_pointer_dismissal(true)` prevents outside press close.
- [ ] `disable_pointer_dismissal(true)` prevents non-modal focus-out close if focus-out close is implemented.
- [x] `on_open_change` receives trigger-press, close-press, outside-press, escape-key, focus-out, and imperative-action reasons where applicable.
- [ ] `on_open_change` receives pointer, keyboard, focus, and imperative sources where applicable.
- [ ] Canceling open prevents uncontrolled mutation.
- [ ] Canceling close leaves uncontrolled dialog open.
- [ ] Controlled cancellation calls handlers but does not mutate internal source of truth.
- [ ] `prevent_unmount_on_close()` keeps a closing dialog mounted until explicit unmount/completion.
- [ ] `on_open_change_complete` fires after open and close completion when no transition is implemented.
- [x] Portal is omitted while closed by default.
- [ ] Portal `keep_mounted(true)` keeps closed contents rendered and hidden/inactive.
- [x] Backdrop renders for root dialog by default.
- [ ] Nested backdrop is suppressed by default.
- [ ] Backdrop `force_render(true)` renders nested backdrops.
- [x] Backdrop click requests outside-press close.
- [ ] Viewport renders only while mounted by default.
- [ ] Viewport stays mounted inside a keep-mounted portal.
- [ ] Popup receives nested and nested-dialog-open style state.
- [ ] Parent nested dialog count increments/decrements when child dialogs open/close.
- [ ] Escape closes only the topmost nested dialog.
- [ ] Unmounting an open nested dialog decrements parent nested count.
- [x] Title and description can render children and register metadata without panics.
- [x] Close can be omitted without preventing Escape/outside close.
- [x] Popup can be omitted without panicking.
- [x] Viewport can be omitted without panicking.
- [x] Backdrop can be omitted without panicking.
- [ ] Initial focus default behavior is tested for the implemented GPUI focus policy.
- [ ] Initial focus suppression is tested if supported.
- [ ] Final focus returns to the trigger by default when supported.
- [ ] Final focus suppression is tested if supported.
- [ ] Modal/trap-focus Tab cycling is tested using Dialog-owned registered focus handles.
- [ ] Non-modal mode does not trap focus.
- [x] `style_with_state(...)` receives correct root state.
- [x] `style_with_state(...)` receives correct trigger state.
- [x] `style_with_state(...)` receives correct portal state.
- [x] `style_with_state(...)` receives correct backdrop state.
- [x] `style_with_state(...)` receives correct viewport state.
- [x] `style_with_state(...)` receives correct popup state.
- [x] `style_with_state(...)` receives correct title state.
- [x] `style_with_state(...)` receives correct description state.
- [x] `style_with_state(...)` receives correct close state.
- [ ] Transition/dimension tests are added if GPUI-native transition or measurement behavior is implemented in the first pass.

## AccessKit accessibility follow-up

Written against `docs/accesskit-gpui-reference.md` (gpui revision `1d029c5ff5654fb1b1e8caf4462993c8ee13a133`, accesskit `0.24.0`). Base UI's authoritative ARIA surface for Dialog is: trigger emits `aria-haspopup="dialog"`, `aria-expanded`, `aria-controls`, and `disabled` (`trigger/DialogTrigger.tsx`); popup emits `role="dialog"` (or `alertdialog`), `aria-labelledby` pointing at the title, and `aria-describedby` pointing at the description (`popup/DialogPopup.tsx`); title/description are plain heading/paragraph content; close is a `button`; backdrop is presentational; outside content is made inert while modal.

### Per accessible part

- `DialogTrigger<P>` (`layers/dialog_trigger.rs`): already has a stable scoped `.id(...)` and `.track_focus(...)`. Add `.role(Role::Button)` and `.aria_expanded(state.open && state.active_trigger)` from `DialogTriggerStyleState<P>` (this is Base UI's per-trigger `isOpenedByThisTrigger` semantics). Add `.aria_label(...)` from a new builder prop when the visible trigger content is not plain text.
- `DialogPopup<P>` (`layers/dialog_popup.rs`): already has a stable `.id(...)` and `.track_focus(&focus_handle...)`. Add `.role(Role::Dialog)` (a future AlertDialog port uses `Role::AlertDialog` on its own popup). Since `aria-labelledby`/`aria-describedby` id-references have no gpui builder, add an `.aria_label(...)` builder on `DialogPopup<P>` that takes the accessible dialog name directly; `DialogRuntime<P>` `title_ids`/`description_ids` metadata stays as-is for a future relationship API.
- `DialogClose<P>` (`layers/dialog_close.rs`): already has `.id(...)`, `.track_focus(...)`, and `.on_click(...)`. Add `.role(Role::Button)` and an `.aria_label(...)` builder (Base UI users typically render an icon-only close button, so a literal label like "Close" matters here).
- `DialogTitle<P>` (`layers/dialog_title.rs`): add `.role(Role::Heading)` plus `.aria_level(2)` (Base UI renders an `h2`). Its visible text children remain the accessible name of the heading node.
- `DialogDescription<P>` (`layers/dialog_description.rs`): no role — leave it out of the a11y tree (no `.role(...)` call) so its text children surface as plain accessible text inside the dialog subtree. `Role::GenericContainer` is filtered/asserted, so do not assign it.
- `DialogRoot<P>`, `DialogPortal<P>`, `DialogViewport<P>`, `DialogBackdrop<P>`: structural/presentational; assign no role so they do not appear in the a11y tree. The backdrop's dismissal behavior stays pointer-only, matching Base UI's presentational backdrop.

### Actions

- `Action::Focus` is auto-registered by the existing `.track_focus(...)` calls on `DialogTrigger<P>`, `DialogClose<P>`, and `DialogPopup<P>` — do not re-add it.
- `DialogTrigger<P>` opens via `.on_mouse_down(MouseButton::Left, ...)`, **not** `.on_click(...)`, so `Action::Click` is *not* auto-registered. Add `.on_a11y_action(AccessibleAction::Click, ...)` that routes into the same `context.open_trigger(scoped_id, DialogOpenChangeReason::TriggerPress, DialogOpenChangeSource::Unknown, window, cx)` transition the pointer path uses, guarded by the same `disabled` check.
- `DialogClose<P>` has `.on_click(...)`, which auto-registers `Action::Click`, but its handler early-returns unless `matches!(event, ClickEvent::Mouse(_))`, so a synthetic AT click would be dropped. Either relax that guard for non-mouse `ClickEvent`s or add an explicit `.on_a11y_action(AccessibleAction::Click, ...)` routing to `context.close(DialogOpenChangeReason::ClosePress, DialogOpenChangeSource::Unknown, window, cx)` with the `disabled` guard.
- No `Increment`/`Decrement`/`SetValue`/`Expand`/`Collapse` handlers are needed; Escape dismissal already flows through `DialogCloseAction` key dispatch on the popup's key context.

### Labels

- Dialog name: `.aria_label(...)` builder on `DialogPopup<P>`, supplied by the consumer with the same string as the visible title. When set, the demo/gallery title content should use `Text::new_inaccessible(...)` instead of `text!(...)` so the name is not announced twice (once from the popup label, once from the heading text). When the consumer relies on the `DialogTitle<P>` heading alone, skip the popup label and keep the title text accessible.
- Trigger/close labels: text children announced via the node's subtree when plain `text!(...)` is used; for icon-only triggers/close buttons, set `.aria_label(...)` and render the icon without accessible text.

### Gaps (no gpui builder in this revision)

- `aria-haspopup="dialog"` (trigger): no builder. Omit and document; `Role::Dialog` on the popup conveys the destination once open.
- `aria-controls` (trigger → popup id): no relationship builders. Omit; blocked pending gpui upstream id-reference support.
- `aria-labelledby` / `aria-describedby` (popup → title/description): no builders. Fallback is the literal `.aria_label(...)` string on `DialogPopup<P>` described above; `title_ids`/`description_ids` runtime metadata is retained for a future upstream relationship API.
- `disabled` / `aria-disabled` (trigger, close): no `.aria_disabled(...)` builder and `write_a11y_info` never sets a disabled flag. Fallback: keep suppressing the runtime transition in the existing `disabled` guards (so AT clicks are no-ops) and document that disabled state is not conveyed to AT; track as blocked pending a gpui `set_disabled` addition.
- Modal state / outside-content inertness (`aria-modal`, Base UI's `aria-hidden` on outside elements): no modal or inert/hidden node API. Fallback: document that `DialogModalMode::Modal` blocks pointer input and traps Tab focus but does not hide outside content from the AccessKit tree; blocked pending gpui upstream.
- Kept-mounted closed content (`DialogPortal::keep_mounted(true)`): no hidden/inert builder. Fallback: while `open == false`, render kept-mounted popup content without assigning `.role(...)` (a node with no role is not reported), so closed content stays out of the a11y tree without new APIs.
- Live regions / announcements: not needed for Dialog; no open-state announcement is emitted (matches Base UI, which relies on focus moving into the dialog).

### Checklist

- [ ] Add `.role(Role::Button)` and `.aria_expanded(open && active_trigger)` to `DialogTrigger<P>`, mapped from `DialogTriggerStyleState<P>`.
- [ ] Add `.on_a11y_action(AccessibleAction::Click, ...)` on `DialogTrigger<P>` routing through `context.open_trigger(...)` with the existing `disabled` guard (its open path is `on_mouse_down`, so Click is not auto-registered).
- [ ] Add `.role(Role::Dialog)` and an `.aria_label(...)` builder to `DialogPopup<P>`.
- [ ] Add `.role(Role::Button)` and an `.aria_label(...)` builder to `DialogClose<P>`, and make AT-dispatched clicks reach `context.close(DialogOpenChangeReason::ClosePress, ...)` despite the `ClickEvent::Mouse` guard.
- [ ] Add `.role(Role::Heading)` + `.aria_level(2)` to `DialogTitle<P>`; leave `DialogDescription<P>`, root, portal, viewport, and backdrop role-less.
- [ ] Do not re-register `Action::Click` on `DialogClose<P>` beyond the guard fix, nor `Action::Focus` anywhere — `.on_click`/`.track_focus` already provide them.
- [ ] Use `Text::new_inaccessible(...)` for visible title text in the gallery demo when the popup `.aria_label(...)` is set, to avoid double announcement.
- [ ] Keep kept-mounted closed popup content role-less so it is absent from the a11y tree without hidden/inert APIs.
- [ ] Document the gaps above (aria-haspopup, aria-controls, aria-labelledby/describedby, disabled, aria-modal/inert outside content) in module docs as blocked pending gpui upstream.
- [ ] Add accessibility tests for trigger role/expanded state, popup dialog role and label, close click via a11y action, and absence of role-less parts from the tree.
