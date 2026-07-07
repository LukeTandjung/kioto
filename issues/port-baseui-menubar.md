# Port Base UI Menubar to GPUI

## Problem

Base UI Menubar is a **single-part coordinator over Menu**: a `role="menubar"`
container that wraps a `CompositeRoot` giving roving arrow-key focus across the
`Menu.Trigger` elements rendered inside it (horizontal or vertical orientation,
`loopFocus`, Home/End), and hosts a `FloatingTree` that tracks whether any child
menu is open (`hasSubmenuOpen`). Once one menu is open, hovering a sibling trigger
switches the open menu, focusing a sibling trigger opens its menu, and perpendicular
arrow keys pressed inside an open menu popup relay back to the menubar to move the
roving highlight and open the neighboring menu. Menubar exports **no menu parts of
its own** — the menus inside are ordinary `Menu.Root` / `Menu.Trigger` composition
that detect the menubar parent.

The critical architectural fact: most menubar *behavior* does not live in
`Menubar.tsx` (169 lines). It lives inside `MenuRoot` / `MenuTrigger` /
`MenuPositioner` / `MenuPopup` branches keyed on `parent.type === 'menubar'` —
hover-switch between sibling menus, click-vs-mousedown toggle semantics, the
menubar-wide modal backdrop cutout, positioner side defaults, the `group` instant
type, the keyboard relay, and disabled propagation. The Menu port
(`issues/port-baseui-menu.md`) reserves these as `MenuParentKind::Menubar` seams;
this issue is the one that implements them, plus the container itself: a
`Menubar` root with a shared menubar runtime/context consumed by the child menus'
triggers, and roving focus across the trigger row.

`crates/base_gpui` has no menubar module. The goal is behavioral parity using
GPUI-native architecture (one deep `MenubarRuntime`, thin `MenubarContext`, one
renderable layer) — not a port of React context, `FloatingTree` events,
`CompositeRoot` DOM event delegation, ARIA attributes, or data attributes.

## Scope

Port the Menubar component from Base UI into a GPUI-native component:

- `Menubar` — the single public part (container + trigger-row roving focus +
  cross-menu coordination state).

This issue also **implements the `MenuParentKind::Menubar` branches inside
`crates/base_gpui/src/menu/`** that `issues/port-baseui-menu.md` reserves as seams
(enumerated below under "Menu seams this issue consumes"). It adds no other Menu
parts and no new shared primitives.

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/menubar/Menubar.tsx` (container,
  `CompositeRoot` wiring, `hasSubmenuOpen` tracking via `menuopenchange` from direct
  children, ignoring `sibling-open` / `list-navigation` close reasons)
- `/home/luke/Projects/base-ui/packages/react/src/menubar/MenubarContext.ts`
  (shared context shape: `modal`, `disabled`, `orientation`, `hasSubmenuOpen`,
  `contentElement` for the backdrop cutout, `allowMouseUpTriggerRef`, `rootId`)
- `/home/luke/Projects/base-ui/packages/react/src/menubar/MenubarDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/menubar/Menubar.test.tsx`
  (the behavioral spec: hover-switch, focus behavior, keyboard interactions,
  `loopFocus`, disabled, outside press, scroll locking)
- `/home/luke/Projects/base-ui/packages/react/src/menu/root/MenuRoot.tsx` (menubar
  parent detection, `group` instant type, `parentOrientation` list-navigation wiring,
  keyboard relay comment at the popup `onKeyDown`)
- `/home/luke/Projects/base-ui/packages/react/src/menu/trigger/MenuTrigger.tsx`
  (`isInMenubar` branches: CompositeItem rendering, hover/focus/click gating on
  `hasSubmenuOpen`, disabled combination, `stickIfOpen` off, mixed toggle handlers)
- `/home/luke/Projects/base-ui/packages/react/src/menu/positioner/MenuPositioner.tsx`
  (menubar side/align defaults, menubar-modal backdrop + whole-menubar cutout,
  scroll lock condition)
- `/home/luke/Projects/base-ui/packages/react/src/menu/popup/MenuPopup.tsx`
  (hover-close disabled for menubar parent, focus-return condition, keyboard relay)
- `/home/luke/Projects/base-ui/packages/react/src/internals/composite/root/CompositeRoot.tsx`
  (roving focus semantics: orientation, `loopFocus`, `enableHomeAndEndKeys`,
  `highlightItemOnHover`, `relayKeyboardEvent`)
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/menubar/page.mdx`
  and `demos/**`

