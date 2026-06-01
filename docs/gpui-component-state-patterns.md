# GPUI Component State Patterns

GPUI components in this repo usually use one of three state shapes.

## 1. Stateless `RenderOnce` builders

Use this for simple components that only store props, callbacks, styles, and children.

```rust
#[derive(IntoElement)]
pub struct Button {
    base: Div,
    children: Vec<AnyElement>,
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base.children(self.children)
    }
}
```

Rules:

- The builder is consumed during render.
- Do not store long-lived UI state directly on the builder.
- Use normal GPUI builder methods for styling.

## 2. App-owned `Entity<State>` components

Use this when a component has durable state owned by the app/view, not by a single rendered element.

Create an entity with `cx.new(...)`:

```rust
let input: Entity<InputState> = cx.new(|cx| InputState {
    value: SharedString::default(),
    focus_handle: cx.focus_handle(),
});
```

If the state itself renders, implement `Render` for it:

```rust
pub struct InputState {
    value: SharedString,
    focus_handle: FocusHandle,
}

impl Render for InputState {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().track_focus(&self.focus_handle).child(self.value.clone())
    }
}
```

Read and update through the entity handle:

```rust
let value = input.read(cx).value.clone();

input.update(cx, |input, cx| {
    input.value = "next".into();
    cx.notify();
});
```

Rules:

- `Entity<T>` is a handle; the data lives in GPUI's app state.
- Use this when state should outlive one render pass or be shared by multiple UI pieces.
- Read with `entity.read(cx)`.
- Mutate with `entity.update(cx, |state, cx| { ... })`.
- Call `cx.notify()` after mutations that should re-render.

## 3. Element-owned keyed state

Use this for compound component internals that should persist while the element remains keyed in the rendered element tree.

"Rendered element tree" means the GPUI elements produced by a view's `render(...)` method for a window frame. The state is tied to an element ID in that rendered tree, not to a standalone view struct.

Create keyed state during `RenderOnce::render(...)` with `window.use_keyed_state(...)`:

```rust
let state: Entity<CheckboxState> = window.use_keyed_state(
    id.clone(),
    cx,
    |_, _| CheckboxState::new(default),
);
```

Create separate keyed state for related internals with `ElementId::NamedChild(...)`:

```rust
let runtime: Entity<CheckboxRuntime> = window.use_keyed_state(
    ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("runtime")),
    cx,
    |_, _| CheckboxRuntime::new(),
);

let focus: Entity<FocusHandle> = window.use_keyed_state(
    ElementId::NamedChild(Arc::new(id), SharedString::from("focus")),
    cx,
    |_, cx| cx.focus_handle(),
);
```

Read/update the returned entity like any other entity:

```rust
let checked = state.read(cx).get_value().copied().unwrap_or(false);

state.update(cx, |state, cx| {
    state.set_value(Some(true));
    cx.notify();
});
```

Rules:

- The `ElementId` must be stable across renders.
- Use named child IDs for separate per-part state, e.g. `runtime`, `focus`.
- Use this when the caller should not have to create/pass an `Entity` manually.
- This pattern powers `GenericContext<State, Props, Runtime>`.

## Controlled vs uncontrolled state

For Base UI-style components, use this split:

```text
controlled value snapshot = caller-provided source of truth for this render
state entity              = internal uncontrolled value
props                     = current render config/callbacks
runtime                   = derived internal metadata
```

Behavior:

- Controlled: read the external value, call callbacks, do not mutate internal value as source of truth.
- Uncontrolled: read/mutate internal keyed state and call callbacks.

Example:

```rust
pub fn request_toggle(&self, window: &mut Window, cx: &mut App) {
    if self.inner.props().disabled() || self.inner.props().read_only() {
        return;
    }

    let next = !self.checked(cx);
    self.inner.set_state(Some(next), cx, |props, value, cx| {
        if let (Some(on_change), Some(value)) = (props.on_checked_change(), value) {
            on_change(*value, window, cx);
        }
    });
}
```

## State vs props vs runtime vs render state

Keep these distinct:

| Kind | Meaning | Example |
|---|---|---|
| State | Primary value | checkbox `checked`, tabs selected value |
| Props | Current render config/callbacks | `disabled`, `orientation`, `on_change` |
| Runtime | Internal derived metadata | tab bounds, highlighted index, focus flag |
| Render state | Public styling snapshot | `checked`, `focused`, `active`, `hidden` |

Render state is not persistent state. It is computed for styling/render decisions.

## Related docs

- `docs/gpui-element-id.md`
- `docs/gpui-focus.md`
- `docs/gpui-keyboard-actions.md`
- `docs/gpui-testing.md`
