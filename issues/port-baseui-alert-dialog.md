# Port Base UI Alert Dialog to GPUI

## Problem

Base UI's Alert Dialog is **not a new interaction model** — it is the Dialog with
three invariants forced on:

- `modal` is always `true`,
- `disablePointerDismissal` is always `true` (no outside-press / backdrop dismissal),
- the accessibility role is `alertdialog` instead of `dialog`.

In Base UI this is expressed by reusing every Dialog part unchanged and only
providing an alert-dialog-specific `Root`, `Trigger`, and `Handle`. See
`alert-dialog/index.parts.ts`: `Backdrop`, `Close`, `Description`, `Popup`,
`Portal`, `Title`, and `Viewport` are re-exported directly from `dialog/...`, and
`useRenderDialogRoot(props, 'alert-dialog')` is the only behavioral fork
(`dialog/root/useRenderDialogRoot.tsx:28-35`).

`crates/base_gpui/src/dialog/` is already a complete GPUI-native port with
`DialogModalMode`, `disable_pointer_dismissal`, controlled/uncontrolled open state,
handle-driven detached triggers, focus trapping, and Escape/pointer dismissal
already implemented. The Alert Dialog port should therefore be **thin**: reuse the
Dialog runtime/context/parts, force the two behavioral invariants, drop the setters
that would let a caller violate them, and give the new parts distinct types so an
Alert Dialog cannot be wired to a plain Dialog trigger/handle (Base UI enforces this
with a private `__alertDialogBrand`).

The goal is behavioral parity with Base UI Alert Dialog using GPUI-native
composition — no re-implementation of Dialog behavior, and no web-only details.

Payloads stay generic via the existing Dialog type parameter `P: Clone + 'static`.

## Scope

Add a GPUI-native Alert Dialog that composes the existing Dialog implementation:

New alert-dialog-specific parts:

- `AlertDialogRoot<P>` — Dialog root with `modal_mode = Modal` and
  `disable_pointer_dismissal = true` forced; no `modal` / `disable_pointer_dismissal`
  setters exposed.
- `AlertDialogTrigger<P>` — Dialog trigger bound to an `AlertDialogHandle<P>`.
- `AlertDialogHandle<P>` + `create_alert_dialog_handle<P>()` — a distinct handle type
  that binds an alert-dialog context.

Reused Dialog parts, re-exported under Alert Dialog names (no reimplementation):

