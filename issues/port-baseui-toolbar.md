# Port Base UI Toolbar to GPUI

## Problem

Base UI Toolbar is a container for grouping a set of controls — buttons, links, inputs, toggles, toggle groups, and trigger-hosting buttons — under one roving-focus keyboard model. The toolbar owns a single tab stop; arrow keys move focus between registered items along the toolbar orientation, wrapping when `loopFocus` is on, and skipping items that are disabled and not focusable-when-disabled. A toolbar-level `disabled` flag cascades through groups onto items, but items default to `focusableWhenDisabled = true`, so disabled items stay in the roving order and remain hoverable (important for tooltips on disabled controls). Separators inside the toolbar automatically render perpendicular to the toolbar orientation.

`crates/base_gpui` has no Toolbar component family. The goal is to port Toolbar behavior into GPUI-native components using the current runtime/context/layers architecture — not to copy React context, `CompositeRoot`/`CompositeItem`, DOM roles, or ARIA implementation details. Roving focus and item registration remain per-component, following the precedents in `crates/base_gpui/src/tabs/` (`runtime.rs` `move_highlight` / highlighted index / focus handles, `child_wiring.rs` index assignment + `focus_handles: Vec<(usize, FocusHandle)>`) and `crates/base_gpui/src/radio_group/`. No new shared primitive is needed: GPUI provides `tab_stop` / `tab_index` out of the box, `Toolbar.Input` reuses the ported `crates/base_gpui/src/input/` component (backed by `primitives/input`), and `Toolbar.Separator` reuses the ported `crates/base_gpui/src/separator/`. gpui-component has no toolbar analog to borrow from.

The toolbar holds no selection value, so no generic `T` value parameter is needed. Complexity is medium: the deep work is the roving-focus runtime with metadata-driven skip logic, the disabled cascade, the input caret-edge interplay, and — critically — an item-registration contract designed so the separately issued Toggle (`issues/port-baseui-toggle.md`) and Toggle Group (`issues/port-baseui-toggle-group.md`) ports can plug into the toolbar's roving focus later.

## Scope

Port the Toolbar component family from Base UI into GPUI-native components:

- `ToolbarRoot`
- `ToolbarButton`
- `ToolbarLink`
- `ToolbarInput`
- `ToolbarGroup`
- `ToolbarSeparator`

Base UI reference files:

- `/home/luke/Projects/base-ui/packages/react/src/toolbar/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/root/ToolbarRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/root/ToolbarRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/root/ToolbarRootDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/root/ToolbarRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/button/ToolbarButton.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/button/ToolbarButtonDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/button/ToolbarButton.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/link/ToolbarLink.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/link/ToolbarLinkDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/link/ToolbarLink.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/input/ToolbarInput.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/input/ToolbarInputDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/input/ToolbarInput.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/group/ToolbarGroup.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/group/ToolbarGroupContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/group/ToolbarGroupDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/group/ToolbarGroup.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/separator/ToolbarSeparator.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toolbar/separator/ToolbarSeparatorDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/internals/composite/root/useCompositeRoot.ts` (behavioral reference only — roving focus, RTL key flipping, input caret-edge rules, disabled-index skipping)
- `/home/luke/Projects/base-ui/packages/react/src/toggle/Toggle.tsx` (toolbar item-metadata integration reference)
- `/home/luke/Projects/base-ui/packages/react/src/toggle-group/ToggleGroup.tsx` (toolbar-nesting reference: disables its own roving focus inside a toolbar)

Current GPUI implementation:

- No `crates/base_gpui/src/toolbar/` implementation exists yet.

Expected GPUI implementation files:

- `crates/base_gpui/src/toolbar/mod.rs`
- `crates/base_gpui/src/toolbar/actions.rs`
- `crates/base_gpui/src/toolbar/child.rs`
- `crates/base_gpui/src/toolbar/child_wiring.rs`
- `crates/base_gpui/src/toolbar/context.rs`
- `crates/base_gpui/src/toolbar/props.rs`
- `crates/base_gpui/src/toolbar/style_state.rs`
- `crates/base_gpui/src/toolbar/runtime.rs`
- `crates/base_gpui/src/toolbar/layers/mod.rs`
- `crates/base_gpui/src/toolbar/layers/toolbar_root.rs`
- `crates/base_gpui/src/toolbar/layers/toolbar_button.rs`
- `crates/base_gpui/src/toolbar/layers/toolbar_link.rs`
- `crates/base_gpui/src/toolbar/layers/toolbar_input.rs`
- `crates/base_gpui/src/toolbar/layers/toolbar_group.rs`
- `crates/base_gpui/src/toolbar/layers/toolbar_separator.rs`
- `crates/base_gpui/src/toolbar/tests/`

