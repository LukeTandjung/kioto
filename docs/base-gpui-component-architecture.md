# `base_gpui` Component Architecture

This document describes the current architecture for compound GPUI components in `crates/base_gpui` and how to follow it when implementing a new component.

The main design goal is to separate:

1. reusable generic mechanics,
2. component-specific behavior/state/runtime,
3. renderable GPUI layers.

Tabs is the reference implementation.

## Directory shape

Reusable primitives live under `api`:

```text
crates/base_gpui/src/api/
  child/
    generic_child.rs
    context/
      generic_context.rs
      state/generic_state.rs
```

A component owns its own folder:

```text
crates/base_gpui/src/<component>/
  actions.rs                 # optional GPUI key dispatch actions/bindings
  child/
    <component>_child.rs      # typed compound child enum
    context/
      <component>_context.rs  # component-specific context wrapper
      props/                  # public/injected props
      runtime/                # runtime metadata and derived runtime state
      state/                  # selected/open/etc. state container
  layers/                     # renderable GPUI elements
```

For Tabs:

```text
crates/base_gpui/src/tabs/
  actions.rs
  child/tabs_child.rs
  child/context/tabs_context.rs
  child/context/props/tabs_props.rs
  child/context/runtime/tabs_runtime.rs
  child/context/state/tabs_state.rs
  layers/tabs_root.rs
  layers/tabs_list.rs
  layers/tabs_tab.rs
  layers/tabs_panel.rs
  layers/tabs_indicator.rs
```

## Layer responsibilities

### `GenericState`

`GenericState` is the minimal reusable interface for state containers used by `GenericContext`.

It should only describe generic state mechanics, such as:

- constructing state from an optional default value,
- reading the current value,
- setting the current value.

Component-specific behavior does not belong here.

### `GenericContext<S, P, R>`

`GenericContext` is the reusable storage/mechanics layer.

It owns:

- controlled vs uncontrolled state resolution,
- keyed GPUI state entity creation,
- keyed GPUI runtime entity creation,
- generic state mutation,
- generic runtime mutation,
- generic props access.

It should expose only generic mechanics like:

```rust
get_state
set_state
set_state_silent
get_runtime
set_runtime
is_controlled
props
```

It should not expose methods like:

```rust
select_tab
register_tab
highlight_tab
apply_tabs_fallback
```

Those are component-specific and belong on the component context wrapper.

### Component context wrapper

Each component should define a wrapper around `GenericContext`.

For Tabs:

```rust
pub struct TabsContext<T: Clone + Eq + 'static> {
    inner: GenericContext<TabsState<T>, TabsProps<T>, TabsRuntime<T>>,
}
```

This wrapper is where component-specific behavior belongs.

For Tabs, this includes:

- selecting a tab,
- applying uncontrolled fallback semantics,
- syncing highlighted index with selected value,
- registering tab/panel metadata,
- navigating highlighted tabs,
- exposing tabs-specific runtime queries.

The component context is allowed to call `inner.get_runtime(...)` and `inner.set_runtime(...)`, but callers outside the context should prefer tabs-specific methods instead of mutating runtime directly.

### `GenericChild<C>`

`GenericChild<C>` only means:

```rust
pub trait GenericChild<C>: IntoElement {
    fn add_state_context(self, context: C) -> Self;
}
```

It is intentionally unbounded. Do not require `C` to expose or contain `GenericContext`.

Reason: child context injection should not leak how a context is implemented internally. A component child should receive the component wrapper context, for example:

```rust
impl<T: Clone + Eq + 'static> GenericChild<TabsContext<T>> for TabsTab<T> {
    fn add_state_context(mut self, context: TabsContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}
```

Do not inject `GenericContext<TabsState<T>, TabsProps<T>, TabsRuntime<T>>` directly into component children.

### Typed compound child enum

Compound roots should keep typed children before GPUI erases elements to `AnyElement`.

For Tabs:

```rust
pub enum TabsChild<T: Clone + Eq + 'static> {
    List(TabsList<T>),
    Panel(TabsPanel<T>),
    Indicator(TabsIndicator),
}
```

This allows `TabsRoot<T>` to pre-register metadata and inject context before rendering.

The child enum is responsible for routing operations across child variants, for example:

- context injection,
- panel indexing,
- component-specific runtime registration traversal.

### Props

Component props are injected into the component context and should hold stable configuration/callbacks needed by child layers.

For Tabs, `TabsProps<T>` currently owns:

- orientation,
- `on_value_change`.

Props should not own runtime metadata. Runtime metadata belongs in runtime.

### Runtime

Runtime stores component-specific metadata and derived runtime state that is not the primary selected/open value.

For Tabs, `TabsRuntime<T>` owns:

- registered tab metadata,
- registered panel metadata,
- highlighted tab index,
- selected-value sync bookkeeping.

Runtime shape is component-specific. Do not force a generic runtime registration abstraction until multiple components prove the same shape.

### Renderable layers

Files under `layers/` are GPUI renderable pieces.