Existing `base_gpui` code to use as **per-component reference** (read, then
implement menubar-locally — the architecture doc forbids extracting shared
primitives for this):

- Roving arrow-key focus across a row of items: `crates/base_gpui/src/tabs/runtime.rs`
  (`move_highlight`, wrap-vs-clamp, Home/End) and
  `crates/base_gpui/src/tabs/layers/tabs_list.rs` (key context + action handlers +
  focus-follows-highlight).
- Cross-menu coordination and open-menu handoff, nearest external analog:
  `/home/luke/Projects/gpui-component/crates/ui/src/menu/app_menu_bar.rs`
  (reference only; do not copy its architecture).
- Menu family (the dependency this issue layers on): `crates/base_gpui/src/menu/`
  once `issues/port-baseui-menu.md` lands.

Current GPUI implementation:

- No `crates/base_gpui/src/menubar/` module exists yet.

Expected GPUI implementation files (flat layout per
`docs/base-gpui-component-architecture.md`):

```text
crates/base_gpui/src/menubar/
  mod.rs            # barrel only
  actions.rs        # trigger-row arrow/Home/End key dispatch (menubar key context)
  runtime.rs        # MenubarRuntime: trigger registration, roving highlight,
                    # has_submenu_open, open-menu link, measured menubar bounds
  context.rs        # MenubarContext (read/update + the menubar commands)
  props.rs          # orientation, loop_focus, modal, disabled
  style_state.rs    # MenubarStyleState
  child.rs          # typed child routing for menubar children
  child_wiring.rs   # private traversal: index triggers, attach menubar context
  layers/
    mod.rs          # barrel only
    menubar.rs      # the single renderable part
  tests/
```

Plus targeted edits inside `crates/base_gpui/src/menu/` (runtime/trigger/positioner/
popup/root) to implement the reserved `MenuParentKind::Menubar` branches — no
restructuring of the Menu module.

Also update:

- `crates/base_gpui/src/lib.rs` (`pub mod menubar;` + `menubar::init(cx);`)
- `crates/base_gpui/src/main.rs` with a Menubar demo (3+ menus, one with a submenu)

## Menu seams this issue consumes (dependency contract)

**This issue is blocked on `issues/port-baseui-menu.md`** landing with the
`MenuParentKind::Menubar` seams in place (storage, injection point, branch sites —
constructible-but-unimplemented is fine). This issue then fills in the branches.
The exact seams, keyed to the Base UI source lines they translate:

1. **Parent detection / injection**: a `MenuRoot` wired under a `Menubar` gets
   `MenuParentKind::Menubar` with a link to the menubar context/runtime entity
   (`MenuRoot.tsx` `useMenubarContext(true)`); submenus inside those menus stay
   `Submenu`.
2. **Trigger as composite item**: in a menubar, `MenuTrigger` registers with the
   *menubar* runtime as a roving-focus participant (trigger index, focus handle,
   disabled) instead of being a standalone tab stop (`MenuTrigger.tsx`
   `CompositeItem` branch).
3. **Hover-switch gating**: trigger `open_on_hover` defaults to the menubar's
   `has_submenu_open`; hover-open is enabled only when `has_submenu_open` is true
   *and* the open menu is not this trigger's own; safe-polygon/hover-intent must not
   block pointer events over the menubar row (`MenuTrigger.tsx` lines 172–181).
4. **Focus-open gating**: focusing a trigger opens its menu only while
   `has_submenu_open` is true (`useFocus` `enabled: parentMenubarHasSubmenuOpen`).
5. **Click toggle semantics**: in a menubar, press-down opens and a full click on
   the already-open trigger closes (mixed toggle, `event: 'click'` when open);
   `stick_if_open` is disabled for menubar triggers (`MenuTrigger.tsx` lines
   190–212).
