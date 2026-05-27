# Tabs Implementation Notes

This file captures Tabs-specific implementation details for `crates/base_gpui/src/tabs`.

Generic component architecture belongs in `docs/base-gpui-component-architecture.md`. Keep this file focused on Tabs behavior and local contracts.

## Component family

The Tabs port exposes these explicit component names:

- `TabsRoot<T>`
- `TabsList<T>`
- `TabsTab<T>`
- `TabsPanel<T>`
- `TabsIndicator<T>`

Tab values are generic:

```rust
T: Clone + Eq + 'static
```

Do not hard-code string or index values into the component model.

## Controlled value semantics

`TabsRoot<T>` uses `Option<Option<T>>` for the controlled value prop:

- `None` = uncontrolled prop absent
- `Some(None)` = controlled empty selection
- `Some(Some(value))` = controlled selected value

This distinction is important. Do not collapse it.

Behavior:

- Controlled mode: caller passes `.value(...)`; interactions call `on_value_change(...)` but do not mutate internal selected value.
- Uncontrolled mode: caller omits `.value(...)`; interactions call `on_value_change(...)` and mutate internal selected value.
- `default_value(...)` initializes uncontrolled state.
- If no uncontrolled default is provided, fallback selects the first enabled tab.
- Controlled mode preserves externally supplied values even if disabled or missing.
- Uncontrolled fallback uses registered tab metadata and falls back to first enabled tab or `None`.

## File layout

Tabs follows the repo architecture:

```text
crates/base_gpui/src/tabs/
  actions.rs
  child/
    tabs_child.rs
    tabs_list_child.rs
    context/
      tabs_context.rs
      props/
        tabs_orientation.rs
        tabs_props.rs
      runtime/
        tabs_activation_direction.rs
        tabs_panel_metadata.rs
        tabs_runtime.rs
        tabs_tab_metadata.rs
        tabs_tab_position.rs
        tabs_tab_size.rs
      state/
        tabs_indicator_render_state.rs
        tabs_list_render_state.rs
        tabs_panel_render_state.rs
        tabs_root_render_state.rs
        tabs_state.rs
        tabs_tab_render_state.rs
  layers/
    tabs_indicator.rs
    tabs_list.rs
    tabs_panel.rs
    tabs_root.rs
    tabs_tab.rs
```

## Context, props, state, runtime

`TabsContext<T>` wraps:

```rust
GenericContext<TabsState<T>, TabsProps<T>, TabsRuntime<T>>
```

Tabs-specific behavior belongs on `TabsContext<T>`, not on `GenericContext`.

`TabsProps<T>` owns injected stable props:

- orientation,
- `on_value_change`.

`TabsState<T>` owns the primary selected value.

`TabsRuntime<T>` owns Tabs-specific runtime facts:

- registered tab metadata,
- registered panel metadata,
- highlighted tab index,
- selected-value sync bookkeeping,
- activation direction bookkeeping,
- measured tab bounds for indicator state,
- tab focus handles for roving focus.

Registered panel metadata is intentionally retained even though panel visibility currently derives directly from selected value during rendering. It preserves a component-owned place for future panel-specific behavior.

## Typed child routing

`TabsRoot<T>` keeps typed `TabsChild<T>` children before GPUI erases to `AnyElement`.

`TabsChild<T>` currently routes:

- `TabsList<T>`
- `TabsPanel<T>`
- `TabsIndicator<T>`

`TabsList<T>` keeps typed `TabsListChild<T>` children so the list can accept both tabs and indicators while preserving tab-only registration and measurement.

`TabsListChild<T>` currently routes:

- `TabsTab<T>`
- `TabsIndicator<T>`

Typed child-routing enums belong under `tabs/child/`, not `tabs/layers/`.

## Runtime registration

Runtime registration is Tabs-specific and should remain explicit.

Current registration flow:

1. `TabsRoot<T>` creates `TabsContext<T>`.
2. `TabsRoot<T>` clears registered tab/panel metadata.
3. `TabsRoot<T>` traverses typed children and pre-registers metadata before fallback/render.
4. `TabsList<T>` registers tab children only.
5. `TabsTab<T>` registers tab metadata and its stable keyed `FocusHandle`.
6. `TabsPanel<T>` registers panel metadata.
7. `TabsRoot<T>` applies uncontrolled fallback after metadata registration.

