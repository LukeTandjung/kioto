# GPUI Testing

Use GPUI test helpers to test rendered behavior instead of calling component internals directly.

## Basic test setup

```rust
#[gpui::test]
fn space_toggles_when_focused(cx: &mut TestAppContext) {
    let window = open_checkbox(cx, CheckboxTestConfig::default());

    focus_checkbox(cx, window);
    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().checked);
}
```

## Open a test window

```rust
pub fn open_checkbox(
    cx: &mut TestAppContext,
    config: CheckboxTestConfig,
) -> WindowHandle<CheckboxTestView> {
    cx.update(init);

    let window = cx.open_window(size(px(320.0), px(180.0)), move |_, _| {
        CheckboxTestView::new(config)
    });
    cx.run_until_parked();
    window
}
```

Always call component/crate `init` before testing keyboard bindings.

## Observe render state

Store render-state snapshots from `style_with_state(...)`:

```rust
root.style_with_state(move |state, root| {
    observations.borrow_mut().root_states.push(state);
    root.debug_selector(|| "checkbox-root".into())
})
```

This tests the public styling API and avoids reaching into private context/runtime state.

## Click elements

Use `debug_selector(...)` plus `VisualTestContext`:

```rust
let bounds = debug_bounds(cx, window, "checkbox-root").expect("root should exist");
let mut visual = VisualTestContext::from_window(window.into(), cx);

visual.simulate_click(bounds.center(), Modifiers::default());
visual.run_until_parked();
```

## Simulate keyboard input

```rust
cx.simulate_keystrokes(window.into(), "space");
cx.run_until_parked();
```

Focus the element first when testing scoped key actions:

```rust
window.update(cx, |_view, window, cx| {
    window.focus_next(cx);
});
cx.run_until_parked();
```

## Check absent elements

Use `debug_bounds(...)`:

```rust
assert!(debug_bounds(cx, window, "checkbox-indicator").is_none());
```

This catches layout bugs where a component rendered an empty `div()` instead of `gpui::Empty`.