For Tabs:

- `TabsRoot<T>` creates `TabsContext<T>`, pre-registers metadata, applies fallback, and injects context.
- `TabsList<T>` renders the tab list and owns Tabs keyboard dispatch handlers.
- `TabsTab<T>` renders an interactive tab and knows its own tab metadata.
- `TabsPanel<T>` renders panel content and knows its own panel metadata.
- `TabsIndicator` is a renderable visual layer.

Renderable layers may know their own metadata, but they should route runtime insertion through the component context.

Good:

```rust
impl<T: Clone + Eq + 'static> TabsTab<T> {
    pub fn register_runtime(&self, index: usize, context: &TabsContext<T>, cx: &mut App) {
        if let Some(value) = self.value.as_ref() {
            context.register_tab(value.clone(), self.disabled, index, cx);
        }
    }
}
```

Avoid:

```rust
runtime.register_tab(...)
```

from arbitrary render layers. `TabsContext<T>` should know how metadata enters `TabsRuntime<T>`.

## State-aware styling

Normal GPUI builder styling remains the default for static styles:

```rust
TabsTab::new()
    .px_3()
    .py_2()
    .rounded_md()
```

For styles that depend on component state, expose a `style_with_state` builder on the relevant renderable layer.

For Tabs:

```rust
TabsTab::new()
    .style_with_state(|state, tab| {
        if state.active {
            tab.bg(/* active color */)
        } else if state.highlighted {
            tab.bg(/* highlighted color */)
        } else {
            tab
        }
    })
```

Render-state structs are component-specific public API. They should model the same information that Base UI exposes through state-aware `className`, `style`, and `render` callbacks, adapted to GPUI.

Current Tabs render states include:

- `TabsRootRenderState`: orientation and activation direction.
- `TabsListRenderState`: orientation and activation direction.
- `TabsTabRenderState`: active, disabled, highlighted, and orientation.
- `TabsPanelRenderState`: hidden, orientation, and activation direction.
- `TabsIndicatorRenderState`: selected, orientation, and activation direction placeholder state; active tab bounds are pending.

Render layers should not independently recompute shared component state when the component context can compute it. Prefer context helpers such as:

```rust
context.tab_render_state(...)
context.panel_render_state(...)
```

## Keyboard dispatch

Use GPUI key dispatch for keyboard behavior instead of raw `on_key_down` when implementing component commands.

A component with keyboard behavior should usually have:

1. `actions.rs`,
2. an `init(cx: &mut App)` function that binds keys,
3. a `key_context(...)` on the relevant rendered layer,
4. `on_action(...)` handlers.

For Tabs:

```rust
pub const TABS_LIST_KEY_CONTEXT: &str = "TabsList";

actions!(base_gpui_tabs, [
    TabsSelectLeft,
    TabsSelectRight,
    TabsSelectUp,
    TabsSelectDown,
    TabsSelectFirst,
    TabsSelectLast,
    TabsActivateHighlighted,
]);
```

Keys are bound in `tabs::init(cx)` and registered from `base_gpui::init(cx)`.

The layer then handles actions:

```rust
div()
    .key_context(TABS_LIST_KEY_CONTEXT)
    .on_action(move |_: &TabsSelectLeft, window, cx| {
        // component-specific behavior via TabsContext
    })
```

## Implementation checklist for a new component

When adding a new compound component:

1. Create `<component>/child/context/state`.
   - Define the primary state container.
   - Implement `GenericState`.

2. Create `<component>/child/context/props`.
   - Define injected props/callbacks/config.

3. Create `<component>/child/context/runtime`.
   - Define component-specific runtime metadata/state.
   - Keep metadata structs specific to the component.

4. Create `<component>/child/context/<component>_context.rs`.
   - Wrap `GenericContext<State, Props, Runtime>`.
   - Put all component-specific behavior here.
   - Hide direct generic runtime mutation behind component-specific methods where practical.

5. Create renderable layers under `<component>/layers`.
   - Root creates the component context.
   - Root keeps typed children before `AnyElement` erasure.
   - Children implement `GenericChild<ComponentContext>`.

6. Create `<component>/child/<component>_child.rs`.
   - Define typed child variants.
   - Route context injection.
   - Route component-specific metadata registration traversal if needed.

7. Add `actions.rs` if the component has keyboard behavior.
   - Define actions.
   - Bind keys in component `init`.
   - Add a key context and `on_action` handlers to the relevant layer.

8. Re-export from module `mod.rs` files.

## Rules of thumb

- Generic primitives should not know tabs, accordions, menus, etc.
- Component context should translate generic mechanics into component language.
- Children should receive the component wrapper context, not the raw generic context.
- Children may know their own metadata.
- Component context should know how metadata enters runtime.
- Runtime registration should stay component-specific unless a real reusable pattern emerges.
- Prefer GPUI key dispatch actions over raw key-down handlers for keyboard commands.
- Avoid `utils/`; put reusable API primitives under `api/` and component code under the component folder.
