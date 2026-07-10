# Port Base UI Progress to GPUI

## Problem

Base UI Progress is a purely presentational compound component: a graphical display of task completion (file upload, download, install). It is the near-twin of Meter — the same Root/Track/Indicator/Value/Label part topology and the same derivation core (clamp, percentage, default percent formatting) — with two additions:

1. **Nullable value.** `value` is `number | null`; `null` means the progress is *indeterminate* (duration unknown).
2. **A three-state status.** The root derives `status: 'indeterminate' | 'progressing' | 'complete'` (`complete` when `value == max`, `indeterminate` when the value is `null`/non-finite, otherwise `progressing`) and exposes it as style state on **every** part, so users style/animate each state.

Like Meter, there is no state machine: Base UI's `ProgressRootState` is `{ status }`, re-derived from the `value` prop on each render. No controlled/uncontrolled machinery, no callbacks, no focus, no keyboard behavior, no measurement. The port is a derivation-only runtime plus five thin renderable parts.

`crates/base_gpui` has no `progress` module. The goal is behavioral parity via GPUI-native architecture: typed props and style state instead of `role="progressbar"`, ARIA attributes, `className`, CSS data attributes, or `Intl.NumberFormat`.

This issue mirrors `issues/port-baseui-meter.md` deliberately. Keep the two implementations structurally parallel, but do **not** prematurely extract a shared Meter/Progress abstraction — duplicate the small derivation core per component and revisit only if a third consumer appears.

## Indeterminate rendering decision

Base UI ships **no** indeterminate animation itself: when `value` is `null` the indicator gets an empty style and users animate it in CSS keyed off `data-indeterminate`. The faithful GPUI translation is **status-only**: expose `ProgressStatus::Indeterminate` through `style_with_state(...)` on all parts and render the indicator with no default fill, letting users style/animate it themselves (e.g. via `gpui::Animation` in their `style_with_state` closure).

- Decision for this issue: **faithful, status-only**. No built-in indeterminate animation.
- Optional follow-up (separate issue, not acceptance criteria here): a convenience built-in sliding animation like `/home/luke/Projects/gpui-component/crates/ui/src/progress/progress.rs`-style infinite slide using GPUI's animation support (`crates/gpui/src/elements/animation.rs` analog).

## Scope

Port the Progress component family from Base UI into GPUI-native components:

- `ProgressRoot`
- `ProgressTrack`
- `ProgressIndicator`
- `ProgressValue`
- `ProgressLabel`

Plus one exported type:

- `ProgressStatus` — `Indeterminate | Progressing | Complete` (Base UI exports `Progress.Status`).

