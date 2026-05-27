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