- `AlertDialogBackdrop` = `DialogBackdrop`
- `AlertDialogClose` = `DialogClose`
- `AlertDialogDescription` = `DialogDescription`
- `AlertDialogPopup` = `DialogPopup`
- `AlertDialogPortal` = `DialogPortal`
- `AlertDialogTitle` = `DialogTitle`
- `AlertDialogViewport` = `DialogViewport`

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/alert-dialog/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/alert-dialog/root/AlertDialogRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/alert-dialog/trigger/AlertDialogTrigger.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/alert-dialog/handle.ts`
- `/home/luke/Projects/base-ui/packages/react/src/dialog/root/useRenderDialogRoot.tsx` (variant fork)
- `/home/luke/Projects/base-ui/packages/react/src/alert-dialog/root/AlertDialogRoot.test.tsx` (behavioral reference)

Existing GPUI implementation to reuse (do not fork/duplicate):

- `crates/base_gpui/src/dialog/` (entire module)
- Specifically: `dialog/props.rs` (`DialogProps::new(modal_mode, disable_pointer_dismissal, ...)`),
  `dialog/runtime.rs` (`pointer_dismissal_enabled`, focus trap, open/close outcome),
  `dialog/context.rs` (`DialogContext`, `DialogHandle`, `create_dialog_handle`),
  `dialog/layers/dialog_root.rs` (`modal`, `disable_pointer_dismissal`, `modal_mode`,
  `trap_focus` setters that must NOT be re-exposed),
  `dialog/layers/dialog_trigger.rs` (`handle(...)` wiring),
  `dialog/style_state.rs` (`DialogModalMode`).

Expected new GPUI files (flat layout, per `docs/base-gpui-component-architecture.md`):

- `crates/base_gpui/src/alert_dialog/mod.rs` — module + `init(cx)` + barrel exports
  (new parts, plus re-exports of the reused Dialog parts under Alert Dialog names).
- `crates/base_gpui/src/alert_dialog/handle.rs` — `AlertDialogHandle<P>` +
  `create_alert_dialog_handle<P>()`.
- `crates/base_gpui/src/alert_dialog/layers/alert_dialog_root.rs` — `AlertDialogRoot<P>`.
- `crates/base_gpui/src/alert_dialog/layers/alert_dialog_trigger.rs` — `AlertDialogTrigger<P>`.
- `crates/base_gpui/src/alert_dialog/layers/mod.rs`.
- `crates/base_gpui/src/alert_dialog/tests/` — behavior tests.
- Register `pub mod alert_dialog;` and `alert_dialog::init(cx);` in
  `crates/base_gpui/src/lib.rs`.

## Reuse strategy (decide before implementing)

Base UI keeps Alert Dialog as a separate namespace that *re-exports Dialog part
components* and forks only the root via a `mode` argument. Two GPUI-native options —
pick one and note it in the PR:

1. **Compose (preferred).** `AlertDialogRoot<P>` internally builds a `DialogRoot<P>`,
   calling `.modal(true)` and `.disable_pointer_dismissal(true)`, and forwards the
   remaining setters (`id`, `default_open`, `open`, `on_open_change`,
   `on_open_change_complete`, `trigger_id`, `default_trigger_id`, `child(ren)`,
   `style_with_state`, `handle`). This guarantees the invariants without touching
   Dialog internals and keeps a single behavioral implementation. `AlertDialogTrigger<P>`
   likewise wraps `DialogTrigger<P>`.
2. **Variant flag on Dialog.** Add a private "alert" flag to `DialogProps` that pins
   `modal_mode`/`disable_pointer_dismissal` inside `reconcile`, mirroring Base UI's
   `mode === 'alert-dialog'` fork. More faithful to Base UI, but pushes alert-specific
   knowledge into the Dialog runtime; only choose this if composition proves leaky
   (e.g. handle-supplied stores must be re-pinned to alert invariants, matching
   `handle.ts` re-applying `alertDialogState`).

Regardless of option, the public `AlertDialogRoot<P>` surface must **not** expose
`modal`, `modal_mode`, `trap_focus`, or `disable_pointer_dismissal`.

## Out of scope / drop from Base UI

- `role: 'alertdialog'` as a DOM ARIA attribute — GPUI Dialog has no `role` concept
  and does not emit DOM ARIA. Track under the AccessKit follow-up
  (`accesskit::Role::AlertDialog`) instead of adding a DOM attribute.
- React context/hooks, `useRenderDialogRoot`, `DialogStore`, `useControlledProp`,
  `useSyncedValues`, `useOnFirstRender` — GPUI uses keyed entity state +
  controlled/uncontrolled resolution in the context, already implemented for Dialog.
- `render` prop support.
- `className`, web `style` props, CSS variable API.
- `nativeButton` / native DOM element switches.
- SSR / hydration / prehydration.
- DOM data attributes as attributes — map to GPUI style-state structs (inherited
  from Dialog's `Dialog*StyleState`).
- The private TypeScript `__alertDialogBrand` marker — replace with a genuinely
  distinct Rust type (`AlertDialogHandle<P>`) rather than a phantom brand.
- Do not re-derive nested-dialog behavior; whatever Dialog already implements for
  `nested` is inherited unchanged.

## Acceptance Criteria

New issue — items are unchecked. Items marked **(inherited)** are already
implemented in `dialog/` and only need to remain correct through Alert Dialog reuse;
they should not be reimplemented.

### Module / API surface

- [x] `crates/base_gpui/src/alert_dialog/` module exists and is registered in
      `lib.rs` (`pub mod alert_dialog;` + `alert_dialog::init(cx);`).
- [x] `AlertDialogRoot<P>` builder exists.
- [x] `AlertDialogTrigger<P>` builder exists.
- [x] `AlertDialogHandle<P>` and `create_alert_dialog_handle<P>()` exist.
- [x] `alert_dialog/mod.rs` re-exports `AlertDialogBackdrop`, `AlertDialogClose`,
      `AlertDialogDescription`, `AlertDialogPopup`, `AlertDialogPortal`,
      `AlertDialogTitle`, `AlertDialogViewport` as aliases of the Dialog parts.
- [x] `AlertDialogRoot<P>` exposes `id`, `default_open`, `open`, `on_open_change`,
      `on_open_change_complete`, `trigger_id`, `default_trigger_id`,
      `no_trigger_id`, `child` / `children` / `child_any`, `style_with_state`,
      `handle`.
- [x] `AlertDialogRoot<P>` does **not** expose `modal`, `modal_mode`, `trap_focus`,
      or `disable_pointer_dismissal`.
- [x] `AlertDialogTrigger<P>` exposes `id`, `disabled`, `payload`,
      `style_with_state`, and `handle` (typed to `AlertDialogHandle<P>`).
- [x] `AlertDialogTrigger<P>` / `AlertDialogRoot<P>` accept `AlertDialogHandle<P>`,
      not `DialogHandle<P>`, so plain Dialog handles cannot drive an alert dialog.
- [x] Payload type parameter constrained as `P: Clone + 'static`, consistent with Dialog.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui` passes.
- [x] No duplication of Dialog runtime/context logic; Alert Dialog composes or
      flags the existing implementation (see Reuse strategy).