Reused existing GPUI components:

- `crates/base_gpui/src/input/` (+ `crates/base_gpui/src/primitives/input`) — `ToolbarInput` wraps the ported `Input`.
- `crates/base_gpui/src/separator/` — `ToolbarSeparator` wraps the ported `Separator` with auto-perpendicular orientation.
- `crates/base_gpui/src/tabs/` and `crates/base_gpui/src/radio_group/` — roving-focus, item registration, and child-wiring precedents.
- `issues/port-baseui-direction-provider.md` — ambient LTR/RTL direction for horizontal arrow-key flipping.

## Out of scope / drop from Base UI

- Do not port React context/hooks directly; use GPUI keyed state/entities plus a thin `ToolbarContext` wrapper.
- Do not port `CompositeRoot` / `CompositeItem` / `CompositeMetadata` as a shared primitive; translate the composite behavior into the toolbar's own runtime registration, focus handles, and key-dispatch actions, following the tabs and radio-group per-component precedent.
- Do not port `render` props. Trigger-hosting (`Toolbar.Button` rendered as a Menu/Select/Dialog trigger) is deferred to the Toggle/ToggleGroup integration contract below and to future per-component wiring.
- Do not port `className`.
- Do not port web `style` props; expose GPUI builder styling and `style_with_state(...)`.
- Do not port `nativeButton` / `useButton`; GPUI interactive items are built from `div()` plus focus/click/action behavior.
- Do not port `<a href>` semantics; `ToolbarLink` is a plain focusable styled item in GPUI with an `on_click` action, no anchor element.
- Do not port DOM `role="toolbar"` / `role="group"` / `aria-orientation`; map accessibility through GPUI-native AccessKit APIs once available.
- Do not port DOM data attributes (`data-disabled`, `data-orientation`, `data-focusable`) as attributes; map them into typed style-state structs.
- Do not port SSR/hydration APIs.
- Do not port CSS variable APIs.
- Do not port DOM event objects, `event.preventDefault()` plumbing, or `stopEventPropagation`; use GPUI events and actions.
- Do not port `Map<Node, CompositeMetadata>` DOM-node-keyed registration; item identity is source-order index assigned by child wiring.
- Do not port the composite `grid`/cols navigation or `highlightItemOnHover`; the toolbar never enables them.

## Toggle/ToggleGroup integration contract

Base UI's design seam: `Toggle` and `ToggleGroup` integrate as toolbar items through the shared `ToolbarRootContext` + `ToolbarRoot.ItemMetadata` registration channel, not through toolbar-specific components.

- A `Toggle` inside a toolbar renders as a composite item and registers metadata `{ disabled, focusableWhenDisabled: false }` — a disabled toggle drops out of roving focus, unlike a toolbar button.
- A `ToggleGroup` nested inside a toolbar disables its own `CompositeRoot` roving focus and renders as a plain `role="group"`; its child toggles register directly as toolbar items, flattened into the toolbar's single roving order. The toolbar owns all arrow-key navigation.
- `ToggleGroup` merges toolbar and toolbar-group disabled state into its own disabled state, which cascades to its toggles.

The GPUI port must design the registration API so the separately issued `issues/port-baseui-toggle.md` and `issues/port-baseui-toggle-group.md` ports can hook in later without reworking the toolbar:

- `ToolbarItemMetadata { disabled: bool, focusable_when_disabled: bool }` is a public type, and item registration flows through toolbar child wiring plus a `ToolbarContext` that future child variants can consume.
- `ToolbarChild` / `ToolbarGroupChild` enums are designed to gain `Toggle(...)` and `ToggleGroup(...)` variants later; a nested `ToggleGroup` contributes its toggles as individual toolbar items (one roving slot per toggle, none for the group container) rather than acting as one item.
- Nested compound children must be able to detect the enclosing toolbar (context presence) and suppress their own roving focus, mirroring Base UI's optional `useToolbarRootContext(true)`.

