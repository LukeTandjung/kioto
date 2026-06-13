# GPUI `ElementId`

`ElementId` gives a rendered element a stable identity. It is also used for keyed runtime state.

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

## Keyed runtime

Use stable IDs with `window.use_keyed_state(...)`:

```rust
let runtime = window.use_keyed_state(id.clone(), cx, |_, _| CheckboxRuntime::new(default));
```

If the ID changes, GPUI treats it as different runtime state.

## Named child IDs

Use `ElementId::NamedChild(...)` for additional state attached to one component part:

```rust
let focus = window.use_keyed_state(
    ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
    cx,
    |_, cx| cx.focus_handle(),
);
```

Common names:

- `"focus"` for a stable `FocusHandle`.

Prefer one keyed component runtime entity at the root ID; use named child IDs only for separate GPUI handles or facts with an independent lifecycle.

## Rules

- IDs must be stable across renders.
- Do not reuse the same ID for unrelated keyed state.
- Use named child IDs instead of manually concatenating strings.
- Prefer caller-supplied IDs for public roots when state must survive reordering.