Values are plain `Option<f64>` (`None` = indeterminate); no generic `T` is needed.

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/progress/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/progress/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/progress/root/ProgressRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/progress/root/ProgressRootContext.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/progress/root/stateAttributesMapping.ts`
- `/home/luke/Projects/base-ui/packages/react/src/progress/track/ProgressTrack.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/progress/indicator/ProgressIndicator.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/progress/value/ProgressValue.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/progress/label/ProgressLabel.tsx`
- Shared utils: `/home/luke/Projects/base-ui/packages/react/src/utils/valueToPercent.ts`, `/home/luke/Projects/base-ui/packages/react/src/utils/formatNumber.ts`
- Tests: `ProgressRoot.test.tsx`, `ProgressIndicator.test.tsx`, `ProgressValue.test.tsx`, `ProgressTrack.test.tsx`, `ProgressLabel.test.tsx` under the same tree

GPUI analog for the percentage fill (no new primitive needed):

- `/home/luke/Projects/gpui-component/crates/ui/src/progress/progress.rs` — the fill is `div().w(relative(percentage / 100.0))`; no bounds measurement or `on_children_prepainted` required.

Sibling issue to stay aligned with:

- `issues/port-baseui-meter.md` — same part topology and derivation core; keep file shape, naming, and criteria parallel.

Current GPUI implementation:

- No `crates/base_gpui/src/progress/` module exists yet.

Expected GPUI implementation files (flat shape per `docs/base-gpui-component-architecture.md`):

```text
crates/base_gpui/src/progress/mod.rs
crates/base_gpui/src/progress/runtime.rs       # derivation-only: clamp, percentage, status, formatting; defines ProgressStatus
crates/base_gpui/src/progress/props.rs         # min, max, value (Option<f64>), format callback
crates/base_gpui/src/progress/style_state.rs   # ProgressStyleState
crates/base_gpui/src/progress/context.rs       # thin injection of derived values into parts
crates/base_gpui/src/progress/child.rs         # typed child enums
crates/base_gpui/src/progress/layers/mod.rs
crates/base_gpui/src/progress/layers/progress_root.rs
crates/base_gpui/src/progress/layers/progress_track.rs
crates/base_gpui/src/progress/layers/progress_indicator.rs
crates/base_gpui/src/progress/layers/progress_value.rs
crates/base_gpui/src/progress/layers/progress_label.rs
crates/base_gpui/src/progress/tests/
```

Because the runtime holds no mutable state, it does not need `Entity<...>` / keyed-state plumbing, `sync_children`/`reconcile` commands, `actions.rs`, or `child_wiring.rs`. The runtime is a plain struct of derived values computed once at the top of root render; the context can hold it behind `Rc` and expose only read access — the same shape as Meter.

## Out of scope / drop from Base UI

- `role="progressbar"`, `aria-valuemin/max/now/valuetext`, `getAriaValueText` (including the `"indeterminate progress"` default value text), and the visually-hidden NVDA workaround span — DOM ARIA. Revisit via GPUI AccessKit APIs once the pinned GPUI revision exposes them, as tracked in the Separator, Tabs, and Meter issues.
- `aria-labelledby` id plumbing between `ProgressLabel` and the root (`setLabelId` / `useRegisteredLabelId`); `ProgressLabel` becomes a plain styled text part.
- React `render` props, `className`, web `style` props.
- React context/hooks implementation details; use the component context wrapper instead.
- `Intl.NumberFormat` / `locale` / `Intl.NumberFormatOptions`. Rust has no Intl; expose a default percent formatter plus an optional user `format` callback `Fn(f64) -> String` (standard translation decision: do not port arbitrary JS value semantics).
- DOM data attributes (`data-progressing` / `data-complete` / `data-indeterminate` on every part); expose `status: ProgressStatus` through `ProgressStyleState` and `style_with_state(...)` instead.
- Indicator CSS (`insetInlineStart`, `height: inherit`, width percentage as an inline style string); translate to GPUI styling builders and `relative(...)` width.
- `ProgressValue`'s function-children override becomes a builder closure, not a React children function; Base UI's `'indeterminate'` magic string argument becomes `None` (typed `Option`, not a sentinel string).
- Built-in indeterminate animation — Base UI has none; ship status-only per the decision above (optional follow-up issue).
- No component-local `utils/` folder; no Rust scoped visibility (`pub(...)`) — `ast-grep scan` must stay clean.

## Acceptance Criteria

### Module/API surface

- [x] Add a top-level `progress` module and export it from `crates/base_gpui/src/lib.rs`, including the `ProgressStatus` type.
- [x] `ProgressRoot::new()` builder with `.value(Option<f64>)` (default `None` = indeterminate, matching Base UI's `@default null`), `.min(f64)` (default `0.0`), `.max(f64)` (default `100.0`), and `.format(impl Fn(f64) -> String)`.
- [x] `ProgressTrack::new()` builder — structural container, accepts `ProgressIndicator` as a typed child.
- [x] `ProgressIndicator::new()` builder — no value props; fill width comes from context.
- [x] `ProgressValue::new()` builder — renders the formatted value by default (nothing when indeterminate); optional builder closure override receiving `(formatted: Option<&str>, value: Option<f64>)` (the GPUI translation of Base UI's function children; `None` replaces the `'indeterminate'` sentinel string).
- [x] `ProgressLabel::new(...)` builder — styled text part, no id plumbing.
- [x] `ProgressStatus` enum (`Indeterminate`, `Progressing`, `Complete`) defined in `runtime.rs` and re-exported, mirroring Base UI's exported `Progress.Status` type.
- [x] Typed child enum(s) in `child.rs` restrict root children to `Track` / `Value` / `Label` (plus `Indicator` under `Track`), matching the composition shown in Base UI docs, before `AnyElement` erasure.
- [x] All parts support normal GPUI styling builders through `Styled` and support `.style_with_state(...)`.
- [x] `progress/mod.rs` is barrel exports only.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui progress` passes.
- [x] `ast-grep scan crates/base_gpui/src/progress` produces no scoped-visibility violations.
- [x] Add a small Progress demo to `crates/base_gpui/src/main.rs` (label + value + track/indicator, mirroring the Base UI hero demo; ideally one determinate and one indeterminate instance).

