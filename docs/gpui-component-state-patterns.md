# GPUI Component State Patterns

## Summary

GPUI state commonly appears in three different places:

1. **Caller-owned state / controlled elements**: lightweight builder structs that implement `RenderOnce`; the parent owns the value and passes callbacks.
2. **Entity-owned state / stateful components**: long-lived GPUI `Entity<State>` values; the app owns the state and callers keep a typed handle.
3. **Window element state / internally stateful elements**: state created during rendering with `window.use_state(...)` or `window.use_keyed_state(...)`; the component still looks like a builder API, but GPUI preserves an internal `Entity<State>` for the element while it remains in the rendered tree.

`Global` state also exists, but it is app-wide singleton state and should not be used for normal per-component state.

## 1. Caller-owned state / controlled elements

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

Use this when:

- the selected/checked/open value belongs to a parent view;
- the component should be easy to control from outside;
- no internal persistence is needed beyond props and callbacks.

For Base UI-style controlled props, this corresponds to `value={...}` plus `onValueChange={...}`.

## 2. Entity-owned state / stateful components

Stateful components use a long-lived state struct stored in a GPUI `Entity<State>`. The state struct owns behavior and often implements `Render`.

GPUI's core model is that `App` owns entity data. `Entity<T>` is a typed handle to state owned by the app. You can only read or update that state when you have a GPUI context, usually `&App`, `&mut App`, or `&mut Context<T>`:

```rust
let value = state.read(cx);

state.update(cx, |state, cx| {
    state.value = next;
    cx.notify();
});
```

Source references:

- `/home/luke/.cargo/git/checkouts/zed-a70e2ad075855582/f7ca86e/crates/gpui/docs/contexts.md`
- `/home/luke/.cargo/git/checkouts/zed-a70e2ad075855582/f7ca86e/crates/gpui/src/_ownership_and_data_flow.rs`

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

Use explicit `Entity<State>` when:

- the state is substantial;
- the parent should be able to inspect or mutate the state;
- multiple sibling elements need to share the same state;
- the state should clearly outlive a single render pass;
- the component is closer to a model/view than a simple element.

Other examples:

- `ListState<D>` implements `Render` and owns selected index, query input, row cache, scroll handle, delegate, etc.
- `DataTable<D>` is a `RenderOnce` wrapper around `Entity<TableState<D>>`.
- `TableState<D>` owns table interaction state.

Source references:

- `/home/luke/Projects/gpui-component/crates/ui/src/list/list.rs`
- `/home/luke/Projects/gpui-component/crates/ui/src/table/state.rs`
- `/home/luke/Projects/gpui-component/crates/ui/src/table/data_table.rs`

## 3. Window element state / internally stateful elements

Some components expose a builder API but internally persist state with `window.use_state(...)` or `window.use_keyed_state(...)`.

These methods create an `Entity<S>` during rendering and store it in the window's element-state map. The state is keyed by the element's global id plus the state type.

```rust
let state: Entity<MyState> = window.use_keyed_state("my-element", cx, |window, cx| {
    MyState::new(window, cx)
});
```

The unkeyed form uses the caller's source location as the key:

```rust
let state: Entity<MyState> = window.use_state(cx, |window, cx| {
    MyState::new(window, cx)
});
```

Prefer `use_keyed_state` when rendering lists or multiple instances, because source-location keys alone may collide or point at the wrong conceptual item after reordering.

Source reference:

- `/home/luke/.cargo/git/checkouts/zed-a70e2ad075855582/f7ca86e/crates/gpui/src/window.rs`

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

The lifecycle is important:

- state persists while the same keyed element is rendered in consecutive frames;
- state is dropped when the element disappears from the rendered tree;
- the returned value is still an `Entity<State>`, so event handlers can update it later using `state.update(cx, ...)`;
- GPUI observes the entity and notifies the current view when the element state changes.

Use window element state when:

- the public API should remain a simple builder API;
- the state is local to one rendered element instance;
- the caller should not need to create or store an `Entity<State>` manually;
- the component needs uncontrolled/default behavior, animation state, measurement state, or transient UI state.

Other examples of keyed state:

- `HoverCard`
- `DropdownMenuPopover`
- `Progress`
- `TabBar` indicator animation
- `Switch` / `Checkbox` animation state

## Choosing between the three

| Pattern | Storage | Public API feel | Best for |
| --- | --- | --- | --- |
| Caller-owned / controlled | Parent view/state | Builder props + callbacks | Values the parent owns |
| Entity-owned / stateful component | Explicit `Entity<State>` | Caller passes/stores entity | Complex reusable models/views |
| Window element state | Internal `Entity<State>` from `window.use_keyed_state` | Builder API with internal persistence | Uncontrolled components, animation, local UI state |

For a compound component like Tabs, the likely split is:

- controlled mode: selected value comes from the public `.value(...)` prop;
- uncontrolled mode: selected value is stored in internal window element state initialized from `.default_value(...)`;
- tab/list/panel children receive or share the resulting `Entity<TabsState<T>>` during render.

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