- [x] Add a small example/demo mounting an Alert Dialog with Trigger + Portal +
      Backdrop + Popup + Title + Description + two Close actions (confirm/cancel).

### Forced-invariant behavior

- [x] Regardless of caller input, an Alert Dialog is always modal
      (`DialogModalMode::Modal`).
- [x] Regardless of caller input, an Alert Dialog always has
      `disable_pointer_dismissal = true`.
- [x] Clicking outside the popup does **not** close an open Alert Dialog.
- [x] Clicking the backdrop does **not** close an open Alert Dialog.
- [x] Pressing Escape **does** close an open Alert Dialog (Escape is not gated by
      `disable_pointer_dismissal`). **(inherited — verify not regressed)**
- [x] An `AlertDialogClose` action closes the dialog. **(inherited)**
- [x] Focus is trapped within the popup while open. **(inherited)**
- [ ] If option 2 (variant flag) is chosen: a handle-supplied store/context created
      as a plain Dialog is re-pinned to alert invariants on bind, mirroring
      `handle.ts` re-applying `alertDialogState`. *(N/A — option 1 (compose) was
      chosen; invariants are forced in `AlertDialogRoot::render`.)*

### Open/close state behavior (inherited from Dialog — verify through Alert Dialog)

- [x] Uncontrolled: `default_open(true)` opens initially.
- [x] Controlled: `open(...)` makes the caller own open state; interaction fires
      `on_open_change` without mutating internal state.
- [x] Controlled precedence: if `open(...)` is supplied, the root is controlled;
      otherwise uncontrolled.
- [x] `on_open_change` / `on_open_change_complete` fire with correct timing and can
      `preventUnmountOnClose` equivalent behavior where Dialog supports it.
- [x] Trigger opens the dialog; `trigger_id` / `default_trigger_id` scope which
      trigger is active.

### Handle / detached trigger behavior

- [x] `create_alert_dialog_handle::<P>()` returns an `AlertDialogHandle<P>`.
- [x] `AlertDialogRoot::handle(...)` binds the handle to the root context.
- [x] `AlertDialogHandle` `open` / `open_with_payload` / `close` / `is_open` /
      `unmount` behave as the Dialog handle does, but the dialog remains
      pointer-dismissal-disabled and modal.
- [x] A detached `AlertDialogTrigger` bound via handle opens the dialog.
- [x] `AlertDialogHandle<P>` and `DialogHandle<P>` are not interchangeable at the
      type level.

### Styling / state exposure

- [x] Alert Dialog parts expose the same `Dialog*StyleState` structs via
      `style_with_state` (reused, not duplicated).
- [x] `DialogRootStyleState` reports `modal_mode = Modal` and
      `disable_pointer_dismissal = true` for an Alert Dialog root.

### Tests / verification

- [x] Outside/backdrop press does not close (the core Alert-vs-Dialog difference).
- [x] Escape closes.
- [x] `AlertDialogClose` closes.
- [x] Trigger opens.
- [x] Handle-driven open/close (including detached trigger).
- [x] Controlled open state does not self-mutate on interaction.
- [x] Uncontrolled `default_open` initial state.
- [ ] Focus trap active while open. *(No direct tab-cycle test; inherited Dialog
      focus-trap machinery is unchanged and tests assert `modal_mode = Modal`.)*
- [x] Type-level test/example showing a `DialogHandle` cannot be passed where an
      `AlertDialogHandle` is required (compile-fail note or doc example is enough).

## AccessKit accessibility follow-up

Base UI Alert Dialog differs from Dialog in the accessibility tree by using
`role="alertdialog"` (`handle.ts` `alertDialogState.role`). GPUI does not currently
emit DOM ARIA, so this is deferred, consistent with the Dialog/Tabs AccessKit
follow-ups.

When this project updates to a GPUI revision containing AccessKit support (commit
`1d029c5ff5654fb1b1e8caf4462993c8ee13a133` or newer):

- [ ] Set `accesskit::Role::AlertDialog` on the Alert Dialog popup (vs
      `Role::Dialog` for plain Dialog).
- [ ] Confirm the exact AccessKit role name in the checked-out GPUI version.
- [ ] Ensure Title/Description labelling relationships (inherited from the Dialog
      AccessKit work) apply to the Alert Dialog popup.

## Uncertain items needing confirmation

- Reuse strategy: compose (option 1) vs Dialog variant flag (option 2). Default to
  compose unless handle re-pinning forces option 2.
- Whether `AlertDialogHandle<P>` should be a newtype wrapping `DialogHandle<P>` or a
  separate struct binding an `AlertDialogContext`. Prefer the smallest thing that
  makes the two handle types non-interchangeable while reusing Dialog's bind logic.