6. **Open-change notification**: each child menu reports open changes with reason
   to the menubar runtime so it maintains `has_submenu_open` — set true on any
   direct-child open; set false on close **unless** the reason is `SiblingOpen` or
   `ListNavigation` (handoff must not momentarily drop the flag)
   (`Menubar.tsx` `onSubmenuOpenChange`). Only direct children count; nested
   submenu open/close does not touch the flag.
7. **`group` instant type**: when the parent is a menubar and the open-change reason
   is `TriggerFocus`, `FocusOut`, `TriggerHover`, `ListNavigation`, or
   `SiblingOpen`, the menu records instant kind `group` (`MenuRoot.tsx` lines
   303–311).
8. **Keyboard relay seam**: arrow keys pressed inside an open menu popup that the
   menu's own list navigation does not consume (the axis perpendicular to the menu,
   informed by the menubar's `parent_orientation`, when no submenu trigger is
   highlighted) relay to the menubar runtime: move the roving highlight to the
   neighboring trigger and hand the open menu off to it (close reason
   `ListNavigation`, open on the new trigger) (`MenuRoot.tsx` `parentOrientation` +
   `keyboardEventRelay`, `CompositeRoot` `relayKeyboardEvent`).
9. **Positioner defaults**: menubar parent defaults to side bottom (horizontal
   menubar) / inline-end (vertical menubar), align start, direction-aware
   (`MenuPositioner.tsx` lines 104–108).
10. **Menubar-wide modal**: when the menubar is modal, the internal backdrop renders
    whenever a child menu is open — including hover-opened menus — and the cutout is
    the **entire menubar row's bounds**, not just the active trigger, so every
    sibling trigger stays hoverable/clickable through the backdrop; the menu root's
    own `modal` branch is bypassed for menubar parents (`MenuPositioner.tsx` lines
    264–295).
11. **Popup hover-close off**: the in-popup hover close-delay behavior is disabled
    for menubar-parent menus (`MenuPopup.tsx` line 88).
12. **Disabled propagation**: menubar `disabled` combines into every child trigger's
    disabled state (`MenuTrigger.tsx` line 113).
13. **Focus return**: closing a menubar-parent menu returns focus to its trigger
    except when the close reason is `OutsidePress` (`MenuPopup.tsx` line 130).
14. **Shared drag window**: the `allow_mouse_up_trigger` drag-release window is
    shared menubar-wide (`MenubarContext.allowMouseUpTriggerRef`), so
    press-on-trigger → drag onto an item → release activates across the shared
    state.

Cross-link maintenance: when this issue lands, check off the corresponding
"Menubar issue" follow-up entry in `issues/port-baseui-menu.md`.

## Out of scope / drop from Base UI

- All Menu parts and Menu-internal behavior beyond the enumerated menubar branches —
  owned by `issues/port-baseui-menu.md` (including submenu trees, items, typeahead,
  checkbox/radio state).
- React context/hooks (`MenubarContext.Provider`, `useMenubarContext`),
  `FloatingTree` / `FloatingNode` / `menuopenchange` event bus, `CompositeRoot` /
  `CompositeItem` DOM event delegation — GPUI keyed entities + `MenubarContext` +
  explicit runtime commands and the typed relay seam.
- `render` prop, `className`, web `style` props.
- SSR/hydration.
- DOM data attributes (`data-modal`, `data-orientation`, `data-has-submenu-open`) —
  typed `MenubarStyleState` fields.
- ARIA (`role="menubar"`, `aria-orientation`, trigger `role="menuitem"`) and DOM id
  linking (`useBaseUiId` root id) — defer to the AccessKit follow-up; keep
  orientation and menuitem semantics in runtime metadata for future mapping.
- DOM `contentElement` refs and `getBoundingClientRect` — GPUI prepaint bounds
  measurement stored in the runtime (feeds the backdrop cutout).
- Browser scroll locking for touch-opened menubar menus (`useAnchoredPopupScrollLock`)
  — follow whatever the Menu/Popover/Dialog ports established or document the same
  deferral.
- Touch-specific interaction heuristics — deferred with the Menu port's touch
  follow-up.
- Do not use Rust scoped visibility syntax (`pub(...)`); `ast-grep scan` must stay
  clean.

## Acceptance Criteria

### Module / API surface

