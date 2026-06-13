# GPUI Keyboard Actions

Prefer GPUI actions/key dispatch over raw key-down handlers for component commands.

## Define actions

Create `actions.rs` for the component:

```rust
use gpui::{actions, App, KeyBinding};

pub const CHECKBOX_ROOT_KEY_CONTEXT: &str = "CheckboxRoot";

actions!(base_gpui_checkbox, [CheckboxToggle]);

pub fn init(cx: &mut App) {
    let context = Some(CHECKBOX_ROOT_KEY_CONTEXT);

    cx.bind_keys([KeyBinding::new("space", CheckboxToggle, context)]);
}
```

Export and register it:

```rust
pub use actions::{init, CheckboxToggle, CHECKBOX_ROOT_KEY_CONTEXT};
```

```rust
pub fn init(cx: &mut gpui::App) {
    checkbox::init(cx);
    tabs::init(cx);
}
```

## Attach key context and handlers

```rust
div()
    .key_context(CHECKBOX_ROOT_KEY_CONTEXT)
    .focusable()
    .on_action(move |_: &CheckboxToggle, window, cx| {
        context.toggle(window, cx);
    })
```

Rules:

- Bind keys in `init(cx)`.
- Scope bindings with `key_context(...)`.
- Translate actions into context/runtime commands rather than open-coding behavior in the layer closure.
- Only bind keys the component should handle. For checkbox, Space toggles; Enter is intentionally unbound.

## Focus requirement

Key dispatch follows the focused element's dispatch path. If actions do not fire, check:

1. the element has a `FocusHandle`,
2. the element uses `.track_focus(...)`,
3. the element is `.focusable()`,
4. the key context is on the dispatch path,
5. component `init(cx)` registered the bindings.

## Example: tabs

Tabs scopes keyboard navigation to the list:

```rust
pub const TABS_LIST_KEY_CONTEXT: &str = "TabsList";
```

```rust
TabsList::new()
    .key_context(TABS_LIST_KEY_CONTEXT)
    .focusable()
    .on_action(move |_: &TabsSelectRight, _window, cx| {
        context.update(cx, |runtime| runtime.move_highlight(Move::Next, loop_focus));
    })
```
