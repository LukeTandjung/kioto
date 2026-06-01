# GPUI `Empty`

Use `gpui::Empty` when a branch should render nothing.

## Why not `div()`?

`div().into_any_element()` still creates a layout element. In flex layouts, it can participate in `gap`, sizing, and ordering.

This caused tabs panel spacing to grow as later panels became active:

```rust
// Bad: inactive panel still contributes a flex child.
div().into_any_element()
```

## Correct absent branch

```rust
use gpui::Empty;

Empty.into_any_element()
```

`Empty` renders with `display: none` and should not affect layout.

## Common usage

Inactive panels without `keep_mounted`:

```rust
if active || keep_mounted {
    base.children(children).into_any_element()
} else {
    Empty.into_any_element()
}
```

Absent indicators:

```rust
if !state.present {
    return Empty.into_any_element();
}
```

## Rule

If the component should be absent, use `Empty`. If it should be mounted but hidden, use a real element with hidden/invisible styling.
