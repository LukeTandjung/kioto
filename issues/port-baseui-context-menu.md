# Port Base UI Context Menu to GPUI

## Problem

Base UI's Context Menu is **not a new interaction model** — it is the Menu opened
at the cursor by a right-click (or touch long-press) instead of by a regular
trigger. In Base UI only two parts are Context-Menu-specific:

- `ContextMenuRoot` (`context-menu/root/ContextMenuRoot.tsx`) renders no element of
  its own. It creates a **virtual cursor-point anchor** (a zero-size rect updated on
  every open gesture), holds shared refs (`initialCursorPointRef`,
  `allowMouseUpTriggerRef`, `actionsRef`, `positionerRef`, backdrop refs, `rootId`),
  and renders `Menu.Root` inside a `ContextMenuRootContext` — which makes the inner
  Menu resolve its parent as `{ type: 'context-menu' }`.
- `ContextMenuTrigger` (`context-menu/trigger/ContextMenuTrigger.tsx`) is an **area**
  (a plain `<div>`, not a button) that opens the menu **at the cursor position** on
  `contextmenu` (right-click, reason `triggerPress`) or a ~500ms touch long-press,
  then arms a post-open **mouseup grace period** so the release of the very gesture
  that opened the menu does not immediately dismiss it.

Every other part — Backdrop, Portal, Positioner, Popup, Arrow, Group, GroupLabel,
Item, LinkItem, CheckboxItem(+Indicator), RadioGroup, RadioItem(+Indicator),
SubmenuRoot, SubmenuTrigger, Separator — is a **literal re-export of the Menu
parts** (`context-menu/index.parts.ts`). The context-menu-specific behavior of
those parts lives inside Menu behind `parent.type === 'context-menu'` branches:
always-modal, cursor-anchor positioner defaults (align start, `sideOffset ≈ -5`,
`alignOffset ≈ 2`), suppressed trigger hover/click/arrow-key-open interactions,
right-button drag-release item activation, and the outside-press grace period.

This makes the port a **thin variant** over the Menu port, exactly like
`issues/port-baseui-alert-dialog.md` layers on Dialog. **Hard dependency:**
`issues/port-baseui-menu.md` must land first — it reserves the
`MenuParentKind::ContextMenu` seam (storage, injection point, branch sites) that
this issue wires on. Cross-link both ways; do not fork or duplicate any Menu
behavior here.

Trigger payloads stay generic via Menu's `P: Clone + 'static`; radio values via
`V: Clone + Eq + 'static`. No new primitive is required: right-click detection is
GPUI `div().on_mouse_down(MouseButton::Right, ...)` (see the analog in
`/home/luke/Projects/gpui-component/crates/ui/src/menu/context_menu.rs`), the
cursor anchor is `anchored().position(point)` (already used by
`crates/base_gpui/src/select/layers/select_positioner.rs`), and the grace period is
a ~500ms timer layered on the outside-press handling in
`crates/base_gpui/src/utils/overlay.rs`.

## Scope

Add a GPUI-native Context Menu that composes the existing Menu implementation
under `crates/base_gpui/src/context_menu/`.

New context-menu-specific parts:

- `ContextMenuRoot<P>` — wraps/builds a `MenuRoot<P>` with
  `MenuParentKind::ContextMenu`; owns no element; exposes the Menu root surface
  **minus** the hover/modal props Base UI omits (see API criteria).
- `ContextMenuTrigger<P>` — the right-click area `div`; records the cursor point,
  commands the menu open at that point, arms the mouseup grace period.

Reused Menu parts, re-exported under Context Menu names (no reimplementation —
`pub use crate::menu::MenuX as ContextMenuX;` matching Base UI's literal
re-exports in `index.parts.ts`):

