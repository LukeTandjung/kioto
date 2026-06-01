# GPUI `ElementId`

`ElementId` gives a rendered element a stable identity. It is also used for keyed state.

## Root ID

Component roots usually store an ID:

```rust
pub struct CheckboxRoot {
    id: ElementId,
    // ...
}
```

```rust
base.id(self.id)
```

Expose a builder method:

```rust
pub fn id(mut self, id: impl Into<ElementId>) -> Self {
    self.id = id.into();
    self
}
```

## Keyed state

Use stable IDs with `window.use_keyed_state(...)`:

```rust
let state = window.use_keyed_state(id.clone(), cx, |_, _| CheckboxState::new(default));
```

If the ID changes, GPUI treats it as different state.

## Named child IDs

Use `ElementId::NamedChild(...)` for separate state attached to one component part:

```rust
let runtime = window.use_keyed_state(
    ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("runtime")),
    cx,
    |_, _| CheckboxRuntime::new(),
);
```

Common names:

- `"runtime"` for component runtime metadata;
- `"focus"` for a stable `FocusHandle`.

## Rules

- IDs must be stable across renders.
- Do not reuse the same ID for unrelated keyed state.
- Use named child IDs instead of manually concatenating strings.
- Prefer caller-supplied IDs for public roots when state must survive reordering.
