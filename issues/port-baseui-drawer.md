# Port Base UI Drawer to GPUI

## Problem

Base UI's Drawer is a swipeable, snap-point-aware panel built **on top of Dialog by
structural reuse, not by a mode flag**. This makes it shaped differently from Alert
Dialog:

- `DrawerRoot` renders `Dialog.Root` beneath an `IsDrawerContext` marker
  (`drawer/root/DrawerRoot.tsx:232-250`), passing through `open` / `defaultOpen` /
  `onOpenChange` / `onOpenChangeComplete` / `disablePointerDismissal` / `modal` /
  `handle` / `triggerId` / `defaultTriggerId` unchanged. The only Dialog-core change
  is nested-count bookkeeping: `useDialogRoot({ ..., isDrawer })` counts drawer
  children separately (`nestedOpenDrawerCount` vs `nestedOpenDialogCount`) so a
  nested plain Dialog does not trigger drawer stacking visuals
  (`dialog/root/useRenderDialogRoot.tsx:71-76`).
- `Trigger`, `Portal`, `Title`, `Description`, `Close`, and `Handle` are **literal
  re-exports of the Dialog parts** (`drawer/trigger/DrawerTrigger.tsx:13`,
  `drawer/portal/DrawerPortal.tsx:13`, `drawer/index.parts.ts:15-18`, etc.). In GPUI
  these must be reused from the shipped `crates/base_gpui/src/dialog/` module, not
  reimplemented.
- The genuinely drawer-specific parts are: **Viewport** (the swipe-to-dismiss
  gesture engine: pointer tracking, direction lock, displacement/velocity,
  thresholds, release-velocity dismiss animation, snap-point selection on release),
  **Popup** (drag transform, self-height measurement, snap offset with sqrt
  overshoot damping, expanded state), **Backdrop** (swipe-progress-driven styling,
  nested suppression), **Content** (a marker that blocks swipe-start over
  interactive content), **SwipeArea** (swipe-to-open a closed drawer, registered as
  a trigger), and **Provider / Indent / IndentBackground** (app-shell iOS-style
  scale-back driven by any-drawer-open state and live swipe progress).

`base_gpui` has no `drawer` module. The goal is behavioral parity using GPUI-native
architecture: the gesture is built from GPUI `div()` mouse handlers (no new shared
primitive), snap points are pure math plus prepaint measurement, and the slide/
release animation uses GPUI's animation support. GPUI has no touch events, so Base
UI's touch pathway ('swipe' via `TouchEvent`) becomes mouse/trackpad drag; the
touch-scroll coordination machinery and the Android `CloseWatcher` are dropped.

Payloads stay generic via the existing Dialog type parameter `P: Clone + 'static`.

Complexity: **large**. Snap points, SwipeArea, and nested-drawer choreography are
the genuinely new work; everything open/close/focus/dismissal-shaped is inherited
from Dialog.

## Scope

Port the Drawer component family from Base UI into GPUI-native components.

Drawer-specific parts (new implementations under `crates/base_gpui/src/drawer/`):

- `DrawerRoot<P = ()>` — wraps/composes `DialogRoot<P>` with the drawer marker and
  drawer runtime; adds `swipe_direction`, snap-point props, and nested-drawer
  reporting.
- `DrawerViewport<P = ()>` — positioning container + the swipe-to-dismiss gesture
  engine (Base UI `DrawerViewport.tsx`, 1382 lines; most of it is touch-scroll
  coordination that is dropped — see Out of scope).
- `DrawerPopup<P = ()>` — drawer contents container: drag transform, self-height
  measurement, snap-point offset, expanded/swiping/nested style state,
  `initial_focus` / `final_focus` pass-through.
- `DrawerBackdrop<P = ()>` — overlay styled by swipe progress; suppressed when
  nested unless `force_render(true)`.
- `DrawerContent<P = ()>` — marker container: pointer-down inside it never starts a
  swipe (Base UI `DRAWER_CONTENT_ATTRIBUTE`).
- `DrawerSwipeArea<P = ()>` — invisible edge area that opens a closed drawer by
  dragging; registers as a trigger.
- `DrawerProvider` — app-level coordinator: any-drawer-open `active` flag plus live
  visual state (swipe progress, frontmost height).
- `DrawerIndent` / `DrawerIndentBackground` — app-shell wrapper styled from the
  provider state (iOS-style scale-back).

Reused Dialog parts, re-exported under Drawer names (no reimplementation — these
are literal re-exports in Base UI):

