# GPUI Component State Patterns

## Summary

`gpui-component` uses two main component styles:

1. **Stateless / controlled elements**: lightweight builder structs that implement `RenderOnce`.
2. **Stateful components**: long-lived GPUI `Entity<State>` values whose state structs implement `Render`.

There is also a middle pattern: components that look stateless from the caller side but keep internal runtime state with `window.use_keyed_state(...)` keyed by an `ElementId`.

## Stateless elements

Stateless components are plain element builders. They store props, callbacks, style, and children, then consume themselves in `RenderOnce`.

Examples:

- `Button`
- `Checkbox`
- `Switch`
- `Tag`
- `Alert`
- `Radio`
- `Accordion`
- `Pagination`
- `TabBar`

Representative shape:

```rust
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    base: Stateful<Div>,
    label: Option<SharedString>,
    disabled: bool,
    selected: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        // Build GPUI element tree.
    }
}
```

Source reference:

- `/home/luke/Projects/gpui-component/crates/ui/src/button/button.rs`

The meaningful state is usually supplied by the parent:

```rust
Checkbox::new("accept")
    .checked(is_checked)
    .on_click(|new_checked, window, cx| {
        // caller updates external state
    })
```

Source references:

- `/home/luke/Projects/gpui-component/crates/ui/src/checkbox.rs`
- `/home/luke/Projects/gpui-component/crates/ui/src/switch.rs`

This is the controlled/stateless pattern: the component emits the desired next value but does not own the app-level value.

## Stateful components

Stateful components use a long-lived state struct stored in a GPUI `Entity<State>`. The state struct owns behavior and implements `Render`.

Example shape:

```rust
pub struct InputState {
    focus_handle: FocusHandle,
    text: Rope,
    history: History<Change>,
    selected_range: Selection,
    disabled: bool,
    masked: bool,
}

impl Render for InputState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Render based on owned state.
    }
}
```

The parent creates and stores the entity:

```rust
struct MyView {
    input: Entity<InputState>,
}

impl MyView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx).default_value("Hello"));
        Self { input }
    }
}
```

Then the render path either returns the entity directly or passes it into a wrapper element.

Source references:

- `/home/luke/Projects/gpui-component/crates/ui/src/input/state.rs`
- `/home/luke/Projects/gpui-component/crates/ui/src/input/input.rs`

`Input` itself is a `RenderOnce` wrapper around `Entity<InputState>`:

```rust
pub struct Input {
    state: Entity<InputState>,
    size: Size,
    prefix: Option<AnyElement>,
    disabled: bool,
}

impl RenderOnce for Input {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.state.update(cx, |state, _| {
            state.disabled = self.disabled;
            state.size = self.size;
        });

        // Render shell around the state entity.
    }
}
```

This gives a split between:

- `InputState`: domain behavior and state.
- `Input`: presentational/configuration wrapper.

Other examples:

- `ListState<D>` implements `Render` and owns selected index, query input, row cache, scroll handle, delegate, etc.
- `DataTable<D>` is a `RenderOnce` wrapper around `Entity<TableState<D>>`.
- `TableState<D>` owns table interaction state.

Source references:

- `/home/luke/Projects/gpui-component/crates/ui/src/list/list.rs`
- `/home/luke/Projects/gpui-component/crates/ui/src/table/state.rs`
- `/home/luke/Projects/gpui-component/crates/ui/src/table/data_table.rs`

## Internal keyed state

Some components expose a builder API but internally persist state with `window.use_keyed_state(...)`.

Example: `Popover`.

```rust
let state = window.use_keyed_state(self.id.clone(), cx, |_, cx| {
    PopoverState::new(default_open, cx)
});
```

`PopoverState` owns:

- open/closed state
- focus handles
- trigger bounds
- dismiss subscriptions
- open-change callback

Source reference:

- `/home/luke/Projects/gpui-component/crates/ui/src/popover.rs`

This allows usage like a stateless builder:

```rust
Popover::new("menu")
    .trigger(...)
    .content(...)
```

while still supporting internal state across renders.

Other examples of keyed state:

- `HoverCard`
- `DropdownMenuPopover`
- `Progress`
- `TabBar` indicator animation
- `Switch` / `Checkbox` animation state

## Important distinction: `Stateful<Div>` vs stateful components

`Stateful<Div>` does not mean a component owns domain state. It usually means the GPUI element has an `ElementId`, so GPUI can attach interactivity, focus tracking, events, and keyed state.

Example:

```rust
base: Stateful<Div>
```

This is different from:

```rust
Entity<InputState>
```

A useful distinction:

- `Stateful<Div>`: keyed/interactable element.
- `Entity<State>`: long-lived stateful component/model.

