# Port Base UI Tabs to GPUI

## Problem

Base UI's React Tabs component has a mature behavioral model that should be recreated for `base_gpui`, but the current GPUI version only preserves a small builder-shaped API surface. It does not yet implement the shared state, tab registration, selection rules, keyboard navigation, panel visibility, or indicator positioning needed to match the Base UI feature set.

The goal is to port the behavior, not the web implementation. Web-only details such as ARIA attributes, DOM ids, SSR hydration scripts, CSS variables, `ResizeObserver`, React hooks, and render props should be dropped or translated into GPUI-native concepts.

Tab values should be generic, using a Rust type parameter constrained around clone/equality semantics, e.g. `T: Clone + Eq + 'static`, instead of hard-coding strings or indexes.

## Scope

Port the Tabs component family from Base UI into GPUI-native components:

- `TabsRoot<T>`
- `TabsList`
- `TabsTab<T>`
- `TabsPanel<T>`
- `TabsIndicator`

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/tabs/root/TabsRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tabs/list/TabsList.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tabs/tab/TabsTab.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tabs/panel/TabsPanel.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/tabs/indicator/TabsIndicator.tsx`

Current GPUI implementation:

- `crates/base_gpui/src/tabs/layers/tabs_root.rs`
- `crates/base_gpui/src/tabs/layers/tabs_list.rs`
- `crates/base_gpui/src/tabs/layers/tabs_tab.rs`
- `crates/base_gpui/src/tabs/layers/tabs_panel.rs`
- `crates/base_gpui/src/tabs/layers/tabs_indicator.rs`
- `crates/base_gpui/src/tabs/child/tabs_child.rs`
- `crates/base_gpui/src/tabs/child/context/tabs_context.rs`
- `crates/base_gpui/src/tabs/child/context/props/tabs_orientation.rs`
- `crates/base_gpui/src/tabs/child/context/props/tabs_props.rs`
- `crates/base_gpui/src/tabs/child/context/runtime/tabs_panel_metadata.rs`
- `crates/base_gpui/src/tabs/child/context/runtime/tabs_runtime.rs`
- `crates/base_gpui/src/tabs/child/context/runtime/tabs_tab_metadata.rs`
- `crates/base_gpui/src/tabs/child/context/state/tabs_state.rs`
- `crates/base_gpui/src/api/child/context/generic_context.rs`

Out of scope / drop from Base UI:

- ARIA roles and attributes.
- DOM `id`, `aria-controls`, and `aria-labelledby` linking.
- React context/hooks implementation details.
- `render` prop support.
- `nativeButton` support.
- SSR, hydration, CSP nonce, and prehydration indicator script.
- CSS variable API for indicator positioning.
- DOM `ResizeObserver`; replace with GPUI bounds/layout mechanisms if available.
- DOM transition attributes like `data-starting-style` / `data-ending-style`.
- Full arbitrary JS-value semantics; use `T: Clone + Eq + 'static`.

## Value change event details follow-up

Current GPUI Tabs support a simple value-change callback:

```rust
.on_value_change(|next: Option<&T>, window: &mut Window, cx: &mut App| {
    // observe next value
})
```

Base UI exposes richer event details through `onValueChange(newValue, eventDetails)`. The current GPUI primitives are sufficient to port the important parts as a Rust-native event/details API; we should not try to port browser event objects directly.

Base UI source audit:

- `TabsRoot.tsx`
  - `onValueChange` computes and attaches `activationDirection` before notifying the caller.
  - User-initiated changes are cancelable through `eventDetails.cancel()`.
  - Automatic fallback notifications are non-cancelable.
  - Automatic fallback reasons are:
    - `initial` for implicit initial selection / fallback when no usable default is present,
    - `disabled` when the uncontrolled selected tab becomes disabled,
    - `missing` when the uncontrolled selected tab is removed or never matches a mounted tab.
  - User-initiated changes use reason `none`.