Do not move runtime mutation back into leaf render paths except through the established registration methods.

Do not generalize runtime registration into `GenericChild`; metadata shapes differ by component.

## Selection and fallback

Selection behavior:

- Enabled inactive tab click selects that tab.
- Clicking active tab is a no-op.
- Clicking disabled tab is a no-op.
- User-initiated selection computes activation direction from registered tab order.
- Automatic fallback has activation direction `None`.
- Disabled or removed selected tabs trigger fallback only in uncontrolled mode.

`TabsContext<T>` owns this behavior through methods such as:

- `select_value(...)`
- `select_highlighted_tab(...)`
- `apply_automatic_fallback(...)`
- `sync_activation_direction_with_selected_value(...)`
- `sync_highlighted_tab_with_selected_value(...)`

## Keyboard and focus

Tabs uses GPUI actions/key dispatch, not raw key-down handlers.

`tabs/actions.rs` defines:

- `TabsSelectLeft`
- `TabsSelectRight`
- `TabsSelectUp`
- `TabsSelectDown`
- `TabsSelectFirst`
- `TabsSelectLast`
- `TabsActivateHighlighted`

`TABS_LIST_KEY_CONTEXT` scopes key bindings to the Tabs list.

Keyboard behavior:

- Horizontal tabs use left/right.
- Vertical tabs use up/down.
- Home moves highlight to first enabled tab.
- End moves highlight to last enabled tab.
- `loop_focus = true` wraps at edges.
- `loop_focus = false` clamps at edges.
- `activate_on_focus = true` selects during navigation.
- `activate_on_focus = false` only moves highlight; Enter/Space activate highlighted tab.
- Disabled tabs are skipped and never activated.

Known GPUI caveat:

- Initial highlighted state is not necessarily actual GPUI focus.
- Keyboard actions require focus to be inside the relevant key context.
- Be careful when moving actual GPUI focus during action dispatch; it can affect subsequent action routing.

## Render-state styling

Tabs exposes state-aware styling through `style_with_state(...)` on:

- `TabsRoot<T>`
- `TabsList<T>`
- `TabsTab<T>`
- `TabsPanel<T>`
- `TabsIndicator<T>`

Render-state structs:

- `TabsRootRenderState`: orientation and activation direction.
- `TabsListRenderState`: orientation and activation direction.
- `TabsTabRenderState`: active, disabled, highlighted, and orientation.
- `TabsPanelRenderState`: hidden, orientation, and activation direction.
- `TabsIndicatorRenderState`: selected state, active tab position/size, orientation, and activation direction.

Do not add DOM-style data attributes or CSS variables unless they become useful GPUI API.

## Indicator measurement

`TabsList<T>` uses `Div::on_children_prepainted(...)` to capture child bounds after layout.

Important details:

- `TabsList<T>` filters bounds through `TabsListChild<T>` so only tab child bounds enter runtime.
- Bounds are registered through `TabsContext<T>`.
- `TabsRuntime<T>` derives active tab position and size.
- `TabsIndicator<T>` exposes those values through `TabsIndicatorRenderState`.

Do not port Base UI indicator CSS variable names directly.

## Panels

Panel behavior:

- Panel is visible when its value equals selected value.
- `keep_mounted = false`: inactive panels are omitted.
- `keep_mounted = true`: inactive panels remain mounted but hidden.
- Panels receive hidden/orientation/activation direction via `TabsPanelRenderState`.

## Base UI differences / intentionally dropped web details

Do not port:

- ARIA roles/attributes,
- DOM id linking (`aria-controls`, `aria-labelledby`),
- React hooks/context details,
- SSR/hydration/prehydration scripts,
- CSS variable API,
- `ResizeObserver` directly,
- DOM transition attributes,
- arbitrary JS value semantics.

Translate behavior into GPUI-native state, runtime, measurement, focus handles, and actions.

## Tests / verification backlog

Current desired verification areas are tracked in:

```text
issues/port-baseui-tabs.md
```

Important remaining areas:

- controlled and uncontrolled usage examples,
- disabled/fallback examples,
- keyboard behavior verification,
- `activate_on_focus`,
- `loop_focus`,
- panel visibility and `keep_mounted`,
- indicator movement.