Cross-link: when `issues/port-baseui-toggle.md` and `issues/port-baseui-toggle-group.md` are written/implemented, they must reference this contract instead of inventing a second registration channel.

## Acceptance Criteria

### Module/API surface

- [ ] Add a `toolbar` module and export it from `crates/base_gpui/src/lib.rs`.
- [ ] Register Toolbar key bindings from `base_gpui::init(cx)`.
- [ ] Add public `ToolbarRoot`, `ToolbarButton`, `ToolbarLink`, `ToolbarInput`, `ToolbarGroup`, and `ToolbarSeparator` layer types.
- [ ] `ToolbarRoot` supports `.orientation(ToolbarOrientation)` with `Horizontal` as the default; orientation is a typed enum, not a string.
- [ ] `ToolbarRoot` supports `.loop_focus(bool)`, defaulting to `true`.
- [ ] `ToolbarRoot` supports `.disabled(bool)`, defaulting to `false`.
- [ ] `ToolbarButton` supports `.disabled(bool)` (default `false`), `.focusable_when_disabled(bool)` (default `true`), and `.on_click(...)`.
- [ ] `ToolbarLink` supports `.on_click(...)` and has no disabled builder; links can never be disabled.
- [ ] `ToolbarInput` supports `.disabled(bool)` (default `false`) and `.focusable_when_disabled(bool)` (default `true`), and forwards the ported `Input` component's value/change builder APIs rather than duplicating them.
- [ ] `ToolbarGroup` supports `.disabled(bool)`, defaulting to `false`.
- [ ] `ToolbarSeparator` supports an optional `.orientation(SeparatorOrientation)` override; when not set, orientation is derived from the toolbar (perpendicular).
- [ ] Add a typed `ToolbarChild` enum routing `Button`, `Link`, `Input`, `Group`, and `Separator` before `AnyElement` erasure.
- [ ] Add a typed `ToolbarGroupChild` enum for group children (`Button`, `Link`, `Input`) before `AnyElement` erasure.
- [ ] Decide whether `ToolbarChild` needs an `AnyElement` escape hatch: Base UI's docs demo places non-toolbar wrappers (e.g. `Menu.Root` around a trigger button) as direct root children; record the decision in this issue before implementation.
- [ ] Expose `ToolbarItemMetadata { disabled, focusable_when_disabled }` as a public type for the Toggle/ToggleGroup integration contract.
- [ ] `toolbar/mod.rs` exposes barrel exports only for component names, style states, context, props, runtime, actions, metadata, and child types; no items defined in `mod.rs`.
- [ ] No `pub(crate)` / `pub(super)` restricted visibility; items are private or fully `pub`, and the module passes the repo's ast-grep lint checks.

### Correctness / compile readiness

- [ ] `cargo check -p base_gpui` passes.
- [ ] `cargo test -p base_gpui toolbar` passes.
- [ ] The component compiles without adding web/React-specific concepts (`className`, render props, DOM roles, native-button switches) to public APIs.
- [ ] The implementation follows `docs/base-gpui-component-architecture.md`: flat module layout (`runtime.rs`, `context.rs`, `props.rs`, `style_state.rs`, `child.rs`, `child_wiring.rs`, `layers/`), no nested `child/context/{props,runtime,state}` taxonomy, no `utils/` folder.
- [ ] Add a small example/demo in `crates/base_gpui/src/main.rs` or a dedicated example rendering a toolbar with buttons, a group, a separator, a link, and an input.

### Architecture / internal primitives