- `DrawerTrigger<P>` = `DialogTrigger<P>`
- `DrawerPortal<P>` = `DialogPortal<P>`
- `DrawerTitle<P>` = `DialogTitle<P>`
- `DrawerDescription<P>` = `DialogDescription<P>`
- `DrawerClose<P>` = `DialogClose<P>`
- `DrawerHandle<P>` = `DialogHandle<P>` and `create_drawer_handle` =
  `create_dialog_handle` (Base UI re-exports `Dialog.createHandle` directly,
  `drawer/index.parts.ts:15-18`; unlike AlertDialog there is no brand/type split —
  a Drawer intentionally shares the Dialog handle type).

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/drawer/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/root/DrawerRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/root/DrawerRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/root/useDrawerSnapPoints.ts`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/root/useDrawerSnapPoints.test.ts`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/root/DrawerRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/viewport/DrawerViewport.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/viewport/DrawerViewportContext.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/viewport/DrawerViewport.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/popup/DrawerPopup.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/popup/DrawerPopup.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/popup/DrawerPopupCssVars.ts`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/popup/DrawerPopupDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/backdrop/DrawerBackdrop.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/backdrop/DrawerBackdropCssVars.ts`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/content/DrawerContent.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/content/DrawerContent.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/swipe-area/DrawerSwipeArea.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/swipe-area/DrawerSwipeArea.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/provider/DrawerProvider.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/provider/DrawerProviderContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/indent/DrawerIndent.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/indent/DrawerIndent.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/indent-background/DrawerIndentBackground.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/indent-background/DrawerIndentBackground.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/drawer/trigger/DrawerTrigger.tsx` (re-export shape)
- `/home/luke/Projects/base-ui/packages/react/src/drawer/portal/DrawerPortal.tsx` (re-export shape)
- `/home/luke/Projects/base-ui/packages/react/src/drawer/title/DrawerTitle.tsx`,
  `description/DrawerDescription.tsx`, `close/DrawerClose.tsx` (re-export shapes)
- `/home/luke/Projects/base-ui/packages/react/src/utils/useSwipeDismiss.ts` (the
  shared gesture engine; behavioral reference for thresholds/velocity/damping)
- `/home/luke/Projects/base-ui/packages/react/src/dialog/root/useRenderDialogRoot.tsx`
  (the `isDrawer` / `mode='drawer'` branch)
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/drawer/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/drawer/types.md`

Existing GPUI implementation to reuse (do not fork/duplicate):

- `crates/base_gpui/src/dialog/` (entire module): `DialogRuntime<P>` open/close
  outcomes and nested counts, `DialogContext<P>` / `DialogHandle<P>` /
  `create_dialog_handle`, focus trap, Escape close, outside-press dismissal,
  `transitioning` style state, `DialogOpenChangeDetails<P>` with
  `prevent_unmount_on_close`.
- GPUI gesture precedents:
  `crates/base_gpui/src/number_field/layers/number_field_scrub_area.rs`
  (mouse-down/move/up drag tracking on a `div()`),
  `/home/luke/Projects/gpui-component/crates/ui/src/slider.rs`
  (`on_drag_move` / `on_mouse_up_out` drag pattern),
  `/home/luke/Projects/gpui-component/crates/ui/src/sheet.rs` (side `Placement`
  enum + slide-in animation via gpui `animation.rs`).

Expected new GPUI files (flat layout, per `docs/base-gpui-component-architecture.md`):

- `crates/base_gpui/src/drawer/mod.rs` — barrel exports only, including the
  re-exported Dialog parts under Drawer names.
- `crates/base_gpui/src/drawer/runtime.rs` — `DrawerRuntime<P>`: swipe gesture
  state machine, snap-point resolution/selection, measured heights, nested-drawer
  reporting state, release-animation parameters.
- `crates/base_gpui/src/drawer/context.rs` — `DrawerContext<P>` wrapping the drawer
  runtime entity plus the inner `DialogContext<P>`; snap-point value-change method.
- `crates/base_gpui/src/drawer/props.rs` — drawer root props/callbacks
  (`swipe_direction`, snap-point config, `on_snap_point_change`).
- `crates/base_gpui/src/drawer/style_state.rs` — drawer-specific style-state
  structs (popup, backdrop, viewport, swipe area, content, indent,
  indent-background) and `DrawerSwipeDirection` / `DrawerSnapPoint` types.
- `crates/base_gpui/src/drawer/child.rs` / `child_wiring.rs` — typed drawer child
  routing and private traversal/context attachment.
- `crates/base_gpui/src/drawer/provider.rs` (or `layers/drawer_provider.rs` if it
  renders) — provider entity for app-shell coordination.
- `crates/base_gpui/src/drawer/layers/drawer_root.rs`
- `crates/base_gpui/src/drawer/layers/drawer_viewport.rs`
- `crates/base_gpui/src/drawer/layers/drawer_popup.rs`
- `crates/base_gpui/src/drawer/layers/drawer_backdrop.rs`
- `crates/base_gpui/src/drawer/layers/drawer_content.rs`
- `crates/base_gpui/src/drawer/layers/drawer_swipe_area.rs`
- `crates/base_gpui/src/drawer/layers/drawer_indent.rs`
- `crates/base_gpui/src/drawer/layers/drawer_indent_background.rs`
- `crates/base_gpui/src/drawer/layers/mod.rs`
- `crates/base_gpui/src/drawer/tests/`
- Register `pub mod drawer;` (and `drawer::init(cx);` only if drawer adds actions —
  Escape and focus actions are inherited from Dialog) in
  `crates/base_gpui/src/lib.rs`.

No `crates/base_gpui/src/drawer/` module exists yet.

## Dependency on open Dialog acceptance items

Drawer inherits behavior from `issues/port-baseui-dialog.md` items that are still
unchecked. Track them there; Drawer items below that depend on them say so:

- Nested dialog behavior (entire section unchecked): nested marking, parent nested
  counts, nested backdrop suppression, Escape-closes-topmost, nested unmount count
  bookkeeping. Drawer's nested-drawer choreography sits on top of this.
- Typed transition status (`DialogTransitionStatus::{Starting, Ending, Idle}` or
  equivalent) and transition sequencing inside the runtime. Drawer's swipe-release
  and slide animations need a real ending phase to look right.
- `.initial_focus(...)` / `.final_focus(...)` on `DialogPopup<P>`. `DrawerPopup`
  forwards these once Dialog implements them.
- Kept-mounted closed content being hidden/inert via GPUI-native mechanisms.

## Out of scope / drop from Base UI

- React context/hooks/store implementation details (`DrawerRootContext`,
  `DrawerViewportContext`, `useSyncExternalStore`-style stores, `useStableCallback`,
  flushSync). GPUI uses keyed entity state and runtime commands.
