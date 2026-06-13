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

## 3. Element-owned keyed runtime

Use this for compound component internals that should persist while the element remains keyed in the rendered element tree.

"Rendered element tree" means the GPUI elements produced by a view's `render(...)` method for a window frame. The runtime is tied to an element ID in that rendered tree, not to a standalone view struct.

Create keyed runtime during `RenderOnce::render(...)` with `window.use_keyed_state(...)`:

```rust
let runtime: Entity<CheckboxRuntime> = window.use_keyed_state(
    id.clone(),
    cx,
    |_, _| CheckboxRuntime::new(default_checked),
);
```

Use `ElementId::NamedChild(...)` for additional keyed facts attached to one component part:

```rust
let focus: Entity<FocusHandle> = window.use_keyed_state(
    ElementId::NamedChild(Arc::new(id), SharedString::from("focus")),
    cx,
    |_, cx| cx.focus_handle(),
);
```

Read/update the returned entity like any other entity:

```rust
let checked = runtime.read(cx).checked();

runtime.update(cx, |runtime, cx| {
    runtime.toggle(false, false);
    cx.notify();
});
```

Rules:

- The `ElementId` must be stable across renders.
- Use named child IDs for additional per-part state, e.g. `focus`.
- Use this when the caller should not have to create/pass an `Entity` manually.
- Prefer one component runtime entity over split state/runtime entities.
- Do not reintroduce `GenericContext` / `GenericState`; inline small context plumbing per component.

## Controlled vs uncontrolled state

For Base UI-style components, keep this split:

```text
controlled value snapshot = caller-provided source of truth for this render
runtime                   = internal value + component metadata + derived transition state
props                     = current render config/callbacks
```

Behavior:

- Controlled: reconcile the runtime to the external value for this render, call callbacks, and do not mutate the runtime as source of truth.
- Uncontrolled: read/mutate the runtime value and call callbacks.
- Props are immutable render inputs; do not put runtime state in props.

Example:

```rust
pub fn toggle(&self, window: &mut Window, cx: &mut App) {
    let controlled = *self.controlled.as_ref();
    let props = Rc::clone(&self.props);
    let outcome = self.runtime.update(cx, |runtime, cx| {
        let current = controlled.unwrap_or_else(|| runtime.checked_value());

        runtime.sync_checked_from_context(current);
        let outcome = runtime.toggle(props.disabled(), props.read_only());

        if controlled.is_some() {
            runtime.sync_checked_from_context(current);
        }

        cx.notify();
        outcome
    });

    if outcome.changed() {
        if let Some(on_change) = self.props.on_checked_change() {
            on_change(outcome.checked(), window, cx);
        }
    }
}
```

The callback fires after the entity update returns, not inside the `Entity::update(...)` closure.

## Props vs runtime vs render state

Keep these distinct:

| Kind | Meaning | Example |
|---|---|---|
| Props | Current render config/callbacks | `disabled`, `orientation`, `on_change` |
| Runtime | Internal value, metadata, derived transitions | checkbox `checked`, tabs selected value, tab bounds, focused flag |
| Render state | Public styling snapshot | `checked`, `focused`, `active`, `hidden` |

Render state is not persistent state. It is computed for styling/render decisions.

## Related docs

- `docs/base-gpui-component-architecture.md`
- `docs/gpui-element-id.md`
- `docs/gpui-focus.md`
- `docs/gpui-keyboard-actions.md`
- `docs/gpui-testing.md`