- [ ] `crates/base_gpui/src/menubar/` exists with the flat architecture layout; `menubar/mod.rs` and `menubar/layers/mod.rs` are barrel-only.
- [ ] `base_gpui::menubar` is exported from `crates/base_gpui/src/lib.rs` and `base_gpui::init(cx)` calls `menubar::init(cx)` to register key bindings.
- [ ] `Menubar` is a public builder supporting `.id(...)`, `.orientation(...)` (default **horizontal** — note: opposite of MenuRoot's vertical default), `.loop_focus(bool)` (default true), `.modal(bool)` (default true), `.disabled(bool)` (default false), plus normal styling builder methods and `.style_with_state(...)`.
- [ ] Menubar children are typed before `AnyElement` erasure in `child.rs`: menu entries (wrapping `MenuRoot<P>` for per-menu payload types — see Uncertain items for the erasure strategy) plus an escape hatch only if Base UI demos show arbitrary non-menu children in the row.
- [ ] `child_wiring.rs` is the only module that walks menubar children, assigns trigger indices, attaches the menubar context/parent-kind to child `MenuRoot`s, and registers trigger focus handles; no index bookkeeping leaks into layers.
- [ ] `MenubarRuntime` (one deep module, unit-testable without a window) owns: orientation/loop/modal/disabled facts, registered trigger metadata (index, disabled, focus handle), roving highlighted-trigger index, `has_submenu_open`, the identity of the currently open child menu, and measured menubar bounds; interface is commands + part-shaped queries in menubar domain language (`sync_triggers`, `move_highlight`, `note_child_open_change`, `hand_off_open_menu`, `set_bounds`, ...) — no getter/setter pairs.
- [ ] `MenubarContext` is `read`/`update` plus the menubar commands; no component vocabulary accretes on any generic context type.
- [ ] Menubar exposes to child menus exactly the Base UI context facts, as runtime queries/commands: modal, disabled, orientation, `has_submenu_open`, menubar bounds (cutout), the shared `allow_mouse_up_trigger` window, and the keyboard-relay command.
- [ ] No new shared/generic primitives are extracted; roving focus is implemented menubar-locally per the Tabs reference (`crates/base_gpui/src/tabs/runtime.rs`, `crates/base_gpui/src/tabs/layers/tabs_list.rs`).
- [ ] No `pub(...)` scoped visibility anywhere in `menubar/` or the Menu edits.

### Correctness / compile readiness

- [ ] `cargo fmt --check` passes.
- [ ] `cargo check -p base_gpui` passes.
- [ ] `cargo test -p base_gpui menubar` passes.
- [ ] `cargo test -p base_gpui` passes (including the existing Menu tests, unbroken by the seam implementations).
- [ ] `cargo clippy -p base_gpui --all-targets` exits successfully with only pre-existing warnings.
- [ ] `ast-grep scan crates/base_gpui/src/menubar crates/base_gpui/src/menu` passes (barrel-only `mod.rs` rule included).
- [ ] Demo in `crates/base_gpui/src/main.rs`: a menubar with 3+ menus (one containing a submenu, one with a disabled trigger); gallery render test passes with all menus initially closed.

### Roving focus across triggers

- [ ] Trigger-row navigation uses GPUI key-dispatch actions and a menubar key context (`actions.rs`), not raw `on_key_down`.
- [ ] Horizontal orientation: ArrowRight/ArrowLeft move the roving highlight/focus to next/previous trigger (direction-aware via `utils::direction` — flipped in RTL); vertical orientation uses ArrowDown/ArrowUp.
- [ ] Home moves highlight/focus to the first trigger; End to the last.
- [ ] `.loop_focus(true)` wraps at both ends; `.loop_focus(false)` clamps.
- [ ] Exactly one trigger is the tab-stop at a time (roving semantics); disabled triggers follow the composite roving behavior so a menubar whose first trigger is disabled remains keyboard-reachable.
- [ ] Focusing a trigger (Tab or arrow roving) does **not** open its menu while no menu is open.
- [ ] Pointer hover moves the roving highlight across triggers only while a menu is open (`highlightItemOnHover: hasSubmenuOpen` parity).
- [ ] Menubar `.disabled(true)` makes every child trigger disabled (no pointer or keyboard opening) while the row itself stays focusable/reachable.

### Cross-menu hover-switch

- [ ] With no menu open, hovering a trigger does not open anything.
- [ ] With one menu open, hovering a sibling trigger closes the open menu (reason `SiblingOpen` on the closing side / hover-open on the opening side per the Menu port's details contract) and opens the hovered trigger's menu — including when the open menu currently has a nested submenu open (the whole branch closes).
- [ ] Hovering the trigger of the already-open menu does not close/reopen it.
- [ ] `has_submenu_open` is maintained from direct-child open changes only: set on open; cleared on close except for reasons `SiblingOpen` and `ListNavigation`, so hover/keyboard handoff between menus never momentarily clears it; nested submenu open/close does not affect it.
- [ ] With one menu open, focusing a sibling trigger (mouse or keyboard roving) opens its menu (focus-open gating, seam 4).
- [ ] Press-down on a trigger opens its menu; a full click on the trigger of the already-open menu closes it (mixed toggle, seam 5); menubar triggers never apply patient-click `stick_if_open`.
- [ ] Hover-intent between the trigger row and an open popup does not block pointer events over the menubar row (safe-polygon `blockPointerEvents: false` parity), so rapid hover across triggers switches menus deterministically without dead zones.
- [ ] Outside press closes the entire open tree (menu + submenus) in one press and clears `has_submenu_open`.
- [ ] Modal menubar: the backdrop renders whenever a child menu is open (hover-opened included) with a cutout over the whole menubar row so sibling triggers stay interactive through it; `.modal(false)` renders no menubar backdrop.

### Keyboard nav into open menu

- [ ] With a closed menu and a focused trigger, ArrowDown (horizontal menubar) opens the menu and highlights its first item; Space/Enter open it likewise (behavior owned by the Menu trigger, exercised here under the menubar parent).
- [ ] Inside an open menu, ArrowDown/ArrowUp navigate items per the Menu port, unaffected by the menubar's horizontal axis.
- [ ] Perpendicular arrow keys inside an open menu relay to the menubar (seam 8): in a horizontal menubar, ArrowRight with no highlighted submenu trigger moves to the next trigger and opens its menu with reason `ListNavigation`; ArrowLeft at top level moves to the previous trigger's menu; RTL flips both; the vertical-menubar case uses ArrowUp/ArrowDown as the relay axis.
- [ ] The relay never fires when the menu consumes the key itself: ArrowRight on a highlighted submenu trigger opens the submenu (not the sibling menu); ArrowLeft inside an open submenu closes the submenu and returns to its trigger (not the sibling menu).
- [ ] Keyboard navigation works after opening a menu by mouse click, and clicking one trigger then arrowing to another menu works (mixed mouse/keyboard parity).
- [ ] Escape closes the open menu and returns focus to its trigger; the menubar roving position stays on that trigger; focus return is skipped for `OutsidePress` closes (seam 13).
- [ ] Handoff opens/closes and focus/hover-driven opens record instant kind `group` (seam 7) and it is observable through the Menu positioner/popup style state.

### Reuse of Menu via `MenuParentKind::Menubar`

- [ ] Child `MenuRoot`s wired under `Menubar` receive `MenuParentKind::Menubar` with the menubar context link through child wiring — no global/ambient lookup; menus outside a menubar are unaffected.
- [ ] All fourteen seams in "Menu seams this issue consumes" are implemented at the branch sites the Menu port reserved, each with a test or an explicit deferral note in this issue; no Menu behavior for `None`/`Submenu` parents changes observably.
- [ ] Menubar-parent menus use positioner defaults side bottom / align start (horizontal) and side inline-end / align start (vertical), direction-aware (seam 9).
- [ ] Menubar-parent triggers participate in the menubar's roving focus instead of standalone trigger focus behavior (seam 2), while retaining trigger drag-release activation through the shared menubar-wide `allow_mouse_up_trigger` window (seam 14).
- [ ] `close_on_click` semantics of items inside menubar menus are honored regardless of whether the menu was opened by click or by hover-switch.
- [ ] The keyboard-relay seam is a typed runtime command (menu → menubar), not simulated event bubbling or a raw key re-dispatch.

### Styling / state exposure

- [ ] `MenubarStyleState` exists with: `orientation`, `modal`, `has_submenu_open`, `disabled` (Base UI exposes the first three as state; `data-*` attributes map to these fields — no DOM attributes).
- [ ] `Menubar` has `.style_with_state(...)` taking `MenubarStyleState`.
- [ ] Trigger open/highlight styling flows through the Menu port's `MenuTriggerStyleState` (open, disabled) — this issue adds no duplicate trigger style surface; if a menubar-roving "highlighted" fact is needed on triggers, it is added to `MenuTriggerStyleState` as part of seam 2 and noted in `issues/port-baseui-menu.md`.
- [ ] Accessibility audit item: confirm the pinned GPUI revision still lacks the AccessKit APIs; keep menubar/menuitem/orientation semantics in runtime metadata for the AccessKit follow-up; no DOM ARIA literals.

### Tests / verification

Runtime tests (no window):

- [ ] Trigger registration order and roving `move_highlight`: wrap vs clamp, Home/End, disabled-trigger reachability.
- [ ] `has_submenu_open` transitions: set on direct-child open; retained across `SiblingOpen` and `ListNavigation` closes; cleared on `OutsidePress`/`EscapeKey`/`ItemPress` closes; unaffected by nested submenu open/close.
- [ ] Hand-off command: open menu moves from trigger A to trigger B, at most one child menu open once settled.
- [ ] Keyboard-relay command: perpendicular key at index i yields highlight i±1 with wrap/clamp and the `ListNavigation` reason; no-op when the menu consumes the axis.
- [ ] Disabled menubar gates trigger interaction commands.

Rendered tests under `crates/base_gpui/src/menubar/tests/`:

- [ ] Click a trigger opens its menu; click again closes it.
- [ ] No hover-open when nothing is open; hover-switch to a sibling once one menu is open; hover-switch works while a nested submenu is open.
- [ ] Focusing a trigger does not open while closed-all; focusing a sibling opens once a menu is open.
- [ ] Arrow-key roving across triggers (both orientations, RTL flip), Home/End, `loop_focus` true/false.
- [ ] Space/Enter/ArrowDown on a focused trigger open the menu with first item highlighted; arrows navigate within; ArrowRight opens a submenu; ArrowLeft closes it back to the submenu trigger.
- [ ] Perpendicular arrow inside an open menu moves to the neighboring menubar menu (and does not when a submenu trigger is highlighted).
- [ ] Escape closes and returns focus to the trigger.
- [ ] `close_on_click` respected for items in menus opened by click and by hover-switch.
- [ ] Outside press closes the whole tree in one press.
- [ ] Modal backdrop cutout keeps all sibling triggers interactive; `.modal(false)` renders no backdrop.
- [ ] Disabled menubar prevents opening any menu; menubar with a disabled first trigger stays keyboard-reachable.
- [ ] Demo renders in `crates/base_gpui/src/main.rs` without panics.

## Uncertain items needing confirmation

- **Ordering**: this issue cannot start until `issues/port-baseui-menu.md` lands
  the `MenuParentKind::Menubar` seams (at minimum: the enum variant with a menubar
  link, the trigger/root/positioner/popup branch sites, and the keyboard-relay
  seam). Confirm whether the seam *implementations* belong here (as written) or
  should partially land with the Menu port.
- **Child payload erasure**: `MenuRoot<P>` is payload-generic per menu, but a
  menubar holds several menus with potentially different `P`. The typed
  `MenubarChild` enum needs an erasure strategy (e.g. a private wiring trait
  implemented by `MenuRoot<P>` that attaches the menubar parent before erasing to
  `AnyElement`). Confirm the approach before implementation.
- **Relay mechanism**: Base UI relies on DOM event bubbling into the CompositeRoot
  (plus an explicit relay for detached triggers). GPUI has no equivalent bubbling
  across the popup/portal boundary — the issue mandates a typed runtime command;
  confirm this against how the Menu port's key dispatch ends up structured.
- **Trigger roving-highlight style fact**: whether menubar roving highlight needs a
  new field on `MenuTriggerStyleState` or focus-based styling suffices.
- **Scroll locking**: Base UI applies viewport-conditional scroll lock for
  touch-opened menubar menus; deferred here unless the Menu port lands scroll
  locking, in which case wire the menubar-modal condition (seam 10) into it.