- React `render` props, `className`, web `style` props, `nativeButton`.
- SSR / hydration / CSP nonce; `CSS.registerProperty` performance registration
  (`DrawerPopup.tsx:38-89`) — a DOM-only optimization with no GPUI analog.
- CSS variables (`--drawer-swipe-movement-x/y`, `--drawer-snap-point-offset`,
  `--drawer-height`, `--drawer-frontmost-height`, `--drawer-swipe-strength`,
  `--drawer-swipe-progress`, `--nested-drawers`): expose typed style-state fields
  instead.
- DOM data attributes (`data-open`, `data-closed`, `data-starting-style`,
  `data-ending-style`, `data-expanded`, `data-swiping`, `data-swipe-dismiss`,
  `data-swipe-direction`, `data-nested-drawer-open`, `data-nested-drawer-swiping`,
  `data-active`/`data-inactive` on Indent): typed style-state fields instead.
- **All touch-event machinery**. GPUI has no touch events; 'swipe' becomes
  mouse/trackpad drag. Concretely dropped from `DrawerViewport.tsx` /
  `useSwipeDismiss.ts`: `TouchEvent` handlers, the capture-phase native `touchmove`
  listener and React-bypass `moveNative` pipeline, touch-scroll coordination
  (`TouchScrollState`, scroll-edge swipe-start rules, cross-axis native scroll
  preservation, `findScrollableTouchTarget` / scrollable-ancestor suppression),
  pinch-zoom and range-input guards, pen/touch pointer-type juggling,
  `touch-action` styles, and DOM text-selection guards
  (`shouldIgnoreSwipeForTextSelection`, selection clearing on swipe start).
- Android `CloseWatcher` back-gesture integration (`DrawerRoot.tsx`
  `DrawerProviderReporter` effect).
- DOM pointer capture (`setPointerCapture`); use GPUI drag/mouse-out-of-bounds
  handling (`on_drag_move`, `on_mouse_up_out`) instead.
- DOM measurement (`offsetHeight`, `getComputedStyle` transform parsing,
  `ResizeObserver`, root font-size probing): use GPUI prepaint measurement
  (`Div::on_children_prepainted(...)`) and `Rems`/`Pixels` types.
- DOM portal `container` targets.
- `BASE_UI_SWIPE_IGNORE_SELECTOR` CSS-selector matching: replace with typed
  knowledge — interactive drawer children (triggers, closes, inputs) and
  `DrawerContent` subtrees are known through child wiring/metadata, not selector
  queries.
- ARIA attributes (`role="presentation"`, `aria-hidden`) — AccessKit follow-up
  only, consistent with Dialog.
- **Do not extract a shared swipe primitive.** Base UI's `useSwipeDismiss` is also
  used by Toast; flag this as a possible future extraction once a GPUI Toast port
  exists, but keep the gesture engine drawer-internal now (arch doc: avoid shared
  generic primitives until the repeated deep concept actually repeats).

## Acceptance Criteria

New issue — all items unchecked.  Items marked **(inherited)** are Dialog behavior
that must keep working through Drawer reuse, not be reimplemented.

### Module/API surface

- [x] Add a `drawer` module and export it from `crates/base_gpui/src/lib.rs`.
- [ ] Re-export the reused Dialog parts under Drawer names from `drawer/mod.rs`:
      `DrawerTrigger<P>`, `DrawerPortal<P>`, `DrawerTitle<P>`,
      `DrawerDescription<P>`, `DrawerClose<P>`, `DrawerHandle<P>`,
      `create_drawer_handle<P>()` — as type aliases / re-exports of the `dialog`
      module items, with no forked implementations.
      *Partially done: Trigger/Title/Description/Close/Handle/create_drawer_handle are
      literal re-exports; `DrawerPortal` is a drawer-typed thin copy because GPUI's
      typed child enums cannot route drawer-specific Backdrop/Viewport through
      `DialogPortal`.*
- [x] Add drawer-specific public layer types `DrawerRoot<P = ()>`,
      `DrawerViewport<P = ()>`, `DrawerPopup<P = ()>`, `DrawerBackdrop<P = ()>`,
      `DrawerContent<P = ()>`, `DrawerSwipeArea<P = ()>`, `DrawerProvider`,
      `DrawerIndent`, `DrawerIndentBackground`.
- [x] Constrain payloads as `P: Clone + 'static`, consistent with Dialog.
- [x] `DrawerRoot<P>` forwards the Dialog root surface unchanged: `id`,
      `default_open` (default `false`), `open`, `on_open_change`,
      `on_open_change_complete`, `disable_pointer_dismissal` (default `false` —
      unlike AlertDialog, Drawer does not force it), `modal` (default modal),
      `trigger_id`, `default_trigger_id`, `handle`, children, `style_with_state`.
- [x] Add `DrawerSwipeDirection::{Up, Down, Left, Right}` and
      `DrawerRoot::swipe_direction(...)`, defaulting to `Down`.
- [x] Add `DrawerSnapPoint` as a typed Rust value instead of Base UI's
      `number | string` union, e.g. `DrawerSnapPoint::{Fraction(f32), Px(Pixels),
      Rems(Rems)}` where `Fraction` covers Base UI numbers `<= 1` (fraction of
      viewport height) and `Px` covers numbers `> 1` / `'…px'` strings.
- [x] Support `DrawerRoot::snap_points(Vec<DrawerSnapPoint>)`.
- [x] Support `DrawerRoot::snap_to_sequential_points(bool)`, default `false`.
- [x] Support uncontrolled snap point via
      `DrawerRoot::default_snap_point(Option<DrawerSnapPoint>)`; when unset, the
      default is the first entry of `snap_points`, or none when there are no snap
      points.
