# GPUI Number Field

`base_gpui::number_field` provides a GPUI-native numeric text-entry component family built on top of the generic `primitives::input::input()` primitive.

```rust
use base_gpui::number_field::{
    NumberFieldDecrement, NumberFieldGroup, NumberFieldIncrement, NumberFieldInput,
    NumberFieldRoot,
};

NumberFieldRoot::new()
    .id("quantity")
    .name("quantity")
    .default_value(Some(2.0))
    .min(0.0)
    .max(10.0)
    .step(0.5)
    .child(
        NumberFieldGroup::new()
            .child(NumberFieldDecrement::new().child("−"))
            .child(NumberFieldInput::new())
            .child(NumberFieldIncrement::new().child("+")),
    )
```

## Supported phase-1 API

Root props:

- `.id(...)`
- `.name(...)`
- `.form(...)` metadata only
- `.default_value(Option<f64>)`
- `.value(Option<f64>)`
- `.on_value_change(...)`
- `.on_value_committed(...)`
- `.min(f64)` / `.max(f64)`
- `.step(f64)` / `.step_any()`
- `.small_step(f64)` / `.large_step(f64)`
- `.snap_on_step(bool)`
- `.allow_out_of_range(bool)`
- `.disabled(bool)` / `.read_only(bool)` / `.required(bool)`
- `.style_with_state(...)`

Parts:

- `NumberFieldRoot`
- `NumberFieldGroup`
- `NumberFieldInput`
- `NumberFieldIncrement`
- `NumberFieldDecrement`
- `NumberFieldScrubArea`
- `NumberFieldScrubAreaCursor`

## Behavior

The runtime tracks both:

- committed numeric `value: Option<f64>`;
- visible text `input_value`.

Typing updates the visible text immediately. Empty text maps to `None`. Parseable ASCII numeric text updates the numeric value. Invalid intermediate text, such as `-`, can remain visible while focused without clobbering the current numeric value. Blur/commit formats the current value back into the input or clears it when the value is `None`.

Increment/decrement buttons and keyboard stepping share the same runtime stepping commands. `ArrowUp`/`ArrowDown` use `.step(...)`, `Shift+Arrow` uses `.large_step(...)`, and `Alt+Arrow` uses `.small_step(...)`. `Home` and `End` move to configured `min`/`max` boundaries.

## Parser and formatter limitations

Phase 1 intentionally uses a small Rust-native ASCII/`f64` parser:

- ASCII digits;
- optional leading `+` or `-`;
- decimal point;
- exponent notation accepted by `f64::from_str`;
- surrounding whitespace ignored;
- non-finite values (`NaN`, `inf`, `-inf`) rejected.

Formatting currently uses Rust `f64` formatting after a small floating-point noise cleanup pass.

Deferred parser/formatter follow-ups:

- locale-aware formatting and parsing;
- Unicode numerals and fullwidth punctuation;
- grouping separators;
- currency, unit, percent, per-mille, and notation options.

## Field integration

`NumberFieldRoot` registers with `FieldRoot` when rendered inside a field. Field label clicks focus the number input. Field state tracks numeric filled/empty, dirty, focused, touched, disabled, and required metadata.

```rust
use base_gpui::{
    field::{FieldError, FieldLabel, FieldRoot},
    number_field::{NumberFieldGroup, NumberFieldInput, NumberFieldRoot},
};

FieldRoot::new()
    .child(FieldLabel::new().child("Quantity"))
    .child(
        NumberFieldRoot::new()
            .id("quantity")
            .required(true)
            .child(NumberFieldGroup::new().child(NumberFieldInput::new())),
    )
    .child(FieldError::new().child("Quantity is required."))
```

## Deferred Base UI features

- Full browser-like numeric validity parity (`badInput`, range underflow/overflow, `stepMismatch`).
- Locale and `Intl.NumberFormatOptions` parity.
- Native DOM form submission / hidden input behavior.
- Pointer lock and cursor teleport/wrap behavior for scrub area.
- AccessKit/accessibility wiring.
- Wheel scrub behavior.
