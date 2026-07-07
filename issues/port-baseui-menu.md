# Port Base UI Menu to GPUI

## Problem

Base UI Menu is a full dropdown-menu family: controlled and uncontrolled popup open
state with reason/source details, click/hover/keyboard trigger activation with
patient-click ("stick if open") behavior, roving-highlight list navigation with
typeahead, activatable items (regular, link, checkbox, radio) with per-item
`closeOnClick` semantics, groups with labels, separators, anchored positioning with
collision handling and an arrow, modal behavior with an internal backdrop that cuts a
hole for the trigger, drag-from-trigger-to-item release activation, detached triggers
through a handle, a viewport for multi-trigger content transitions, and — the piece
that makes Menu architecturally bigger than Select or Popover — **nested submenus**
forming a tree of open popups coordinated through Base UI's `FloatingTree` events
(`menuopenchange`, `itemhover`, `close`).

`crates/base_gpui` has no Menu family yet, but it already contains every popup
building block Menu needs as per-component precedents: Select's roving
highlight/typeahead runtime and anchored positioner, Popover's trigger
payloads/portal/arrow/backdrop/positioner, Tooltip's delayed hover timers, Dialog's
focus trap/return, and the shared `utils::overlay` / `utils::presence` helpers. The
goal is to port Menu **behavior and contracts** into one GPUI-native
runtime/context/layers component under `crates/base_gpui/src/menu/`, not to copy
React hooks, Floating UI DOM internals, ARIA attributes, data attributes, or CSS
variables.

Two constraints shape this port:

1. **Parent typing must be preserved.** Base UI's `MenuRoot`, `MenuTrigger`, and
   `MenuPositioner` branch on a parent discriminator
   (`undefined | menu (submenu) | menubar | context-menu`). Context Menu and Menubar
   are separate future issues that *layer on Menu* exactly through these branches, so
   the GPUI runtime must model the parent kind explicitly from day one even though
   only `None` and `Submenu` are exercised by this issue.
2. **Nested submenus are the main new architectural work.** Closing a parent closes
   all descendants; opening a submenu closes its open siblings; hovering a different
   item in the parent closes unrelated child branches (after the child's
   `close_delay`); outside-press dismissal must test the pointer against the whole
   open tree, not one popup; Escape closes only the innermost menu unless
   `close_parent_on_esc` is set.

Trigger payloads are Rust-native typed values using `P: Clone + 'static` (matching
Popover). Radio-group values use `V: Clone + Eq + 'static` rather than Base UI's
JavaScript `any`.

## Scope

Port the Menu component family from Base UI into GPUI-native components
(all parts inside the root child tree carry the root payload parameter `P`):

- `MenuRoot<P>`
- `MenuTrigger<P>`
- `MenuPortal<P>`
- `MenuBackdrop<P>`
- `MenuPositioner<P>`
- `MenuPopup<P>`
- `MenuArrow<P>`
- `MenuItem<P>`
- `MenuLinkItem<P>`
- `MenuGroup<P>`
- `MenuGroupLabel<P>`
- `MenuCheckboxItem<P>`
- `MenuCheckboxItemIndicator<P>`
- `MenuRadioGroup<P, V>`
- `MenuRadioItem<P, V>`
- `MenuRadioItemIndicator<P, V>`
- `MenuSubmenuRoot<P>`
- `MenuSubmenuTrigger<P>`
- `MenuSeparator` backed by the shared `base_gpui::separator::Separator`
- `MenuHandle<P>` + `create_menu_handle<P>()` — **stretch scope** (see below)
- `MenuViewport<P>` — **stretch scope** (see below)

Clearly-marked cuttable/stretch scope for the first pass (keep the API open, do not
silently drop):

- **`MenuHandle<P>` / detached triggers** (`store/MenuHandle.ts`): follow the
  Popover handle pattern if implemented; otherwise defer with the type reserved.