- [x] Support controlled snap point via
      `DrawerRoot::snap_point(Option<DrawerSnapPoint>)`; calling the builder marks
      the snap point controlled even when `None`.
- [x] Support `DrawerRoot::on_snap_point_change(...)` with a Rust-native cancelable
      details API reusing the Dialog details shape, e.g.
      `Fn(Option<DrawerSnapPoint>, &mut DrawerSnapPointChangeDetails, &mut Window,
      &mut App)`.
- [x] Extend the shared open-change reason vocabulary with a swipe reason (Base UI
      `REASONS.swipe`): add `DialogOpenChangeReason::Swipe` (or an equivalent
      drawer-visible reason) so swipe-dismiss/swipe-open report a distinct reason
      through the reused `on_open_change`. Do not add a `CloseWatcher` reason.
- [ ] `DrawerPopup<P>` supports `initial_focus(...)` / `final_focus(...)`
      pass-through to the Dialog popup focus policy once Dialog implements it
      (dependency noted above).
- [x] `DrawerBackdrop<P>` supports `.force_render(bool)`, default `false`
      (renders a nested drawer's backdrop anyway).
- [x] `DrawerSwipeArea<P>` supports `.disabled(bool)` (default `false`),
      `.swipe_direction(DrawerSwipeDirection)` (default: opposite of the root
      `swipe_direction`), and `.id(...)` for trigger association.
- [x] `DrawerContent<P>` takes arbitrary GPUI children and exposes no behavior
      props; its job is the swipe-exclusion marker.
- [x] `DrawerProvider` takes children and no other required props;
      `DrawerIndent` / `DrawerIndentBackground` take children plus
      `style_with_state`.
- [x] `drawer/mod.rs` remains barrel-only and exposes ergonomic exports for
      components, style states, context, props, runtime, snap-point types, and
      child types.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui drawer` passes.
- [x] `cargo test -p base_gpui` passes (Dialog suite unaffected).
- [x] `ast-grep scan crates/base_gpui/src` passes.
- [x] No `pub(...)` restricted visibility; module privacy via `mod` boundaries.
- [x] The implementation follows `docs/base-gpui-component-architecture.md` flat
      layout (`runtime.rs` / `context.rs` / `props.rs` / `style_state.rs` /
      `child.rs` / `child_wiring.rs` / `layers/`); no `utils/` folder for Drawer.
- [x] Add a gallery demo in `crates/base_gpui/src/main.rs` with a basic
      bottom drawer (Trigger + Portal + Backdrop + Viewport + Popup + Content +
      Title + Close) that can be drag-dismissed.
- [x] Add a compact gallery example for either snap points or SwipeArea +
      Provider/Indent.

### Architecture / internal primitives

- [x] `DrawerRoot<P>` composes the existing Dialog root machinery: it creates the
      shared `DialogContext<P>` (or builds a `DialogRoot<P>` internally) with an
      `is_drawer` marker, mirroring Base UI's `IsDrawerContext` +
      `useRenderDialogRoot(props, 'drawer')`. No duplication of open/close,
      trigger, focus, or dismissal logic.
- [ ] The only Dialog-core change is nested bookkeeping: `DialogRuntime<P>` (or its
      registration path) distinguishes drawer children so parents track a
      `nested_open_drawer_count` separate from the nested dialog count, mirroring
      `nestedOpenDrawerCount`. A nested plain Dialog inside a Drawer must not
      trigger drawer-stack visuals; a nested Drawer must.
      *Skipped: Dialog's own nested-dialog machinery is still unimplemented (tracked in
      the Dialog issue); drawer nesting is tracked drawer-side in `DrawerRuntime`.*
- [x] Add `DrawerRuntime<P>` as the single owner of drawer-specific state: swipe
      gesture state (pending/active, start position, locked axis, intended
      direction, drag offset, velocity samples, reverse-cancel baseline), resolved
      snap points, active snap point (uncontrolled value + controlled observed
      value), measured popup height and viewport height, frontmost height, nested
      drawer presence/swiping/progress, swipe-release strength, and pending
      swipe-close snap point for cancel/revert.
- [x] Gesture transitions (direction lock, threshold crossing, cancel-by-reversal,
      release decision, snap selection) are computed inside `DrawerRuntime<P>` as
      commands returning outcomes; layers translate mouse events into commands and
      query style state only.
- [x] Add `DrawerProps<P>` for `swipe_direction`, snap-point config, and the
      snap-point callback. Dialog-level props stay on `DialogProps<P>`.
- [x] Add `DrawerContext<P>` as thin injection: the drawer runtime entity, the
      inner `DialogContext<P>`, drawer props, and the controlled snap-point marker,
      with only `read(...)`, `update(...)`, and `set_snap_point(...)` (the one
      value-changing method — resolves controlled/uncontrolled and fires
      `on_snap_point_change` from the runtime outcome). Open/close continues to go
      through `DialogContext<P>` methods.
- [x] Nested drawer coordination is parent-linked through context: a nested
      `DrawerRoot` receives its parent `DrawerContext` (as Base UI's
      `notifyParent*` callbacks) and reports presence, frontmost height, swiping,
      and swipe progress via runtime commands, propagating up the chain.
- [x] `DrawerProvider` owns a keyed entity with the open-drawer id set and the
      visual state (swipe progress, frontmost height); drawers report into it when
      a provider ancestor exists; all drawer parts work without a provider.
- [x] Typed child routing in `drawer/child.rs` covers the drawer anatomy shown in
      Base UI docs (Root > Trigger/SwipeArea/Portal; Portal > Backdrop/Viewport;
      Viewport > Popup; Popup > Content/Title/Description/Close); private
      traversal/context attachment in `drawer/child_wiring.rs`. Reused Dialog parts
      appear in drawer child enums as the Dialog layer types.
- [x] The gesture engine lives inside the drawer module (runtime + viewport/swipe
      area layers). Do not extract a shared swipe primitive now; leave a comment
      noting Base UI shares `useSwipeDismiss` with Toast as a possible future
      extraction point.
- [x] The swipe gesture uses GPUI mouse handling (`on_mouse_down` /
      `on_mouse_move` / `on_drag_move` / `on_mouse_up` / `on_mouse_up_out`)
      following the `number_field` scrub-area and `gpui-component` slider
      precedents; no simulated touch layer.
- [x] Popup and viewport measurement uses GPUI prepaint mechanisms
      (`Div::on_children_prepainted(...)`), feeding `set_*_bounds`-style runtime
      commands that report whether anything changed.

### Stateful/stateless behavior (inherited via Dialog — verify through Drawer)

- [x] Uncontrolled Drawer initializes from `default_open`, defaulting to closed. **(inherited)**
- [x] Controlled Drawer reflects external `open` and fires `on_open_change`
      without self-mutating. **(inherited)**
- [x] Trigger press, Close press, Escape, outside press, and handle open/close all
      work through the reused Dialog parts with their existing reasons. **(inherited)**
- [x] `disable_pointer_dismissal(true)` prevents outside-press close but not swipe
      dismissal or Escape. Swipe-to-dismiss is a drawer gesture, not a pointer
      "outside" dismissal.
- [x] Focus trap in modal / trap-focus modes works inside the drawer popup. **(inherited)**
- [x] Detached triggers via `DrawerHandle<P>` (= `DialogHandle<P>`) open/close the
      drawer; a plain Dialog handle is intentionally interchangeable. **(inherited)**
- [x] Closing a drawer with snap points resets the active snap point to the
      resolved default after the close is accepted (`DrawerRoot.tsx:151-170`);
      canceling the close leaves the snap point unchanged.

### Snap point state behavior

- [x] Uncontrolled snap point initializes from `default_snap_point`, else the first
      snap point, else none.
- [x] Controlled snap point reflects the external value; interactions fire
      `on_snap_point_change` without mutating internal state as source of truth.
- [x] Canceling `on_snap_point_change` (uncontrolled) prevents the internal
      snap-point mutation.
- [x] When uncontrolled and the active snap point is no longer present in
      `snap_points` (or is none while snap points exist), the resolved active snap
      point falls back to the resolved default (`DrawerRoot.tsx:97-114`).
- [x] Snap-point resolution: `Fraction(f)` resolves to `clamp(f, 0, 1) *
      viewport_height`; `Px`/`Rems` resolve via GPUI pixel/rem conversion; results
      clamp to `min(popup_height, viewport_height)`; non-finite/unresolvable values
      are skipped (`useDrawerSnapPoints.ts:30-64,124-170`).
- [x] Each resolved snap point carries `offset = max(0, popup_height - height)`.
- [x] Snap points whose resolved heights are within 1px of a later snap point are
      deduped, keeping the later entry (`useDrawerSnapPoints.ts:150-169`).
- [x] A controlled snap-point value that is not an exact member of `snap_points`
      resolves to the closest resolved snap point by height
      (`useDrawerSnapPoints.ts:172-194`).
- [x] Snap-point offsets only apply for vertical swipe directions (`Down`/`Up`);
      horizontal drawers ignore snap points, matching Base UI.
- [ ] Viewport height comes from the drawer viewport's measured bounds when
      present, else the window content height; re-measures on resize through
      prepaint, not `ResizeObserver`.
      *Partially done: window content height only; per-viewport bounds measurement not
      yet wired.*

### Swipe-to-dismiss gesture (Viewport)

- [x] The gesture engine is enabled while the drawer is mounted and no nested
      drawer is open.
- [x] Only the primary mouse button starts a swipe; a non-primary button appearing
      mid-drag (or losing the primary button without a release event) cancels the
      swipe and restores position (`useSwipeDismiss.ts:772-792`).
- [x] A press over an interactive element (trigger/close/input-like children) or
      over a `DrawerContent` subtree does not start a swipe; this uses typed child
      knowledge, not CSS selectors (`DrawerViewport.tsx:1008-1022`,
      `DRAWER_CONTENT_SELECTOR`).
- [x] Allowed swipe directions: `[swipe_direction]`, extended to both vertical
      directions when snap points exist and the direction is vertical
      (`DrawerViewport.tsx:161-171`).
- [x] The intended swipe direction locks from initial movement (axis lock after >=
      1px movement when both axes are allowed; dominant-axis pick otherwise), and
      the swipe threshold resolves per direction as `max(size * 0.5, 10.0)` where
      size is popup width/height along the axis (`getBaseSwipeThreshold`).
- [x] Movement along disallowed directions is damped with the signed square-root
      curve (`applyDirectionalDamping`), so dragging a bottom drawer upward resists
      past its rest position.
- [x] With snap points and `swipe_direction = Down`, the popup translation applies
      the snap-point base offset plus movement with sqrt overshoot damping past the
      fully-open edge (`getSnapPointSwipeMovement`: overshoot beyond offset 0 maps
      to `-sqrt(-next_offset) - base_offset`).
- [x] Reverse-cancel: after the drag exceeds the threshold, moving back so
      displacement drops 10px below its maximum marks the swipe canceled; releasing
      then restores position without dismissing (`REVERSE_CANCEL_THRESHOLD`).
- [x] Velocity tracking: overall velocity = delta / max(duration, 50ms); release
      velocity from the last drag sample when it is at most 80ms old, with a 16ms
      minimum sample duration (`useSwipeDismiss.ts:281-297,925-953`).
- [x] Release decision without snap points: dismiss when directional displacement
      exceeds the threshold, or when directional velocity >= 0.5 px/ms with
      positive displacement; otherwise restore (`DrawerViewport.tsx:545-585`).
- [x] Dismiss requests close through the Dialog context with the `Swipe` reason;
      the request is cancelable through `on_open_change`; canceling restores the
      drag position and any pending snap point (`DrawerViewport.tsx:736-761`).
- [x] Controlled-mode dismissal is optimistic: the dismiss animation starts, and if
      the external `open` remains true after the handler runs (parent rejected),
      the position and snap point revert (`DrawerViewport.tsx:763-795`); document
      that async external closes fall back to a non-animated close, as Base UI
      does.
- [x] Swipe-release strength: on dismiss, compute the remaining travel distance
      along the dismiss direction, clamp release velocity to 0.2..4 px/ms, map
      remaining/velocity to a duration clamped to 80..360ms, and normalize to a
      0.1..1 scalar exposed as typed style state (`resolveSwipeRelease`); the close
      animation duration scales by this scalar instead of a `--drawer-swipe-strength`
      CSS var.
- [ ] While swiping, the popup's transform is frozen to the live drag offset (no
      transition); on release without dismissal the popup animates back to rest.
      *Partially done: live drag offset exposed/frozen; return-to-rest is instant
      (animation depends on the open Dialog transition items).*
- [x] Swiping state and live swipe progress (0..1 of popup size, or snap-range
      progress when snap points exist) are runtime state exposed through popup,
      backdrop, and indent style states, and reported to the parent drawer and
      provider (`applySwipeProgress`).
- [x] Opening the drawer resets any in-flight gesture and release state
      (`DrawerViewport.tsx:965-970`).

### Snap-point selection on release

- [x] Drag target offset = `clamp(current_offset + drag_delta, 0, popup_height)`.
- [x] Velocity skipping (default): when |release velocity| >= 0.5 px/ms, add
      `clamp(velocity, -4, 4) * 300` px to the target offset before selecting
      (`SNAP_VELOCITY_*`, `DrawerViewport.tsx:624-632`).
- [x] A release-velocity direction that contradicts the drag direction (touch/drag
      reversal) falls back to the overall gesture velocity
      (`DrawerViewport.tsx:610-620`).
- [x] Fast dismissal: directional velocity >= 0.5 px/ms while dragging toward
      dismissal closes from snap points directly (`DrawerViewport.tsx:711-713`).
- [x] Default selection: the resolved snap point whose offset is closest to the
      target offset wins; when the close offset (`popup_height`) is strictly
      closer than any snap point, the drawer closes instead
      (`DrawerViewport.tsx:715-734`).
- [x] `snap_to_sequential_points(true)` disables velocity skipping: selection uses
      drag distance only, advancing at most to the adjacent snap point in the drag
      direction when velocity confirms the direction; advancing past the last
      point toward dismissal closes (`DrawerViewport.tsx:641-709`).
- [x] Closing from snap points stores the pre-close snap point so a canceled or
      rejected close can restore it; an accepted close leaves the snap point reset
      to default per root behavior.
- [x] Snap-point changes from gestures fire `on_snap_point_change` with the swipe
      reason and are cancelable.

### Swipe-to-open (SwipeArea)

- [x] `DrawerSwipeArea<P>` is enabled when the drawer is closed (or while its own
      gesture is active) and not disabled.
- [x] The open direction defaults to the opposite of the root `swipe_direction`
      and can be overridden per swipe area.
- [x] Dragging in the open direction opens the drawer optimistically once
      displacement >= 1px, with the `Swipe` reason and the swipe area's id as the
      associated trigger id (`DrawerSwipeArea.tsx:337-345`, trigger registration).
- [x] While the gesture is active, the popup position maps displacement to the
      remaining closed offset (`closed_offset - displacement`), with sqrt damping
      past the fully-open position, and backdrop progress maps inversely
      (`applySwipeMovement`); popup transitions are suspended during the drag.
- [x] Release opens (keeps open) when displacement >= 50% of the popup size along
      the axis (fallback threshold 40px when unmeasured) or release velocity >=
      0.1 px/ms; otherwise, if the drawer was opened by this gesture, it closes
      again with the `Swipe` reason (`DrawerSwipeArea.tsx:347-374`).
- [x] Outside-press dismissal is suppressed for the duration of the gesture and
      re-enabled after release (deferred re-enable), so the opening drag cannot
      immediately dismiss the drawer it opened (`disableDismissForSwipe` /
      `enableDismissAfterRelease`).
- [ ] The closed offset accounts for the popup's current animated transform when
      the gesture starts mid-transition (`resolveClosedOffset`).
      *Skipped: no open/close transition animation yet, so there is no mid-transition
      transform to resolve.*
- [ ] Disabling or unmounting a mid-gesture swipe area resets gesture state,
      clears applied movement, and re-enables dismissal.
      *Partially done: disabling cancels the gesture; unmount reset relies on the next
      open reset.*
- [x] The swipe area registers as a drawer trigger so `trigger_id`-based styling
      and focus-return treat swipe-opens like trigger opens.
- [x] Swipe area style state exposes `open`, `swiping`, `swipe_direction`, and
      `disabled`.

### Nested-drawer coordination

- [x] A drawer rendered inside another drawer is marked nested; parents track
      `nested_open_drawer_count` separately from nested dialogs (depends on the
      open Dialog nested items).
- [x] A nested drawer reports presence to its parent while open **or transitioning
      out** (`DrawerPopup.tsx:285-296`); unmounting reports absence.
- [x] A nested drawer reports its frontmost height up the chain; the parent's
      `frontmost_height` reflects the deepest open drawer's popup height and falls
      back to the parent's own popup height when the nested drawer leaves
      (`DrawerRoot.tsx:116-135`).
- [x] Nested swipe progress propagates to all ancestors; a parent popup exposes the
      nested drawer's live swipe progress in its style state so the parent can
      scale back proportionally (`nestedSwipeProgressStore`,
      `DrawerPopup.tsx:244-271`).
- [x] Nested swiping starts reporting only after 10px of directional movement and
      always reports false on gesture end/unmount (`updateNestedSwipeActive`,
      `finishNestedSwipe`).
- [x] The parent popup holds its measured height while a nested drawer is present,
      instead of re-measuring a stretched layout (`DrawerPopup.tsx:181-214`).
- [x] A parent drawer's own swipe gesture is disabled while a nested drawer is
      open.
- [x] Nested drawer backdrops are suppressed by default; `force_render(true)`
      renders them (`DrawerBackdrop.tsx:60`).
- [ ] Escape closes only the topmost drawer. **(inherited — depends on the open
      Dialog nested items)**

### Provider / Indent app-shell behavior

- [x] `DrawerProvider` tracks per-drawer open state by id; `active` is true when
      any registered drawer is open; drawers deregister on unmount
      (`DrawerProvider.tsx`).
- [x] Drawers report open state to the nearest provider automatically (Base UI's
      `DrawerProviderReporter`); no user wiring required beyond mounting the
      provider.
- [x] The provider's visual state carries live `swipe_progress` and
      `frontmost_height` from the frontmost non-nested drawer's gesture, and is
      reset to zero on gesture end, dismissal, and drawer unmount.
- [x] `DrawerIndent` style state exposes `active`, `swipe_progress`, and
      `frontmost_height` (typed, replacing the `--drawer-swipe-progress` /
      `--drawer-height` vars written onto the indent element).
- [x] `DrawerIndentBackground` style state exposes `active`.
- [ ] Indent/IndentBackground render arbitrary children and are visually neutral by
      default; the iOS-style scale-back is achieved by the user through
      `style_with_state` (documented in the gallery example).
      *Partially done: parts are neutral and user-styled, but the gallery example covers
      snap points instead of Provider/Indent.*
- [x] All drawer parts function without a provider; Indent/IndentBackground without
      a provider are inactive, not panicking.

### Animation / transition behavior

- [ ] Open/close slide animation along `swipe_direction` uses GPUI animation
      (`gpui-component` `sheet.rs` slide precedent), driven from runtime
      presence/transition state; layers only query style state.
      *Skipped: depends on the unchecked Dialog typed-transition items; style state
      exposes everything needed once transitions land.*
- [ ] Swipe-release close animation duration scales by the release-strength scalar
      (0.1..1) computed at release.
- [x] After a swipe dismissal, popup and backdrop expose a `swipe_dismissed` style
      flag during the ending phase (replacing `data-swipe-dismiss`), cleared when
      the drawer reopens.
- [ ] Snap-point changes animate the popup between snap offsets when not swiping;
      the live drag offset takes precedence while swiping.
- [ ] `on_open_change_complete` fires after the close/open animation completes.
      **(inherited — depends on the open Dialog transition items)**
- [x] Transition sequencing lives inside runtimes; no shadow previous-value fields
      in layers.

### Styling/state exposure

- [x] Add `DrawerPopupStyleState<P>` with at least: `open`, `closed`, `mounted`,
      `transitioning`, `expanded` (active snap point is the full `Fraction(1.0)`
      point), `nested`, `nested_drawer_open`, `nested_drawer_count`,
      `nested_drawer_swiping`, `nested_swipe_progress`, `swipe_direction`,
      `swiping`, `swipe_movement` (typed x/y pixels), `snap_point_offset`
      (signed pixels, negated for `Up`), `popup_height`, `frontmost_height`,
      `swipe_strength`, `swipe_dismissed`, active trigger id, and active payload.
- [x] Add `DrawerBackdropStyleState` with at least: `open`, `closed`, `mounted`,
      `transitioning`, `swipe_progress`, `frontmost_height`, `swiping`,
      `swipe_dismissed`, `nested`, and force-/effective-rendered info.
- [x] Add `DrawerViewportStyleState<P>` extending the Dialog viewport fields with
      drawer nested/swiping facts; drawer nested state replaces the generic
      dialog nested flag on this part (Base UI suppresses
      `data-nested-dialog-open` on the drawer viewport).
- [x] Add `DrawerSwipeAreaStyleState`, `DrawerContentStyleState` (typed, may be
      empty), `DrawerIndentStyleState`, and `DrawerIndentBackgroundStyleState`.
- [x] Expose `style_with_state(...)` on every drawer-specific part that draws.
- [x] Re-exported Dialog parts keep their existing `Dialog*StyleState` structs
      (reused, not duplicated).
- [x] Map every Base UI drawer CSS var and data attribute listed in Out of scope to
      a typed style-state field; expose none of them as string attributes or vars.
- [x] The docs hero pattern is recreatable with GPUI builders: backdrop opacity
      from `swipe_progress`, popup translate from `swipe_movement` +
      `snap_point_offset`, indent scale-back from provider progress, and
      release-speed-scaled closing.

### Tests / verification

Add behavior-level tests under `crates/base_gpui/src/drawer/tests/`. Runtime
gesture/snap logic must be unit-testable without a window (plain `&mut self`
commands with synthetic positions/timestamps).

Inherited-path smoke tests (through Drawer composition):

- [ ] Uncontrolled `default_open` initial state; controlled `open(...)` does not
      self-mutate.
- [ ] Trigger opens; Close closes; Escape closes; outside press closes when
      pointer dismissal is enabled and not when disabled.
- [ ] Handle-driven open/close and detached trigger work through
      `DrawerHandle` re-export.
- [x] Re-exported parts (`DrawerTrigger`, `DrawerPortal`, `DrawerTitle`,
      `DrawerDescription`, `DrawerClose`) are the Dialog types (compile-level
      alias check is enough).

Snap-point resolution (unit tests mirroring `useDrawerSnapPoints.test.ts`):

- [x] Fractions resolve against viewport height and clamp to 0..1.
- [x] Pixel and rem values resolve and clamp to `min(popup, viewport)`.
- [x] Unresolvable values are skipped; near-duplicate heights (<= 1px) dedupe
      keeping the later entry.
- [x] Default snap point falls back to the first entry; missing/unknown active
      values resolve to default (uncontrolled) or closest point (controlled).
- [x] `get_snap_point_swipe_movement` sqrt overshoot damping matches Base UI for
      in-range and overshoot inputs.

Swipe-to-dismiss:

- [x] Drag past `max(size * 0.5, 10px)` and release dismisses with the `Swipe`
      reason.
- [x] Fast flick (velocity >= 0.5 px/ms) dismisses below the distance threshold.
- [x] Drag below threshold releases back to rest without closing.
- [x] Reversal of 10px+ from max displacement cancels the dismissal on release.
- [x] Drag in a disallowed direction produces sqrt-damped movement and no
      dismissal.
- [x] Canceling the swipe close through `on_open_change` restores position and
      pending snap point.
- [x] Controlled root that rejects the close reverts the dismiss animation.
- [x] Release-strength scalar maps distance/velocity to the 0.1..1 range at the
      clamp boundaries.
- [x] Press on `DrawerContent` or an interactive child does not start a swipe.
- [x] Non-primary button and mid-drag button loss cancel the gesture.
- [x] Gesture is inert while a nested drawer is open.

Snap selection on release:

- [x] Closest snap point wins by drag distance.
- [x] Velocity offset skips to a further snap point by default.
- [x] Velocity direction contradicting drag direction falls back to gesture
      velocity.
- [x] `snap_to_sequential_points(true)` advances at most one adjacent point.
- [x] Target closer to the close offset than any snap point closes the drawer.
- [ ] Gesture snap changes fire cancelable `on_snap_point_change`; canceling keeps
      the previous snap point.
- [x] Accepted close resets the snap point to default; canceled close restores the
      pre-close snap point.
- [x] Controlled snap point does not self-mutate on gesture release.

Swipe-to-open:

- [x] Drag on the swipe area in the open direction opens the drawer with the
      `Swipe` reason and the swipe area as trigger.
- [x] Release below distance and velocity thresholds closes the
      gesture-opened drawer again.
- [x] Release past 50% popup size (or velocity >= 0.1 px/ms) keeps it open.
- [x] Outside-press dismissal stays disabled during the gesture and re-enables
      after release.
- [ ] `disabled(true)` swipe area does nothing and exposes disabled style state.

Nested drawers / provider:

- [x] Nested drawer marks the parent's `nested_drawer_open` / count, including
      while transitioning out; unmount decrements.
- [x] Parent `frontmost_height` follows the nested drawer's height and falls back
      to its own when the nested drawer leaves.
- [x] Nested swipe progress and swiping flags reach the parent popup style state
      and reset on gesture end/unmount.
- [ ] Nested backdrop is suppressed by default; `force_render(true)` renders it.
- [ ] Provider `active` flips with any registered drawer's open state and cleans
      up on drawer unmount.
- [ ] Indent/IndentBackground expose `active` and live progress with a provider,
      and are inactive without one.

Measurement / style state:

- [x] Popup height measurement updates runtime state through prepaint and is held
      while a nested drawer is present.
- [ ] `style_with_state(...)` receives correct popup, backdrop, viewport, swipe
      area, content, indent, and indent-background state for open/closed/swiping/
      snap-offset cases.
- [x] Drawer style states never regress the Dialog style states of reused parts.

## AccessKit accessibility follow-up

Consistent with the Dialog/AlertDialog follow-ups — no DOM ARIA now. When the
project updates to a GPUI revision with AccessKit support:

- [ ] Drawer popup keeps `Role::Dialog` (Base UI drawers use `role="dialog"`).
- [ ] SwipeArea and Backdrop are presentation-only / hidden in the accessibility
      tree.
- [ ] Title/Description labelling relationships inherit from the Dialog AccessKit
      work.

## Uncertain items needing confirmation

- Whether `DrawerRoot<P>` should literally build a `DialogRoot<P>` layer internally
  or share the `DialogContext`/`DialogRuntime` construction path directly. Prefer
  whichever keeps the Dialog module untouched except for the nested-drawer count
  and the `Swipe` reason.
- Whether the `Swipe` reason belongs on the shared `DialogOpenChangeReason` (small
  addition, matches Base UI where `REASONS.swipe` flows through the Dialog store)
  or behind a drawer-specific reason wrapper. Default: extend the shared enum.
- `DrawerSnapPoint::Rems` — keep only if GPUI rem conversion is trivially available
  at resolution time; otherwise ship `Fraction` + `Px` and note the omission.
- Exact animation mechanism (gpui `Animation` element vs manual frame-driven
  offset) for the release-strength-scaled close; decide against the `sheet.rs`
  precedent during implementation.