- `TabsTab.tsx`
  - Click/focus activation passes reason `none` and lets `TabsRoot` fill in `activationDirection`.
  - Base UI includes the native DOM event in details; GPUI should omit this or replace it with a GPUI-native source enum.

Proposed GPUI API shape:

```rust
pub enum TabsValueChangeReason {
    None,
    Initial,
    Disabled,
    Missing,
}

pub enum TabsValueChangeSource {
    Pointer,
    Keyboard,
    Programmatic,
    Automatic,
}

pub struct TabsValueChangeDetails {
    pub reason: TabsValueChangeReason,
    pub activation_direction: TabsActivationDirection,
    pub source: TabsValueChangeSource,
    pub cancelable: bool,
    canceled: bool,
}
```

Potential handler signature:

```rust
.on_value_change(|next, details, window, cx| {
    if details.cancelable {
        details.cancel();
    }
})
```

Implementation notes:

- User-initiated tab click / keyboard activation should use `reason = None`, `cancelable = true`.
- `activate_on_focus = true` keyboard focus activation should also use `reason = None`, `cancelable = true`.
- Automatic fallback should use `reason = Initial | Disabled | Missing`, `source = Automatic`, and `cancelable = false`.
- In uncontrolled mode, call the handler before mutating internal state; skip mutation if a cancelable details object was canceled.
- In controlled mode, call the handler but never mutate internal state.
- Do not expose a DOM-like event object. GPUI action handlers do not provide raw keyboard events, and pointer handlers provide GPUI-specific `ClickEvent` rather than browser events.
- If useful, add source-specific detail later, e.g. `TabsValueChangeSource::Pointer` vs `Keyboard`; keep the initial API minimal.

Implementation checklist for the follow-up:

- [ ] Add `TabsValueChangeReason`.
- [ ] Add `TabsValueChangeSource` or decide to omit source from the first version.
- [ ] Add `TabsValueChangeDetails` with `cancel()`, `is_canceled()`, and `cancelable()` APIs.
- [ ] Change `TabsValueChangeHandler<T>` to receive `&mut TabsValueChangeDetails`.
- [ ] Update user-initiated click and keyboard activation to pass `reason = None` and computed activation direction.
- [ ] Update uncontrolled initial/fallback paths to notify with `Initial`, `Disabled`, or `Missing` as appropriate.
- [ ] Add tests for cancellation in uncontrolled user-initiated changes.
- [ ] Add tests that automatic fallback is non-cancelable.
- [ ] Add tests for emitted reasons and activation direction.

## AccessKit accessibility follow-up

The pinned GPUI revision (`1d029c5ff5654fb1b1e8caf4462993c8ee13a133`, accesskit `0.24.0`) exposes AccessKit-backed builders on `.id(...)` stateful elements. See `docs/accesskit-gpui-reference.md` for the full API surface; everything below is written against that reference and the real implementation in `crates/base_gpui/src/tabs/`.

An element appears in the accessibility tree only when it has both a non-`None` id and a `.role(...)`. `TabsTab<T>` and `TabsList<T>` already call `.id(...)` (the list uses the literal `"tabs-list"` id in `tabs_list.rs`); `TabsPanel<T>` renders a bare `Div` with no id and will need one before it can carry a role.

Base UI source audit (authoritative ARIA output):

- `TabsList.tsx` — `role="tablist"`; `aria-orientation="vertical"` only when vertical (omitted for horizontal); passes through user `aria-label` / `aria-labelledby`.
- `TabsTab.tsx` — `role="tab"`, `aria-selected={active}`, `aria-controls={tabPanelId}` (only when the matching panel is mounted), generated DOM `id` for the panel's `aria-labelledby`, and `focusableWhenDisabled` semantics for disabled tabs.
- `TabsPanel.tsx` — `role="tabpanel"`, `aria-labelledby={correspondingTabId}`, `hidden` + `inert` when inactive, `tabIndex` 0/-1.
- `TabsIndicator.tsx` — no semantic role; purely decorative.

