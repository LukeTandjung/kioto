# Port Base UI Navigation Menu to GPUI

## Problem

Base UI Navigation Menu is a website-navigation family built around one **shared popup**
serving many triggers: a horizontal (or vertical) list of triggers where hovering or
clicking a trigger opens a single anchored panel showing that trigger's `Content`, and
moving to a different trigger **retargets** the same panel — repositioning it to the new
trigger and morphing its width/height to the new content's size, with an activation
direction (`left`/`right`/`up`/`down`) derived from the relative position of the previous
and next triggers so content can slide accordingly. Open state is value-driven: the root
holds `value: Option<T>` and the popup is open exactly when the value is non-null. Hover
activation uses ~50ms open/close delays plus Floating UI's `safePolygon` hover-intent so
the pointer can travel diagonally from trigger to panel; click activation adds
patient-click stickiness. Links can close the menu on click, and nested navigation menus
(a root inside a content panel) cascade `LinkPress` closes up to the parent.

This is **not** a thin variant of Menu or Popover — it is an independent part family
(Root, List, Item, Trigger, Content, Portal, Positioner, Popup, Viewport, Backdrop,
Arrow, Link, Icon) with its own state model (one shared popup keyed by an item value,
rather than one popup per trigger).

The React implementation is dominated by fights with the DOM that GPUI does not have:

- `NavigationMenuTrigger.tsx` is 914 lines, most of it four `useAnimationFrame` loops,
  a `ResizeObserver`, and a `MutationObserver` choreographing CSS-variable size fixing
  (`--popup-width`/`--positioner-height`/...) so the shared popup can animate between
  content sizes without layout jumps.
- `NavigationMenuContent.tsx` re-parents itself into the `Viewport` with
  `ReactDOM.createPortal`, because in the DOM a node can only animate inside the popup
  if it physically moves there.
- Focus guards (`beforeInside`/`afterInside`/`beforeOutside`/`afterOutside` spans),
  `aria-owns` bridges, and `pointer-events` mutations stitch the detached DOM subtree
  back into tab order and hover geometry.

**In GPUI this collapses.** One runtime knows the active value; the popup layer simply
renders the active item's content directly inside the single popup element every frame.
DOM re-parenting drops out entirely (there is no detached subtree — the viewport is just
a clipping child of the popup). The rAF measurement loops become GPUI prepaint
measurement (`Div::on_children_prepainted`) plus `window.on_next_frame` for one-frame
deferrals. CSS-variable size morphing becomes typed measured-size facts in style state,
optionally animated with GPUI's element animation (`with_animation`, per
`gpui/src/elements/animation.rs` in the pinned checkout). Focus guards become plain
GPUI `FocusHandle` navigation. The port must state and exploit this collapse — do not
transliterate the trigger's frame loops.

Item values are generic: `T: Clone + Eq + 'static`, matching Tabs/Select. Open state is
`Option<T>` (`None` = closed), controlled or uncontrolled.

Priority note: this is the lowest-priority member of the menu family for a desktop
editor (navigation menus are website chrome; Menu/Context Menu cover editor needs).
Spec is complete regardless so the port can be picked up without re-scouting.

## Scope

Port the Navigation Menu component family from Base UI into GPUI-native components:

