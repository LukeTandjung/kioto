# GPUI Input Primitive

`base_gpui::primitives::input::input()` creates a reusable single-line text input primitive. It is intentionally HTML-like at the builder API level while staying GPUI-native internally.

```rust
use base_gpui::primitives::input::input;

input()
    .id("email")
    .name("email")
    .placeholder("hello@example.com")
    .required(true)
```

## Supported builder-method subset

| HTML concept | GPUI builder |
|---|---|
| `id` | `.id(...)` |
| `name` | `.name(...)` |
| `value` | `.value(...)` |
| `defaultValue` | `.default_value(...)` |
| `placeholder` | `.placeholder(...)` |
| `disabled` | `.disabled(bool)` |
| `readonly` | `.read_only(bool)` |
| `required` | `.required(bool)` |
| `autofocus` | `.auto_focus(bool)` |
| `tabIndex` | `.tab_index(isize)` |

The primitive also supports:

- `.on_value_change(...)`
- `.on_enter(...)`
- `.style_with_state(...)`

## Current scope

This is a single-line text primitive. It handles platform text insertion, IME composition, cursor movement, selection, copy/cut/paste, and disabled/read-only behavior.

`input()` is intentionally field-agnostic. Field integration is provided by `base_gpui::field::FieldControl`, which composes `input()` and registers text value/focus metadata with `FieldRoot`.

Deferred follow-ups include input type variants, pattern/min/max/minLength/maxLength validation, Number Field, password masking, and multiline `textarea()`.