### Per accessible part

- `TabsList<T>` (`layers/tabs_list.rs`, on the existing `base...id("tabs-list")` chain next to `.focusable()` / `.key_context(TABS_LIST_KEY_CONTEXT)`):
  - `.role(Role::TabList)`.
  - `.aria_orientation(Orientation::Vertical)` only when the `orientation` local (from `props.orientation()`) is `TabsOrientation::Vertical`; omit for horizontal, mirroring Base UI.
  - Add an `aria_label(...)` builder on `TabsList<T>` so callers can name the tab list (Base UI accepts `aria-label`); apply it via `.aria_label(...)` when set.
- `TabsTab<T>` (`layers/tabs_tab.rs`, on the existing `base.id(id).track_focus(...)` chain):
  - `.role(Role::Tab)`.
  - `.aria_selected(active)` from the `active` local (i.e. `TabsTabStyleState.active` computed by `runtime.tab_state(...)`).
  - `.aria_position_in_set(index + 1)` from the tab's registered `index` (assigned by `wiring.register_tab(...)` / the `TabsTabMetadata` index), and `.aria_size_of_set(n)` where `n` is the registered tab count — expose a small `TabsRuntime<T>` accessor (e.g. tab count) via `context.read(...)` alongside `tab_state`.
  - `.aria_label(...)` from a new optional builder when the caller's visible label is icon-only or should differ from the text child.
- `TabsPanel<T>` (`layers/tabs_panel.rs`):
  - Give the rendered `base` a stable `.id(...)` (add an `id(...)` builder mirroring `TabsTab<T>::id`), then `.role(Role::TabPanel)` on the active mounted panel.
  - For `keep_mounted = true` inactive panels (currently rendered with `.invisible()`), do **not** set a role/id so the hidden panel stays out of the a11y tree — there is no `aria_hidden`/`inert` builder in this revision (see Gaps).
- `TabsIndicator<T>` (`layers/tabs_indicator.rs`): decorative; assign no role and no aria props so it never enters the accessibility tree.

### Actions

- `Action::Click` is auto-registered by the existing `.on_click(...)` in `TabsTab<T>` (attached only when `selectable`, i.e. enabled and inactive), and `Action::Focus` is auto-registered by the existing `.track_focus(...)` on tabs and `.focusable()` on the list. Do **not** re-add handlers for these.
- The tab's `on_click` closure already routes through `context.select(Some(value), window, cx)` — the same `TabsContext<T>::select` transition used by the keyboard actions (`TabsSelectLeft`/`Right`/`Up`/`Down`/`First`/`Last`, `TabsActivate`), so an AT-dispatched Click takes the identical path, including `on_value_change` notification and controlled/uncontrolled handling.
- Note the parity consequence: disabled and already-active tabs have no `on_click`, so they expose no Click action to AT. That matches Base UI's "click is a no-op" behavior; document it rather than adding a separate `on_a11y_action`.
- No `Increment`/`Decrement`/`SetValue`/`Expand` handlers are needed for Tabs.

### Labels

- Tab names come from the tab's visible child text by default. When a caller sets an explicit `.aria_label(...)` on `TabsTab<T>`, the visible label should be created with `Text::new_inaccessible(...)` instead of `text!(...)` so the name is not announced twice. Same rule for any visible heading a caller pairs with the list's `.aria_label(...)`.
- `TabsList<T>` has no intrinsic name; the new `aria_label(...)` builder is the only naming path in this revision (no `aria-labelledby`, see Gaps).

### Gaps (no gpui builder in this revision — do not invent APIs)