- `NavigationMenuRoot<T>`
- `NavigationMenuList<T>`
- `NavigationMenuItem<T>`
- `NavigationMenuTrigger<T>`
- `NavigationMenuContent<T>`
- `NavigationMenuPortal<T>`
- `NavigationMenuPositioner<T>`
- `NavigationMenuPopup<T>`
- `NavigationMenuViewport<T>`
- `NavigationMenuBackdrop<T>`
- `NavigationMenuArrow<T>`
- `NavigationMenuLink<T>`
- `NavigationMenuIcon<T>`

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/root/NavigationMenuRoot.tsx` (value-derived open, close-reason bookkeeping, focus-return blocklist, nested `LinkPress` cascade, unmount actions, close-transition size seeding)
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/root/NavigationMenuRootContext.ts` (full shared-state inventory: activation direction, viewport targets, prev-trigger ref, auto-size reset ownership, delays, orientation, viewport inert)
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/root/NavigationMenuRoot.test.tsx` (3769 lines — the primary behavioral spec: hover/click/touch interactions, safe-polygon pointer-events scoping, patient click, delay/closeDelay, controlled value, tabbing, nested menus, popup sizing)
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/root/NavigationMenuRoot.webkit.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/root/NavigationMenuRoot.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/list/NavigationMenuList.tsx` (CompositeRoot roving arrow-focus, `useDismiss` with trigger-aware outside-press, `useHoverFloatingInteraction` close-delay, arrow-key propagation guard) + `NavigationMenuDismissContext.ts` + test
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/item/NavigationMenuItem.tsx` + `NavigationMenuItemContext.ts` + test (item value context, generated fallback value)
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/trigger/NavigationMenuTrigger.tsx` (914 lines: hover with `safePolygon` + `restMs`/`closeDelay`, click with `stickIfOpen`, pointer-type guards, activation-direction computation from trigger rects, keyboard `ListNavigation` open, focus-out close, focus guards, and the entire rAF/observer size-sync machinery this port replaces) + `NavigationMenuTriggerDataAttributes.ts` + test
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/content/NavigationMenuContent.tsx` (portal re-parenting into the viewport, per-content transition status, `keepMounted`, inert inactive content, activation-direction state) + `NavigationMenuContentDataAttributes.ts` + test
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/portal/NavigationMenuPortal.tsx` + `NavigationMenuPortalContext.ts` + test
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/positioner/NavigationMenuPositioner.tsx` (anchor = active trigger with prev-trigger fallback, `instant` flag on initial open and window resize, adaptive origin for top/left sides, portal tabbable management) + `NavigationMenuPositionerContext.ts` + `NavigationMenuPositionerCssVars.ts` + `NavigationMenuPositionerDataAttributes.ts` + test
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/popup/NavigationMenuPopup.tsx` (shared surface, origin-side anchoring for top/left so size morphs grow away from the anchor) + `NavigationMenuPopupCssVars.ts` + `NavigationMenuPopupDataAttributes.ts` + test
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/viewport/NavigationMenuViewport.tsx` (clipping container, focus guards, viewport-inert focus-loop guard, prev-trigger recording) + test
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/backdrop/NavigationMenuBackdrop.tsx` + data attributes + test
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/arrow/NavigationMenuArrow.tsx` + data attributes + test
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/link/NavigationMenuLink.tsx` (`active`, `closeOnClick` → `LinkPress`, focus-out close) + data attributes + test
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/icon/NavigationMenuIcon.tsx` + data attributes + test
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/utils/constants.ts` (`OPEN_DELAY = 50`, `CLOSE_DELAY = 50`, trigger identifier attribute)
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/utils/isOutsideMenuEvent.ts` (replace with runtime bounds/tree knowledge)
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/utils/setSharedFixedSize.ts` (CSS-var size fixing — replaced by runtime measured-size facts)
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/navigation-menu/page.mdx` and `demos/**`

Existing `base_gpui` infrastructure to use **as per-component reference** — the
architecture doc forbids extracting these into shared primitives; read them, then
implement Navigation Menu-local equivalents under `crates/base_gpui/src/navigation_menu/`:

- Anchored positioning + collision handling, to be extended with **trigger retargeting**
  (the anchor changes identity between opens without unmounting the popup):
  `crates/base_gpui/src/select/layers/select_positioner.rs`,
  `crates/base_gpui/src/popover/layers/popover_positioner.rs`
- Deferred/anchored portal + keep-mounted: `crates/base_gpui/src/popover/layers/popover_portal.rs`
- Outside-press dismiss + modal occlusion patterns: `crates/base_gpui/src/utils/overlay.rs`
- Roving-focus list navigation: `crates/base_gpui/src/tabs/runtime.rs` (highlight +
  roving focus + activation direction), `crates/base_gpui/src/select/runtime.rs`
  (`move_highlight`, item metadata registration)
- Hover open/close delay timers with generation-based cancellation:
  `crates/base_gpui/src/tooltip/layers/tooltip_trigger.rs` (`spawn_delayed_hover`),
  `crates/base_gpui/src/tooltip/runtime.rs` (hover generations)
- Presence/transition facts: `crates/base_gpui/src/utils/presence.rs`
- Direction awareness (RTL open keys, physical-left detection): `crates/base_gpui/src/utils/direction`

Safe-polygon hover intent is the **separate primitive issue**
`issues/add-gpui-safe-polygon-hover-primitive.md` (which already names
`NavigationMenuTrigger` as a consumer — the cross-link holds in both directions). This
issue requires the trigger to consume that primitive when available; until then the
documented interim fallback is tooltip-style close-delay timers, and the acceptance
criteria below are phrased so either satisfies them.

Current GPUI implementation:

- No `crates/base_gpui/src/navigation_menu/` module exists yet.

Expected GPUI implementation files (flat layout per
`docs/base-gpui-component-architecture.md`):

```text
crates/base_gpui/src/navigation_menu/
  mod.rs                    # barrel only
  actions.rs                # arrow-key open/list navigation + Escape key dispatch
  child.rs                  # typed child enums (root children, item children, popup/viewport chains)
  child_wiring.rs           # private traversal/indexing/context attachment/content collection
  context.rs                # NavigationMenuContext<T>
  props.rs                  # root props/callbacks/config (delays, orientation)
  runtime.rs                # NavigationMenuRuntime<T>: all state, command enums, change details
  style_state.rs            # NavigationMenu*StyleState structs (one per part that draws)
  layers/
    mod.rs                  # barrel only
    navigation_menu_root.rs
    navigation_menu_list.rs
    navigation_menu_item.rs
    navigation_menu_trigger.rs
    navigation_menu_content.rs
    navigation_menu_portal.rs
    navigation_menu_positioner.rs
    navigation_menu_popup.rs
    navigation_menu_viewport.rs
    navigation_menu_backdrop.rs
    navigation_menu_arrow.rs
    navigation_menu_link.rs
    navigation_menu_icon.rs
  tests/
```

Also update:

- `crates/base_gpui/src/lib.rs` (`pub mod navigation_menu;` + `navigation_menu::init(cx);`)
- `crates/base_gpui/src/main.rs` with a Navigation Menu demo

## Initial design decisions

### Single-popup shared-content model (replaces DOM re-parenting)

This is the architectural heart of the port. In Base UI, each `Content` lives inside its
`Item` in the React tree and is *physically moved* into the shared `Viewport` DOM node
with `ReactDOM.createPortal` when its item becomes active; the trigger then runs rAF
loops and observers to fix popup/positioner sizes through CSS variables so the move
animates. In GPUI:

- `child_wiring.rs` collects each item's typed `NavigationMenuContent<T>` (keyed by item
  value) while walking the root's children. Content is **not rendered in place** under
  the item; the wiring routes it to the popup side of the tree.
- The single `NavigationMenuPopup<T>`/`NavigationMenuViewport<T>` renders the **active
  item's content directly** as its child on every frame — the runtime answers "which
  value is active", the viewport renders that entry. There is no re-parenting, no
  portal-into-viewport, no `aria-owns` bridge, no focus-guard spans stitching subtrees.
- Popup and positioner sizes come from prepaint measurement of the rendered content
  (`Div::on_children_prepainted` → `set_bounds`-style runtime command), exposed as typed
  measured-size facts in style state. Size/position morphing between two active values
  is a runtime-derived transition fact (previous size → next size + activation
  direction), optionally animated with GPUI element animation
  (`with_animation` / `gpui/src/elements/animation.rs`); the four `useAnimationFrame`
  loops, `ResizeObserver`, `MutationObserver`, `setSharedFixedSize`, and the
  auto-size-reset abort-controller ownership dance from `NavigationMenuTrigger.tsx` are
  all replaced by this one measurement + interpolation path. One-frame deferrals, where
  genuinely needed, use `window.on_next_frame`.
- `keep_mounted(true)` content stays mounted (hidden, closed style state) inside the
  viewport; the SSR/web-crawler rationale is dropped but the mounted-while-hidden
  behavior is kept for parity.

### Value model

`NavigationMenuRoot<T>` holds `value: Option<T>`; open is **derived**: open ⇔
`value.is_some()`. There is no separate `open` prop anywhere in the family.

- `.default_value(Option<T>)` (default `None`) — uncontrolled.
- `.value(Option<T>)` — controlled; takes precedence over `default_value`.
- `.on_value_change(...)` — fires with `(Option<T>, &mut details)`; cancelable;
  fires only when the next value actually differs (deduplicated); fires before
  uncontrolled mutation; canceled details prevent the change.
- `.on_open_change_complete(...)` — fires after close presence settles (immediately
  without transition infrastructure), matching Base UI's `onOpenChangeComplete(false)`.
  Base UI's `actionsRef.unmount` manual-unmount escape hatch is deferred alongside
  transition infrastructure (same posture as Popover's `actionsRef` audit).
- Closing (any reason) resets activation direction.

`NavigationMenuItem<T>` requires `.value(T)`. Base UI auto-generates a fallback id when
`value` is omitted; with a generic `T` that is not portable, so the GPUI port makes the
item value required (see Uncertain items).

Change details follow the Popover/Menu details shape (`reason`, `source`, `cancel()`,
`is_canceled()`) with the Navigation Menu reason set from `NavigationMenuRoot.tsx`:
`TriggerPress`, `TriggerHover`, `OutsidePress`, `ListNavigation`, `FocusOut`,
`EscapeKey`, `LinkPress`, `None`. No DOM event objects.

### Hover intent / safe polygon

`NavigationMenuTrigger.tsx` uses `safePolygon({ blockPointerEvents, getScope })` so the
pointer can cut across sibling triggers en route to the popup. The GPUI equivalent is
the primitive from `issues/add-gpui-safe-polygon-hover-primitive.md`: on trigger
unhover, arm the tracker with the exit point, trigger bounds, popup bounds, and
effective side; feed it pointer positions during the close-delay grace window; compose
its verdicts with generation-based close timers per the tooltip substrate. The DOM
`pointer-events` blocking/scoping mutations (`applySafePolygonPointerEventsMutation`,
`getScope`, the trigger-identifier attribute) drop out — bounds hit-testing subsumes
them. Until the primitive lands, the interim fallback is tooltip-style open/close delay
timers (`delay` 50ms, `close_delay` 50ms defaults from `utils/constants.ts`).

### Trigger retargeting

The positioner anchors the single popup to the **active** trigger. When the active value
changes while open, the anchor switches identity: position re-resolves against the new
trigger's bounds, the arrow follows, and size morph facts update — without closing,
unmounting, or re-opening the popup, and without a duplicate `on_value_change`. Base UI
also keeps a prev-trigger fallback anchor for the close transition after the active
trigger's item unmounts; the runtime keeps last-known anchor bounds for the same
purpose. This is new logic relative to `select_positioner.rs` (whose anchor is fixed);
build it Navigation Menu-locally per the architecture doc.

### Roving list navigation

`NavigationMenuList` is a roving arrow-focus composite over triggers and top-level
links: arrow keys (axis per orientation) move GPUI focus between focusable list items
without wrapping (`loopFocus=false` in Base UI), and the list stops arrow-key
propagation at its ends. With the menu closed and a trigger focused, the "open" arrow
key (ArrowDown for horizontal; ArrowRight for vertical, ArrowLeft in RTL) opens that
trigger's content with reason `ListNavigation`. Implement with GPUI actions + a key
context in `actions.rs`, reusing the roving pattern from `tabs/runtime.rs` /
`select/runtime.rs` as per-component reference. The list also owns dismissal wiring
(outside press) and popup-hover close-delay behavior, mirroring
`NavigationMenuList.tsx`'s `useDismiss` + `useHoverFloatingInteraction` placement —
in GPUI these become runtime commands wired from the list/portal layers.

### Nested navigation menus

Base UI supports a `NavigationMenuRoot` nested inside a `Content` (rendered as `div`
instead of `nav`, inline content without its own positioner, `FloatingTree` node links,
focus-guard chains). The GPUI port scopes nesting to the behaviors with observable
contracts:

- a nested `NavigationMenuRoot<T2>` may render inside a `Content` with its own
  independent value state and its own inline viewport (no portal/positioner) — since
  GPUI content is rendered directly, "inline nested viewport" is just normal child
  rendering plus the parent's size-morph measurement seeing the change;
- a nested link with `close_on_click(true)` closes the nested menu **and cascades**
  `LinkPress` closes up through parent roots (`setValue(null)` propagation in
  `NavigationMenuRoot.tsx`);
- nested content participates in the parent's popup size sync (content growing when a
  nested menu opens updates the measured morph target);
- nested triggers do not intercept the parent's roving arrow keys
  (the `nested` guards in `NavigationMenuTrigger.tsx`/`NavigationMenuList.tsx`).

The DOM-specific nested machinery (inline safe-polygon `pointer-events` scoping,
`FloatingTree` node bookkeeping, `isOutsideMenuEvent` tree walks, focus-guard
retargeting) is replaced by runtime bounds knowledge and parent-context links.

## Out of scope / drop from Base UI

- React context/hooks/`useControlled`/`useStableCallback` implementation details,
  `FloatingTree`/`FloatingNode`/`FloatingPortal`, `useFloatingRootContext` plumbing —
  GPUI keyed entity + `NavigationMenuContext<T>` + explicit parent links for nesting.
- `ReactDOM.createPortal` re-parenting of `Content` into the `Viewport`, the
  `viewportTargetElement` indirection, and `currentContentRef` observer targeting —
  replaced by the single-popup shared-content model above.
- The four `useAnimationFrame` loops, `ResizeObserver`, `MutationObserver`,
  `getCssDimensions`, `setSharedFixedSize`, auto-size-reset `AbortController`
  ownership, and `skipAutoSizeSync` flags in `NavigationMenuTrigger.tsx` — replaced by
  prepaint measurement + runtime-derived morph facts (+ optional GPUI animation).
- CSS variable APIs (`--popup-width/-height`, `--positioner-width/-height`,
  `--available-width/-height`, `--anchor-width/-height`, `--transform-origin`) — typed
  measured-size fields in style state.
- DOM data attributes (`data-popup-open`, `data-activation-direction`,
  `data-starting-style`/`data-ending-style`, `data-instant`,
  `data-base-ui-navigation-menu-trigger`) — typed `NavigationMenu*StyleState` fields.
- `render` props, `className`, web `style` props, `nativeButton`.
- SSR/hydration behavior (`keepMounted` for web crawlers, `hasMountedInPortal`,
  inline pre-portal rendering) — `keep_mounted` keeps only the mounted-while-hidden
  runtime behavior.
- Focus guards (`FocusGuard` spans, `beforeInside`/`afterInside`/`beforeOutside`/
  `afterOutside` refs), `aria-owns` bridge, `disableFocusInside`/`enableFocusInside`
  portal tabbable management, `viewportInert` focus-loop guard, `inert` attributes —
  use GPUI `FocusHandle`, tab-stop, and focus-tracking mechanics; document any tab-order
  gap GPUI cannot express yet.
- `safePolygon` DOM `pointer-events` mutations and scope elements
  (`useHoverInteractionSharedState`, `applySafePolygonPointerEventsMutation`,
  `getScope`, trigger identifier attribute) — bounds hit-testing via the safe-polygon
  primitive.
- `isOutsideMenuEvent` DOM containment walks — runtime bounds/tree knowledge.
- ARIA attributes and semantic elements (`<nav>`/`<ul>`/`<li>`/`<a>`/`<button>`,
  `aria-expanded`, `aria-controls`, `aria-current="page"`, `aria-hidden`,
  `role="presentation"`, DOM id linking) — defer to the AccessKit follow-up; the pinned
  GPUI revision lacks the needed APIs (per the Popover audit). Keep `Link`'s `active`
  fact in style state.
- Touch/pen pointer-type heuristics as literal mechanisms (`pointerType` guards,
  touch-click-only opens) — implement GPUI-native equivalents if pointer-type metadata
  is observable, otherwise document deferral.
- Exact CSS transition semantics (`useTransitionStatus`, `useAnimationsFinished`,
  `getDisabledMountTransitionStyles`, adaptive-origin middleware wobble fix) — preserve
  `transition`/`instant`/presence facts in style state via `utils::presence`; real
  animation timing goes through GPUI animation if implemented.
- `actionsRef` manual unmount — deferred with transition infrastructure (documented).
- Arbitrary JS value semantics — `T: Clone + Eq + 'static`.
- No Rust scoped visibility syntax (`pub(...)`); `ast-grep scan` must stay clean;
  `mod.rs` files are barrel-only.

## Acceptance Criteria

### Module / API surface

- [x] `crates/base_gpui/src/navigation_menu/` exists with the flat architecture layout; `navigation_menu/mod.rs` and `navigation_menu/layers/mod.rs` are barrel-only.
- [x] `base_gpui::navigation_menu` is exported from `crates/base_gpui/src/lib.rs` and `base_gpui::init(cx)` calls `navigation_menu::init(cx)` to register key bindings.
- [x] All Scope parts exist as public builders: `NavigationMenuRoot<T>`, `NavigationMenuList<T>`, `NavigationMenuItem<T>`, `NavigationMenuTrigger<T>`, `NavigationMenuContent<T>`, `NavigationMenuPortal<T>`, `NavigationMenuPositioner<T>`, `NavigationMenuPopup<T>`, `NavigationMenuViewport<T>`, `NavigationMenuBackdrop<T>`, `NavigationMenuArrow<T>`, `NavigationMenuLink<T>`, `NavigationMenuIcon<T>`.
- [x] `NavigationMenuRoot<T>` supports `.id(...)` (stable keyed-state identity), `.default_value(Option<T>)` (default `None`), `.value(Option<T>)`, `.on_value_change(...)`, `.on_open_change_complete(...)`, `.delay(Duration)` (default 50ms), `.close_delay(Duration)` (default 50ms), `.orientation(...)` (default horizontal).
- [x] `NavigationMenuItem<T>` supports required `.value(T)`.
- [x] `NavigationMenuTrigger<T>` supports `.disabled(bool)` (default false).
- [x] `NavigationMenuContent<T>` supports `.keep_mounted(bool)` (default false).
- [x] `NavigationMenuPortal<T>` supports `.keep_mounted(bool)` (default false).
- [x] `NavigationMenuPositioner<T>` supports side (default `Bottom`) / align (default `Center`), `side_offset`, `align_offset`, collision padding (default 5px), and the collision-avoidance subset established by Select/Popover.
- [x] `NavigationMenuLink<T>` supports `.active(bool)` (default false), `.close_on_click(bool)` (default false), and an activation callback (`.on_activate(...)` or equivalent — GPUI has no href).
- [x] `NavigationMenuIcon<T>` renders caller-provided children (no hard-coded `▼` default) and exposes open state for styling.
- [x] A `NavigationMenuValueChangeDetails`-style type exists with `reason` (`TriggerPress`, `TriggerHover`, `OutsidePress`, `ListNavigation`, `FocusOut`, `EscapeKey`, `LinkPress`, `None`), GPUI-native `source` metadata, `cancelable`, `cancel()`, and `is_canceled()`; no DOM/browser event objects are exposed.
- [x] Public value APIs consistently constrain `T: Clone + Eq + 'static`; no `Debug`/`Display` bounds sneak in.
- [x] All public style payloads use `*StyleState` names from `style_state.rs`; no `pub(...)` scoped visibility anywhere in `navigation_menu/`.

### Correctness / compile readiness

- [x] `cargo fmt --check` passes.
- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui navigation_menu` passes.
- [x] `cargo test -p base_gpui` passes.
- [x] `cargo clippy -p base_gpui --all-targets` exits successfully with only pre-existing warnings.
- [x] `ast-grep scan crates/base_gpui/src/navigation_menu` passes (including the barrel-only `mod.rs` rule).
- [x] A Navigation Menu demo exists in `crates/base_gpui/src/main.rs` (triggers with distinct-size contents, a link with `close_on_click`, arrow, icon); the gallery render test passes with the menu initially closed.

### Architecture — single-popup shared-content model

- [x] Follow `docs/base-gpui-component-architecture.md`: one deep `NavigationMenuRuntime<T>`, thin `NavigationMenuContext<T>`, thin layers under `layers/`.
- [x] `NavigationMenuRuntime<T>` owns: current `Option<T>` value (uncontrolled), open/mounted/presence facts, registered item/trigger metadata (value, disabled, focus handle, measured bounds), active + previous trigger identity and bounds, activation direction, measured popup/positioner/content/arrow bounds, previous-vs-next content size morph facts, hover timer generations and safe-polygon arming state, patient-click (`stick_if_open`) state, and `instant` classification.
- [x] Runtime interface is commands + part-shaped queries in domain language (`sync_children`, `reconcile`, `set_value` with details/outcome, `activate_trigger`, `retarget`, `schedule_hover_close`/`cancel_hover`, `set_bounds`, ...) — no getter/setter pairs; parts ask "am I the active item?" not "what is the active value?".
- [x] Content is collected by `child_wiring.rs` (keyed by item value) and rendered by the popup/viewport side of the tree; there is no DOM-style re-parenting, no per-item portal, and no duplicated content trees. `child_wiring.rs` is the only module that walks children, assigns indices, and attaches context.
- [x] Typed child routing exists before `AnyElement` erasure for: root children (List / Portal), list children (Item / Link), item children (Trigger / Content), portal→positioner chain, positioner children (Popup / Arrow), popup children (Viewport plus arbitrary chrome where Base UI examples show it), trigger children (Icon + arbitrary visual children).
- [x] The React size-sync machinery is not transliterated: no frame-loop equivalents of `handleValueChange` / `handleInterruptedMutationResize` / `syncCurrentSize` / `scheduleAutoSizeReset`; sizes are measured once per layout via prepaint hooks feeding one runtime command, and any one-frame deferral uses `window.on_next_frame`.
- [x] Every transition (activation direction, morph source/target sizes, instant classification) is computed inside the runtime, once; no shadow previous-value diffing in layers.
- [x] Runtime is unit-testable without a GPUI window, including retargeting, activation-direction, morph-fact, and hover-timer-generation logic (timers driven by explicit ticks/injected time).
- [x] `NavigationMenuContext<T>` is `read`/`update` plus the value-change command; the controlled/uncontrolled rule lives only there; no component vocabulary accretes on it.
- [x] Positioning, portal, dismissal, roving focus, and hover timers are implemented Navigation Menu-locally from the per-component references listed in Scope; **no new shared/generic primitives are extracted** (the safe-polygon primitive is its own pre-approved issue, not something this port extracts).

### Active-value controlled / uncontrolled behavior

- [x] Uncontrolled root initializes from `.default_value(...)`; `None` means closed.
- [x] `.default_value(Some(v))` renders initially open on item `v` with the initial positioner transition suppressed (`instant` fact, per the "initial open" tests).
- [x] Controlled `.value(...)` is the source of truth and takes precedence over `.default_value(...)`; interactions fire `on_value_change` without mutating internal value.
- [x] Open state is derived: the popup is open exactly when the effective value is non-`None`; there is no independent open flag to drift.
- [x] `on_value_change` fires only when the next value differs (switching items via keyboard/hover produces no duplicate callbacks); it fires before uncontrolled mutation; canceling prevents both open-, switch-, and close-changes.
- [x] Every change path reports the correct reason: trigger click `TriggerPress`; hover open `TriggerHover`; keyboard open `ListNavigation`; outside press `OutsidePress`; Escape `EscapeKey`; focus leaving the menu `FocusOut`; link close `LinkPress`.
- [x] Closing resets activation direction; a later open starts with direction `None`.
- [x] Controlled close from outside (value set to `None` externally) closes cleanly, preserving last-known popup size facts for the exit transition (per `getPositionerFixedSize` seeding) and clearing activation direction.
- [x] `on_open_change_complete` fires after close presence settles (immediately without transition infrastructure); the `actionsRef`-style manual unmount is documented as deferred.
- [x] Focus return on close: focus returns to the previously active trigger when focus is inside the popup, **except** for closes with reason `TriggerHover`, `OutsidePress`, or `FocusOut` (Base UI's blocked-return set).

### Hover / click activation, delays, safe polygon

- [x] Hovering an enabled trigger opens it after `.delay(...)` (default 50ms); when the popup is already mounted (switching triggers), the open is immediate (Base UI's `restMs` collapse to 0).
- [x] Unhovering schedules close after `.close_delay(...)` (default 50ms); re-hovering the trigger or hovering the popup within the window cancels the pending close (generation-safe, per the tooltip timer substrate).
- [x] During the close-delay grace window, hover intent is gated by the safe-polygon primitive from `issues/add-gpui-safe-polygon-hover-primitive.md` when available: diagonal traversal from trigger to popup stays open, exit from the safe region closes immediately, landing on the popup or trigger disarms; until the primitive lands, tooltip-style close-delay timers are the documented interim fallback and satisfy this criterion.
- [x] Hovering a **different** trigger while open retargets (fires `on_value_change` with `TriggerHover`, switches the active item) rather than close-then-reopen.
- [x] Clicking a closed enabled trigger opens it (`TriggerPress`); clicking the active trigger closes it — except within the 500ms patient-click threshold after a hover open (`stick_if_open`), during which clicks keep it open; after the threshold, clicks toggle-close normally.
- [x] Clicking a different trigger while open switches the active item without closing the menu.
- [x] Disabled triggers never open by hover, click, or keyboard, and expose disabled state; a disabled trigger remains focusable (Base UI `focusableWhenDisabled`).
- [x] Activation direction is computed on trigger switch from previous vs next trigger bounds: horizontal orientation compares x (`Left`/`Right`), vertical compares y (`Up`/`Down`); equal positions leave direction unchanged; first open has direction `None`.
- [x] Touch/pen pointer-type hover suppression (hover does not open for touch input) is implemented if GPUI exposes pointer-type metadata, otherwise documented as deferred.

### Roving list navigation

- [x] The list uses GPUI key-dispatch actions and a Navigation Menu key context (`actions.rs`), not raw `on_key_down`.
- [x] Arrow keys along the orientation axis move GPUI focus between the list's focusable items (triggers and links) without wrapping; navigation clamps at the ends and does not leak arrow keys past the list boundary.
- [x] With the menu closed and a trigger focused, ArrowDown (horizontal orientation) opens that trigger's content with reason `ListNavigation`; for vertical orientation the open key is ArrowRight (ArrowLeft in RTL via `utils::direction`).
- [x] The keyboard open path does not double-fire `on_value_change` through the click/activation path (per Base UI's keydown/`listNavigation` dedupe).
- [x] Escape while focus is within the menu closes with reason `EscapeKey`.
- [ ] Tab moves focus from the active trigger into the open popup content, through it, and out the other side; tabbing out of the menu entirely closes with `FocusOut` (document any tab-order gap the pinned GPUI cannot express instead of silently skipping). *(not implemented: GPUI-native tab order flows through mounted focusables, but tab-out FocusOut close is not wired — tab-order gap documented)*

### Positioner retargeting + size/position morph

- [x] The positioner anchors the popup to the active trigger's measured bounds using GPUI `deferred(...)`/`anchored()` per the Select/Popover precedent; defaults side `Bottom`, align `Center`, collision padding 5px; collision avoidance follows the practical flip/shift subset.
- [x] When the active value changes while open, the anchor retargets to the new trigger: position re-resolves, the arrow follows, and the popup is not unmounted or re-created.
- [x] The runtime records previous and current content/popup sizes on trigger switch and exposes them (plus activation direction) as morph facts; consumers can animate the size change via `style_with_state` + GPUI animation, and without animation the popup snaps to the new size deterministically.
- [x] Size morphs grow away from the anchor: for effective side `Top` (and physical-left placements, RTL-aware) the popup's anchored edge stays fixed while size changes (Base UI's origin-side positioning + adaptive origin).
- [x] If the active trigger unmounts while open, positioning falls back to the last-known anchor bounds for the close transition (prev-trigger fallback).
- [ ] `instant` transition classification is exposed in positioner style state: true for the first frame of an initially open menu, and for ~100ms around window resizes (resize re-position does not animate); cleared otherwise. *(partial: `Initial` and `Resize` facts exposed; `Resize` clears on the next committed change rather than a ~100ms timer)*
- [x] Positioner style state exposes open, side, align, anchor-hidden, instant, and measured anchor/available/positioner sizes as typed fields (the CSS-var concepts).
- [x] Window/layout resize re-measures and re-positions using GPUI-native mechanisms (no `ResizeObserver`); popup size facts update when content size changes while open (e.g. nested content growing).

### Viewport content mounting + focus

- [x] The viewport renders exactly the active item's content, clipped to the viewport bounds; switching values swaps content in place inside the single popup.
- [x] `NavigationMenuContent<T>` with `.keep_mounted(false)` (default) is absent while inactive; with `.keep_mounted(true)` it stays mounted hidden and reports closed style state, and reopening it does not animate stale size (per the kept-portal sizing tests).
- [x] Content style state exposes `open`, transition status, and `activation_direction` so entering/leaving panels can slide by direction.
- [x] During a value switch, previous-content presence facts are available long enough for an exit transition when animation infrastructure is used; without it the swap is immediate and deterministic.
- [ ] Focus inside content follows GPUI-native rules: interactive children are reachable by Tab when the popup is open, unreachable when closed or when content is kept-mounted-hidden; the DOM focus-guard/`aria-owns`/`viewportInert` machinery is not ported. *(partial: content unmounts when closed; kept-mounted-hidden content renders invisible but tab-unreachability is not enforced)*
- [ ] The nested scope from the design decisions works: a nested `NavigationMenuRoot` inside a `Content` maintains independent value state, renders inline content without its own portal/positioner, contributes to the parent's measured size, does not intercept the parent list's roving arrow keys, and a nested link's `LinkPress` close cascades to close all ancestor roots (each firing its own `on_value_change`). *(partial: a nested root renders inline with independent state and contributes to measured size; `NavigationMenuParentClose` exists for the LinkPress cascade but automatic parent-context wiring for nested roots is not implemented)*

### Positioning / portal / dismiss

- [x] `NavigationMenuPortal<T>` renders the positioner chain through GPUI deferred/anchored overlay rendering only while mounted or `.keep_mounted(true)`; closed keep-mounted content reports closed style state; `utils::presence` facts distinguish open/mounted/transitioning.
- [x] Outside press closes with reason `OutsidePress` — except presses on any navigation-menu trigger of the same menu tree (the trigger's own handler decides switch-vs-toggle; the trigger-identifier attribute check becomes a runtime bounds test), and presses inside any open popup of the tree (including nested menus) never dismiss.
- [x] `NavigationMenuBackdrop<T>` is user-renderable below the popup, presentational (it does not capture or block pointer interaction), and exposes open/mounted/transition style state.
- [x] `NavigationMenuArrow<T>` follows the resolved side/align of the **active** trigger, exposes `uncentered`, updates when the anchor retargets or the side flips, and is decorative (no ARIA literal).
- [ ] Focus leaving the whole menu tree (trigger, popup, nested popups) closes with reason `FocusOut`, using runtime bounds/focus knowledge instead of `isOutsideMenuEvent` DOM walks. *(not implemented: FocusOut close on focus leaving the tree is not wired; reason/blocklist support exists in the runtime)*
- [x] There is no modal behavior and no scroll locking (Base UI Navigation Menu has none); the popup does not trap focus.

### Styling / state exposure

- [x] `NavigationMenuRootStyleState`: open (and nested, if nesting support records it).
- [x] `NavigationMenuListStyleState`: open.
- [x] `NavigationMenuTriggerStyleState`: open (active-item), disabled.
- [x] `NavigationMenuContentStyleState`: open, transition status, activation direction.
- [x] `NavigationMenuPositionerStyleState`: open, side, align, anchor-hidden, instant, measured anchor/available/positioner sizes.
- [x] `NavigationMenuPopupStyleState`: open, transition status, side, align, anchor-hidden, measured popup size (the `--popup-width/-height` concepts as typed fields).
- [x] `NavigationMenuViewportStyleState`: activation direction, transitioning, measured viewport size.
- [x] `NavigationMenuBackdropStyleState`: open, mounted, transition status.
- [x] `NavigationMenuArrowStyleState`: open, side, align, uncentered.
- [x] `NavigationMenuLinkStyleState`: active.
- [x] `NavigationMenuIconStyleState`: open.
- [x] `NavigationMenuItemStyleState` exists even if initially empty (Base UI's item state is empty), preserving `style_with_state(...)` extensibility.
- [x] Every part that draws has `.style_with_state(...)` taking its component-specific struct; Base UI data attributes and CSS vars appear only as typed style-state fields.

### Tests / verification

Runtime tests (no window):

- [x] Uncontrolled default-closed and `default_value(Some(v))` initial states; controlled value reconciliation and precedence.
- [ ] Controlled callbacks without internal mutation; canceled open, canceled switch, canceled close. *(cancellation lives in the context (`set_value`); not covered by a runtime-only test)*
- [ ] Reason correctness per change path, including `LinkPress` and `ListNavigation`; no duplicate `on_value_change` on keyboard/hover trigger switch. *(dedupe covered by `request_value_deduplicates_and_respects_disabled`; no per-path reason test)*
- [x] Activation direction from trigger-bounds pairs in both orientations, reset on close, `None` on first open.
- [x] Retargeting: anchor identity/bounds switch without close, morph source/target size facts, prev-trigger fallback after active-trigger removal.
- [x] Hover timer generations: delayed open, delayed close, cancellation by re-hover, immediate open when already mounted, patient-click threshold behavior.
- [x] Safe-polygon composition (or the documented interim fallback): inside-region verdicts withhold the pending close, outside runs it, landing disarms.
- [x] Outside-press hit-testing: inside popup, on a trigger of the tree, inside nested popup, and genuinely outside.
- [ ] Nested cascade: nested `LinkPress` close propagates to ancestor roots; non-`LinkPress` nested closes do not. *(nested cascade wiring not implemented — see nested scope note)*
- [x] `instant` classification for initial open and resize.

Rendered tests under `crates/base_gpui/src/navigation_menu/tests/`:

*(Rendered/windowed behavior tests are not included in this first pass; runtime tests plus the gallery demo render cover the checked items below only where marked.)*

- [ ] Trigger click opens, click on active trigger closes; hover opens after delay and unhover closes after close-delay; disabled trigger opens by neither path.
- [ ] Hovering/clicking a different trigger switches content without closing; activation direction updates.
- [ ] Keyboard: arrow-key roving focus across triggers/links; open-key opens with first content focused per GPUI rules; Escape closes with focus returned to the trigger; tab-out closes with `FocusOut` and no focus return.
- [ ] Focus return blocked for hover/outside-press/focus-out closes.
- [ ] Outside click dismisses; click inside the popup does not.
- [ ] `keep_mounted` content and portal behavior (mounted-hidden style state, clean reopen).
- [ ] Positioner side/align/measurement style state; arrow follows retargeting; popup size facts update when switching between different-sized contents.
- [ ] Link `close_on_click(true)` closes with `LinkPress`; `false` keeps open; `active` link style state.
- [ ] Icon open state flips with its item's active status.
- [ ] Nested menu demo scope: nested root opens inline, parent size updates, nested `LinkPress` cascades closed.
- [x] Demo renders in `crates/base_gpui/src/main.rs` without panics.

## Follow-ups to track explicitly if not completed in the first port

- [x] Swapping the interim hover-delay fallback onto the safe-polygon primitive once `issues/add-gpui-safe-polygon-hover-primitive.md` lands (cross-linked both ways).
- [ ] Real size/position morph animation via GPUI `with_animation` if the first pass ships snap-to-size only.
- [ ] Touch/pen pointer-type hover suppression once GPUI exposes pointer-type metadata.
- [ ] `actionsRef`-style manual unmount, together with transition/animation completion infrastructure.
- [ ] AccessKit roles/relationships (menubar-like list semantics, expanded/controls, `aria-current` equivalent for links) when the pinned GPUI revision supports them.
- [ ] Full tab-order parity with Base UI's focus-guard chains if the first pass documents gaps.

## AccessKit accessibility follow-up

Written against the pinned gpui revision per `docs/accesskit-gpui-reference.md`. A part
enters the a11y tree only when it has both a stable `.id(...)` and a `.role(...)`; the
layers below already carry stable keyed ids. Base UI's emitted ARIA (authoritative:
`NavigationMenuTrigger.tsx` `aria-expanded`/`aria-controls`/`aria-owns`,
`NavigationMenuLink.tsx` `aria-current="page"`, `NavigationMenuIcon.tsx` and
`NavigationMenuArrow.tsx` `aria-hidden`, plus the `nav`/`ul`/`li`/`button`/`a` semantic
tags) maps onto AccessKit as follows.

### Per accessible part

- **`NavigationMenuRoot<T>`** (`layers/navigation_menu_root.rs`): Base UI renders
  `<nav>` (or `<div>` when nested). Assign `Role::Navigation` on the non-nested root
  (verify the exact accesskit 0.24 variant name; fall back to `Role::Group` if absent)
  plus a caller-provided `.aria_label(...)` so multiple navigation landmarks are
  distinguishable. Nested roots (`NavigationMenuRootStyleState.nested == true`) get no
  role — plain container.
- **`NavigationMenuList<T>`** (`layers/navigation_menu_list.rs`): Base UI renders
  `<ul>`. Assign `Role::List` and `.aria_orientation(...)` from the root's orientation
  prop (`Orientation::Horizontal`/`Vertical`).
- **`NavigationMenuItem<T>`** (`layers/navigation_menu_item.rs`): Base UI renders
  `<li>`. Assign `Role::ListItem` and set `.aria_position_in_set(i)` /
  `.aria_size_of_set(n)` from the index/count `child_wiring.rs` already assigns while
  walking the list's children.
- **`NavigationMenuTrigger<T>`** (`layers/navigation_menu_trigger.rs`): Base UI renders
  `<button aria-expanded={isActiveItem}>`. Assign `Role::Button` and
  `.aria_expanded(state.open)` from the existing
  `runtime.trigger_state(&value, disabled)` query
  (`NavigationMenuTriggerStyleState.open` is the active-item fact). `.aria_label(...)`
  from a new trigger builder prop (see Labels).
- **`NavigationMenuLink<T>`** (`layers/navigation_menu_link.rs`): Base UI renders
  `<a aria-current={active ? 'page' : undefined}>`. Assign `Role::Link` with
  `.aria_label(...)`; `aria-current` has no builder (see Gaps).
- **`NavigationMenuPopup<T>`** (`layers/navigation_menu_popup.rs`): Base UI renders
  `<nav>`. Assign `Role::Group` with the same `.aria_label(...)` text as the root
  (a second `Navigation` landmark for one menu is noise in AccessKit; document the
  divergence in the layer doc comment).
- **No role (excluded from the tree)**: `NavigationMenuContent`, `NavigationMenuPortal`,
  `NavigationMenuPositioner`, `NavigationMenuViewport`, `NavigationMenuBackdrop` are
  structural (Base UI backdrop/positioner are presentational `div`s);
  `NavigationMenuArrow` and `NavigationMenuIcon` are `aria-hidden` in Base UI — in gpui
  the equivalent is simply giving them no `.role(...)`, which keeps them out of the
  tree entirely. Nothing to add on these seven layers.

### Actions

- `NavigationMenuTrigger` and `NavigationMenuLink` are wired with **`on_mouse_down`,
  not `on_click`** (`navigation_menu_trigger.rs:124`, `navigation_menu_link.rs:77`), so
  the auto-registered `Action::Click` handler from `.on_click` does **not** exist here.
  Add `.on_a11y_action(AccessibleAction::Click, ...)` on both, routing into the exact
  same runtime path the mouse handler uses: the trigger's toggle/retarget branch
  (`runtime.is_active_value` + value-change with reason `TriggerPress`, bypassing the
  patient-click `stick_if_open` window, which is pointer-only), and the link's
  `on_activate` + `close_on_click` `LinkPress` close. Alternatively switch these two
  handlers to `.on_click(...)` and get `Action::Click` for free — either satisfies the
  checklist.
- `Action::Focus` **is** auto-registered: both layers already call
  `.track_focus(&focus_handle.tab_stop(true).tab_index(0))`. Do not re-add it.
- Optional parity: `.on_a11y_action(AccessibleAction::Expand, ...)` /
  `AccessibleAction::Collapse` on the trigger, mapping to the same open/close
  transitions with reason `ListNavigation`-equivalent semantics; skip if AT coverage of
  Click suffices.

### Labels

- Add `.aria_label(impl Into<SharedString>)` builder props on `NavigationMenuRoot`,
  `NavigationMenuTrigger`, `NavigationMenuLink`, and `NavigationMenuPopup`; when omitted
  on trigger/link, derive nothing (no id-reference labelling exists — see Gaps), so the
  demo in `main.rs` should pass explicit labels.
- Where a trigger/link's visible caption is rendered with `text!(...)` (which derives an
  id and *is* mirrored into the tree), switch it to `Text::new_inaccessible(...)`
  whenever the parent sets `.aria_label(...)`, to avoid double announcement. The
  `NavigationMenuIcon`'s children need no change — the icon has no role/id and stays
  out of the tree.

### Gaps (no builder in this gpui revision — do not invent)

- **`aria-controls`** (trigger → popup id) and **`aria-owns`** (trigger's hidden
  viewport-owner span): no relationship builders exist. The `aria-owns` bridge is
  already architecturally obsolete (content renders inside the popup), so it is
  dropped-by-design; `aria-controls` is **omitted + documented**, blocked pending gpui
  upstream relationship support.
- **`disabled` / `aria-disabled`** on the trigger: no `.aria_disabled(...)` exists.
  Fallback: while `NavigationMenuTrigger.disabled` is true, do not register the
  `Action::Click` a11y handler (mirroring the pointer path's early return at
  `navigation_menu_trigger.rs:125`) and document that AT cannot perceive the disabled
  state; track "expose disabled" as blocked pending gpui upstream.
- **`aria-current="page"`** on an active link: no builder. Fallback: omit + document;
  `NavigationMenuLinkStyleState.active` remains the visual-only fact. (Do not abuse
  `.aria_selected` — `Link` is not a selectable widget role.)
- **`aria-haspopup`**: Base UI does not set it here and gpui has no builder — nothing
  to do beyond noting it.
- **Live regions / announcements** for content switches: no API; retargeting the shared
  popup produces no announcement. Omit + document.

### Checklist

- [ ] `NavigationMenuRoot` (non-nested): `.role(Role::Navigation)` (or `Role::Group`
  fallback after verifying accesskit 0.24) + `.aria_label(...)` builder prop.
- [ ] `NavigationMenuList`: `.role(Role::List)` + `.aria_orientation(...)` from root
  orientation.
- [ ] `NavigationMenuItem`: `.role(Role::ListItem)` + `.aria_position_in_set` /
  `.aria_size_of_set` from `child_wiring.rs` indices.
- [ ] `NavigationMenuTrigger`: `.role(Role::Button)` +
  `.aria_expanded(state.open)` from `runtime.trigger_state(...)` +
  `.aria_label(...)` prop.
- [ ] `NavigationMenuLink`: `.role(Role::Link)` + `.aria_label(...)` prop.
- [ ] `NavigationMenuPopup`: `.role(Role::Group)` + `.aria_label(...)`, divergence from
  Base UI's `<nav>` documented in the layer doc comment.
- [ ] `Action::Click` a11y handlers on trigger and link (added explicitly or via
  switching `on_mouse_down` → `on_click`), routed through the same runtime transitions;
  no re-registration of `Action::Focus` (already provided by `track_focus`).
- [ ] Disabled trigger: a11y Click handler withheld while `disabled`; limitation
  documented.
- [ ] Visible trigger/link captions use `Text::new_inaccessible(...)` when the parent
  carries `.aria_label(...)`.
- [ ] Arrow/Icon/Backdrop/Positioner/Portal/Viewport/Content confirmed role-less
  (absent from the a11y tree), matching Base UI's `aria-hidden`/presentational intent.
- [ ] Gaps documented in the module: `aria-controls`, `aria-disabled`,
  `aria-current="page"`, live announcements — all omit + document, blocked pending gpui
  upstream.

## Uncertain items needing confirmation

- **Required item value**: Base UI auto-generates an item value when omitted
  (`useBaseUiId`); with generic `T: Clone + Eq + 'static` there is no portable
  auto-value. Proposal: make `NavigationMenuItem::value(T)` required. Confirm, or
  choose an index-keyed internal fallback that weakens the public value contract.
- **Viewport necessity**: with content rendered directly in the popup, `Viewport` may
  carry no behavior beyond clipping + measured size + activation direction. Keep it as
  a distinct part for Base UI parity (proposed), or fold it into `Popup` and document
  the omission.
- **Nesting scope**: the scoped nested-menu behaviors (independent value state, inline
  rendering, `LinkPress` cascade, size participation, arrow-key non-interception) are
  proposed as first-pass scope because GPUI's direct rendering makes them cheap; confirm
  whether to defer all nesting instead, given this component's low priority.
- **Morph animation in the first pass**: acceptance criteria require morph *facts* and
  deterministic snap behavior, with animation optional. Confirm whether animated
  size/position morphing is required for the first pass or lands as the follow-up.
- **List dismissal ownership**: Base UI wires outside-press dismissal and popup-hover
  close-delay through `List`; in GPUI these could live on the portal/positioner layers
  where bounds are known. The runtime owns the logic either way — confirm the wiring
  layer during implementation.
