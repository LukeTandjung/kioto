# GPUI Focus

Use a `FocusHandle` when a rendered part needs keyboard focus or focus-aware styling.

## Stable focus handle

For reusable/compound components, create focus handles with keyed state:

```rust
let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
    ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
    cx,
    |_, cx| cx.focus_handle(),
);
let focus_handle = focus_handle_entity.read(cx).clone();
```

Why:

- focus must survive re-renders;
- each focusable part needs a stable identity;
- named child IDs avoid colliding with the root keyed state.

## Attach focus to an element

```rust
div()
    .id(id)
    .track_focus(&focus_handle.tab_stop(true).tab_index(0))
    .focusable()
```

Notes:

- `track_focus(...)` connects the element to the handle.
- `tab_stop(false)` removes it from normal tab navigation.
- `tab_index(-1)` makes it programmatically focusable but not normally tabbable.
- `focusable()` participates in GPUI focus/key dispatch.

## Read focus state

```rust
let focused = focus_handle.is_focused(window);
```

If styling needs focus state, sync it into runtime/style state:

```rust
context.update(cx, |runtime| {
    runtime.sync_focused(focus_handle.is_focused(window));
});
```

## Programmatic focus

```rust
focus_handle.focus(window, cx);
```

For tests or generic tab navigation:

```rust
window.focus_next(cx);
window.focus_prev(cx);
```

## Common gotcha

`focusable()` alone is not a substitute for a stable `FocusHandle` when a component needs to preserve or inspect focus across renders.
