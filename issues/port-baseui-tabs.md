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
- `crates/base_gpui/src/tabs/state/tabs_state.rs`
- `crates/base_gpui/src/tabs/state/tabs_props.rs`
- `crates/base_gpui/src/tabs/child/tabs_child.rs`
- `crates/base_gpui/src/utils/state/controlled_context.rs`

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
- [ ] Dead fields are either used by behavior or intentionally documented.
- [ ] Add a small example/demo using the Tabs components.

### Architecture / internal primitives

- [x] Add typed Tabs child tree so `TabsRoot<T>` can drill shared state before `AnyElement` erasure.
- [x] Add reusable `GenericState` trait for component state containers.
- [x] Add reusable `GenericChild` trait for state-context propagation through child layers.
- [x] Add reusable `ControlledContext<S, D>` helper for controlled/uncontrolled state resolution backed by GPUI keyed entity state plus injected component props.
- [x] Reorganize `base_gpui` architecture into `api`, `utils`, and component-specific `state`, `child`, and `layers` folders.

### Stateful/stateless behavior

- [x] Support Base UI-style controlled/stateless usage: caller passes `value(...)` and owns the selected tab.
- [x] Support Base UI-style uncontrolled/stateful usage: caller passes `default_value(...)` and Tabs owns the selected tab internally.
- [x] Define clear precedence: if `value(...)` is supplied, the root is controlled; otherwise it is uncontrolled.
- [x] In controlled mode, user interaction calls `on_value_change(...)` but does not mutate internal selected value.
- [x] In uncontrolled mode, user interaction calls `on_value_change(...)` and then mutates internal selected value unless canceled/blocked by API design.
- [ ] In uncontrolled mode, automatic fallback changes mutate internal selected value.
- [ ] In controlled mode, automatic fallback does not override the caller-provided value.
- [x] Decide implementation style: `window.use_keyed_state(...)`, explicit `Entity<TabsState<T>>`, or both.
- [x] If using `window.use_keyed_state(...)`, require stable `ElementId` on `TabsRoot<T>`.
- [ ] If using explicit `Entity<TabsState<T>>`, provide ergonomic wrapper constructors around it.
- [ ] Document usage examples for both controlled and uncontrolled Tabs.

### Shared state model

- [x] Introduce GPUI-native shared tabs state owned by `TabsRoot<T>` or `Entity<TabsState<T>>`.
- [x] Track current selected value as `Option<T>`.
- [x] Track whether the root is controlled or uncontrolled.
- [ ] Track orientation as a typed enum instead of free-form string.
- [ ] Track registered tabs in order.
- [ ] Track tab metadata: value, disabled state, index, and optional measured bounds.
- [ ] Track registered panels in order.
- [ ] Track highlighted/focused tab index separately from selected value.
- [ ] Track tab activation direction: `Left`, `Right`, `Up`, `Down`, `None`.
- [ ] Support dynamic registration/unregistration of tabs and panels.

### Selection behavior

- [x] In uncontrolled mode, initialize selection from `default_value` when provided.
- [ ] In uncontrolled mode, when no `default_value` is provided, select the first enabled tab.
- [x] Support `None` as selected value, meaning no active tab.
- [x] Clicking an enabled inactive tab selects it.
- [x] Clicking the already active tab is a no-op.
- [x] Clicking a disabled tab is a no-op.
- [x] User-initiated selection calls `on_value_change`.
- [x] Controlled mode calls `on_value_change` but does not mutate internal selected value.
- [x] Uncontrolled mode mutates internal selected value after change notification.
- [ ] If selected tab becomes disabled in uncontrolled mode, fall back to first enabled tab or `None`.
- [ ] If selected tab is removed in uncontrolled mode, fall back to first enabled tab or `None`.
- [ ] Controlled mode preserves the externally supplied value even if disabled or missing.
- [ ] Automatic fallback changes have activation direction `None`.
- [ ] User-initiated changes compute activation direction from tab order/bounds.

### Keyboard/focus behavior

- [ ] Arrow key navigation works for horizontal tabs: previous/next via left/right.
- [ ] Arrow key navigation works for vertical tabs: previous/next via up/down.
- [ ] `Home` moves highlight/focus to the first tab.
- [ ] `End` moves highlight/focus to the last tab.
- [ ] `loop_focus = true` wraps arrow navigation at the ends.
- [ ] `loop_focus = false` clamps arrow navigation at the ends.
- [ ] `activate_on_focus = true` activates the highlighted/focused enabled tab.
- [ ] `activate_on_focus = false` only moves highlight/focus with arrows.
- [ ] `Enter` activates the highlighted/focused tab when `activate_on_focus = false`.
- [ ] `Space` activates the highlighted/focused tab when `activate_on_focus = false`.
- [ ] Disabled tabs can be highlighted/focused if matching Base UI behavior is desired.
- [ ] Disabled tabs are never activated by keyboard or pointer interaction.
- [ ] Highlighted tab stays synchronized with externally controlled value when appropriate.

### Panel behavior

- [x] `TabsPanel<T>` renders as visible when its value equals selected value.
- [x] `TabsPanel<T>` is hidden or omitted when its value does not equal selected value.
- [x] `keep_mounted = false` means inactive panels are not rendered.
- [x] `keep_mounted = true` means inactive panels remain mounted but hidden.
- [ ] Panels receive/access state needed for styling: hidden, orientation, activation direction.

### Indicator behavior

- [ ] `TabsIndicator` renders only when a tab is selected.
- [ ] Indicator follows the active tab.
- [ ] Indicator can access active tab bounds/size through GPUI-native measurement.
- [ ] Indicator updates when selected tab changes.
- [ ] Indicator updates when tab/list layout changes, if GPUI exposes the needed layout hooks.
- [ ] Indicator exposes activation direction for styling/animation.
- [ ] Do not port Base UI CSS variable names directly unless they become useful for GPUI styling.

### Styling/state exposure

- [ ] Root exposes orientation and activation direction to styling logic.
- [ ] List exposes orientation and activation direction to styling logic.
- [ ] Tab exposes active, disabled, and orientation state to styling logic.
- [ ] Panel exposes hidden, orientation, and activation direction state to styling logic.
- [ ] Indicator exposes active tab position/size, orientation, and activation direction state.

### Tests / verification

- [ ] Add tests or examples for uncontrolled initial selection.
- [ ] Add tests or examples for controlled selection.
- [ ] Add tests or examples for disabled tabs.
- [ ] Add tests or examples for fallback when selected tab is disabled.
- [ ] Add tests or examples for fallback when selected tab is removed.
- [ ] Add tests or examples for click activation.
- [ ] Add tests or examples for keyboard navigation.
- [ ] Add tests or examples for `activate_on_focus`.
- [ ] Add tests or examples for `loop_focus`.
- [ ] Add tests or examples for panel visibility and `keep_mounted`.
- [ ] Add tests or examples for indicator movement.
