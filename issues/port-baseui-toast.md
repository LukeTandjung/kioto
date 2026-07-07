# Port Base UI Toast to GPUI

## Problem

Base UI's React Toast component family provides queued, self-dismissing notifications: a
per-app provider owning a `ToastStore`, a viewport stack region that pauses all dismiss
timers while hovered/focused/window-blurred, per-toast roots with swipe-to-dismiss and
escape-to-close, content/title/description/close/action parts, an optional portal, and an
imperative manager (`useToastManager` inside React, `createToastManager` outside it) with
`add`/`close`/`update`/`promise` operations.

The deep module is the store: a newest-first toast queue with per-toast pausable timers
that track remaining milliseconds, upsert-by-id that resets the timer, an over-limit model
that flags the oldest active toasts as `limited` (inert, still mounted) instead of removing
them, promise-driven toasts (`loading` → `success`/`error`), removal deferred until the
exit animation finishes (`ending` transition status), and stacking offsets derived from
cumulative measured toast heights.

`crates/base_gpui/src/toast/` does not exist. The goal is behavioral parity with Base UI
Toast using GPUI-native architecture — port behavior and contracts, not React/DOM
internals. React hooks/context, DOM refs, `ResizeObserver`/`MutationObserver`, CSS
variables, data attributes, ARIA/`aria-live` duplication, render props, and pointer-capture
details must be dropped or translated per the standard port decisions.