- `disabled` / `aria-disabled` on `TabsTab<T>`: no `.aria_disabled(...)` exists and `write_a11y_info` never sets a disabled flag. Fallback: keep the current behavior (disabled tabs expose no Click action because `on_click` is skipped) and document that AT cannot see a distinct "disabled" state; revisit when gpui upstreams `set_disabled`. This also defers Base UI's `focusableWhenDisabled` parity.
- `aria-controls` (tab → panel) and `aria-labelledby` (panel → tab, list naming): no relationship builders. Fallback: omit the cross-links entirely and rely on `Role::Tab` / `Role::TabPanel` adjacency plus `.aria_label`; track as blocked pending gpui relationship support (`aria-controls`, `aria-labelledby`, `aria-owns` all absent).
- `hidden` / `inert` for `keep_mounted = true` inactive panels: no `aria_hidden`/inert builder. Fallback: withhold the id/role from inactive panels so they are simply absent from the tree, which is semantically equivalent for AT.
- Panel `tabIndex = 0` when open: GPUI focus primitives (`.focusable()` / `.tab_stop(...)`) cover this if we choose to make active panels tab stops; this is a focus-behavior decision, not an a11y gap, but note Base UI makes open panels focusable.

### Implementation checklist for the follow-up

- [ ] Add `.role(Role::TabList)` and vertical-only `.aria_orientation(Orientation::Vertical)` to `TabsList<T>`.
- [ ] Add an `aria_label(...)` builder to `TabsList<T>` and wire it through `.aria_label(...)`.
- [ ] Add `.role(Role::Tab)` and `.aria_selected(active)` to `TabsTab<T>`.
- [ ] Expose registered tab count from `TabsRuntime<T>` and set `.aria_position_in_set(index + 1)` / `.aria_size_of_set(count)` on `TabsTab<T>`.
- [ ] Add an optional `aria_label(...)` builder to `TabsTab<T>`; document pairing it with `Text::new_inaccessible(...)` for the visible label.
- [ ] Add an `id(...)` builder to `TabsPanel<T>` and set `.role(Role::TabPanel)` on the active mounted panel only.
- [ ] Keep `keep_mounted` inactive panels out of the a11y tree (no id/role) and document this as the hidden/inert fallback.
- [ ] Keep `TabsIndicator<T>` role-free and out of the a11y tree.
- [ ] Verify no extra `on_a11y_action` handlers are needed: Click/Focus come from the existing `.on_click` / `.track_focus` / `.focusable()` wiring.
- [ ] Document the disabled-state gap (no `.aria_disabled`) and the missing `aria-controls`/`aria-labelledby` relationships as blocked pending gpui upstream support.
- [ ] Decide whether active `TabsPanel<T>` should be a tab stop (`.focusable()` + `.tab_stop(true)`) to match Base UI's `tabIndex={0}`.
- [ ] Add a11y assertions to `tabs/tests` once gpui exposes an AccessKit-tree test helper.

## Acceptance Criteria

### Module/API surface

- [x] `crates/base_gpui` exists and is registered in the workspace.
- [x] Tabs modules exist for root, list, tab, panel, and indicator.
- [x] `TabsRoot<T>` builder exists.
- [x] `TabsList` builder exists.
- [x] `TabsTab<T>` builder exists.
- [x] `TabsPanel<T>` builder exists.
- [x] `TabsIndicator` builder exists.
- [x] `TabsRoot<T>` has `default_value(...)` builder API.
- [x] `TabsRoot<T>` has controlled `value(...)` builder API.
- [x] `TabsRoot<T>` has `on_value_change(...)` builder API.
- [x] `TabsRoot<T>` has `orientation(...)` builder API.
- [x] `TabsList` has `activate_on_focus(...)` builder API.
- [x] `TabsList` has `loop_focus(...)` builder API.
- [x] `TabsTab<T>` has `value(...)` builder API.
- [x] `TabsTab<T>` has `disabled(...)` builder API.
- [x] `TabsPanel<T>` has `value(...)` builder API.
- [x] `TabsPanel<T>` has `keep_mounted(...)` builder API.
- [x] Public API consistently constrains tab values as `T: Clone + Eq + 'static`.
- [x] `tabs/mod.rs` exposes ergonomic barrel exports for component names.