- `ContextMenuBackdrop<P>` = `MenuBackdrop<P>`
- `ContextMenuPortal<P>` = `MenuPortal<P>`
- `ContextMenuPositioner<P>` = `MenuPositioner<P>`
- `ContextMenuPopup<P>` = `MenuPopup<P>`
- `ContextMenuArrow<P>` = `MenuArrow<P>`
- `ContextMenuGroup<P>` = `MenuGroup<P>`
- `ContextMenuGroupLabel<P>` = `MenuGroupLabel<P>`
- `ContextMenuItem<P>` = `MenuItem<P>`
- `ContextMenuLinkItem<P>` = `MenuLinkItem<P>`
- `ContextMenuCheckboxItem<P>` = `MenuCheckboxItem<P>`
- `ContextMenuCheckboxItemIndicator<P>` = `MenuCheckboxItemIndicator<P>`
- `ContextMenuRadioGroup<P, V>` = `MenuRadioGroup<P, V>`
- `ContextMenuRadioItem<P, V>` = `MenuRadioItem<P, V>`
- `ContextMenuRadioItemIndicator<P, V>` = `MenuRadioItemIndicator<P, V>`
- `ContextMenuSubmenuRoot<P>` = `MenuSubmenuRoot<P>`
- `ContextMenuSubmenuTrigger<P>` = `MenuSubmenuTrigger<P>`
- `ContextMenuSeparator` = `MenuSeparator` (itself backed by
  `base_gpui::separator::Separator`)

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/context-menu/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/context-menu/index.parts.ts` (the re-export map above)
- `/home/luke/Projects/base-ui/packages/react/src/context-menu/root/ContextMenuRoot.tsx` (virtual anchor, shared refs, `Menu.Root` wrapping, prop omissions)
- `/home/luke/Projects/base-ui/packages/react/src/context-menu/root/ContextMenuRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/context-menu/root/ContextMenuRoot.test.tsx` (drag-release over items, cursor-point mouseup suppression, disabled root, collision flip at cursor anchor)
- `/home/luke/Projects/base-ui/packages/react/src/context-menu/root/ContextMenuRoot.non-mac.test.tsx` (platform difference: `contextmenu` fires on mousedown on macOS, on mouseup elsewhere)
- `/home/luke/Projects/base-ui/packages/react/src/context-menu/trigger/ContextMenuTrigger.tsx` (right-click open at cursor, 500ms `LONG_PRESS_DELAY` for both touch long-press and the mouseup grace window, once-only document mouseup close with reason `cancelOpen`)
- `/home/luke/Projects/base-ui/packages/react/src/context-menu/trigger/ContextMenuTrigger.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/context-menu/trigger/ContextMenuTriggerDataAttributes.ts` (`popupOpen`, `pressed`)

Menu source files containing the `parent.type === 'context-menu'` branches this
issue activates (behavioral reference for the seams, ported by the Menu issue):

- `/home/luke/Projects/base-ui/packages/react/src/menu/root/MenuRoot.tsx` (`useMenuParent` resolution, `allowOutsidePressDismissalRef` 500ms grace, `openOnArrowKeyDown: false`, imperative `setOpen`/positioner handles for the context)
- `/home/luke/Projects/base-ui/packages/react/src/menu/store/MenuStore.ts` (`modal` selector: context-menu parents are modal by default with no way to opt out)
- `/home/luke/Projects/base-ui/packages/react/src/menu/positioner/MenuPositioner.tsx` (cursor anchor adoption, align `start`, `alignOffset` 2 / `sideOffset` -5 defaults, fixed position method, internal backdrop **without** a trigger cutout)
- `/home/luke/Projects/base-ui/packages/react/src/menu/popup/MenuPopup.tsx` (`returnFocus`/modal focus-manager behavior for context menus, in-popup hover close disabled)
- `/home/luke/Projects/base-ui/packages/react/src/menu/trigger/MenuTrigger.tsx` (hover/click interactions disabled under a context-menu parent)
- `/home/luke/Projects/base-ui/packages/react/src/menu/item/useMenuItemCommonProps.ts` (mouseup-at-initial-cursor-point ±1px suppression, non-mac right-button mouseup suppression, right-button drag-release activation)
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/context-menu/` (docs/demos)

Existing GPUI implementation to reuse (do not fork/duplicate):

- `crates/base_gpui/src/menu/` — the entire Menu port from
  `issues/port-baseui-menu.md`, especially the `MenuParentKind::ContextMenu` seams
  in its runtime/positioner/trigger/item behavior.
- `crates/base_gpui/src/utils/overlay.rs` — outside-press/modal occlusion; the
  post-open grace check layers on top of it in the ContextMenu parent branch.
- `crates/base_gpui/src/select/layers/select_positioner.rs` —
  `anchored().position(point)` precedent for anchoring at an arbitrary point.
- Analog for right-click + point-anchored menus:
  `/home/luke/Projects/gpui-component/crates/ui/src/menu/context_menu.rs`.