Toast payloads (Base UI's `data`) are in scope as a Rust-native generic `P: Clone +
'static`. Toast ids are in scope as typed ids with runtime-generated defaults; caller-
supplied ids enable upsert.

No new shared primitive is required. Every mechanism has local precedent:

- provider/context = the entity-context pattern in
  `crates/base_gpui/src/tooltip/layers/tooltip_provider.rs` + `tooltip/context.rs`;
- pausable background timers = the generation-counted timer in
  `crates/base_gpui/src/tooltip/layers/tooltip_trigger.rs` (`spawn_delayed_hover`:
  `window.spawn` + `cx.background_executor().timer(...)` + generation invalidation);
- portal = `crates/base_gpui/src/dialog/layers/dialog_portal.rs` (deferred + anchored,
  priority);
- imperative manager = GPUI app-global / late-bound handle precedent
  (`/home/luke/Projects/gpui-component/crates/ui/src/global_state.rs`, `root.rs`,
  `notification.rs`);
- swipe = GPUI `div()` mouse handlers;
- stacking/heights = prepaint measurement (Tabs `set_bounds` precedent) plus GPUI
  animation support where useful.

## Scope

Port the Toast component family from Base UI into GPUI-native components:

- `ToastProvider<P>` — per-app/per-subtree toast runtime owner; `timeout` (default
  5000 ms), `limit` (default 3), optional bound `ToastManager<P>`.
- `ToastViewport<P>` — the stack region; hover/focus/window-blur timer pausing, F6
  focus entry, expanded state, per-toast content builder.
- `ToastRoot<P>` — one toast; swipe-to-dismiss, escape-to-close, self-height
  measurement, transition status.
- `ToastContent<P>` — content container; size-change height recalculation, `behind`
  state.
- `ToastTitle<P>` / `ToastDescription<P>` — default text rendering from the toast
  record; render nothing when there is no content.
- `ToastClose<P>` — closes its toast on click/activation.
- `ToastAction<P>` — typed action button; renders only when action content exists.
- `ToastPortal<P>` — hoists the viewport above app content via GPUI deferred/anchored
  rendering.
- Imperative manager: `ToastManager<P>` + `create_toast_manager::<P>()` (framework-
  independent handle usable before/outside the provider render) and a GPUI-native
  accessor for code that already has `&mut App` (the `useToastManager` analog),
  exposing `toasts` facts, `add -> ToastId`, `close(Option<ToastId>)`, `update`, and
  `promise`.

Deferred (see Out of scope): `ToastPositioner` / `ToastArrow` anchored-toast mode.

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/toast/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toast/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toast/store.ts` (the deep module)
- `/home/luke/Projects/base-ui/packages/react/src/toast/store.test.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toast/useToastManager.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toast/useToastManager.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toast/createToastManager.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toast/createToastManager.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toast/provider/ToastProvider.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toast/provider/ToastProviderContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toast/viewport/ToastViewport.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toast/viewport/ToastViewport.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toast/root/ToastRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toast/root/ToastRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toast/root/ToastRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toast/content/ToastContent.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toast/title/ToastTitle.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toast/description/ToastDescription.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toast/close/ToastClose.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toast/action/ToastAction.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toast/portal/ToastPortal.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toast/positioner/ToastPositioner.tsx` (deferred mode)
- `/home/luke/Projects/base-ui/packages/react/src/toast/arrow/ToastArrow.tsx` (deferred mode)
- `/home/luke/Projects/base-ui/packages/react/src/toast/utils/resolvePromiseOptions.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toast/utils/focusVisible.ts`
- Part test suites under each part folder (`ToastViewport.test.tsx` and
  `ToastRoot.test.tsx` carry most of the timer/swipe behavioral reference).

Base UI docs reference:

- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/toast/`

Relevant local GPUI precedent:

- `crates/base_gpui/src/tooltip/layers/tooltip_provider.rs` and
  `crates/base_gpui/src/tooltip/context.rs` — provider entity-context pattern.
- `crates/base_gpui/src/tooltip/layers/tooltip_trigger.rs` — `spawn_delayed_hover`:
  generation-counted background timer via `window.spawn` +
  `cx.background_executor().timer(...)`, invalidated by generation checks.
- `crates/base_gpui/src/dialog/layers/dialog_portal.rs` — deferred + anchored portal
  with priority.
- `crates/base_gpui/src/tabs/` — prepaint bounds measurement (`set_bounds` command).
- `/home/luke/Projects/gpui-component/crates/ui/src/global_state.rs`,
  `/home/luke/Projects/gpui-component/crates/ui/src/root.rs`,
  `/home/luke/Projects/gpui-component/crates/ui/src/notification.rs` — imperative
  notification queue driven through GPUI globals (`cx.set_global` / `cx.global`).

Current GPUI implementation:

- None. `crates/base_gpui/src/toast/` does not exist.

Expected GPUI implementation files (flat layout per
`docs/base-gpui-component-architecture.md`):

```text
crates/base_gpui/src/toast/
  mod.rs            # barrel only
  actions.rs        # F6 focus-viewport, Escape close, Shift+Tab exit
  runtime.rs        # ToastRuntime<P>: queue, timers, limit, metadata, outcomes
  manager.rs        # ToastManager<P> + create_toast_manager (event-bus handle)
  context.rs        # ToastContext<P>: read / update / value-changing commands
  props.rs          # provider/viewport/root props and callbacks
  style_state.rs    # Toast*StyleState structs
  child.rs          # typed child enums
  child_wiring.rs   # private traversal/context attachment (only if needed)
  layers/
    mod.rs          # barrel only
    toast_provider.rs
    toast_viewport.rs
    toast_root.rs
    toast_content.rs
    toast_title.rs
    toast_description.rs
    toast_close.rs
    toast_action.rs
    toast_portal.rs
  tests/
```

Also update:

- `crates/base_gpui/src/lib.rs` (`pub mod toast;` + `toast::init(cx);`)
- `crates/base_gpui/src/main.rs` with Toast gallery smoke scenarios

## Out of scope / drop from Base UI

- **Anchored-toast mode (`ToastPositioner`, `ToastArrow`, `positionerProps.anchor`) is
  deferred** to keep the first pass tractable. The GPUI toast record should not paint
  itself into a corner (keep the toast record extensible), but no positioner/arrow parts,
  no anchor plumbing, and no anchored swipe-disable logic ship in this pass. Note: in
  Base UI, an anchored toast disables swipe; record this so the deferred pass restores it.
- React context/hooks/store implementation details: `ReactStore`, selectors,
  `useRefWithInit`, `useIsoLayoutEffect`, `useStableCallback`, `flushSync`,
  `useOpenChangeComplete`, `FloatingPortalLite`.
- `render` prop support and React render-function children. Per-toast content is
  expressed through a typed GPUI content-builder closure receiving typed toast facts —
  an explicit Rust API, not a ported render prop.
- `className` and web `style` props.
- `nativeButton` / native DOM element switches — Close/Action are GPUI `div()`-based
  interactive elements with focus handles.
- SSR / hydration / prehydration.
- CSS variable APIs (`--toast-index`, `--toast-offset-y`, `--toast-height`,
  `--toast-frontmost-height`, `--toast-swipe-movement-x/y`). Expose the same facts as
  typed style-state values.
- DOM data attributes (`data-limited`, `data-swipe-direction`, transition status
  attributes) — map into typed `Toast*StyleState` structs.
- DOM ARIA: `role="region"` / `aria-live` / `aria-atomic` / `aria-relevant`,
  `role="dialog"`/`"alertdialog"` on roots, `aria-labelledby`/`aria-describedby`
  title/description id linking, `aria-hidden` on collapsed close buttons, and the
  hidden duplicated `role="alert"` high-priority announcement tree. Revisit via GPUI
  AccessKit APIs when the pinned GPUI revision supports them (same follow-up posture
  as Dialog/Tooltip). The `priority: 'high' | 'low'` field itself is kept on the toast
  record (it is announced state + styling input), only its ARIA projection is dropped.
- DOM measurement: `ResizeObserver`, `MutationObserver`, `offsetHeight` tricks,
  `getElementTransform` — translate to GPUI prepaint measurement.
- Browser pointer plumbing: pointer capture, `AbortController` listener teardown,
  non-passive `touchmove` scroll blocking, iOS first-pointer-move compensation,
  `pointerType === 'touch'` special paths (GPUI exposes no pointer type; drop
  touch-only branches and document them), text-selection `preventDefault`.
- DOM focus internals: `FocusGuard` elements, `isFocusVisible`, `activeElement` /
  `contains` document traversal, `prevFocusElement` DOM refs — translate to GPUI
  `FocusHandle`, key contexts, and actions where an equivalent exists.
- `actionProps` as a React DOM-props bag — replace with a typed action definition on
  the add/update options (label/content + `on_click` callback).
- Arbitrary JS value semantics for toast `data` — use `P: Clone + 'static`.
- The manager's `' subscribe'` stringly-private API and `setPromise` handoff hack —
  Rust visibility and return types express this directly.

## Acceptance Criteria

New issue — all items unchecked.

### Module/API surface

- [x] `crates/base_gpui/src/toast/` exists with the expected flat component layout.
- [x] `toast/mod.rs` and `toast/layers/mod.rs` are barrel-only files (module
      declarations, re-exports, test declarations only).
- [x] `base_gpui::toast` is exported from `crates/base_gpui/src/lib.rs` and
      `base_gpui::init(cx)` calls `toast::init(cx)` for action/key registration.
- [x] `ToastProvider<P>`, `ToastViewport<P>`, `ToastRoot<P>`, `ToastContent<P>`,
      `ToastTitle<P>`, `ToastDescription<P>`, `ToastClose<P>`, `ToastAction<P>`, and
      `ToastPortal<P>` exist and are publicly exported.
- [x] `ToastManager<P>` and `create_toast_manager::<P>()` exist and are publicly
      exported from `toast/manager.rs`.
- [x] A GPUI-native `useToastManager` analog exists for code with app access (e.g.
      methods on the bound manager/context reading through the provider runtime):
      `toasts` facts, `add(options) -> ToastId`, `close(Option<ToastId>)`,
      `update(id, options)`, `promise(...)`.
- [x] `ToastId` is a typed id (not a raw string parameter type on every API); the
      runtime generates unique ids when the caller supplies none.
- [x] Toast add/update options carry: optional `id`, `title`, `description`, typed
      `toast_type` (with `Loading` semantics), `timeout: Option<Duration>` (with `0` =
      sticky), `priority` (`Low`/`High`), typed action definition, `on_close`,
      `on_remove`, and `payload: Option<P>`.
- [x] Payload APIs use `P: Clone + 'static`; no `serde_json`-style dynamic values.
- [x] Per-toast content rendering is a typed GPUI content-builder on the viewport (or
      equivalent explicit Rust API) receiving typed toast facts and returning a
      `ToastRoot<P>` subtree; no React-style render props.
- [x] All public style payloads use `*StyleState` names from `style_state.rs`; no
      `*RenderState` / `render_state.rs`.
- [x] No Toast code uses Rust scoped visibility syntax (`pub(...)`).
- [x] `ast-grep scan crates/base_gpui/src` passes.

### Correctness / compile readiness

- [x] `cargo fmt --check` passes.
- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui` passes.
- [x] `cargo clippy -p base_gpui --all-targets` passes or only reports pre-existing
      warnings.
- [x] `crates/base_gpui/src/main.rs` gains gallery smoke scenarios: basic add/dismiss,
      limit overflow, hover-pause, upsert, promise toast, close-all, and manager-driven
      add from outside the viewport subtree.

### Architecture / internal primitives

- [x] `ToastRuntime<P>` (in `runtime.rs`) is the single deep module owning **all**
      toast state: the newest-first queue, per-toast records (id, type, title,
      description, payload, timeout, priority, action, callbacks, update generation,
      transition status, limited flag, measured height), timer bookkeeping
      (delay/remaining/deadline per toast + paused flag), hover/focus/window-focus
      facts, limit/timeout defaults, and derived metadata (visible index, dom index,
      cumulative `offset_y`, frontmost height).
- [x] `ToastRuntime<P>` exposes domain commands, one per thing-that-can-happen, e.g.:
      `add_toast`, `update_toast`, `close_toast(Option<ToastId>)`, `remove_toast`,
      `pause_timers`, `resume_timers`, `timer_fired(id, generation)`, `set_hovering`,
      `set_focused`, `set_window_focused`, `set_heights`/`set_toast_height`,
      `sync_provider_props(timeout, limit)`, and promise-transition commands.
- [x] `ToastRuntime<P>` exposes part-shaped queries returning `Toast*StyleState`
      values; no getter/setter pairs, no "give me the whole queue internals" query on
      the part path (the manager-facing `toasts` facts query is the one documented
      exception, mirroring Base UI's public `toasts` array).
- [x] Every transition (limited recomputation, ending status, metadata/offset
      recalculation, timer rescheduling decisions) is computed inside the runtime,
      once; parts never diff shadow copies.
- [x] Timer **decisions** live in the runtime; timer **execution** lives in layers:
      the runtime returns schedule/cancel outcomes (id, duration, generation) and
      layers spawn generation-counted background timers
      (`cx.background_executor().timer`) following the tooltip `spawn_delayed_hover`
      precedent. A fired timer with a stale generation is a no-op.
- [x] The runtime's time arithmetic (remaining-ms on pause, resume-with-remaining) is
      expressed against injected `Instant`s / durations passed into commands, so it is
      unit-testable without a window or real clock.
- [x] `ToastContext<P>` remains a thin injection vehicle (`read`, `update`, and the
      value-changing command wrappers); it grows no queue/timer/measurement
      vocabulary of its own.
- [x] `ToastProvider<P>` creates and owns the keyed runtime entity per provider
      subtree (per-app `ToastStore` analog), following the tooltip provider pattern;
      provider state does not leak across unrelated provider subtrees.
- [x] `ToastManager<P>` is a late-bound handle (event-bus analog of
      `createToastManager`): creatable before any provider mounts, bound via
      `ToastProvider::manager(...)`, delivering `add`/`close`/`update`/`promise` to the
      bound provider runtime. Operations on an unbound manager are queued or
      documented no-ops — decide and document; do not panic.
- [x] If an app-global convenience accessor is provided (gpui-component
      `Root`/`notification.rs` analog via `cx.set_global`/`cx.global`), it is layered
      on top of the manager/provider binding and its single-provider assumption is
      documented; multiple providers must still work through explicit managers.
- [x] Child routing uses typed enums before `AnyElement` erasure; toast-root children
      route `Content`/`Title`/`Description`/`Close`/`Action` (with an `AnyElement`
      escape hatch only if Base UI examples show arbitrary children — they do).
- [x] No new shared generic primitive is introduced under `utils/` for toast plumbing.
- [x] No `runtime_control.rs`-style trait-boundary file is introduced.

### Queue, limit, and upsert behavior

- [x] New toasts are prepended (newest-first queue order preserved exactly).
- [x] Provider `timeout` defaults to 5000 ms; `limit` defaults to 3.
- [x] `add` with no id generates a unique id and returns it.
- [x] `add` with an existing id whose toast is **not** ending upserts in place: fields
      updated, auto-dismiss timer reset, update generation incremented, queue position
      unchanged, and the same id returned.
- [x] `add` with an existing id whose toast **is** ending removes the ending toast
      without firing `on_remove`, then adds a fresh toast.
- [x] `update(id, ...)` on a live toast applies partial updates and increments the
      update generation; `update` on an ending toast is ignored (prevents async
      promise updates from blocking a dismissal).
- [x] `update` clears the timer when the toast should no longer have one (became
      `Loading`, timeout became 0/sticky, or ending) and (re)schedules when timeout
      changed, the toast was previously `Loading`, or a reset is requested.
- [x] Over-limit behavior: active (non-ending) toasts beyond `limit`, oldest first,
      are flagged `limited` — **not removed**. Limited toasts render inert
      (non-interactive, skipped by focus traversal) and expose `limited` in style
      state so users can hide/animate them.
- [x] `limited` flags are recomputed on every add/close/limit change; ending toasts
      are ignored by the limit count; a toast whose flag is unchanged is not treated
      as updated.
- [x] Changing provider `limit`/`timeout` at render time syncs into the runtime and
      recomputes `limited` flags (Base UI `syncProviderProps` parity).
- [x] `close(Some(id))` marks that toast `ending` (height treated as 0 for stacking)
      and cancels its timer; `close(None)` closes **all** toasts and clears all timers.
- [x] `on_close` fires when a toast transitions to `ending` (once — not re-fired for
      already-ending toasts); `on_remove` fires when the toast is actually removed
      after its exit transition.
- [x] When no active (non-ending) toasts remain, hovering/focused facts reset so a
      future toast starts from a collapsed viewport.
- [x] Removal is deferred until the exit animation/transition completes; with no
      animation infrastructure active, removal completes immediately after `ending`.

### Timer pause/resume model

- [x] A toast's dismiss duration is its own `timeout` if set, else the provider
      `timeout`.
- [x] Duration `0` means sticky: no timer is ever scheduled.
- [x] `toast_type = Loading` never schedules a timer regardless of duration.
- [x] A toast added while the viewport is hovered, focus-visible-focused, or the
      window is unfocused starts with its timer **paused** (full duration remaining).
- [x] `pause_timers` is idempotent and records each toast's remaining duration
      (`delay - elapsed`, clamped at 0).
- [x] `resume_timers` is idempotent and restarts each timer with its remaining
      duration (full delay if never started).
- [x] Viewport hover (mouse enter/move) pauses all timers and sets hovering; mouse
      leave resumes timers (only if the window is focused) and clears hovering.
- [ ] Mouse leave while any toast is `ending` (or mid-swipe) defers the
      collapse/resume until transitions settle, then flushes it (Base UI
      `flushMouseLeave` parity), avoiding mid-gesture stack collapse.
      > Unchecked: removal is immediate after `ending` in this pass, so no deferred `flushMouseLeave` collapse exists yet.
- [ ] Viewport keyboard focus (focus-visible) pauses timers and sets focused; blur to
      outside the viewport clears focused and resumes timers if the window is focused.
      > Unchecked: `focused` is driven only by the F6/Shift+Tab actions; no focus/blur observation of the viewport subtree yet.
- [ ] Window blur pauses timers and clears window-focused; window activation resumes
      timers (unless focus is inside the viewport) and restores window-focused — use
      GPUI window activation observation as the `window blur/focus` analog.
      > Unchecked: window activation is sampled at provider render time; no activation subscription in this pass.
- [x] After the last timer fires or is cleared, internal paused bookkeeping resets so
      a fresh toast's running timer can be paused again by the next hover/focus
      (store.test.ts re-pause regression cases: last toast closed, all closed,
      last active closes while ending remain, last timed closes while untimed remain,
      last timed toast becomes untimed).
- [x] Timer expiry closes the toast (transition to `ending`), not instant removal.
- [x] Stale timer completions (generation mismatch after reset/pause/close) never
      mutate current state.

### Swipe-to-dismiss behavior

- [x] `ToastRoot<P>::swipe_direction(...)` accepts one or more of `Up`, `Down`,
      `Left`, `Right`; default is `[Down, Right]`; an empty set disables swiping.
- [x] Swipe is implemented with GPUI mouse handlers (down/move/up on the root plus
      window-level move/up capture while dragging); no DOM pointer capture is ported.
- [x] Mouse-down on an interactive descendant (Close, Action, or other focusable
      control) does not begin a swipe.
- [x] Drag displacement beyond a minimum threshold (Base UI: 1 px) marks a real swipe;
      when both axes are allowed, the dominant axis at that moment locks the drag
      direction.
- [x] Displacement along a permitted direction moves the toast 1:1; displacement along
      non-permitted directions is damped (Base UI: `delta ** 0.5`).
- [x] Release with displacement beyond the dismiss threshold (Base UI: 40 px) in a
      permitted direction closes the toast, recording the dismiss direction.
- [x] Change-of-mind cancel: when only one direction on an axis is permitted, moving
      back toward origin by at least the reverse-cancel threshold (Base UI: 10 px)
      after peak displacement cancels the swipe on release; the toast springs back.
- [x] Release below the threshold (or a cancelled gesture) resets the toast offset to
      rest and clears the swipe direction.
- [x] Swiping facts (`swiping`, `swipe_direction`, current x/y swipe movement) are
      exposed through `ToastRootStyleState` (translating the `--toast-swipe-movement-*`
      CSS vars into typed values).
- [ ] While dragging, timers stay paused (the pointer is over the viewport; verify the
      hover-pause path covers the whole gesture, including release outside the
      viewport).
      > Unchecked: hover-pause covers drags inside the viewport; releasing/leaving the viewport mid-drag can resume timers early.
- [x] Escape closes the toast whose subtree contains focus, via `actions.rs` key
      dispatch + `key_context` on the root (not raw key-down handlers).

### Stacking, measurement, and transition behavior

- [x] Each toast's natural height is measured GPUI-natively (prepaint /
      `on_children_prepainted`, Tabs `set_bounds` precedent) and recorded in the
      runtime; content size changes trigger recalculation (`ToastContent` replaces the
      `ResizeObserver`/`MutationObserver` watch).
- [x] Height changes recompute stacking metadata: per-toast `offset_y` is the
      cumulative height of preceding (newer) toasts in queue order; ending toasts
      contribute 0.
- [x] `ToastRootStyleState` exposes: `transition_status` (`Starting`/`Ending`/steady),
      `expanded`, `limited`, `toast_type`, `swiping`, `swipe_direction`, stacking
      `index` (visible index while live, dom index while ending; ending toasts have no
      visible index), `offset_y`, and measured `height`.
- [x] `ToastViewportStyleState` exposes `expanded` (hovering || focused) and
      `frontmost_height` (newest toast's measured height, for collapsed-stack sizing).
- [x] `ToastContentStyleState` exposes `expanded` and `behind` (visible index > 0).
- [x] `transition_status` starts at `Starting` on add and clears after the first
      height measurement (so enter animations can key off it), matching Base UI.
- [x] `Ending` status is preserved until removal so exit animations can run; where
      GPUI animation infrastructure (`gpui` `animation.rs`) is used, removal follows
      animation completion; otherwise removal is immediate and documented.
- [x] Toasts render in the viewport via GPUI deferred/anchored rendering
      (`ToastPortal` following `dialog_portal.rs`) so the stack overlays app content;
      portal priority is documented relative to dialog/tooltip overlays.

### Promise toasts

- [x] `promise(future, options)` adds a `Loading` toast immediately (no timer) and
      returns a handle/future preserving the original output (`Result<T, E>`; errors
      propagate to the caller as in Base UI's rejected-promise passthrough).
- [x] On success, the toast updates in place to the success options with
      `toast_type = Success` and gains a dismiss timer per the resolved timeout.
- [x] On error, the toast updates in place to the error options with
      `toast_type = Error` and gains a dismiss timer per the resolved timeout.
- [x] Loading/success/error options accept either plain text or full update options;
      success/error variants may compute options from the resolved value/error
      (Rust closures replacing `resolvePromiseOptions`).
- [x] A promise resolution arriving after the toast was closed/ending does not revive
      or block dismissal (ending-update-ignored rule).
- [x] Future execution uses GPUI executors (`cx.spawn` /
      `cx.background_executor()`); no JS Promise semantics are ported.
- [x] `ToastManager::promise` works from outside the provider subtree once bound.

### Focus and keyboard behavior

- [x] F6 (via `actions.rs` global/window key dispatch) focuses the viewport when
      toasts exist, records the previously focused element's `FocusHandle` for
      restoration, pauses timers, and sets focused.
- [ ] From the focused viewport, focus moves into the first focusable toast (not
      ending, not limited); GPUI tab-stop traversal replaces the DOM `FocusGuard`
      mechanism where possible, otherwise document the gap.
      > Unchecked: the viewport itself takes focus; per-toast focus traversal is not implemented.
- [x] Shift+Tab on the focused viewport returns focus to the recorded previous focus
      handle and resumes timers.
- [ ] Closing the focused toast moves focus to the next non-ending toast (or previous
      when none follows), else restores the recorded previous focus handle
      (`handleFocusManagement` parity); close-all restores previous focus directly.
      > Unchecked: per-toast focus management (`handleFocusManagement` parity) is not implemented.
- [x] Focus-driven expansion only occurs for keyboard (focus-visible-like) focus where
      GPUI can distinguish it; if GPUI cannot, document the divergence and pick the
      conservative behavior.
- [ ] Limited toasts are skipped by focus traversal (inert parity).
      > Unchecked: limited roots render inert, but Close/Action children inside them are not excluded from tab traversal.

### Part behavior (Content/Title/Description/Close/Action)

- [x] `ToastContent<P>` wraps toast contents, watches its own size, and triggers
      height recalculation.
- [x] `ToastTitle<P>` renders the toast record's title by default (children override);
      renders nothing when no title exists.
- [x] `ToastDescription<P>` renders the toast record's description by default
      (children override); renders nothing when no description exists.
- [x] `ToastClose<P>` closes its own toast on click/keyboard activation.
- [x] `ToastAction<P>` renders the typed action (from the toast record or its own
      children), invokes the action callback on activation, and renders nothing when
      no action content exists.
- [x] Title/Description/Close/Action expose `toast_type` in their style states;
      Close/Action are focusable GPUI interactive elements.
- [x] DOM id wiring (`setTitleId`/`setDescriptionId` for `aria-labelledby`/
      `aria-describedby`) is not ported; it returns with the AccessKit follow-up.

### Styling/state exposure

- [x] One `*StyleState` struct per part that draws, in `style_state.rs`:
      `ToastViewportStyleState`, `ToastRootStyleState`, `ToastContentStyleState`,
      `ToastTitleStyleState`, `ToastDescriptionStyleState`, `ToastCloseStyleState`,
      `ToastActionStyleState` (portal only if it draws; provider renders no element
      and gets none).
- [x] All drawing parts expose `style_with_state(...)`.
- [x] Every Base UI data attribute / CSS var fact is available as typed style state:
      `transition_status`, `expanded`, `limited`, `toast_type`, `swiping`,
      `swipe_direction`, `behind`, stacking `index`, `offset_y`, `height`,
      `frontmost_height`, swipe movement x/y.
- [x] No `className`, web `style`, render-prop, CSS-var, or DOM data-attribute public
      API is introduced.

### Tests / verification

Runtime tests (unit-testable without a window — timer arithmetic driven by injected
instants/ticks, timer execution mocked as explicit `timer_fired` calls):

- [x] Newest-first ordering and metadata (dom index, visible index, offset_y) stay
      synchronized across add/close/remove/height mutations.
- [x] Default timeout 5000 ms / per-toast override / `0` sticky / `Loading` no-timer.
- [x] Upsert-by-id resets the timer, increments update generation, keeps queue
      position; upsert onto an ending toast removes silently and re-adds.
- [x] Update on ending toast is ignored; update timeout changes reschedule; update to
      `Loading` clears the timer; update from `Loading` schedules one.
- [x] Limit: newest 3 visible, older flagged limited; limit change recomputes flags;
      ending toasts excluded from the count.
- [x] Pause records remaining ms; resume restarts with remaining; pause/resume are
      idempotent; toast added while expanded/out-of-focus starts paused.
- [x] Re-pause regression suite mirroring `store.test.ts` timer-pausing cases.
- [x] Stale timer generations are no-ops after reset/pause/close.
- [x] `close(None)` closes all, clears timers, fires each `on_close` once;
      `on_remove` fires on removal, not on close; skip-on-remove path for
      upsert-over-ending.
- [x] Hover/focus flags reset when no active toasts remain.
- [x] Swipe state machine: direction locking, damping of non-permitted axes,
      threshold dismiss, reverse-cancel, spring-back reset.
- [ ] Focus-management target selection (next, then previous, then previous-focus
      restore).
      > Unchecked: no focus-target selection logic exists in the runtime yet.
- [ ] Promise transitions: loading→success, loading→error, resolution after
      close is ignored, text vs options vs closure option forms.
      > Unchecked: promise plumbing lives in `ToastContext` (needs `App`); the underlying update rules are unit tested instead.
- [ ] Manager event delivery: add/close/update/promise before and after binding;
      unbound behavior matches its documentation.
      > Unchecked: unbound-queue behavior is documented but untested (needs a gpui test harness).

Rendered behavior tests (`toast/tests/`):

- [ ] Add renders a toast; timer expiry transitions it to ending and removes it.
      > Unchecked: rendered-behavior (windowed) test harness for toast was not built in this pass.
- [ ] Hover pauses auto-dismiss; unhover resumes with remaining time.
      > Unchecked: rendered-behavior (windowed) test harness for toast was not built in this pass.
- [ ] Unhover while a toast is ending defers collapse until it finishes.
      > Unchecked: rendered-behavior (windowed) test harness for toast was not built in this pass.
- [ ] Window deactivation pauses; reactivation resumes.
      > Unchecked: rendered-behavior (windowed) test harness for toast was not built in this pass.
- [ ] Limit overflow renders limited toasts inert with `limited` style state.
      > Unchecked: rendered-behavior (windowed) test harness for toast was not built in this pass.
- [ ] Swipe past threshold dismisses; short swipe springs back; reverse-cancel holds.
      > Unchecked: rendered-behavior (windowed) test harness for toast was not built in this pass.
- [ ] Escape closes the focused toast.
      > Unchecked: rendered-behavior (windowed) test harness for toast was not built in this pass.
- [ ] F6 focuses the viewport; Shift+Tab restores previous focus; closing the focused
      toast moves focus to a neighbor.
      > Unchecked: rendered-behavior (windowed) test harness for toast was not built in this pass.
- [ ] Close and Action parts activate correctly; Action absent when no action content.
      > Unchecked: rendered-behavior (windowed) test harness for toast was not built in this pass.
- [ ] Title/Description default rendering from the toast record.
      > Unchecked: rendered-behavior (windowed) test harness for toast was not built in this pass.
- [ ] Stacking offsets and frontmost height update as heights are measured and toasts
      close.
      > Unchecked: rendered-behavior (windowed) test harness for toast was not built in this pass.
- [ ] Style-state exposure for viewport, root, content, close, and action.
      > Unchecked: rendered-behavior (windowed) test harness for toast was not built in this pass.
- [ ] Manager-driven add from outside the provider subtree.
      > Unchecked: rendered-behavior (windowed) test harness for toast was not built in this pass.
- [x] `cargo test -p base_gpui toast` passes.

## Uncertain items to confirm during implementation

- `toast_type` representation: enum (`Loading`/`Success`/`Error`/`Custom(SharedString)`)
  vs plain `SharedString` with documented `"loading"` semantics. Base UI uses free-form
  strings with `loading` special-cased; the enum is more Rust-native — pick one and keep
  the `Loading` timer rule intact.
- Whether unbound `ToastManager` operations queue until a provider binds (friendlier)
  or drop as documented no-ops (simpler, closer to Base UI's subscribe-on-mount).
- Whether an app-global single-provider convenience accessor is worth shipping in this
  pass or should wait until a real consumer needs it.
- Whether GPUI can distinguish keyboard-visible focus from pointer focus for the
  focused-expansion rule, and whether GPUI tab-stop traversal can fully replace the DOM
  `FocusGuard` pattern.
- Exit-animation-aware removal: whether to integrate GPUI `animation.rs` timing in this
  pass or ship immediate-removal-after-ending with the animation hookup as a follow-up.
- How portal priority should interleave with the existing dialog/tooltip overlay
  priorities in `dialog_portal.rs`-style deferred rendering.