### Correctness / compile readiness

- [x] `crates/base_gpui` passes `cargo check -p base_gpui`.
- [x] `TabsPanel<T>::default()` initializes `keep_mounted`, not a nonexistent `disabled` field.
- [x] React-style `utils/use_controlled.rs` is replaced with a small Rust helper for selecting controlled vs internal values.
- [x] Dead fields are either used by behavior or intentionally documented.
- [x] Add a small example/demo using the Tabs components.

### Architecture / internal primitives

- [x] Add typed Tabs child tree so `TabsRoot<T>` can drill shared state before `AnyElement` erasure.
- [x] Add reusable `GenericState` trait for component state containers.
- [x] Add reusable `GenericChild` trait for state-context propagation through child layers.
- [x] Add reusable `GenericContext<S, P, R>` helper for controlled/uncontrolled state resolution backed by GPUI keyed entity state plus injected component props and runtime state.
- [x] Keep `GenericContext<S, P, R>` limited to generic storage/state/runtime mutation mechanics.
- [x] Keep tabs-specific behavior and runtime metadata registration APIs on `TabsContext<T>`.
- [x] Let `TabsContext<T>` construct and own the initial `TabsRuntime<T>` instead of leaking runtime construction to `TabsRoot<T>`.
- [x] Keep tab/panel metadata extraction on `TabsTab<T>` / `TabsPanel<T>` and route metadata insertion through `TabsContext<T>`.
- [x] Reorganize `base_gpui` architecture into `api` and component-specific `child/context/{props,runtime,state}` plus `layers` folders.
- [x] Document the `base_gpui` component architecture and responsibility boundaries in `docs/base-gpui-component-architecture.md`.

### Stateful/stateless behavior

- [x] Support Base UI-style controlled/stateless usage: caller passes `value(...)` and owns the selected tab.
- [x] Support Base UI-style uncontrolled/stateful usage: caller passes `default_value(...)` and Tabs owns the selected tab internally.
- [x] Define clear precedence: if `value(...)` is supplied, the root is controlled; otherwise it is uncontrolled.
- [x] In controlled mode, user interaction calls `on_value_change(...)` but does not mutate internal selected value.
- [x] In uncontrolled mode, user interaction calls `on_value_change(...)` and then mutates internal selected value unless canceled/blocked by API design.
- [x] In uncontrolled mode, automatic fallback changes mutate internal selected value.
- [x] In controlled mode, automatic fallback does not override the caller-provided value.
- [x] Decide implementation style: `window.use_keyed_state(...)`, explicit `Entity<TabsState<T>>`, or both.
- [x] If using `window.use_keyed_state(...)`, require stable `ElementId` on `TabsRoot<T>`.
- [ ] Document usage examples for both controlled and uncontrolled Tabs.

### Shared state model

- [x] Introduce GPUI-native shared tabs state owned by `TabsRoot<T>` or `Entity<TabsState<T>>`.
- [x] Track current selected value as `Option<T>`.
- [x] Track whether the root is controlled or uncontrolled.
- [x] Track orientation as a typed enum instead of free-form string.
- [x] Track registered tabs in order.
- [x] Track tab metadata: value, disabled state, index, and measured bounds.
- [x] Pre-register tab runtime metadata before fallback so uncontrolled fallback uses the current child tree.
- [x] Track registered panels in order.
- [x] Track panel metadata: value and index.
- [x] Pre-register panel runtime metadata before fallback/render.
- [x] Track highlighted/focused tab index separately from selected value.
- [x] Track tab activation direction: `Left`, `Right`, `Up`, `Down`, `None`.
- [x] Support dynamic registration/unregistration of tabs and panels.

### Selection behavior

