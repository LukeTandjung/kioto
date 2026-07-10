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

Written against the AccessKit surface in the pinned gpui revision
(`1d029c5ff5654fb1b1e8caf4462993c8ee13a133`, accesskit 0.24); see
`docs/accesskit-gpui-reference.md`. In Base UI the only a11y fork from Dialog is
`role="alertdialog"` on the popup (`alert-dialog/handle.ts` `alertDialogState.role`,
threaded through `dialog/popup/DialogPopup.tsx` via `store.useState('role')`); the
trigger emits `aria-haspopup="dialog"` / `aria-expanded` / `aria-controls`
(`dialog/trigger/DialogTrigger.tsx:91-93`) and the popup emits
`aria-labelledby`/`aria-describedby` pointing at Title/Description
(`dialog/popup/DialogPopup.tsx:90-91`).

Because `AlertDialogPopup` is a plain re-export of `DialogPopup`
(`alert_dialog/mod.rs`), the role fork needs a small hook in the shared part: give
`DialogPopup` an internal `role: accesskit::Role` field defaulting to
`Role::Dialog`, and have the alert-dialog composition (or a crate-private setter)
pin it to `Role::AlertDialog` — mirroring Base UI's store-driven `role`.

### Per accessible part

- **`AlertDialogTrigger<P>`** (wraps `DialogTrigger<P>`,
  `dialog/layers/dialog_trigger.rs`): `.role(Role::Button)` on the interactive div;
  `.aria_expanded(state.active_trigger && state.open)` from the same
  `DialogTriggerStyleState` fields (`open`, `active_trigger`) already computed for
  `style_with_state` — this maps Base UI's `aria-expanded: isOpenedByThisTrigger`.
  `.aria_label(...)` from a new builder prop (see Labels).
- **`AlertDialogPopup` (= `DialogPopup<P>`, `dialog/layers/dialog_popup.rs`)**:
  `.role(Role::AlertDialog)` (vs `Role::Dialog` for the plain Dialog), on the same
  element that already carries `.track_focus(&focus_handle.tab_stop(true)...)` and
  `.focusable()`. `.aria_label(...)` from the Title text (see Labels), standing in
  for `aria-labelledby`.
- **`AlertDialogTitle` (= `DialogTitle<P>`)**: `.role(Role::Heading)` +
  `.aria_level(2)` so the title is still discoverable by heading navigation even
  though there is no `aria-labelledby` wiring.
- **`AlertDialogClose` (= `DialogClose<P>`, `dialog/layers/dialog_close.rs`)**:
  `.role(Role::Button)`; `.aria_label(...)` per instance (e.g. "Confirm" /
  "Cancel") when the visible child is not plain text.
- **`AlertDialogDescription`, `AlertDialogBackdrop`, `AlertDialogPortal`,
  `AlertDialogViewport`, `AlertDialogRoot`**: no role — they stay out of the a11y
  tree (Base UI gives them none either; the description is reached by reading
  inside the alertdialog). Do not use `Role::GenericContainer` (filtered/asserts).

### Actions

- No new `.on_a11y_action(...)` handlers are needed. `Action::Click` is
  auto-registered by the existing `.on_click` on `DialogTrigger` (opens via the
  runtime transition) and `DialogClose` (closes via
  `DialogOpenChangeReason`-carrying close path); `Action::Focus` is auto-registered
  by the existing `.track_focus`/`.focusable()` on trigger, popup, and close.
  Escape dismissal already flows through the popup key handler
  (`DialogOpenChangeReason::EscapeKey`); AccessKit has no dismiss action to add in
  this revision.

### Labels

- Add an `aria_label` builder prop to `DialogTrigger`/`DialogClose` (forwarded by
  `AlertDialogTrigger`), used when the visible child is an icon or non-text.
- When a trigger/close label is visible text rendered by the component itself, emit
  it with `Text::new_inaccessible(...)` instead of `text!(...)` so the string is
  not announced twice (once as child text node, once as `.aria_label`).
- Popup label: since `aria-labelledby` cannot be expressed (see Gaps), pass the
  title string to the popup as `.aria_label(...)`; the `DialogTitle` text itself
  then remains a normal accessible heading (do **not** make the title
  inaccessible — it is a separate heading node, not a duplicate of the popup
  label announcement style used for buttons).

### Gaps (no gpui builder in this revision)

- **`aria-haspopup="dialog"`** on the trigger: no builder. Fallback: omit;
  `Role::Button` + `.aria_expanded` is the best available signal. Document.
- **`aria-controls` (trigger → popup id)**: no relationship builders. Fallback:
  omit + document; blocked pending gpui upstream id-reference support.
- **`aria-labelledby` / `aria-describedby` (popup → Title/Description ids)**: no
  builders. Fallback: literal `.aria_label(title_text)` on the popup for the
  label; the description has no equivalent (`set_description` is not exposed) —
  omit + document; blocked pending gpui upstream.
- **`disabled` / `aria-disabled` on trigger/close**: no `.aria_disabled(...)`
  builder and `write_a11y_info` never sets a disabled flag. Fallback: the existing
  `disabled` field already suppresses `on_click` and `tab_stop`, so the node has no
  Click action while disabled; document that AT will not see an explicit disabled
  state.
- **`aria-hidden` on outside elements while modal** (`useDialogRoot.ts` scroll/hide
  handling): no per-element hidden builder; the focus trap is the only modality
  signal. Omit + document.
- No live-region needs: Alert Dialog relies on the `AlertDialog` role itself, not
  `aria-live`.

### Checklist

- [ ] Add a crate-private `role` knob to `DialogPopup` (default `Role::Dialog`) and
      pin `Role::AlertDialog` for the alert-dialog composition.
- [ ] `DialogPopup`: `.role(...)` + `.aria_label(title_text)` on the focus-trapped
      element.
- [ ] `DialogTrigger`: `.role(Role::Button)` +
      `.aria_expanded(open && active_trigger)` + optional `.aria_label` prop,
      forwarded by `AlertDialogTrigger`.
- [ ] `DialogClose`: `.role(Role::Button)` + optional `.aria_label` prop.
- [ ] `DialogTitle`: `.role(Role::Heading)` + `.aria_level(2)`.
- [ ] Swap component-rendered visible button label text to
      `Text::new_inaccessible(...)` where `.aria_label` is set on the parent.
- [ ] Confirm no `Action::Click`/`Action::Focus` re-registration (rely on existing
      `on_click`/`track_focus`).
- [ ] Document the omitted `aria-haspopup`/`aria-controls`/`aria-labelledby`/
      `aria-describedby`/`aria-disabled` attributes as gpui gaps in the module docs.

## Uncertain items needing confirmation

- Reuse strategy: compose (option 1) vs Dialog variant flag (option 2). Default to
  compose unless handle re-pinning forces option 2.
- Whether `AlertDialogHandle<P>` should be a newtype wrapping `DialogHandle<P>` or a
  separate struct binding an `AlertDialogContext`. Prefer the smallest thing that
  makes the two handle types non-interchangeable while reusing Dialog's bind logic.
