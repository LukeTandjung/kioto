# `base_gpui` Component Architecture

This document describes the generic architecture for compound GPUI components in `crates/base_gpui`.

The main design goal is to separate:

1. reusable generic mechanics,
2. component-specific behavior/state/runtime,
3. renderable GPUI layers.

Component-specific details belong near the component, for example in `crates/base_gpui/src/<component>/AGENTS.md`.

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
  AGENTS.md                  # component-specific implementation notes
  actions.rs                 # optional GPUI key dispatch actions/bindings
  child/
    <component>_child.rs      # typed compound child enum
    context/
      <component>_context.rs  # component-specific context wrapper
      props/                  # public/injected props
      runtime/                # runtime metadata and derived runtime state
      state/                  # selected/open/etc. state container
  layers/                     # renderable GPUI elements only
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

It should not expose component-specific operations like:

```rust
select_item
register_item
highlight_item
apply_component_fallback
```

Those belong on the component context wrapper.

### Component context wrapper

Each component should define a wrapper around `GenericContext`:

```rust
pub struct ComponentContext<T: Clone + Eq + 'static> {
    inner: GenericContext<ComponentState<T>, ComponentProps<T>, ComponentRuntime<T>>,
}
```

This wrapper is where component-specific behavior belongs.

Typical responsibilities include:

- interpreting selected/open/active values,
- applying uncontrolled fallback semantics,
- syncing derived runtime state with controlled props,
- registering component-specific metadata,
- navigating highlighted/focused items,
- exposing component-specific render-state helpers,
- hiding direct generic runtime mutation behind component vocabulary.

The component context may call `inner.get_runtime(...)` and `inner.set_runtime(...)`, but callers outside the context should prefer component-specific methods instead of mutating runtime directly.

Do not add component-specific inherent impls to `GenericContext<SpecificState, SpecificProps, SpecificRuntime>`. Such impls are globally visible and blur generic vs component responsibility.

### `GenericChild<C>`

`GenericChild<C>` only means:

```rust
pub trait GenericChild<C>: IntoElement {
    fn add_state_context(self, context: C) -> Self;
}
```

It is intentionally unbounded. Do not require `C` to expose or contain `GenericContext`.

Reason: child context injection should not leak how a context is implemented internally. A component child should receive the component wrapper context, not the raw generic context.

Good shape:

```rust
impl<T: Clone + Eq + 'static> GenericChild<ComponentContext<T>> for ComponentPart<T> {
    fn add_state_context(mut self, context: ComponentContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}
```

Avoid injecting `GenericContext<ComponentState<T>, ComponentProps<T>, ComponentRuntime<T>>` directly into component children.

`GenericChild` should not grow runtime registration APIs. Registration shapes are component-specific and should remain on component-owned types/context methods until multiple components prove a common abstraction.

### Typed compound child enums

Compound roots should keep typed children before GPUI erases elements to `AnyElement`.

Generic shape:

```rust
pub enum ComponentChild<T: Clone + Eq + 'static> {
    PartA(ComponentPartA<T>),
    PartB(ComponentPartB<T>),
}
```

This allows the root to pre-register metadata and inject context before rendering.

Child enums are responsible for routing operations across variants, for example:

- context injection,
- child indexing,
- component-specific runtime registration traversal,
- constrained child sets for nested compound layers.

Nested compound layers can define their own typed child enums under `child/` when they need a constrained child set.

Typed child-routing enums are not renderable layers. Keep them under `child/`, not `layers/`.

### Props

Component props are injected into the component context and should hold stable configuration/callbacks needed by child layers.

Props may include:

- orientation/configuration,
- controlled callback handlers,
- behavior flags,
- stable public configuration needed across children.

Props should not own runtime metadata. Runtime metadata belongs in runtime.

### State

Component state is the primary selected/open/active value that participates in controlled/uncontrolled semantics.

State should be small and generic enough to satisfy `GenericState`:

- `new(default)`
- `get_value()`
- `set_value(...)`

Derived state and registered metadata belong in runtime, not primary state.

### Runtime

Runtime stores component-specific metadata and derived runtime state that is not the primary selected/open value.

Runtime may own:

- registered child metadata,
- child ordering,
- highlighted/focused indices,
- activation direction bookkeeping,
- measurement/cache data,
- GPUI handles needed by component behavior.

Runtime shape is component-specific. Do not force a generic runtime registration abstraction until multiple components prove the same shape.

### Renderable layers

Files under `layers/` are GPUI renderable pieces.

Typical responsibilities:

- Root creates the component context, pre-registers metadata, applies fallback, and injects context.
- Composite containers render child groups and own relevant key contexts/actions.
- Leaf parts render interactive or visual elements and know their own metadata.
- Visual helpers render purely visual layers while reading component render state.

Renderable layers may know their own metadata, but should route runtime insertion through the component context.

Good shape:

```rust
impl<T: Clone + Eq + 'static> ComponentPart<T> {
    pub fn register_runtime(&self, index: usize, context: &ComponentContext<T>, cx: &mut App) {
        if let Some(value) = self.value.as_ref() {
            context.register_part(value.clone(), index, cx);
        }
    }
}
```

Avoid mutating runtime directly from arbitrary render layers. The component context should know how metadata enters component runtime.

## State-aware styling

Normal GPUI builder styling remains the default for static styles:

```rust
ComponentPart::new()
    .px_3()
    .py_2()
    .rounded_md()
```

For styles that depend on component state, expose a `style_with_state` builder on the relevant renderable layer:

```rust
ComponentPart::new()
    .style_with_state(|state, part| {
        if state.active {
            part.bg(/* active color */)
        } else {
            part
        }
    })
```

Render-state structs are component-specific public API. They should model the same information that Base UI exposes through state-aware `className`, `style`, and `render` callbacks, adapted to GPUI.

Use GPUI render-state structs instead of porting DOM data attributes or CSS variable APIs directly.

Render layers should not independently recompute shared component state when the component context can compute it. Prefer context helpers such as:

```rust
context.part_render_state(...)
```

## Measurement and layout-derived state

When a Base UI component relies on DOM measurement APIs, translate the behavior into GPUI-native layout/prepaint mechanisms.

Guidelines:

- Use GPUI measurement hooks such as `Div::on_children_prepainted(...)` when appropriate.
- Store measured facts in component runtime.
- Expose measured facts through render-state structs when styling or visual layers need them.
- Do not port Base UI CSS variable names unless they become useful GPUI API.

## Keyboard dispatch

Use GPUI key dispatch for keyboard behavior instead of raw `on_key_down` when implementing component commands.

A component with keyboard behavior should usually have:

1. `actions.rs`,
2. an `init(cx: &mut App)` function that binds keys,
3. a `key_context(...)` on the relevant rendered layer,
4. `on_action(...)` handlers.

Generic shape:

```rust
pub const COMPONENT_KEY_CONTEXT: &str = "Component";

actions!(base_gpui_component, [
    ComponentMovePrevious,
    ComponentMoveNext,
    ComponentActivate,
]);
```

Keys are bound in the component `init(cx)` and registered from `base_gpui::init(cx)`.

The relevant layer handles actions through component context methods:

```rust
div()
    .key_context(COMPONENT_KEY_CONTEXT)
    .on_action(move |_: &ComponentMoveNext, window, cx| {
        // component-specific behavior via ComponentContext
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

7. Add nested child enums under `<component>/child/` when nested layers need constrained child sets.

8. Add `actions.rs` if the component has keyboard behavior.
   - Define actions.
   - Bind keys in component `init`.
   - Add a key context and `on_action` handlers to the relevant layer.

9. Add component-specific `AGENTS.md` if implementation notes would otherwise clutter generic docs.

10. Re-export from module `mod.rs` files.

## Rules of thumb

- Generic primitives should not know tabs, accordions, menus, etc.
- Component context should translate generic mechanics into component language.
- Children should receive the component wrapper context, not the raw generic context.
- Children may know their own metadata.
- Component context should know how metadata enters runtime.
- Runtime registration should stay component-specific unless a real reusable pattern emerges.
- Prefer GPUI key dispatch actions over raw key-down handlers for keyboard commands.
- Avoid `utils/`; put reusable API primitives under `api/` and component code under the component folder.