Current GPUI implementation:

- No `crates/base_gpui/src/context_menu/` module exists yet.

Expected new GPUI files (flat layout per
`docs/base-gpui-component-architecture.md`; thin, mirroring `alert_dialog/`):

```text
crates/base_gpui/src/context_menu/
  mod.rs                          # barrel only: module decls + re-exports of Menu
                                  # parts under ContextMenu names (pub use ... as ...)
  style_state.rs                  # ContextMenuTriggerStyleState (+ root style state if it draws nothing, may be omitted)
  layers/
    mod.rs                        # barrel only
    context_menu_root.rs          # ContextMenuRoot<P>
    context_menu_trigger.rs       # ContextMenuTrigger<P>
  tests/
```

No `runtime.rs` / `context.rs` of its own: the cursor anchor point, the initial
cursor point, and the grace-deadline facts belong to the **Menu runtime** behind
its `MenuParentKind::ContextMenu` branch (commands in Menu domain language, e.g.
`open_at_cursor(point, details)`), reachable through the existing `MenuContext<P>`.
If implementation shows this bloats the Menu runtime, a minimal
`context_menu/context.rs` holding only the cursor-point/grace facts is acceptable —
decide during implementation and note it in the PR.

Also update:

- `crates/base_gpui/src/lib.rs` (`pub mod context_menu;` + `context_menu::init(cx);`
  if any bindings/actions are registered — likely none beyond Menu's).
- `crates/base_gpui/src/main.rs` with a Context Menu demo (right-click area with
  items, separator, and a submenu).

## Design decisions

- **Thin variant, hard Menu dependency.** Everything except Root/Trigger is a
  re-export. All context-menu conditional behavior lives inside `menu/` behind
  `MenuParentKind::ContextMenu` — this issue *activates* those branches; it must
  not reimplement highlight, typeahead, submenu trees, positioning, or dismissal.
- **Cursor-point virtual anchor.** `ContextMenuTrigger` records the right-click
  window position and the positioner anchors with `anchored().position(point)`
  instead of measured trigger bounds. The point is refreshed on every open gesture
  (re-right-click while open re-anchors, matching Base UI's `setAnchor`).
- **Right-click gesture.** GPUI has no `contextmenu` event; use
  `on_mouse_down(MouseButton::Right, ...)` on the trigger area. Base UI's
  mac-vs-non-mac `contextmenu` timing difference (mousedown vs mouseup) collapses
  into this single mousedown-based gesture; the non-mac "the gesture's own
  right-button mouseup must not activate an item" rule still applies and is
  handled by the grace/initial-point rules below.
- **Post-open mouseup grace.** On open, start a ~500ms timer. A window mouse-up
  before the deadline never closes the menu (it is the tail of the opening
  gesture). After the deadline, a mouse-up **outside** the open tree (any open
  popup of the tree, per Menu's tree hit-testing) closes with reason `CancelOpen`;
  a mouse-up inside the tree is handled by item drag-release activation. A
  mouse-up within ±1px of the initial cursor point never activates the item that
  happened to spawn under the cursor.
- **Touch long-press deferred.** Base UI's 500ms touch long-press (10px move
  threshold, 10×10 anchor rect) has no GPUI desktop equivalent; document as
  deferred until GPUI exposes touch pointer metadata. The `WebkitTouchCallout`
  suppression and document-level `contextmenu` `preventDefault` are web-only and
  dropped.
- **Distinct part types only where behavior differs.** `ContextMenuRoot<P>` and
  `ContextMenuTrigger<P>` are new types; the rest are `pub use` renames — Base UI
  itself re-exports the identical components, so no Alert-Dialog-style type
  branding is needed for the shared parts.

## Out of scope / drop from Base UI

- Reimplementing any Menu behavior (navigation, typeahead, submenu tree,
  positioning, portal, backdrop, checkbox/radio state) — reuse `menu/` wholesale.
- React context/hooks (`ContextMenuRootContext`, `useContextMenuRootContext`,
  shared refs like `backdropRef` / `internalBackdropRef` / `positionerRef` /
  `actionsRef`) — GPUI runtime facts behind `MenuParentKind::ContextMenu` instead
  of DOM ref plumbing.
- `render` props, `className`, web `style` props (including `WebkitTouchCallout`
  / `userSelect` inline styles), CSS variable APIs.
- DOM data attributes (`data-popup-open`, `data-pressed`) — typed
  `ContextMenuTriggerStyleState` fields.
- Touch long-press open, touch move-threshold cancel, and `allowTouchToClose`
  guards — deferred (no GPUI touch metadata); keep the API shaped so a long-press
  source can be added later.
- Document-level `contextmenu` listener that `preventDefault`s the native browser
  menu over trigger/backdrops — no native context menu to suppress in GPUI.
- `ownerDocument` / `addEventListener` once-only mouseup mechanics — translate to
  GPUI window mouse-up observation.
- Detached triggers / handles: Base UI's context menu root does not support them
  ("doesn't support detached triggers yet"); do not expose `.handle(...)`,
  `.trigger_id(...)`, or `.default_trigger_id(...)` on `ContextMenuRoot<P>`.
- ARIA (`role="menu"` tree semantics) and DOM id linking — inherited from the Menu
  issue's AccessKit follow-up; nothing context-menu-specific to add now.
- Do not use Rust scoped visibility syntax (`pub(...)`); `ast-grep scan` must stay
  clean.

## Acceptance Criteria

New issue — items are unchecked. Items marked **(inherited)** are Menu behaviors
that must keep working through the Context Menu names; verify, do not reimplement.

### Module / API surface

- [x] `crates/base_gpui/src/context_menu/` exists with the flat layout above; `context_menu/mod.rs` and `context_menu/layers/mod.rs` are barrel-only.
- [x] `base_gpui::context_menu` is registered in `crates/base_gpui/src/lib.rs`.
- [x] `ContextMenuRoot<P>` builder exists with `P: Clone + 'static`; it renders no element of its own (children pass through to the wrapped Menu root tree).
- [x] `ContextMenuRoot<P>` exposes the `MenuRoot` surface **minus** `modal`, `open_on_hover`, `delay`, `close_delay`, `handle`, `trigger_id`, and `default_trigger_id`: `.id(...)`, `.default_open(bool)`, `.open(bool)`, `.on_open_change(...)`, `.on_open_change_complete(...)`, `.disabled(bool)`, `.loop_focus(bool)`, `.orientation(...)`, `.close_parent_on_esc(bool)`, `.highlight_item_on_hover(bool)`, children.
- [x] `ContextMenuRoot<P>` does **not** expose `modal` (context menus are unconditionally modal) or any hover-open configuration.
- [x] `ContextMenuTrigger<P>` builder exists: an area `div` supporting `.id(...)`, arbitrary visual children, and `.style_with_state(...)`; it has no `disabled` prop of its own (disabling is the root's `.disabled(bool)`, parity with Base UI).
- [x] `context_menu/mod.rs` re-exports the full Menu part list under Context Menu names: `ContextMenuBackdrop`, `ContextMenuPortal`, `ContextMenuPositioner`, `ContextMenuPopup`, `ContextMenuArrow`, `ContextMenuGroup`, `ContextMenuGroupLabel`, `ContextMenuItem`, `ContextMenuLinkItem`, `ContextMenuCheckboxItem`, `ContextMenuCheckboxItemIndicator`, `ContextMenuRadioGroup`, `ContextMenuRadioItem`, `ContextMenuRadioItemIndicator`, `ContextMenuSubmenuRoot`, `ContextMenuSubmenuTrigger`, `ContextMenuSeparator` — as `pub use` re-exports of the `menu` parts, not reimplementations.
- [x] `ContextMenuTriggerStyleState` exists in `style_state.rs` with `open` and `pressed` facts; `.style_with_state(...)` on the trigger takes it.
- [x] No `pub(...)` scoped visibility anywhere in `context_menu/`; payload generics stay `P: Clone + 'static` (and `V: Clone + Eq + 'static` via the radio re-exports) with no extra bounds.

### Correctness / compile readiness

- [x] `cargo fmt --check` passes.
- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui context_menu` passes.
- [x] `cargo test -p base_gpui` passes.
- [x] `cargo clippy -p base_gpui --all-targets` exits successfully with only pre-existing warnings.
- [x] `ast-grep scan crates/base_gpui/src/context_menu` passes (including barrel-only `mod.rs` rule).
- [x] No duplication of Menu runtime/context/layer logic; `git grep` shows Context Menu layers delegating to `menu::` types.
- [x] Demo in `crates/base_gpui/src/main.rs`: a right-click surface opening a menu with items, a separator, a checkbox item, and a submenu; gallery render test passes with the menu initially closed.

### Right-click / cursor-anchor open behavior

- [x] Right mouse-down (`MouseButton::Right`) inside the trigger area opens the menu with reason `TriggerPress`, anchored at the exact cursor window position via a zero-size virtual anchor (`anchored().position(point)` semantics), not at the trigger's bounds.
- [x] The initial cursor point is recorded in the runtime on every open gesture; right-clicking at a new position while open re-anchors the menu at the new point (single open state, updated anchor).
- [x] Positioner defaults under the ContextMenu parent kind: align `start`, and when no explicit side is set (and align is not `center`), `align_offset` defaults to `2` and `side_offset` to `-5`, so the popup's corner sits at/under the cursor; explicit positioner props override these defaults.
- [x] Root `.disabled(true)` makes right-click a no-op: menu does not open and `on_open_change` never fires.
- [x] ArrowDown/ArrowUp never open the menu from the trigger (Menu's `openOnArrowKeyDown` suppression under the ContextMenu parent kind); the trigger area is not a focusable button.
- [x] Left-click and hover on the trigger area do nothing (no hover-open, no click-open, no click-toggle).
- [x] Collision avoidance still applies to the cursor anchor: with no space on the default side the popup flips (Menu positioner behavior at a point anchor).
- [x] Touch long-press open is documented as deferred (no runtime panic path; the open command is gesture-source-agnostic so long-press can be added later).

### Post-open outside-press / mouseup grace behavior

- [x] Opening arms a ~500ms grace timer in the ContextMenu parent branch (grace state lives with Menu's outside-press handling per `utils/overlay.rs` layering, not in the trigger layer's render state).
- [x] A window mouse-up before the grace deadline never closes the menu (the release of the opening right-click is inert).
- [x] After the grace deadline, a mouse-up outside the open tree (union of all open popups plus nothing else — there is no button trigger) closes with reason `CancelOpen`.
- [x] A mouse-up within ±1px of the initial cursor point does not activate the item that spawned under the cursor and does not close the menu; the recorded initial point is consumed after the first mouse-up check.
- [x] After real cursor movement away from the initial point, a right-button mouse-up over an enabled item activates it via drag-release and closes the whole tree with reason `ItemPress` (overriding `close_on_click`), including closing open submenus. **(inherited drag-release, right-button variant)**
- [x] Outside-press (mouse-down based) dismissal against the open tree still works after open: clicking empty space closes with reason `OutsidePress`; clicking inside any open popup of the tree does not. **(inherited)**
- [x] Closing clears grace state and the initial cursor point; a subsequent open re-arms both deterministically (no stale timers, no panics on rapid open/close).

### Reuse of Menu behavior via `MenuParentKind::ContextMenu`

- [x] `ContextMenuRoot<P>` constructs the Menu tree with `MenuParentKind::ContextMenu`; no context-menu conditionals are added to `context_menu/` layers that Menu already branches on.
- [x] The menu is unconditionally modal (Menu's modal selector treats the ContextMenu parent as modal-by-default with no opt-out), and the internal modal backdrop has **no trigger cutout** (unlike standalone menus — there is no button trigger to keep clickable).
- [x] Escape closes and focus/highlight behavior, submenu trees (hover-open delay, ArrowLeft/ArrowRight, sibling-close, parent-close cascade), typeahead, checkbox/radio state, groups/labels/separators, portal keep-mounted, arrow, and backdrop behavior all work unchanged through the Context Menu re-exports. **(inherited)**
- [x] Submenus inside a context menu use the ContextMenu-rooted tree for outside-press hit-testing and close cascades (Menu's tree logic keyed off the root parent kind). **(inherited)**
- [x] `on_open_change` details flow through with Menu's reason set (`TriggerPress` open, `OutsidePress` / `EscapeKey` / `ItemPress` / `CancelOpen` closes); controlled `.open(...)` and uncontrolled `.default_open(...)` precedence and cancellation semantics are Menu's. **(inherited)**
- [x] The trigger's `pressed`/`open` style facts come from a runtime query (part-shaped: "is my menu open?"), not from local trigger state.
- [x] Any seam this issue needs that `menu/` does not yet provide (cursor-anchor command, grace state, positioner defaults, arrow-key-open suppression, modal-without-cutout) is added to `menu/` under its ContextMenu branch and checked off against `issues/port-baseui-menu.md`'s reserved-seam criteria — not implemented as a `context_menu/` special case.

### Tests / verification

Runtime tests (no window, driving the Menu runtime with `MenuParentKind::ContextMenu`):

- [x] Open-at-cursor command records the anchor point and initial cursor point; reopening at a new point re-anchors.
- [x] Grace window: mouse-up before deadline is inert; after deadline, outside mouse-up produces a `CancelOpen` close outcome; inside-tree mouse-up does not.
- [x] ±1px initial-cursor-point mouse-up suppression, consumed after first check; movement beyond the threshold re-enables drag-release activation.
- [ ] Disabled root ignores the open command; no callback outcome. — *not unit-tested: the disabled fact lives in `MenuProps`/`MenuContext`, which requires a window; enforced in `ContextMenuTrigger` and `MenuContext::set_open`.*
- [ ] Modal fact is true regardless of configuration under the ContextMenu parent kind; positioner default side/align/offset facts match the ContextMenu branch. — *implemented in `menu_positioner.rs` render branches, which are not unit-reachable without a window; no runtime-level fact to assert.*

Rendered tests under `crates/base_gpui/src/context_menu/tests/`:

*Deferred: the menu family has no windowed pointer-simulation harness (menu/menubar ship runtime-only tests); behavior is covered by the runtime tests above plus the green menu suite through the re-exports.*

- [ ] Right-click on the trigger area opens the popup anchored at the cursor position (assert positioner placement derives from the click point, not trigger bounds).
- [ ] Right-click at a second position while open moves the popup to the new point.
- [ ] Immediate mouse-up after right-click (same frame / before grace deadline) leaves the menu open with exactly one `on_open_change` call.
- [ ] Mouse-up over the item under the cursor at open time (within ±1px) does not activate it; after moving the cursor, right-button release over an item activates and closes the whole tree (submenu included) with reason `ItemPress`.
- [ ] After the grace period, click outside closes (`OutsidePress`); mouse-up outside closes (`CancelOpen`); click inside a nested submenu popup does not dismiss ancestors.
- [ ] Escape closes; keyboard navigation, typeahead, and submenu ArrowRight/ArrowLeft work through the `ContextMenu*` re-exported parts.
- [ ] Left-click and hover on the trigger never open; disabled root never opens and fires no callbacks.
- [ ] Trigger `style_with_state` reflects `open`/`pressed` transitions.
- [x] Demo renders in `crates/base_gpui/src/main.rs` without panics.

## Follow-ups to track explicitly if not completed in the first pass

- [ ] Touch long-press open (500ms, 10px move-cancel threshold, 10×10 touch anchor rect) once GPUI exposes touch/pointer-type metadata.
- [ ] AccessKit semantics — nothing beyond the Menu issue's follow-up; confirm the context menu popup inherits the Menu role mapping when that lands.
- [x] If the Menu port deferred any ContextMenu seam (grace state slot, cursor-anchor positioner defaults, arrow-key-open suppression), land those in `menu/` as part of this issue and update `issues/port-baseui-menu.md`'s checklist.

## Uncertain items needing confirmation

- **Where grace/cursor facts live**: proposed in the Menu runtime behind
  `MenuParentKind::ContextMenu` (keeps one deep module); alternative is a tiny
  `context_menu/context.rs` carrying only cursor-point/grace facts. Confirm during
  implementation against how the Menu port shaped its parent-kind storage.
- **Right-button vs any-button drag-release**: Base UI requires `button === 2` for
  context-menu drag-release activation and suppresses non-mac right-button mouseups;
  confirm GPUI's right-drag event stream makes the same distinction observable, or
  simplify to "the button that opened the menu".
- **Mac vs non-mac gesture timing**: Base UI has separate mac/non-mac tests because
  `contextmenu` fires on mousedown (mac) vs mouseup (elsewhere). The GPUI port opens
  on right mouse-down everywhere; confirm this uniform behavior is acceptable on all
  desktop platforms (matching the gpui-component analog).
- **Trigger focusability**: Base UI's trigger is a plain non-focusable `div`.
  Confirm the GPUI trigger area should stay out of the tab order (no `FocusHandle`),
  with keyboard access to the menu only once it is open.