- [ ] Add `ToolbarRuntime` as the single owner of toolbar business state: registered item metadata in source order, focus handles, highlighted/roving-tab-stop index, orientation, loop-focus flag, and toolbar disabled state.
- [ ] `ToolbarRuntime` computes the roving-focus skip set internally from item metadata (`disabled && !focusable_when_disabled`), Base UI's `disabledIndices` equivalent; the skip set never escapes as public state — parts ask item-shaped questions only.
- [ ] `ToolbarRuntime` exposes commands (`sync_children`, `move_highlight`, `set_highlight`/click-focus, initial-tab-stop resolution) and per-part style-state queries; no getter/setter pairs.
- [ ] Add `ToolbarProps` for stable root configuration (orientation, loop focus, disabled).
- [ ] Add `ToolbarContext` as a thin injection type with only `read(...)` / `update(...)`-style methods; toolbar vocabulary (registration, highlight movement) lives on the runtime.
- [ ] Child wiring in `child_wiring.rs` is the only place that assigns item indices; it flattens group children into the toolbar's single item order (groups occupy no roving slot) and collects `Vec<ToolbarItemMetadata>` plus `focus_handles: Vec<(usize, FocusHandle)>`, following `crates/base_gpui/src/tabs/child_wiring.rs`.
- [ ] Separators and groups register no item metadata; only buttons, links, and inputs occupy roving slots.
- [ ] Renderable GPUI elements live only under `toolbar/layers/`; child routing lives in `child.rs`, not `layers/`.
- [ ] Root render is the single mutation site outside event handlers: wire children once, `sync_children`, resolve the initial tab stop; no index bookkeeping in parts.
- [ ] Do not add new shared generic primitives; roving focus and registration stay toolbar-local per the tabs/radio-group precedent.

### Item registration and metadata

- [ ] Each item registers `ToolbarItemMetadata { disabled, focusable_when_disabled }` with its effective (cascaded) disabled state, matching Base UI's `ItemMetadata`.
- [ ] `ToolbarButton` registers its merged disabled state and its `focusable_when_disabled` flag (default `true`).
- [ ] `ToolbarLink` always registers `{ disabled: false, focusable_when_disabled: true }`; it occupies a roving slot even when the toolbar is disabled.
- [ ] `ToolbarInput` registers its merged disabled state and its `focusable_when_disabled` flag (default `true`).
- [ ] Items are registered in source order and re-synced on every root render so add/remove/disable changes are reflected without stale indices.
- [ ] Registration flows through the same channel the Toggle/ToggleGroup contract will use; no button-specific registration path.

### Roving focus / keyboard behavior

- [ ] The toolbar is a single tab stop: exactly one item (the current roving tab stop) participates in window Tab order, via GPUI `tab_stop` / `tab_index`.
- [ ] Toolbar keyboard handling uses GPUI actions and a toolbar key context (`actions.rs` + `key_context(...)` + `on_action(...)`), not raw key-down handlers.
- [ ] Horizontal orientation: ArrowRight moves highlight/focus to the next item and ArrowLeft to the previous item in ambient LTR direction.
- [ ] Horizontal orientation: ArrowLeft moves next and ArrowRight moves previous in ambient RTL direction (consume the shared direction primitive from `issues/port-baseui-direction-provider.md`).
- [ ] Vertical orientation: ArrowDown moves next and ArrowUp moves previous; direction does not flip vertical arrows.
- [ ] Perpendicular arrows are inert: vertical arrows do nothing in a horizontal toolbar and horizontal arrows do nothing in a vertical toolbar.
- [ ] `loop_focus = true` wraps arrow navigation at both ends; `loop_focus = false` clamps at the ends.
- [ ] Home and End do not navigate: Base UI Toolbar leaves `enableHomeAndEndKeys` off (`useCompositeRoot.ts` defaults it to `false` and `ToolbarRoot.tsx` never enables it).
- [ ] Arrow navigation skips items whose metadata is `disabled && !focusable_when_disabled`.
- [ ] Disabled items with `focusable_when_disabled = true` (the default) remain in the roving order and receive focus during arrow navigation.
- [ ] Arrow navigation moves real GPUI focus (via the stored `FocusHandle`s) to the newly highlighted item, and the roving tab stop follows it.
- [ ] The initial tab stop is the first item not excluded by the skip set; a disabled, non-focusable first item never becomes the initial tab stop.
- [ ] Clicking or focusing an item makes it the current roving tab stop, so tabbing away and back returns to that item rather than resetting to the first.
- [ ] Modifier-held arrow presses (other than plain Shift text-selection inside the input, covered below) do not navigate, matching composite modifier handling.

### Disabled cascade behavior