- **`MenuViewport<P>`** (`viewport/MenuViewport.tsx`): multi-trigger content
  transitions; only meaningful together with detached triggers, mirrors
  `PopoverViewport`'s simplified activation-direction port.

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/menu/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/menu/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/menu/root/MenuRoot.tsx` (open state, reasons, parent typing, `MenuParent`, list navigation/typeahead wiring, modal/touch guards)
- `/home/luke/Projects/base-ui/packages/react/src/menu/root/MenuRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/menu/root/MenuRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/menu/root/MenuRoot.detached-triggers.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/menu/store/MenuStore.ts` (full state inventory: `activeIndex`, `hoverEnabled`, `allowMouseEnter`, `stickIfOpen`, `instantType`, `closeDelay`, parent propagation)
- `/home/luke/Projects/base-ui/packages/react/src/menu/store/MenuHandle.ts`
- `/home/luke/Projects/base-ui/packages/react/src/menu/trigger/MenuTrigger.tsx` (mousedown open, patient click, drag-release `allowMouseUpTrigger`, hover with safe polygon, focus guards)
- `/home/luke/Projects/base-ui/packages/react/src/menu/trigger/MenuTrigger.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/menu/trigger/MenuTriggerDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/menu/portal/MenuPortal.tsx` + `MenuPortalContext.ts` + test
- `/home/luke/Projects/base-ui/packages/react/src/menu/backdrop/MenuBackdrop.tsx` (+ data attributes, test)
- `/home/luke/Projects/base-ui/packages/react/src/menu/positioner/MenuPositioner.tsx` (parent-dependent side/align defaults, tree events `menuopenchange` / `itemhover`, internal backdrop + cutout, scroll lock)
- `/home/luke/Projects/base-ui/packages/react/src/menu/positioner/MenuPositionerContext.ts` + `MenuPositioner.test.tsx` + `MenuPositionerDataAttributes.ts` + `MenuPositionerCssVars.ts`
- `/home/luke/Projects/base-ui/packages/react/src/menu/popup/MenuPopup.tsx` (focus manager rules, `finalFocus`, tree `close` handler, in-popup hover close-delay) + test + data attributes
- `/home/luke/Projects/base-ui/packages/react/src/menu/arrow/MenuArrow.tsx` (+ test, data attributes)
- `/home/luke/Projects/base-ui/packages/react/src/menu/item/MenuItem.tsx` + `useMenuItem.ts` + `useMenuItemCommonProps.ts` (tabIndex roving, Space-during-typeahead suppression, `itemhover` emission, mouseup drag activation) + test + data attributes
- `/home/luke/Projects/base-ui/packages/react/src/menu/link-item/MenuLinkItem.tsx` (+ test, data attributes)
- `/home/luke/Projects/base-ui/packages/react/src/menu/group/MenuGroup.tsx` + `MenuGroupContext.ts` + test
- `/home/luke/Projects/base-ui/packages/react/src/menu/group-label/MenuGroupLabel.tsx` + test
- `/home/luke/Projects/base-ui/packages/react/src/menu/checkbox-item/MenuCheckboxItem.tsx` + `MenuCheckboxItemContext.ts` + test + data attributes
- `/home/luke/Projects/base-ui/packages/react/src/menu/checkbox-item-indicator/MenuCheckboxItemIndicator.tsx` + test + data attributes
- `/home/luke/Projects/base-ui/packages/react/src/menu/radio-group/MenuRadioGroup.tsx` + `MenuRadioGroupContext.ts` + test
- `/home/luke/Projects/base-ui/packages/react/src/menu/radio-item/MenuRadioItem.tsx` + `MenuRadioItemContext.ts` + test + data attributes
- `/home/luke/Projects/base-ui/packages/react/src/menu/radio-item-indicator/MenuRadioItemIndicator.tsx` + test + data attributes
- `/home/luke/Projects/base-ui/packages/react/src/menu/submenu-root/MenuSubmenuRoot.tsx` + `MenuSubmenuRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/menu/submenu-trigger/MenuSubmenuTrigger.tsx` + test + data attributes
- `/home/luke/Projects/base-ui/packages/react/src/menu/viewport/MenuViewport.tsx` + test + css vars + data attributes
- `/home/luke/Projects/base-ui/packages/react/src/menu/utils/types.ts` (`MenuOpenEventDetails` — the tree-event payload)
- `/home/luke/Projects/base-ui/packages/react/src/menu/utils/findRootOwnerId.ts` (DOM tree-ownership walk — replace with runtime tree knowledge)
- `/home/luke/Projects/base-ui/packages/react/src/menu/utils/stateAttributesMapping.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/menu/page.mdx` and `demos/**`

Existing `base_gpui` infrastructure to use **as per-component reference** — the
architecture doc forbids extracting these into shared primitives; read them, then
implement Menu-local equivalents under `crates/base_gpui/src/menu/`:

- Anchored positioning + collision handling: `crates/base_gpui/src/select/layers/select_positioner.rs`, `crates/base_gpui/src/popover/layers/popover_positioner.rs`
- Deferred/anchored portal + keep-mounted: `crates/base_gpui/src/popover/layers/popover_portal.rs`, `crates/base_gpui/src/select/layers/select_portal.rs`
- Arrow placement/uncentered facts: `crates/base_gpui/src/popover/layers/popover_arrow.rs`
- Backdrop + outside-press + modal occlusion: `crates/base_gpui/src/popover/layers/popover_backdrop.rs`, `crates/base_gpui/src/utils/overlay.rs` (`modal_backdrop`)
- Roving-highlight list navigation + typeahead: `crates/base_gpui/src/select/runtime.rs` (`highlighted_item_index`, `move_highlight`, `apply_typeahead`) and `crates/base_gpui/src/select/key.rs` (`typeahead_text`)
- Hover open/close delay timers: `crates/base_gpui/src/tooltip/layers/tooltip_trigger.rs` (`spawn_delayed_hover`)
- Trigger payloads, detached handle, open-change details shape: `crates/base_gpui/src/popover/` (`runtime.rs`, `context.rs`, `props.rs`)
- Focus trap / focus return: `crates/base_gpui/src/dialog/`
- Presence/transition facts: `crates/base_gpui/src/utils/presence.rs`
- Shared separator: `crates/base_gpui/src/separator/`

Current GPUI implementation:

- No `crates/base_gpui/src/menu/` module exists yet.

Expected GPUI implementation files (flat layout per
`docs/base-gpui-component-architecture.md`):

```text
crates/base_gpui/src/menu/
  mod.rs                    # barrel only
  actions.rs                # arrow/Home/End/Enter/Space/Escape key dispatch
  child.rs                  # typed child enums (root children, popup children, group children, radio-group children)
  child_wiring.rs           # private traversal/indexing/context attachment
  context.rs                # MenuContext<P> (+ MenuHandle<P> if in scope)
  props.rs                  # root props/callbacks/config
  runtime.rs                # MenuRuntime<P>: all state incl. parent kind + submenu tree links, command enums, outcome/details types
  style_state.rs            # Menu*StyleState structs (one per part that draws)
  layers/
    mod.rs                  # barrel only
    menu_root.rs
    menu_trigger.rs
    menu_portal.rs
    menu_backdrop.rs
    menu_positioner.rs
    menu_popup.rs
    menu_arrow.rs
    menu_viewport.rs        # stretch
    menu_item.rs
    menu_link_item.rs
    menu_group.rs
    menu_group_label.rs
    menu_checkbox_item.rs
    menu_checkbox_item_indicator.rs
    menu_radio_group.rs
    menu_radio_item.rs
    menu_radio_item_indicator.rs
    menu_submenu_root.rs
    menu_submenu_trigger.rs
    menu_separator.rs       # thin re-export/wrapper over base_gpui::separator
  tests/
```

Also update:

- `crates/base_gpui/src/lib.rs` (`pub mod menu;` + `menu::init(cx);`)
- `crates/base_gpui/src/main.rs` with Menu demos

## Initial design decisions

### Parent typing (forward-compatibility hooks)

Model Base UI's `MenuParent` as a Rust enum owned by the runtime, e.g.:

```rust
pub enum MenuParentKind {
    None,                    // standalone dropdown menu
    Submenu { /* link to parent MenuContext/runtime entity */ },
    Menubar,                 // reserved: implemented by the Menubar issue
    ContextMenu,             // reserved: implemented by the Context Menu issue
}
```

Every behavior in Base UI that branches on parent type must branch on this enum in
the GPUI runtime, even where only `None`/`Submenu` are reachable today: modal
applies only for `None` (and later `ContextMenu`); positioner side/align defaults
(`None` → bottom/center; `Submenu` → inline-end/start); trigger interaction wiring
(submenu triggers are items of the parent menu); Escape/focus-return rules;
`instant` classification of open transitions. `Menubar` and `ContextMenu` variants
may be constructible-but-unimplemented (documented), but the *seams* — where the
parent kind is stored, injected, and consulted — must exist so Context Menu and
Menubar can layer on without restructuring.

**Ownership split (reconciled with the Context Menu and Menubar issues).** This Menu
port owns *declaring and consulting* the seams: the `MenuParentKind` variants, every
branch site that reads them, and any parent-supplied data the runtime must thread
(cursor anchor point for context menus, keyboard-relay target and `group` instant
kind for menubars). The parent-specific *behavior* at those sites is filled in by
`issues/port-baseui-context-menu.md` and `issues/port-baseui-menubar.md`. The rule:
if a variant needs a seam this port did not reserve, the fix is to add the seam here
in `menu/` — a variant must never work around a missing seam with `context_menu/`- or
`menubar/`-local special-casing of behavior the runtime should branch on.

### Nested submenu tree

Each `MenuSubmenuRoot<P>` creates its own keyed `MenuRuntime<P>` (its own open
state, highlight, items) linked to the parent menu's context — a tree of runtimes
replacing Base UI's `FloatingTree`. Replace the DOM/event mechanisms
(`menuopenchange`, `itemhover`, `close` events, `data-rootownerid` +
`findRootOwnerId`) with explicit runtime commands across the tree link:

- parent close (any reason) closes all open descendants;
- opening a submenu closes the previously open sibling submenu (`sibling-open` reason);
- highlighting/hovering a *different* item in the parent schedules the open child
  branch to close after the child's `close_delay` (immediately when zero), and
  re-hovering the submenu trigger cancels the pending close;
- opening a child disables the parent's hover-driven close while the child is open
  (`hover_enabled` handling);
- outside-press hit-testing considers the union of all open popups' bounds plus the
  root trigger — a press inside any menu of the tree is not "outside";
- root-level facts (root id, keyboard relay seam for Menubar) propagate from parent
  to child runtimes.

### Open-change details

Follow the Popover details shape (`reason`, `source`, `trigger_id`, payload,
`cancel()`, `is_canceled()`, `prevent_unmount_on_close()`), with Menu's reason set:
`TriggerHover`, `TriggerFocus`, `TriggerPress`, `OutsidePress`, `FocusOut`,
`ListNavigation`, `EscapeKey`, `ItemPress`, `ClosePress`, `SiblingOpen`,
`CancelOpen`, `ImperativeAction`, `None`. Do not expose DOM event objects.

### Hover intent / safe polygon

Base UI submenu triggers (and hover-enabled root triggers) use Floating UI's
`safePolygon` to keep a submenu open while the pointer travels diagonally from the
trigger into the popup. That is a GPUI primitive of its own — tracked as the
separate issue `issues/add-gpui-safe-polygon-hover-primitive.md` (planned; cross-link
both ways when it lands). **This issue requires `MenuSubmenuTrigger` to consume that
primitive when available**; until then the interim fallback is tooltip-style
open/close delay timers (`delay` default 100ms, `close_delay` default 0) per
`tooltip_trigger.rs`, with the acceptance criterion phrased so either satisfies it.

### Checkbox / radio item state

`MenuCheckboxItem` owns per-item controlled/uncontrolled `checked`
(`.checked(bool)` / `.default_checked(bool)` / `.on_checked_change(...)` with
cancelable details, `close_on_click` default **false**). `MenuRadioGroup<P, V>` owns
controlled/uncontrolled `value: Option<V>` with `V: Clone + Eq + 'static` and a
cancelable `on_value_change`; `MenuRadioItem` derives `checked = group_value ==
item_value`. Register this state in the menu runtime (keyed by wired item index)
so the single controlled/uncontrolled rule stays in one place, per the architecture
doc — do not scatter `useControlled` clones across layers.