### Value derivation (runtime)

- [x] Runtime is derivation-only: a plain struct computed from `(value, min, max, format)` at the top of root render; no `Entity`, no commands, no callbacks, no controlled/uncontrolled resolution — same shape as Meter's runtime.
- [x] Nullable value: `value: Option<f64>`; `None` (and non-finite `f64` — `NaN`, infinities, matching Base UI's `Number.isFinite` guard) yields no clamped value, no percentage, and no default formatted string.
- [x] Clamped value (determinate only): `value` clamped into `[min, max]`, aligned with Meter's clamping so out-of-range inputs cannot render an overflowing fill.
- [x] Percentage (determinate only): `((value - min) / (max - min)) * 100`, with `NaN` (including the degenerate `min == max` range) falling back to `0`, then clamped to `[0, 100]` — identical math to Meter.
- [x] Default formatting (determinate only): percent-of-range, i.e. the derived percentage rendered as a percent string (e.g. `50%`), keeping the text in sync with the indicator fill for any `min`/`max` — matching the Meter issue's decision, not Base UI's raw `value / 100` Intl percent formatting.
- [x] Custom `format` callback receives the raw (unclamped) `f64` value and its output replaces the default string; it is never invoked when indeterminate.
- [x] Derivation (clamp, percentage, status, formatting) lives in one place in `runtime.rs`; parts never re-derive any of it.

### Status behavior

- [x] `status` derived once at the root: `Indeterminate` when `value` is `None` or non-finite; `Complete` when the (clamped) value equals `max`; `Progressing` otherwise — matching Base UI's `value === max ? 'complete' : 'progressing'` with the clamping decision applied (raw `value >= max` therefore reads `Complete`).
- [x] Degenerate range `min == max` with a determinate value: value clamps to `max`, so status is `Complete` and percentage falls back to `0` without panic/NaN leak — the derivation must not contradict itself (document this case in a test).
- [x] No built-in indeterminate animation; the indeterminate treatment is status exposure only, per the decision section above.

### Styling/state exposure

- [x] `ProgressStyleState` in `style_state.rs` exposes `{ value: Option<f64>, clamped_value: Option<f64>, percentage: Option<f64>, formatted: Option<String>, min, max, status: ProgressStatus }` (the GPUI translation of `ProgressRootContext` plus the state attributes); Base UI's five part states are all identical (`{ status }`), so one shared struct for all five parts is acceptable — same pattern as `MeterStyleState`.
- [x] Every part's `.style_with_state(...)` receives the current `ProgressStyleState` — status must be observable on Root, Track, Indicator, Value, and Label alike (Base UI applies the status data attributes to every part).
- [x] `ProgressIndicator` fills the track using `w(relative(percentage / 100.0))` by default when determinate; when indeterminate it renders with no default fill width (Base UI's empty style object), leaving appearance entirely to `style_with_state`.
- [x] No DOM data attributes or CSS variable API in the public surface.

### Tests / verification

- [x] Clamping: value below `min` clamps to `min` (percentage `0`, status `Progressing`); value above `max` clamps to `max` (percentage `100`, status `Complete`).
- [x] Percentage math: mid-range values with default `0..100` and with custom ranges (e.g. `value 30, min 20, max 40` → `50%`).
- [x] Status transitions: `None` → `Indeterminate`; `Some(v)` with `v < max` → `Progressing`; `Some(max)` → `Complete`; non-finite `Some(f64::NAN)` → `Indeterminate`.
- [x] Default formatting produces the percent-of-range string for non-default `min`/`max`; no formatted string when indeterminate.
- [x] Custom `format` callback output is used verbatim, receives the raw unclamped value, and is not called when indeterminate.
- [x] Edge cases: `value == min`, `value == max`, `min == max` (percentage `0`, status `Complete`, no panic/NaN leak).
- [x] `ProgressValue` default rendering shows the formatted string when determinate and renders nothing when indeterminate; closure override receives `(Some(formatted), Some(value))` when determinate and `(None, None)` when indeterminate, and its output is rendered.
- [x] `ProgressIndicator` has no default fill width when indeterminate (empty-style parity) and the correct relative width when determinate, including zero width at `value == min` (Base UI's "sets zero width when value is 0" test).
- [x] `style_with_state` on all five parts observes the same `status`, and observes the derived percentage on the indicator/track.

## Ratified cross-component decisions (Meter/Progress)

- Default formatting is **percent-of-range**, not Base UI's raw `value / 100` Intl percent — **RATIFIED** as the shared Meter/Progress convention: it keeps the displayed text in sync with the indicator fill for any custom `min`/`max`. `issues/port-baseui-meter.md` applies the identical rule. Callers needing Base UI's exact number use the `format` callback, which receives the raw unclamped value.
- Determinate values are **clamped** into `[min, max]` (Base UI Progress does not clamp; its `valueToPercent` can exceed 100) — **RATIFIED** for parity with Meter and to prevent overflowing fills. The raw unclamped value remains available through the `format` callback.

## AccessKit accessibility follow-up

Base UI Progress emits ARIA on exactly one element: the root carries `role="progressbar"`, `aria-valuemin`, `aria-valuemax`, `aria-valuenow`, `aria-valuetext` (default `"indeterminate progress"` when `value == null`, else the formatted string), and `aria-labelledby` wired to `ProgressLabel`. Every other part is deliberately hidden from AT: `ProgressValue` sets `aria-hidden`, `ProgressLabel` sets `role="presentation"`, and Track/Indicator are plain divs. The root also renders a visually-hidden NVDA-workaround span containing the formatted value. The GPUI translation follows the same shape: one accessible node (the root), everything else stays out of the tree.

All derived values below come from `ProgressRuntime::state()` (`ProgressStyleState { value, clamped_value, percentage, formatted, min, max, status }`), read via `ProgressContext::read` — no new derivation.

### Per accessible part

- **`ProgressRoot`** (`layers/progress_root.rs`) — the only part in the a11y tree. On its already-`.id(...)`ed base div:
  - `.role(Role::ProgressIndicator)` (accesskit's progressbar role).
  - `.aria_min_numeric_value(state.min)` and `.aria_max_numeric_value(state.max)` — always.
  - `.aria_numeric_value(v)` from `state.clamped_value` **only when `Some(v)`** (determinate). When `status == ProgressStatus::Indeterminate`, omit the numeric value entirely — that is the AccessKit idiom for an indeterminate progressbar, standing in for Base UI's `aria-valuenow: undefined` + `"indeterminate progress"` value text.
  - `.aria_label(...)` from a **new** `ProgressRoot::label(impl Into<SharedString>)` builder prop (see Labels below).
- **`ProgressTrack`**, **`ProgressIndicator`** (`layers/progress_track.rs`, `layers/progress_indicator.rs`) — no `.role(...)`; with no role they are not reported, matching Base UI's role-less divs. Nothing to add.
- **`ProgressValue`** (`layers/progress_value.rs`) — must stay out of the tree (Base UI sets `aria-hidden`). No `.role(...)`; render its text inaccessibly (see Labels).
- **`ProgressLabel`** (`layers/progress_label.rs`) — must stay out of the tree (Base UI sets `role="presentation"`). No `.role(...)`; its visible text becomes the source of the root's `.aria_label` string.

### Actions

None. Progress is purely presentational — Base UI wires no click, focus, or keyboard behavior, and the port has no `on_click`/`track_focus` anywhere in `layers/`. Do **not** add `.on_a11y_action(...)`, `.focusable()`, or tab stops; the root node is value-reporting only. (For interactive components, `Action::Click`/`Action::Focus` come free from `.on_click`/`.track_focus`; that machinery is simply absent here by design.)

### Labels

- Base UI's `aria-labelledby` id plumbing between `ProgressLabel` and the root has no gpui equivalent (no relationship builders). Fallback: a literal-string `.label(...)` prop on `ProgressRoot` that becomes `.aria_label(...)`. Callers pass the same string they render inside `ProgressLabel`.
- `ProgressLabel` renders arbitrary `IntoElement` children; document that label text passed as `text!(...)` should instead be `Text::new_inaccessible(...)` so the label is not announced twice (once via the root's `aria_label`, once as a stray text node).
- `ProgressValue`'s rendered `SharedString` child (default formatted string or `display` closure output) should be wrapped in `Text::new_inaccessible(...)` — this is the translation of Base UI's `aria-hidden` on the Value part; the value already reaches AT through the root's `aria_numeric_value`.

### Gaps (no gpui builder in the pinned revision)

- **`aria-valuetext`** (including the default `"indeterminate progress"` string and custom `getAriaValueText`): no builder (`aria_valuetext` does not exist). Fallback: omit + document — AT gets `aria_numeric_value`/min/max instead, and indeterminate is conveyed by omitting the numeric value. Revisit if gpui grows a `set_value` string setter.
- **`aria-labelledby`**: no relationship builders. Fallback: literal `.aria_label(...)` via the new root `label` prop, as above.
- **`aria-hidden` / `role="presentation"`**: no builders — not needed; leaving a part without `.role(...)` keeps it out of the tree, which is the equivalent outcome.
- **NVDA visually-hidden announcement span / live-region updates**: no announcement or live-region API in this gpui revision. Blocked pending gpui upstream; do not attempt a workaround.

### Checklist

- [ ] `ProgressRoot`: add `.role(Role::ProgressIndicator)` plus `.aria_min_numeric_value` / `.aria_max_numeric_value` from `ProgressStyleState`, and `.aria_numeric_value(state.clamped_value)` only when determinate.
- [ ] `ProgressRoot`: add a `.label(impl Into<SharedString>)` builder prop mapped to `.aria_label(...)` (fallback for `aria-labelledby`).
- [ ] `ProgressValue`: render its text via `Text::new_inaccessible(...)` (fallback for `aria-hidden`).
- [ ] `ProgressLabel` / `ProgressTrack` / `ProgressIndicator`: confirm no `.role(...)` is set so they stay out of the a11y tree; document the `Text::new_inaccessible` convention for label text in the module docs.
- [ ] Document the gaps (`aria-valuetext`, `aria-labelledby`, NVDA announcement span) in `progress/mod.rs` docs with the fallbacks above.
- [ ] Keep root/part `ElementId`s stable across frames so the AccessKit `NodeId` (derived from the global id) does not churn.

## Uncertain items

- `ProgressValue` closure signature uses `Option<&str>` / `Option<f64>` instead of Base UI's `'indeterminate'` sentinel string — confirm the typed translation is preferred.
- Whether the optional built-in indeterminate sliding animation should get its own follow-up issue now or wait until a consumer needs it.