- [ ] Effective item disabled state is `toolbar.disabled || group.disabled || item.disabled` for buttons and inputs.
- [ ] `ToolbarGroup` merges `toolbar.disabled || group.disabled` and cascades the merged value to its contained items.
- [ ] `ToolbarLink` ignores the cascade entirely: toolbar/group disabled never disables a link, matching Base UI's "disables all toolbar items except links" behavior.
- [ ] Disabling the toolbar or a group updates the registered metadata and therefore the skip set on the next render, without any part diffing previous values.

### Button behavior

- [ ] An enabled `ToolbarButton` fires `on_click` on pointer click.
- [ ] An enabled, focused `ToolbarButton` activates via Enter and Space through the toolbar key context (GPUI has no native button activation).
- [ ] A disabled `ToolbarButton` never fires `on_click` from pointer or keyboard.
- [ ] A disabled `ToolbarButton` with `focusable_when_disabled = true` still receives hover state so tooltip-style interactions on disabled buttons keep working.
- [ ] A disabled `ToolbarButton` with `focusable_when_disabled = false` is removed from roving focus and cannot be focused by arrow navigation.
- [ ] The button is designed to later host trigger-style children (menu/select/dialog triggers) without changing the toolbar registration contract; do not implement trigger hosting in this issue.

### Link behavior

- [ ] `ToolbarLink` renders as a plain focusable styled item (no anchor semantics) and fires `on_click` on click, Enter, and Space.
- [ ] `ToolbarLink` occupies exactly one roving slot and is always navigable, including when the toolbar or an enclosing group is disabled.

### Input behavior

- [ ] `ToolbarInput` wraps the existing `crates/base_gpui/src/input/` component; text editing, value state, and change callbacks come from the reused input, not a reimplementation.
- [ ] `ToolbarInput` is a composite item: it occupies one roving slot and arrow navigation can move focus into and out of it.
- [ ] While the input has focus, a forward arrow only moves roving focus to the next item when the caret sits at the end of the text with no selection; otherwise the arrow performs native caret movement.
- [ ] While the input has focus, a backward arrow only moves roving focus to the previous item when the caret sits at position 0 with no selection; otherwise the arrow performs native caret movement.
- [ ] Shift+arrow inside the input always performs text selection and never navigates the toolbar.
- [ ] When roving focus moves into the input, its text content is fully selected, matching the composite `onFocus` select-all behavior.
- [ ] A disabled `ToolbarInput` does not accept text input or trap focus; with `focusable_when_disabled = false` it is skipped by roving focus, and with the default `true` it stays in the roving order.
- [ ] If the ported input does not yet expose caret/selection queries needed for the edge checks, extend `crates/base_gpui/src/primitives/input` rather than duplicating editing logic in the toolbar.

### Group behavior

- [ ] `ToolbarGroup` renders as a plain container (Base UI `role="group"` div) and is not a composite item: it has no focus handle and no roving slot.
- [ ] Group children participate in the toolbar's roving order exactly as if they were direct toolbar children (flattened indices).
- [ ] Group disabled state cascades to contained buttons and inputs but not links.

### Separator behavior

- [ ] `ToolbarSeparator` wraps the existing `crates/base_gpui/src/separator/` component.
- [ ] Default orientation is perpendicular to the toolbar: a horizontal toolbar renders vertical separators and a vertical toolbar renders horizontal separators.
- [ ] An explicit `.orientation(...)` on `ToolbarSeparator` overrides the derived perpendicular orientation.
- [ ] Separators are not composite items and are never focused by roving navigation.

### Toggle/ToggleGroup integration seam

- [ ] `ToolbarItemMetadata` and the registration path are public/extensible enough that a future `Toggle` port can register `{ disabled, focusable_when_disabled: false }` as a toolbar item without toolbar changes beyond a new child variant.
- [ ] The child-wiring design documents (in code comments or the child enum docs) how a future `ToggleGroup` variant contributes its child toggles as individual flattened toolbar items while the group container itself takes no roving slot.
- [ ] A future toolbar-nested `ToggleGroup` must be able to suppress its own roving focus when a toolbar context is present; the toolbar context is designed to be optionally detectable by nested compound children.
- [ ] Toolbar/group disabled cascade reaches future nested `ToggleGroup`/`Toggle` items through the same merged-disabled rule used for buttons and inputs.
- [ ] Cross-link this contract from `issues/port-baseui-toggle.md` and `issues/port-baseui-toggle-group.md` when those issues are written.

