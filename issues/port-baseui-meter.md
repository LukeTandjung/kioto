# Port Base UI Meter to GPUI

## Problem

Base UI Meter is a purely presentational compound component: a graphical display of a numeric value within a known range (battery level, disk usage, password strength). The root clamps the value into `[min, max]`, derives a 0–100 percentage, formats a display string, and provides those derived values to the parts; the indicator fills the track by percentage width; the value and label parts render text.

There is no state machine at all — Base UI's `MeterRootState` is empty and every context value is re-derived from the `value` prop on each render. There is no controlled/uncontrolled machinery, no callbacks, no focus, no keyboard behavior, and no measurement. The port is a derivation-only runtime plus five thin renderable parts.

`crates/base_gpui` has no `meter` module. The goal is behavioral parity via GPUI-native architecture: typed props and style state instead of `role="meter"`, ARIA attributes, `className`, CSS, or `Intl.NumberFormat`.

## Scope

Port the Meter component family from Base UI into GPUI-native components:

- `MeterRoot`
- `MeterTrack`
- `MeterIndicator`
- `MeterValue`
- `MeterLabel`

Values are plain `f64`; no generic `T` is needed.

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/meter/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/meter/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/meter/root/MeterRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/meter/root/MeterRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/meter/track/MeterTrack.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/meter/indicator/MeterIndicator.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/meter/value/MeterValue.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/meter/label/MeterLabel.tsx`

GPUI analog for the percentage fill (no new primitive needed):

- `/home/luke/Projects/gpui-component/crates/ui/src/progress/progress.rs` — the fill is `div().w(relative(percentage / 100.0))`; unlike the Tabs indicator, Meter requires no bounds measurement or `on_children_prepainted`.

Current GPUI implementation:

- No `crates/base_gpui/src/meter/` module exists yet.

Expected GPUI implementation files (flat shape per `docs/base-gpui-component-architecture.md`):

```text
crates/base_gpui/src/meter/mod.rs
crates/base_gpui/src/meter/runtime.rs       # derivation-only: clamp, percentage, formatting
crates/base_gpui/src/meter/props.rs         # min, max, value, format callback
crates/base_gpui/src/meter/style_state.rs   # MeterStyleState
crates/base_gpui/src/meter/context.rs       # thin injection of derived values into parts
crates/base_gpui/src/meter/child.rs         # typed child enums
crates/base_gpui/src/meter/layers/mod.rs
crates/base_gpui/src/meter/layers/meter_root.rs
crates/base_gpui/src/meter/layers/meter_track.rs
crates/base_gpui/src/meter/layers/meter_indicator.rs
crates/base_gpui/src/meter/layers/meter_value.rs
crates/base_gpui/src/meter/layers/meter_label.rs
crates/base_gpui/src/meter/tests/
```

Because the runtime holds no mutable state, it does not need `Entity<...>` / keyed-state plumbing, `sync_children`/`reconcile` commands, `actions.rs`, or `child_wiring.rs`. The runtime is a plain struct of derived values computed once at the top of root render; the context can hold it behind `Rc` and expose only read access.

## Co-planning note: Progress

Base UI Progress has a near-identical part topology (Root/Track/Indicator/Value/Label) and the same derivation core. Progress adds a nullable value (indeterminate) and a three-state status (`progressing` / `complete` / `indeterminate`). The shared derivation (clamp, percentage, default percent formatting) is mirrored by `issues/port-baseui-progress.md` (already written) rather than diverging; do not prematurely extract shared abstractions.

**Ratified cross-component convention.** Both Meter and Progress deliberately deviate from Base UI in two ways, kept symmetric across the pair: (1) default formatting is **percent-of-range** (not Base UI's raw `value / 100` Intl percent), so displayed text stays in sync with the indicator fill for any custom `min`/`max`; (2) the value is **clamped** into `[min, max]` so out-of-range inputs cannot render an overflowing fill. In both cases the `format` callback receives the raw unclamped value, so a caller can recover Base UI's exact number. See the matching "Ratified cross-component decisions" section in `issues/port-baseui-progress.md`.

## Out of scope / drop from Base UI

- `role="meter"`, `aria-valuemin/max/now/valuetext`, `getAriaValueText`, and the visually-hidden NVDA workaround span — DOM ARIA. Revisit via GPUI AccessKit APIs (`Role::Meter` exists in AccessKit) once the pinned GPUI revision exposes them, as tracked in the Separator and Tabs issues.
- `aria-labelledby` id plumbing between `MeterLabel` and the root (`setLabelId` / `useRegisteredLabelId`); `MeterLabel` becomes a plain styled text part.
- React `render` props, `className`, web `style` props.
- React context/hooks implementation details; use the component context wrapper instead.
- `Intl.NumberFormat` / `locale` / `Intl.NumberFormatOptions`. Rust has no Intl; expose a default percent-of-range formatter plus an optional user `format` callback `Fn(f64) -> String` (standard translation decision: do not port arbitrary JS value semantics).
- DOM data attributes; expose derived values through `MeterStyleState` and `style_with_state(...)`.
- `MeterValue`'s function-children override becomes a builder closure, not a React children function.
- No component-local `utils/` folder; no Rust scoped visibility (`pub(...)`) — `ast-grep scan` must stay clean.

## Acceptance Criteria

### Module/API surface

- [ ] Add a top-level `meter` module and export it from `crates/base_gpui/src/lib.rs`.
- [ ] `MeterRoot::new()` builder with `.value(f64)` (required input), `.min(f64)` (default `0.0`), `.max(f64)` (default `100.0`), and `.format(impl Fn(f64) -> String)`.
- [ ] `MeterTrack::new()` builder — structural container, accepts `MeterIndicator` as a typed child.
- [ ] `MeterIndicator::new()` builder — no value props; fill width comes from context.
- [ ] `MeterValue::new()` builder — renders the formatted value by default; optional builder closure override receiving `(formatted: &str, value: f64)` (the GPUI translation of Base UI's function children).
- [ ] `MeterLabel::new(...)` builder — styled text part, no id plumbing.
- [ ] Typed child enum(s) in `child.rs` restrict root children to `Track` / `Value` / `Label` (plus `Indicator` under `Track`), matching the composition shown in Base UI docs, before `AnyElement` erasure.
- [ ] All parts support normal GPUI styling builders through `Styled` and support `.style_with_state(...)`.
- [ ] `meter/mod.rs` is barrel exports only.

### Correctness / compile readiness

- [ ] `cargo check -p base_gpui` passes.
- [ ] `cargo test -p base_gpui meter` passes.
- [ ] `ast-grep scan crates/base_gpui/src/meter` produces no scoped-visibility violations.
- [ ] Add a small Meter demo to `crates/base_gpui/src/main.rs` (label + value + track/indicator, mirroring the Base UI hero demo).

### Value derivation (runtime)

- [ ] Runtime is derivation-only: a plain struct computed from `(value, min, max, format)` at the top of root render; no `Entity`, no commands, no callbacks, no controlled/uncontrolled resolution.
- [ ] Clamped value: `value` clamped into `[min, max]`; `NaN` value falls back to `min` before clamping (matches Base UI).
- [ ] Percentage: `((value - min) / (max - min)) * 100`, with `NaN` (including the degenerate `min == max` range) falling back to `0`, then clamped to `[0, 100]`.
- [ ] Default formatting: percent-of-range, i.e. the derived percentage rendered as a percent string (e.g. `50%`), so the text stays in sync with the indicator fill for any `min`/`max` — not the raw value.
- [ ] Custom `format` callback receives the raw (unclamped) `value`, matching Base UI, and its output replaces the default string.
- [ ] Derivation lives in one place in `runtime.rs`; parts never re-derive percentage or formatting.

### Styling/state exposure

- [ ] `MeterStyleState` in `style_state.rs` exposes `{ value, clamped_value, percentage, formatted, min, max }` (the GPUI translation of `MeterRootContext`); Base UI's part states are all empty, so one shared struct for all five parts is acceptable.
- [ ] Every part's `.style_with_state(...)` receives the current `MeterStyleState`.
- [ ] `MeterIndicator` fills the track using `w(relative(percentage / 100.0))` by default; user styling via `style_with_state` can restyle but the default fill requires no configuration and no measurement.
- [ ] No DOM data attributes or CSS variable API in the public surface.

### Tests / verification

- [ ] Clamping: value below `min` clamps to `min` (percentage `0`); value above `max` clamps to `max` (percentage `100`).
- [ ] Percentage math: mid-range values with default `0..100` and with custom ranges (e.g. `value 30, min 20, max 40` → `50%`).
- [ ] Default formatting produces the percent-of-range string for non-default `min`/`max`.
- [ ] Custom `format` callback output is used verbatim and receives the raw unclamped value.
- [ ] Edge cases: `value == min`, `value == max`, `min == max` (percentage `0`, no panic/NaN leak), `NaN` value falls back to `min` / percentage `0`.
- [ ] `MeterValue` closure override receives `(formatted, value)` and its output is rendered.
- [ ] `style_with_state` on indicator/track observes the derived percentage.