- [x] In uncontrolled mode, initialize selection from `default_value` when provided.
- [x] In uncontrolled mode, when no `default_value` is provided, select the first enabled tab.
- [x] Support `None` as selected value, meaning no active tab.
- [x] Clicking an enabled inactive tab selects it.
- [x] Clicking the already active tab is a no-op.
- [x] Clicking a disabled tab is a no-op.
- [x] User-initiated selection calls `on_value_change`.
- [x] Controlled mode calls `on_value_change` but does not mutate internal selected value.
- [x] Uncontrolled mode mutates internal selected value after change notification.
- [x] If selected tab becomes disabled in uncontrolled mode, fall back to first enabled tab or `None`.
- [x] If selected tab is removed in uncontrolled mode, fall back to first enabled tab or `None`.
- [x] Controlled mode preserves the externally supplied value even if disabled or missing.
- [x] Automatic fallback changes have activation direction `None`.
- [x] User-initiated changes compute activation direction from registered tab order.
- [ ] User-initiated changes compute activation direction from measured tab bounds.

### Keyboard/focus behavior

- [x] Tabs use GPUI key dispatch actions and a `TabsList` key context instead of raw key-down handlers.
- [x] Arrow key navigation updates highlight for horizontal tabs: previous/next via left/right.
- [x] Arrow key navigation updates highlight for vertical tabs: previous/next via up/down.
- [x] `Home` moves highlight to the first tab.
- [x] `End` moves highlight to the last tab.
- [x] `loop_focus = true` wraps arrow navigation at the ends.
- [x] `loop_focus = false` clamps arrow navigation at the ends.
- [x] `activate_on_focus = true` activates the highlighted enabled tab.
- [x] `activate_on_focus = false` only moves highlight with arrows.
- [x] `Enter` activates the highlighted tab when `activate_on_focus = false`.
- [x] `Space` activates the highlighted tab when `activate_on_focus = false`.
- [x] Keyboard navigation moves GPUI focus to the highlighted tab like Base UI's roving focus.
- [x] Disabled tabs are never activated by keyboard or pointer interaction.
- [x] Highlighted tab stays synchronized with externally controlled value when appropriate.

### Panel behavior

- [x] `TabsPanel<T>` renders as visible when its value equals selected value.
- [x] `TabsPanel<T>` is hidden or omitted when its value does not equal selected value.
- [x] `keep_mounted = false` means inactive panels are not rendered.
- [x] `keep_mounted = true` means inactive panels remain mounted but hidden.
- [x] Panels receive/access state needed for styling: hidden, orientation, activation direction.

### Indicator behavior

- [x] `TabsIndicator` renders only when a tab is selected.
- [x] Indicator can follow the active tab when styled from active tab position/size state.
- [x] Indicator can access active tab bounds/size through GPUI-native measurement.
- [x] Indicator updates when selected tab changes.
- [x] Indicator updates when tab/list layout changes via `Div::on_children_prepainted`.
- [x] Indicator exposes activation direction for styling/animation.
- [x] Do not port Base UI CSS variable names directly unless they become useful for GPUI styling.

### Styling/state exposure

- [x] Root exposes orientation and activation direction to styling logic.
- [x] List exposes orientation and activation direction to styling logic.
- [x] Tab computes active, disabled, highlighted, and orientation state; expose it to styling logic.
- [x] Panel computes hidden and orientation state; expose it plus activation direction to styling logic.
- [x] Indicator exposes active tab position/size state.
- [x] Indicator exposes selected, orientation, and activation direction state.

### Tests / verification

- [x] Add tests or examples for uncontrolled initial selection.
- [x] Add tests or examples for controlled selection.
- [x] Add tests or examples for disabled tabs.
- [x] Add tests or examples for fallback when selected tab is disabled.
- [x] Add tests or examples for fallback when selected tab is removed.
- [x] Add tests or examples for click activation.
- [x] Add tests or examples for keyboard navigation.
- [x] Add tests or examples for `activate_on_focus`.
- [x] Add tests or examples for `loop_focus`.
- [x] Add tests or examples for panel visibility and `keep_mounted`.
- [x] Add tests or examples for indicator movement.