### Styling/state exposure

- [ ] Add `ToolbarRootStyleState { disabled, orientation }`.
- [ ] Add `ToolbarButtonStyleState` with at least `disabled`, `orientation`, `focusable` (the `focusable_when_disabled` fact), plus GPUI-native `focused`/tab-stop facts.
- [ ] Add `ToolbarLinkStyleState { orientation }` plus GPUI-native focused facts.
- [ ] Add `ToolbarInputStyleState` with at least `disabled`, `orientation`, `focusable`, composing the reused input's style state rather than hiding it.
- [ ] Add `ToolbarGroupStyleState { disabled, orientation }`.
- [ ] `ToolbarSeparator` styling flows through the existing `SeparatorStyleState`.
- [ ] Expose `style_with_state(...)` on every part that draws.
- [ ] Map Base UI data attributes (`data-disabled`, `data-orientation`, `data-focusable`) into these typed style-state fields; no DOM attributes, `className`, or CSS variables in the public surface.

### Accessibility follow-up

- [ ] Once the target GPUI revision supports the needed AccessKit APIs, map the root to the AccessKit toolbar role with orientation, groups to the group role, and expose item disabled state; do not port DOM ARIA attributes in this issue.

### Tests / verification

Add one behavior per file under `crates/base_gpui/src/toolbar/tests/`.

- [ ] Horizontal LTR ArrowRight/ArrowLeft roving navigation moves focus next/previous.
- [ ] Horizontal RTL flips ArrowLeft/ArrowRight navigation under a DirectionProvider.
- [ ] Vertical ArrowDown/ArrowUp roving navigation moves focus next/previous.
- [ ] Perpendicular arrows do not navigate.
- [ ] `loop_focus = true` wraps at both ends; `loop_focus = false` clamps.
- [ ] Home and End do not navigate.
- [ ] A disabled item with `focusable_when_disabled = false` is skipped by arrow navigation.
- [ ] A disabled item with default `focusable_when_disabled = true` is retained in the roving order and receives focus.
- [ ] The initial tab stop moves off a disabled, non-focusable first item.
- [ ] Clicking an item makes it the roving tab stop; tabbing away and back returns to it.
- [ ] Toolbar `disabled` cascades to buttons and inputs but not links.
- [ ] Group `disabled` cascades to contained buttons and inputs but not links.
- [ ] Disabled button click and Enter/Space do not fire `on_click`.
- [ ] Enabled button fires `on_click` from pointer and from Enter/Space.
- [ ] Link stays navigable and clickable inside a disabled toolbar.
- [ ] Input: forward arrow at caret-end leaves the input; forward arrow mid-text moves the caret instead.
- [ ] Input: backward arrow at caret position 0 leaves the input; backward arrow mid-text moves the caret instead.
- [ ] Input: Shift+arrow selects text and never navigates.
- [ ] Input: roving focus into the input selects all its text.
- [ ] Disabled input is skipped when `focusable_when_disabled = false` and does not block vertical/horizontal roving traversal.
- [ ] Separator renders perpendicular to toolbar orientation by default and honors an explicit orientation override.
- [ ] `style_with_state(...)` receives correct root, button, link, input, and group state (including `focusable` on disabled-but-focusable items).

## Uncertain / needs confirmation

- Home/End: Base UI Toolbar verifiably does not enable Home/End navigation (`enableHomeAndEndKeys` defaults to `false` in `useCompositeRoot.ts` and `ToolbarRoot.tsx` does not pass it), so this issue specifies Home/End as inert for parity. Confirm whether GPUI Toolbar should instead opt into Home/End as first/last-item navigation, as the tabs port does.
- `ToolbarChild` escape hatch: Base UI's docs demo composes non-toolbar wrappers (e.g. `Menu.Root` wrapping a trigger `Toolbar.Button`) as direct root children. Confirm whether the GPUI child enum should include an `AnyElement` variant now or defer until menu/select trigger hosting is designed.
- Hover state on disabled-but-focusable buttons exists in Base UI to keep tooltips working; confirm how much of this matters before the tooltip component integrates with toolbar items.
- `ToolbarLink` keyboard activation (Enter/Space) is a GPUI-native decision; DOM anchors activate on Enter only. Confirm the desired activation keys.