### Link items

GPUI has no `<a>`/href. Port `MenuLinkItem` as an activatable item with an
`on_activate` callback for navigation, `close_on_click` default **false**, and a
highlighted-only style state. If this adds no behavior over `MenuItem` with
`close_on_click(false)`, it may be a thin wrapper — but keep the distinct public
type for Base UI parity.

## Out of scope / drop from Base UI

- React context/hooks/store (`MenuStore`, `ReactStore` selectors, `useControlled`,
  `useSyncedValues`), `FloatingTree`/`FloatingNode`/`FloatingPortal` internals —
  GPUI keyed entities + `MenuContext<P>` + explicit tree links.
- `render` props and the payload children-render-function
  (`children={({ payload }) => ...}`). Payload-driven content is in scope via a
  GPUI-native content builder / runtime query, matching Popover.
- `className`, web `style` props.
- `nativeButton` / `NonNativeButtonProps` switches.
- SSR/hydration/prehydration.
- CSS variable APIs (`MenuPositionerCssVars`, `MenuViewportCssVars`) — typed
  style-state measurement fields instead.
- DOM data attributes (`data-highlighted`, `data-checked`, `data-popup-open`,
  `data-rootownerid`, transition-status attributes) — typed `Menu*StyleState`
  fields.
- ARIA roles/attributes (`role="menu"/"menuitem"/"menuitemcheckbox"/"menuitemradio"/"group"`,
  `aria-haspopup`, `aria-expanded`, `aria-controls`, `aria-labelledby`,
  `aria-checked`, `aria-hidden`) and DOM id linking (`useBaseUiId`, group label id
  registration) — defer to an AccessKit follow-up; keep group-label metadata in the
  runtime for future mapping.
- DOM focus guards (`FocusGuard`, `beforeContentFocusGuardRef`,
  `triggerFocusTargetRef`) — use GPUI `FocusHandle` mechanics per Dialog.
- Browser event objects in details, `getPseudoElementBounds`, `ownerDocument`
  mouseup listeners as literal mechanisms — translate to GPUI window mouse events
  and prepaint bounds.
- Touch-specific heuristics literally (`allowTouchToClose` 300ms guard, touch
  scroll lock, context-menu 500ms outside-press grace) — implement GPUI-native
  equivalents where observable or document deferral; the context-menu grace period
  belongs to the Context Menu issue.
- Browser scroll locking and `inert` outside trees — use `utils::overlay` modal
  occlusion behavior as Popover/Dialog do.
- The toolbar integration branch (`useToolbarRootContext`) and menubar keyboard
  event relay behavior — reserve the relay seam, implement in the Menubar issue.
- Exact CSS animation semantics — preserve `instant` / transition facts in style
  state via `utils::presence`.
- Do not use Rust scoped visibility syntax (`pub(...)`); `ast-grep scan` must stay clean.

## Acceptance Criteria

### Module / API surface

- [x] `crates/base_gpui/src/menu/` exists with the flat architecture layout; `menu/mod.rs` and `menu/layers/mod.rs` are barrel-only.
- [x] `base_gpui::menu` is exported from `crates/base_gpui/src/lib.rs` and `base_gpui::init(cx)` calls `menu::init(cx)` to register key bindings.
- [x] All Scope parts exist as public builders: `MenuRoot<P>`, `MenuTrigger<P>`, `MenuPortal<P>`, `MenuBackdrop<P>`, `MenuPositioner<P>`, `MenuPopup<P>`, `MenuArrow<P>`, `MenuItem<P>`, `MenuLinkItem<P>`, `MenuGroup<P>`, `MenuGroupLabel<P>`, `MenuCheckboxItem<P>`, `MenuCheckboxItemIndicator<P>`, `MenuRadioGroup<P, V>`, `MenuRadioItem<P, V>`, `MenuRadioItemIndicator<P, V>`, `MenuSubmenuRoot<P>`, `MenuSubmenuTrigger<P>`, `MenuSeparator`.
- [x] `MenuRoot<P>` supports `.id(...)`, `.default_open(bool)` (default false), `.open(bool)`, `.on_open_change(...)`, `.on_open_change_complete(...)`, `.disabled(bool)` (default false), `.modal(bool)` (default **true**), `.loop_focus(bool)` (default true), `.orientation(...)` (default vertical), `.close_parent_on_esc(bool)` (default false), `.highlight_item_on_hover(bool)` (default true), `.trigger_id(...)`, `.default_trigger_id(...)`.
- [x] `MenuTrigger<P>` supports `.id(...)`, `.disabled(bool)`, `.payload(P)`, `.open_on_hover(bool)`, `.delay(Duration)` (default 100ms), `.close_delay(Duration)` (default 0).
- [x] `MenuItem<P>` supports `.label(...)` (typeahead label override), `.disabled(bool)`, `.close_on_click(bool)` (default **true**), and an activation callback (`.on_click(...)` or equivalent).
- [x] `MenuLinkItem<P>` supports `.label(...)`, `.close_on_click(bool)` (default **false**), and `.on_activate(...)` navigation callback; it has no disabled prop (parity with Base UI).
- [x] `MenuCheckboxItem<P>` supports `.checked(bool)`, `.default_checked(bool)` (default false), `.on_checked_change(...)` (cancelable), `.disabled(bool)`, `.label(...)`, `.close_on_click(bool)` (default **false**).
- [x] `MenuCheckboxItemIndicator<P>` supports `.keep_mounted(bool)` (default false).
- [x] `MenuRadioGroup<P, V>` supports `.value(Option<V>)`, `.default_value(Option<V>)`, `.on_value_change(...)` (cancelable), `.disabled(bool)` with `V: Clone + Eq + 'static`.
- [x] `MenuRadioItem<P, V>` supports required `.value(V)`, `.disabled(bool)`, `.label(...)`, `.close_on_click(bool)` (default **false**); `MenuRadioItemIndicator<P, V>` supports `.keep_mounted(bool)`.
- [x] `MenuSubmenuRoot<P>` supports the `MenuRoot` surface **minus** `modal`, `handle`, `trigger_id`, and `default_trigger_id` (Base UI omits these on submenus), plus `.close_parent_on_esc(bool)`.
- [x] `MenuSubmenuTrigger<P>` supports `.label(...)`, `.disabled(bool)`, `.open_on_hover(bool)` (default **true**), `.delay(Duration)` (default 100ms), `.close_delay(Duration)` (default 0).
- [x] `MenuPositioner<P>` supports side/align, `side_offset`, `align_offset`, collision padding, and the collision-avoidance subset established by Select/Popover; `MenuPortal<P>` supports `.keep_mounted(bool)`.
- [x] `MenuPopup<P>` supports a `final_focus`-equivalent override or documents its deferral (matching Popover's audit).
- [x] `MenuSeparator` reuses `base_gpui::separator::Separator` behavior.
- [x] Payload generic is `P: Clone + 'static`; no `Debug`/`Display` bounds sneak in.
- [x] Stretch: `MenuHandle<P>` + `create_menu_handle<P>()` with `open(trigger_id)` (deterministic failure on unknown id), `close()`, `is_open()`, and root/trigger `.handle(...)` binding — or the deferral is documented in this issue before implementation starts.
- [x] Stretch: `MenuViewport<P>` with activation-direction facts per `PopoverViewport` — or documented deferral.
- [x] Imperative `actionsRef` equivalent (`close`, `unmount`) is exposed through the handle or documented as deferred alongside transition infrastructure.
- [x] All public style payloads use `*StyleState` names from `style_state.rs`; no `pub(...)` scoped visibility anywhere in `menu/`.

### Correctness / compile readiness

- [x] `cargo fmt --check` passes.
- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui menu` passes.
- [x] `cargo test -p base_gpui` passes.
- [x] `cargo clippy -p base_gpui --all-targets` exits successfully with only pre-existing warnings.
- [x] `ast-grep scan crates/base_gpui/src/menu` passes (including barrel-only `mod.rs` rule).
- [x] Demos in `crates/base_gpui/src/main.rs`: basic menu (trigger/portal/positioner/popup/items/separator/group), checkbox+radio menu, and a nested submenu demo; gallery render test passes with menus initially closed.

### Architecture / internal primitives

- [x] Follow `docs/base-gpui-component-architecture.md`: one deep `MenuRuntime<P>`, thin `MenuContext<P>`, thin layers under `layers/`.
- [x] `MenuRuntime<P>` owns: open/mounted/presence state, `MenuParentKind`, active trigger id/metadata/payload, registered item metadata (kind, label, disabled, `close_on_click`, focus handle, bounds), highlighted index, checkbox/radio state, typeahead buffer + typing flag, hover facts (`hover_enabled`, `allow_mouse_enter`, `stick_if_open`, pending timers' state), `instant` classification, measured trigger/positioner/popup/arrow bounds, and submenu tree links (children handles/entities, pending sibling-close state).
- [x] Runtime interface is commands + part-shaped queries in Menu domain language (`sync_children`, `reconcile`, `request_open`/`request_close` with details, `activate_item`, `move_highlight`, `apply_typeahead`, `open_submenu`, `close_descendants`, `notify_parent_item_highlight`, `set_bounds`, ...) — no getter/setter pairs, no "what is the highlighted index" queries (parts ask "am I highlighted?").
- [ ] Runtime is unit-testable without a GPUI window, including tree-coordination logic (parent-close cascades, sibling-open, pending delayed closes driven by injected clocks/explicit ticks). — *deferred/partial: runtime tree decisions (pending closes, reconcile_child_hover, sibling queries) are window-free; the cascade/sibling *execution* lives in MenuContext and needs a window*
- [x] `MenuContext<P>` is `read`/`update` plus the open/close/toggle and item-activation commands; the controlled/uncontrolled rule for open state (and only that rule) lives in the context; no component vocabulary accretes on it.
- [x] Typed child routing before `AnyElement` erasure: root children (`Trigger`/`Portal`), portal→positioner→popup chain, popup children (`Item`, `LinkItem`, `CheckboxItem`, `RadioGroup`, `Group`, `GroupLabel`, `Separator`, `SubmenuRoot`, `Arrow`, `Viewport`), group children, and radio-group children each get constrained enums in `child.rs`; arbitrary visual children remain allowed *inside* items/triggers/labels where Base UI examples rely on it.
- [x] `child_wiring.rs` is the only module that walks children, assigns item indices (skipping groups/labels/separators), collects labels for typeahead, attaches context, and registers submenu links; no index bookkeeping leaks into layers or public helper methods.
- [x] The nested-popup tree is modeled as linked per-menu runtimes (one per `MenuRoot`/`MenuSubmenuRoot`), not one god-runtime for the whole tree and not DOM-style broadcast events; cross-menu effects flow through explicit parent/child commands.
- [x] Positioning, portal, arrow, backdrop, hover timers, and typeahead are implemented Menu-locally using the reference files listed in Scope; **no new shared/generic primitives are extracted** unless a concept is already proven deep and repeated (and then only as flat `utils/` files).
- [x] `MenuParentKind::{Menubar, ContextMenu}` seams exist (storage, injection point, branch sites) with documented "implemented by the Menubar/Context Menu issues" markers; parent-supplied data the variants require (cursor anchor point, keyboard-relay target, `group` instant kind) is threaded through the runtime, not special-cased in the variant modules. — *Menubar issue landed: `MenuMenubarLink` threads the keyboard-relay target and the `Group` instant kind is recorded for menubar-parent menus; ContextMenu anchor point remains reserved*

### Controlled / uncontrolled open state

- [x] Uncontrolled root initializes from `.default_open(false)`.
- [x] Controlled root reflects `.open(...)` as source of truth; interactions fire `on_open_change` without mutating internal open state.
- [x] Controlled `.open(...)` takes precedence over `.default_open(...)`.
- [x] `on_open_change` fires before uncontrolled mutation; canceling prevents the change (open and close).
- [ ] Details carry reason/source/trigger/payload; every close path reports the correct reason: `TriggerPress` (toggle), `ItemPress`, `EscapeKey`, `OutsidePress`, `FocusOut`, `SiblingOpen`, `CancelOpen` (drag-release outside), `ImperativeAction`; hover open reports `TriggerHover`. — *deferred/partial: CancelOpen (drag-release) not implemented — drag-from-trigger activation deferred*
- [x] `prevent_unmount_on_close()` keeps the popup mounted through one close cycle and does not leak into later closes.
- [x] `on_open_change_complete` fires after presence settles (immediately without transition infrastructure).
- [x] Root `.disabled(true)` ignores all open/close user interaction.
- [x] Redundant `set_open` calls (same open state, same trigger, same reason) are deduplicated — no duplicate callbacks.
- [ ] `instant` transition classification is recorded: keyboard-click and dismiss (Escape/no-reason) closes mark the corresponding instant kind; trigger-change marks `trigger-change` (menubar `group` kind reserved for the Menubar issue). — *deferred/partial: Click/Dismiss instant kinds recorded; trigger-change requires multi-trigger (MenuHandle stretch, deferred)*

### List navigation & typeahead

- [x] Menu uses GPUI key-dispatch actions and a Menu key context (`actions.rs`), not raw `on_key_down`, for Arrow keys, Home, End, Enter, Space, and Escape.
- [x] Highlight is separate per menu in the tree; each submenu tracks its own highlighted index.
- [ ] ArrowDown/ArrowUp move highlight through activatable items (vertical orientation; the axis flips for horizontal orientation), wrapping per `.loop_focus(true)` and clamping when false. — *deferred/partial: vertical orientation only; horizontal axis flip deferred*
- [x] Home/End move highlight to first/last activatable item.
- [x] Groups, group labels, and separators are never highlighted and do not consume item indices.
- [x] Navigation skips nothing else: disabled items follow Base UI's roving behavior (focusable-when-disabled highlight, activation no-op).
- [x] With a closed menu and focused trigger, ArrowDown opens and highlights the first item; ArrowUp opens and highlights the last item (reserved: suppressed for the future context-menu parent).
- [x] Pointer hover highlights items only when `highlight_item_on_hover == true` **and** the pointer has actually moved since open (`allow_mouse_enter` guard — the item under the cursor at open time is not highlighted by accident).
- [x] Enter activates the highlighted item.
- [x] Space activates the highlighted item unless a typeahead session is in progress (Space then contributes to the match and does not activate).
- [x] Typeahead matches registered item labels (explicit `.label(...)` override wins), skips disabled items, cycles on repeated characters, and resets after the standard timeout; matching moves highlight only (never activates).
- [x] Highlight movement scrolls the highlighted item into view when the popup scrolls.

### Item activation behavior

- [x] Clicking an enabled `MenuItem` fires its activation callback; the menu closes when `close_on_click == true` (reason `ItemPress`) and stays open when false.
- [x] Disabled items (and items under a disabled root) never activate by pointer, keyboard, or drag-release.
- [ ] Drag-from-trigger: pressing the trigger, dragging onto an item, and releasing activates that item and always closes the menu (overriding `close_on_click`), gated by the 200ms `allow_mouse_up_trigger` window equivalent; releasing outside the trigger/popup tree bounds closes with reason `CancelOpen`; drag-release never activates submenu triggers. — *deferred/partial: drag-from-trigger release activation deferred (needs window-level mouseup tracking)*
- [x] `MenuLinkItem` fires `on_activate` and follows its `close_on_click` default of false.

### Checkbox / radio item behavior

- [x] Uncontrolled checkbox item initializes from `.default_checked(false)`; controlled `.checked(...)` is source of truth and never self-mutates.
- [x] Activating a checkbox item fires `on_checked_change(!checked, details)` before uncontrolled mutation; canceling prevents the toggle.
- [x] Checkbox activation keeps the menu open by default (`close_on_click` false) and closes when explicitly set true.
- [x] Uncontrolled radio group initializes from `.default_value(...)`; controlled `.value(...)` is source of truth.
- [x] Activating an enabled radio item calls `on_value_change(item_value, details)`; canceling prevents uncontrolled mutation; selecting the already-selected value is deterministic.
- [x] Radio group `.disabled(true)` disables all its items; item-level disabled combines with group and root disabled.
- [x] Radio items outside a `MenuRadioGroup` fail clearly at construction/wiring time (typed child enums should make this unrepresentable).
- [ ] Indicators render only while checked unless `.keep_mounted(true)`; keep-mounted indicators expose unchecked state through style state; presence/transition facts use `utils::presence`. — *deferred/partial: indicators use a simple present flag; utils::presence transition facts deferred with transition infrastructure*

### Submenu behavior (nested-popup tree)

- [x] `MenuSubmenuTrigger` is simultaneously an item of the parent menu (participates in parent highlight, typeahead via its label, roving tab-stop) and the trigger of the child menu.
- [x] Hovering a submenu trigger opens the child after `.delay(...)` when `open_on_hover == true`, gated on the parent's `allow_mouse_enter` (no hover-open before real pointer movement); hover intent uses the safe-polygon primitive from `issues/add-gpui-safe-polygon-hover-primitive.md` when available, with tooltip-style close-delay timers as the documented interim fallback.
- [x] Clicking a submenu trigger opens the child; when `open_on_hover == true` mouse clicks do not toggle-close (parity with `toggle: !openOnHover`, `ignoreMouse: openOnHover`).
- [x] Keyboard: ArrowRight on a highlighted submenu trigger opens the child and highlights its first item (ArrowLeft in RTL via `utils::direction`); ArrowLeft inside an open submenu closes it and returns focus/highlight to the submenu trigger (ArrowRight in RTL).
- [x] Enter/Space on a highlighted submenu trigger opens the child and highlights its first item.
- [x] Closing a parent menu (any reason) closes all descendant submenus.
- [x] Opening a submenu closes the open submenu of any sibling branch (reason `SiblingOpen`).
- [x] Highlighting/hovering a different item in the parent closes the open child branch after the child's `close_delay` (immediately when zero); re-hovering the same submenu trigger cancels the pending close.
- [x] While a child is open, the parent popup's hover-driven behavior is adjusted (`hover_enabled` semantics) so moving within the parent does not flicker-close the child except through the item-hover rule above.
- [x] Escape inside a submenu closes only that submenu and returns focus to its trigger; with `.close_parent_on_esc(true)` Escape propagates to close the parent chain.
- [x] Outside-press dismissal tests against the union of all open popups in the tree plus the active trigger; presses inside any open menu of the tree never dismiss ancestors.
- [x] Submenu open/close fires the submenu root's own `on_open_change` with correct reasons.
- [x] Submenus never apply modal behavior regardless of configuration.
- [x] Rapid hover across multiple submenu triggers is deterministic: at most one child branch per menu is open once timers settle, and no panics.

### Positioning / portal / dismiss

- [x] `MenuPortal` renders popup content through GPUI deferred/anchored overlay rendering only while mounted or `.keep_mounted(true)` (closed keep-mounted content reports closed style state).
- [x] `MenuPositioner` measures trigger/anchor and popup bounds via GPUI prepaint hooks and stores them in the runtime.
- [x] Default placement: side bottom / align center for a standalone menu; side inline-end / align start for submenus (direction-aware via `utils::direction`); parent-kind seams reserve the menubar and context-menu defaults.
- [ ] `side_offset` / `align_offset` and collision padding map to GPUI anchored placement; collision avoidance implements the practical flip/shift subset established by Select/Popover (submenus default to the popup-style avoidance variant, standalone menus to the dropdown variant, matching Base UI's differing defaults). — *deferred/partial: single flip subset shared by both parent kinds; per-parent avoidance variants deferred*
- [x] Positioner style state exposes open, side, align, anchor-hidden, `nested`, `instant`, and measured anchor/available sizes.
- [x] `MenuArrow` follows resolved side/align, exposes `uncentered`, and updates when the side flips.
- [x] `MenuBackdrop` is user-renderable, sits below the popup, and closes on press with reason `OutsidePress`; a hover-opened menu's backdrop does not capture pointer events (Base UI sets `pointer-events: none` for `trigger-hover` opens).
- [x] Modal (`.modal(true)`, the default, root menus only): outside interaction is blocked via `utils::overlay::modal_backdrop`-style occlusion **with a cutout for the active trigger element** so the trigger stays clickable; modal behavior is skipped entirely when the menu was opened by hover; scroll locking follows whatever Popover/Dialog established or is documented as deferred.
- [x] `.modal(false)` allows normal outside interaction except dismissal handling.
- [x] Closing by focus leaving the tree reports `FocusOut`; window-resize/layout-invalidation closing follows the Select/Popover precedent or documents the same deferral.

### Keyboard / focus

- [x] Trigger is focusable when enabled, skipped when disabled; keyboard activation (Enter/Space) toggles the menu via GPUI actions.
- [x] Opening by pointer or keyboard moves focus into the popup per GPUI-native rules (submenus do not steal initial focus from the parent — `initialFocus` only applies to non-submenu parents); opening by hover does not steal focus.
- [x] Item tab-stop follows roving semantics: only the highlighted item of the open menu is the tab-stop.
- [x] Closing returns focus to the active trigger by default (`final_focus` override honored if implemented); when the active trigger is gone, focus falls back deterministically or is left unchanged as documented.
- [x] Hover-opened root menus honor patient-click: clicks on the trigger within the 500ms threshold do not close (`stick_if_open`); after the threshold, trigger clicks toggle-close normally — or this is explicitly deferred with the Popover patient-click precedent cited.
- [x] Escape/focus-out/close all restore focus without panics when triggered from deep submenu levels.
- [x] Accessibility audit item: confirm the pinned GPUI revision still lacks AccessKit role/expanded/label APIs; keep menu/menuitem/checkbox/radio/group semantics and group-label metadata in the runtime for the AccessKit follow-up; no DOM ARIA literals.

### Styling / state exposure

- [x] `MenuRootStyleState` exists (may be small: open, disabled, parent kind facts).
- [x] `MenuTriggerStyleState`: open, disabled, active-trigger, payload-present.
- [x] `MenuPositionerStyleState`: open, side, align, anchor-hidden, nested, instant, measured sizes.
- [x] `MenuPopupStyleState`: open, mounted, transition status, side, align, nested, instant.
- [x] `MenuBackdropStyleState`: open, mounted, transition status, hover-opened (pointer-inert) fact.
- [x] `MenuArrowStyleState`: open, side, align, uncentered.
- [x] `MenuItemStyleState`: highlighted, disabled.
- [x] `MenuLinkItemStyleState`: highlighted.
- [x] `MenuCheckboxItemStyleState`: checked, highlighted, disabled.
- [x] `MenuCheckboxItemIndicatorStyleState`: checked, highlighted, disabled, transition status.
- [x] `MenuRadioGroupStyleState`: disabled.
- [x] `MenuRadioItemStyleState`: checked, highlighted, disabled.
- [x] `MenuRadioItemIndicatorStyleState`: checked, highlighted, disabled, transition status.
- [x] `MenuSubmenuTriggerStyleState`: open, highlighted, disabled.
- [x] `MenuGroupStyleState` / `MenuGroupLabelStyleState` exist even if initially empty; `MenuViewportStyleState` (stretch): activation direction, transitioning, instant.
- [x] Every part that draws has `.style_with_state(...)` taking its component-specific struct; Base UI data attributes and CSS vars appear only as typed style-state fields.

### Tests / verification

Runtime tests (no window):

- [x] Uncontrolled default-closed and `default_open(true)` initial states.
- [ ] Controlled open reconciliation; controlled callbacks without internal mutation; canceled open and canceled close. — *deferred/partial: controlled sync covered; cancellation paths live in MenuContext callbacks (need rendered tests)*
- [ ] Open-change reason/source correctness per close path, including `SiblingOpen` and `CancelOpen`. — *deferred/partial: reasons recorded per path in context; per-reason assertions need rendered tests*
- [x] Item registration order with groups/labels/separators interleaved (indices unaffected).
- [x] Highlight movement: wrap vs clamp, Home/End, disabled-item highlight with activation no-op.
- [x] Typeahead: label matching, explicit label override, disabled skipping, repeated-character cycling, reset timeout, Space suppression while typing.
- [ ] Checkbox controlled/uncontrolled toggle and cancellation; radio group value selection, group disabled, cancellation. — *deferred/partial: toggle/selection covered; cancellation lives in wiring closures (needs rendered tests)*
- [ ] Submenu tree: parent close cascades to descendants; sibling-open closes other branches; parent item-hover schedules/cancels delayed child close; Escape innermost-only vs `close_parent_on_esc`. — *deferred/partial: pure directives tested; cascade/Escape execution is context-level (needs rendered tests)*
- [ ] Outside-press hit-test against multi-level open-tree bounds. — *deferred/partial: own-node union tested; multi-level link closures need a window*
- [x] `prevent_unmount_on_close` one-cycle behavior.
- [ ] Handle open/close with unknown trigger id (if handle implemented). — *deferred/partial: MenuHandle deferred (stretch)*

Rendered tests under `crates/base_gpui/src/menu/tests/`:

- [ ] Trigger click opens; click again closes; disabled trigger and disabled root do not open.
- [ ] Item click activates and closes (`close_on_click` true) vs stays open (false).
- [ ] ArrowDown/ArrowUp navigation, Enter/Space activation, Escape close with focus return to trigger.
- [ ] ArrowDown/ArrowUp on closed focused trigger opens with first/last item highlighted.
- [ ] Hover highlight respects `highlight_item_on_hover` and the no-pointer-movement guard.
- [ ] Checkbox and radio items toggle/select with indicators mounting/keep-mounted correctly.
- [ ] Submenu: hover-open after delay, ArrowRight/ArrowLeft open/close with correct highlight and RTL flip, parent close cascades, hovering a different parent item closes the branch.
- [ ] Outside click dismisses the whole tree; click inside a nested popup does not dismiss ancestors.
- [ ] Modal cutout keeps the trigger interactive; hover-opened menu skips modal/backdrop capture.
- [ ] Keep-mounted portal behavior; positioner side/align/measurement style state; arrow side updates.
- [ ] Separator/group/group-label render without corrupting navigation.

  *Rendered behavior tests under `menu/tests/` are not yet written (runtime tests + the gallery render smoke test cover the port so far); the unchecked rendered-test boxes above remain open.*
- [x] Demos render in `crates/base_gpui/src/main.rs` without panics.

## Port notes / documented deferrals (first pass, 2026-07-07)

- **`MenuHandle<P>` / `create_menu_handle<P>()` (stretch)**: deferred. Menu currently
  supports one wired trigger per root; the handle + detached triggers (and the
  `trigger_id` activation semantics that depend on multiple triggers) follow the
  Popover handle pattern in a follow-up. `.trigger_id(...)` / `.default_trigger_id(...)`
  exist on `MenuRoot` for API parity but are inert until the handle lands.
- **`MenuViewport<P>` (stretch)**: deferred together with detached triggers; the
  `trigger-change` instant kind is likewise unreachable until then.
- **Imperative `actionsRef` (`close`, `unmount`)**: deferred alongside the handle and
  transition infrastructure.
- **Drag-from-trigger release activation** (and the `CancelOpen` close reason it
  produces): deferred; needs window-level mouseup tracking.
- **Patient-click (`stick_if_open`)**: recorded in the runtime as a reserved seam,
  behavior deferred with the Popover patient-click precedent.
- **Safe polygon**: `MenuSubmenuTrigger` arms `primitives::safe_polygon` on unhover
  (exit point, item bounds, child popup bounds, resolved side); the parent popup
  evaluates it on mouse moves and maps verdicts onto the generation-counted child
  close timers. Pointer travel outside any popup's element area is not observed yet
  (window-scope mouse observation is a noted limitation); the unhover close is
  therefore scheduled with `max(close_delay, 40ms)` as the armed grace.
- **`final_focus`**: deferred, matching the Popover audit (documented on `MenuPopup`).
- **Horizontal orientation**: `.orientation(...)` is stored/threaded; the arrow-key
  axis flip is not implemented yet.
- **Scroll locking / window-resize close**: follows the Popover approach (modal
  backdrop swallows scroll; no explicit lock), same deferral.

## Follow-ups to track explicitly if not completed in the first port

- [ ] Safe-polygon hover-intent primitive (`issues/add-gpui-safe-polygon-hover-primitive.md`) and swapping `MenuSubmenuTrigger` (and hover-enabled `MenuTrigger`) onto it.
- [ ] `MenuHandle<P>` detached triggers + `MenuViewport<P>` trigger-change transitions (stretch scope above).
- [x] Context Menu issue: builds on `MenuParentKind::ContextMenu` (anchor-at-pointer, outside-press grace period, right-click gesture handling). — *Landed by `issues/port-baseui-context-menu.md`: cursor anchor + grace/initial-point facts live in `MenuRuntime`, positioner branches on the ContextMenu parent kind.*
- [x] Menubar issue: builds on `MenuParentKind::Menubar` (composite root, keyboard relay seam, hover-across-triggers, `group` instant type).
- [ ] Patient-click stickiness if deferred (shared decision with Popover).
- [ ] Touch-specific open/close guards (`allowTouchToClose` equivalent) once GPUI exposes pointer-type metadata.
- [ ] AccessKit menu/menuitem/menuitemcheckbox/menuitemradio/group roles and group-label relationships when the pinned GPUI revision supports them.
- [ ] Scroll locking for modal menus if not inherited from the Popover/Dialog approach.

## Uncertain items needing confirmation

- **Generic parameter spread**: threading `P` through every part (and `V` on the
  radio family) mirrors Popover/Select but makes deeply generic signatures; confirm
  this over alternatives (e.g. non-payload-generic inner parts) before implementation.
- **Submenu tree representation**: linked per-menu runtime entities (proposed) vs a
  single tree-owning runtime; the proposal keeps each menu's knowledge local but
  requires careful cross-entity command plumbing in GPUI.
- **Safe-polygon issue file**: `issues/add-gpui-safe-polygon-hover-primitive.md`
  does not exist yet — create it (or confirm its final name) so the cross-links hold.
- **`MenuLinkItem`**: keep as a distinct part with `on_activate`, or document it as
  an alias of `MenuItem` with different defaults.
- **Handle/Viewport stretch scope**: confirm whether to include in the first pass or
  defer both (they are only meaningful together with detached triggers).
