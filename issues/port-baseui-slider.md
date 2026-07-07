# Port Base UI Slider to GPUI

## Problem

Base UI Slider is a pointer- and keyboard-driven numeric range control with
controlled/uncontrolled value, single-value and multi-thumb range support,
min/max/step/large-step math with decimal-precision cleanup, minimum distance
between range values, three thumb-collision behaviors (`push` / `swap` /
`none`), center/edge thumb alignment, orientation + RTL awareness, formatted
value display, an associated label, and Field/Form integration.

`crates/base_gpui` has no slider. The goal is behavioral parity with Base UI
Slider using the local GPUI-native architecture — deep runtime, thin context,
thin layers, typed children, typed style state — with no DOM/React leakage.

Two structural translations dominate this port:

1. **Value model.** Base UI's value is `number | readonly number[]` decided at
   runtime. In Rust this is a typed enum, e.g.
   `SliderValues::Single(f64)` / `SliderValues::Range(Vec<f64>)`, supporting N
   thumbs with 2 the common case. Internally the runtime works on a
   `Vec<f64>` (single = one-element vec), like Base UI's derived sorted
   `values` array, and converts back at the callback/props boundary.
2. **No hidden `<input type="range">`.** Base UI nests a visually hidden range
   input inside each thumb for focus, keyboard, form submission, and ARIA.
   GPUI replaces this with one `FocusHandle` per thumb, GPUI actions for
   keyboard stepping, and a Field control registration from the root. Nothing
   about the hidden input survives as public API.

## Scope

Port the Slider component family from Base UI into GPUI-native components:

- `SliderRoot` — owns value(s), min/max/step/large-step/min-steps-between-values,
  orientation, disabled, thumb collision behavior, thumb alignment, format
  callback; fires cancelable `on_value_change` and `on_value_committed`.
- `SliderControl` — the interactive strip: pointer-down maps position to
  value, selects the closest thumb, captures the pointer, drags, commits on
  release.
- `SliderTrack` — structural container for the indicator (relative
  positioning context).
- `SliderIndicator` — the filled range, positioned from value percentages.
- `SliderThumb` — draggable handle; focusable; per-thumb keyboard stepping;
  per-thumb `disabled`; z-index priority for active/last-used thumb.
- `SliderValue` — formatted textual display of the current value(s).
- `SliderLabel` — labels the slider; clicking it focuses the thumb.

Base UI source references:

- `/home/luke/Projects/base-ui/packages/react/src/slider/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/slider/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/slider/root/SliderRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/slider/root/SliderRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/slider/root/SliderRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/slider/control/SliderControl.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/slider/control/SliderControl.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/slider/track/SliderTrack.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/slider/track/SliderTrack.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/slider/indicator/SliderIndicator.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/slider/indicator/SliderIndicator.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/slider/thumb/SliderThumb.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/slider/thumb/SliderThumb.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/slider/value/SliderValue.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/slider/value/SliderValue.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/slider/label/SliderLabel.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/slider/label/SliderLabel.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/slider/utils/roundValueToStep.ts` (+ `.test.ts`)
- `/home/luke/Projects/base-ui/packages/react/src/slider/utils/getSliderValue.ts` (+ `.test.ts`)
- `/home/luke/Projects/base-ui/packages/react/src/slider/utils/resolveThumbCollision.ts` (+ `.test.ts`)
- `/home/luke/Projects/base-ui/packages/react/src/slider/utils/getPushedThumbValues.ts` (+ `.test.ts`)
- `/home/luke/Projects/base-ui/packages/react/src/slider/utils/validateMinimumDistance.ts`
- `/home/luke/Projects/base-ui/packages/react/src/slider/utils/valueArrayToPercentages.ts`
- `/home/luke/Projects/base-ui/packages/react/src/slider/utils/getMidpoint.ts`
- `/home/luke/Projects/base-ui/packages/react/src/slider/utils/asc.ts`

Current GPUI implementation: none exists under `crates/base_gpui/src/slider/`;
this issue creates it.

Local GPUI references:

- `docs/base-gpui-component-architecture.md` — deep runtime / thin context /
  thin parts.
- `crates/base_gpui/src/number_field/` — closest sibling: numeric value,
  step/precision math (`number_field/number.rs`), keyboard actions
  (`number_field/actions.rs`), drag precedent
  (`number_field/layers/number_field_scrub_area.rs`), Field registration.
- `crates/base_gpui/src/tabs/layers/tabs_list.rs` — `on_children_prepainted`
  bounds measurement into the runtime.
- `crates/base_gpui/src/tabs/layers/tabs_indicator.rs` — measured/derived
  positioning exposed through a style-state query.
- `crates/base_gpui/src/field/context.rs` (`register_control`),
  `crates/base_gpui/src/field/runtime.rs` (`FieldControlRegistration`),
  `crates/base_gpui/src/field/layers/field_control.rs`.
- `crates/base_gpui/src/form/` — `clear_errors` on value change.
- `crates/base_gpui/src/utils/direction.rs` — `TextDirection`,
  `HorizontalArrowKey` mapping for RTL.
- `crates/base_gpui/src/collapsible/runtime.rs` — cancelable change-details
  pattern (`CollapsibleOpenChangeDetails`).
- `/home/luke/Projects/gpui-component/crates/ui/src/slider.rs` — layout
  reference only; it has no keyboard support, no step precision math, and no
  collision behavior, so it is not a behavioral reference.

Expected new files (flat layout per the architecture doc):

```text
crates/base_gpui/src/slider/mod.rs
crates/base_gpui/src/slider/actions.rs
crates/base_gpui/src/slider/child.rs
crates/base_gpui/src/slider/child_wiring.rs
crates/base_gpui/src/slider/context.rs
crates/base_gpui/src/slider/props.rs
crates/base_gpui/src/slider/style_state.rs
crates/base_gpui/src/slider/runtime.rs
crates/base_gpui/src/slider/math.rs
crates/base_gpui/src/slider/layers/mod.rs
crates/base_gpui/src/slider/layers/slider_root.rs
crates/base_gpui/src/slider/layers/slider_control.rs
crates/base_gpui/src/slider/layers/slider_track.rs
crates/base_gpui/src/slider/layers/slider_indicator.rs
crates/base_gpui/src/slider/layers/slider_thumb.rs
crates/base_gpui/src/slider/layers/slider_value.rs
crates/base_gpui/src/slider/layers/slider_label.rs
crates/base_gpui/src/slider/tests/
```

`math.rs` holds the pure value math ported from Base UI's `slider/utils/`
(step rounding, percent conversion, neighbor clamping, minimum distance,
collision resolution, push propagation). These are slider-specific; do not
put them under `utils/`. Where the logic is identical to
`number_field/number.rs` helpers (`clamp_value`, decimal-precision cleanup),
either reuse those or port the Base UI versions — decide during
implementation and avoid drift between two copies of the same precision rule.

## Initial design decisions

### Value model

```rust
pub enum SliderValues {
    Single(f64),
    Range(Vec<f64>),
}
```

- `Single` is the common scalar slider; `Range` supports N thumbs (2 typical).
- The runtime normalizes to a sorted ascending `Vec<f64>` for all internal
  math, mirroring Base UI's derived `values` (`SliderRoot.tsx` sorts a copy
  with `asc`; single values are clamped to `[min, max]`). The
  single-vs-range distinction is preserved so callbacks return the same shape
  the caller provided.
- Uncontrolled default when `.default_value(...)` is absent is
  `Single(min)`, matching Base UI's `defaultValue ?? min`.
- Do not use generics over the value shape; a runtime enum matches Base UI's
  behavior (shape can differ between `value` updates) and keeps the API flat.

### Public API shape

```rust
SliderRoot::new()
    .id("volume")
    .default_value(SliderValues::Range(vec![20.0, 80.0]))
    .min(0.0)
    .max(100.0)
    .step(1.0)
    .thumb_collision_behavior(SliderThumbCollisionBehavior::Push)
    .child(SliderLabel::new().child("Volume"))
    .child(SliderValue::new())
    .child(
        SliderControl::new()
            .child(SliderTrack::new().child(SliderIndicator::new()))
            .child(SliderThumb::new())
            .child(SliderThumb::new()),
    )
```

Thumbs are declared explicitly as children (one per range value), matching
Base UI composition. Thumb index is assigned by child wiring in declaration
order — never recomputed inside a part.

### Drag interaction (GPUI-native, no new primitive)

All drag behavior is per-component `div` handlers, following the
`number_field_scrub_area.rs` precedent (mouse down/move/up on a `div`), with
one key difference: the slider maps **absolute pointer position to value**,
so the control's content bounds must be measured. Concretely:

- `SliderControl` measures its own bounds (and each thumb's bounds) via
  prepaint (`on_children_prepainted` on the root/control, per
  `tabs/layers/tabs_list.rs`) into runtime commands
  (`set_control_bounds` / `set_thumb_bounds`). Measurement is free every
  frame; the runtime command returns whether anything changed.
- Pointer-down on the control: convert position to a value using measured
  bounds (`(max - min) * t + min`, `t` clamped to `[0, 1]`, RTL/vertical
  aware), round to the step grid, clamp, select the target thumb, apply as a
  `TrackPress` change, and begin tracking the drag.
- Use GPUI mouse-capture-equivalent handlers (`on_mouse_move` +
  `on_mouse_up`/`on_mouse_up_out` or `on_drag_move`) so the drag keeps
  receiving events when the pointer leaves the control, replacing DOM
  `setPointerCapture` + document-level listeners.
- Pointer-down **on a thumb** records the pressed thumb index and the
  pointer-to-thumb-center offset (`SliderThumb.tsx` `onPointerDown`); every
  subsequent position→value conversion subtracts this offset so the value
  does not jump to the pointer position. Pressing a thumb does not apply a
  value change until movement occurs (Base UI only applies `track-press`
  immediately when the press was *not* on a thumb).
- `dragging` becomes true only after more than 2 move events
  (`INTENTIONAL_DRAG_COUNT_THRESHOLD = 2` in `SliderControl.tsx`), except a
  control (track) press which sets dragging immediately.

### Keyboard (no hidden input)

Each `SliderThumb` owns a `FocusHandle` and a `key_context`; `actions.rs`
defines key-shaped actions bound from `base_gpui::init(cx)` (precedent:
`number_field/actions.rs`): Up, Down, Left, Right, Shift+Arrow variants,
PageUp, PageDown, Home, End. Handlers resolve orientation/RTL and dispatch
runtime step commands. Step math (grid rounding, decimal-precision cleanup,
clamping) is ported from `roundValueToStep.ts` / `getNewValue` and shared or
aligned with `number_field/number.rs` (`step_value`, `snap_to_step_grid`,
`clamp_value`, `clean_floating_point_noise`).

### Change details and cancellation

Mirror the collapsible cancelable-details pattern
(`CollapsibleOpenChangeDetails`): `on_value_change` receives the proposed
`SliderValues` plus `&mut SliderValueChangeDetails` carrying a reason
(`TrackPress`, `Drag`, `Keyboard`, `None`) and the active thumb index; calling
`details.cancel()` prevents the change from being applied (and suppresses the
subsequent commit). Base UI's `input-change` reason exists only for the hidden
DOM input and is dropped. `on_value_committed` receives generic (non-cancelable)
details with the reason of the last applied change.

### Field/Form integration

`SliderRoot` (not the thumbs) registers one `FieldControlRegistration`,
following Number Field: merged disabled, name inheritance
(`field name ?? slider name`), focused when any thumb is focused, touched on
blur/interaction, dirty when values differ from the initially registered
values, focus handle (first thumb) for label click focus. On every applied
value change, clear matching Form errors and run `OnChange` validation;
`OnBlur` validation runs on thumb blur. A slider always has a value, so it
registers as filled.

## Out of scope / drop from Base UI

- React context/hooks, `useControlled`, `useStableCallback`, `CompositeList`
  DOM registration — replaced by keyed entity runtime + typed child wiring.
- `render` props, `className`, web `style` props, CSS variables
  (`--position`, `--start-position`, `--relative-size`) — replaced by
  `style_with_state(...)` and runtime-computed `relative()` lengths.
- DOM data attributes (`data-dragging`, `data-base-ui-slider-*`,
  thumb index attribute) — mapped into typed style-state structs.
- The hidden `<input type="range">` per thumb, `inputRef`, `tabIndex`,
  `form` attribute, name-on-input form submission, and the cloned-event
  `event.target.value` trick — replaced by `FocusHandle` + actions + Field
  registration.
- ARIA (`aria-label`, `aria-valuetext`, `getAriaValueText`, `aria-live`,
  `role="group"`, label id linking) — deferred to the AccessKit follow-up.
- SSR/hydration: `prehydrationScript`, `renderBeforeHydration`,
  `edge-client-only` thumb alignment variant, `suppressHydrationWarning`.
- DOM measurement machinery: `getBoundingClientRect`, `getComputedStyle`
  padding/border offsets, `ResizeObserver` — replaced by GPUI prepaint bounds
  (padding/border compensation comes free from measuring content bounds).
- Touch identifier tracking (`touchIdRef`, `changedTouches`) — GPUI delivers
  unified mouse events.
- `Intl.NumberFormat` `format`/`locale` props — phase 1 exposes a plain
  Rust formatting closure `Fn(f64) -> SharedString` on the root (default:
  plain numeric display); locale-aware formatting is a follow-up shared with
  Number Field.
- Firefox/Safari disabled-focus blur workaround, `:focus-visible`
  restoration tricks, animation-frame focus deferral — use GPUI focus
  semantics directly.
- Do not use Rust scoped visibility syntax (`pub(...)`); `ast-grep scan`
  must remain clean.

## Acceptance Criteria

### Module/API surface

- [ ] Add a `slider` module and export it from `crates/base_gpui/src/lib.rs`.
- [ ] Register slider key bindings from `base_gpui::init(cx)` via `slider::init(cx)`.
- [ ] Add public `SliderRoot`, `SliderControl`, `SliderTrack`,
      `SliderIndicator`, `SliderThumb`, `SliderValue`, and `SliderLabel`
      layer types.
- [ ] Add `SliderValues` (`Single(f64)` / `Range(Vec<f64>)`) as the public
      value type.
- [ ] Add `SliderOrientation` (`Horizontal` default, `Vertical`).
- [ ] Add `SliderThumbCollisionBehavior` (`Push` default, `Swap`, `None`).
- [ ] Add `SliderThumbAlignment` (`Center` default, `Edge`).
- [ ] Support root `.id(...)` as the stable keyed identity.
- [ ] Support `.name(...)` metadata.
- [ ] Support uncontrolled `.default_value(SliderValues)`; absent default
      initializes to `Single(min)`.
- [ ] Support controlled `.value(SliderValues)`.
- [ ] Support `.on_value_change(...)` receiving the proposed `SliderValues`
      and `&mut SliderValueChangeDetails` (cancelable).
- [ ] Support `.on_value_committed(...)` receiving the committed
      `SliderValues` and non-cancelable commit details.
- [ ] Support `.min(f64)` (default `0.0`) and `.max(f64)` (default `100.0`).
- [ ] Support `.step(f64)`, defaulting to `1.0`, with `min` as the step-grid
      origin.
- [ ] Support `.large_step(f64)`, defaulting to `10.0`.
- [ ] Support `.min_steps_between_values(f64-count or usize)`, defaulting to
      `0`, expressed in steps (distance = `step * min_steps_between_values`).
- [ ] Support `.orientation(...)`, defaulting to horizontal.
- [ ] Support `.thumb_collision_behavior(...)`, defaulting to `Push`.
- [ ] Support `.thumb_alignment(...)`, defaulting to `Center`.
- [ ] Support `.format(...)` as a Rust closure formatting a single `f64` for
      `SliderValue` display (no `Intl` options).
- [ ] Support root `.disabled(bool)`, defaulting to `false`.
- [ ] Support per-thumb `.disabled(bool)` on `SliderThumb`, defaulting to
      `false`, merged with root/Field disabled.
- [ ] `SliderValue` supports a custom display closure over
      (formatted values, raw values); default display joins formatted values
      with `" – "`.
- [ ] Expose ergonomic barrel exports from `slider/mod.rs` (barrel only —
      no items defined in `mod.rs`).
- [ ] Do not expose CSS class, web style, CSS variable, or data-attribute
      APIs.

### Correctness / compile readiness

- [ ] `cargo check -p base_gpui` passes.
- [ ] `cargo test -p base_gpui` passes.
- [ ] `cargo clippy -p base_gpui --all-targets` passes.
- [ ] `ast-grep scan` passes (no `pub(...)` scoped visibility).
- [ ] Debug builds warn (log or debug assert) when `min >= max`, matching
      Base UI's dev warning; release behavior stays defined (no panic).

### Architecture / internal primitives

- [ ] `SliderRuntime` is the single deep module owning: normalized sorted
      values, single-vs-range shape, active thumb index, last-used thumb
      index, dragging flag, move count, pressed thumb index,
      pointer-to-thumb-center offset, drag-start values (for push/swap
      baselines), last change reason, pending commit value, measured control
      bounds, measured thumb bounds, and per-thumb focus handles.
- [ ] `SliderProps` holds stable public configuration and callbacks only —
      no runtime state.
- [ ] `SliderContext` stays thin: `read` / `update` / the value-changing
      command that resolves controlled vs uncontrolled and fires callbacks
      from the runtime outcome; no slider vocabulary beyond that.
- [ ] The runtime never calls user callbacks; value-change commands return
      outcomes and the context/root fire `on_value_change` /
      `on_value_committed`.
- [ ] Typed `SliderChild` enum for root children (`Control`, `Track` nesting
      as composed, `Value`, `Label`) and typed child enums for `SliderControl`
      (`Track`, `Thumb`) and `SliderTrack` (`Indicator`) matching Base UI
      composition; children stay typed until wiring erases to `AnyElement`.
- [ ] `child_wiring.rs` is the only place that assigns thumb indices (in
      declaration order) and attaches context; no index bookkeeping in parts.
- [ ] Root render is the single non-event mutation site: wire children,
      `sync_children` (thumb metadata + focus handles), `reconcile`
      (controlled value or uncontrolled current — the one transition
      resolution point; no shadow-value diffing anywhere else).
- [ ] Control and thumb bounds are measured via prepaint
      (`on_children_prepainted`, per `tabs/layers/tabs_list.rs`) into
      runtime `set_*_bounds` commands that report whether anything changed;
      no DOM-style measurement APIs.
- [ ] Position→value conversion, closest-thumb selection, collision
      resolution, step rounding, and clamping all live in the runtime /
      `math.rs`, not in layers.
- [ ] Pure value math in `math.rs` has no GPUI types except `Pixels`/`Bounds`
      where geometry is inherent, and is unit-testable without a window.
- [ ] Step/precision helpers are shared with or deliberately aligned to
      `number_field/number.rs`; no silently divergent duplicate of the same
      decimal-precision rule.
- [ ] No new shared generic primitives; slider-specific helpers stay under
      `slider/`.

### Controlled/uncontrolled value behavior

- [ ] Uncontrolled root initializes from `.default_value(...)`;
      absent default initializes to `Single(min)`.
- [ ] Controlled root reflects `.value(...)` across renders; interactions
      fire `on_value_change` without mutating internal state as source of
      truth.
- [ ] Controlled precedence: if `.value(...)` is supplied the root is
      controlled; otherwise uncontrolled.
- [ ] Range values are normalized to sorted ascending order for rendering
      and math even if provided unsorted.
- [ ] Single values outside `[min, max]` are clamped for rendering/math
      (Base UI clamps the derived value; the caller's controlled prop is not
      mutated).
- [ ] `on_value_change` fires at most once per applied change; a proposed
      value equal to the current value (scalar equality / element-wise array
      equality) is a no-op that fires nothing.
- [ ] NaN proposals are rejected without firing callbacks.
- [ ] `details.cancel()` inside `on_value_change` prevents the internal
      update (uncontrolled) and suppresses the eventual commit callback for
      that interaction, matching `changeDetails.isCanceled`.
- [ ] A canceled swap does not leak the swapped thumb index into subsequent
      drag moves (pressed index only updates when the change is applied).
- [ ] `on_value_committed` fires only if the value actually changed during
      the interaction and the change was not canceled.
- [ ] Value shape is preserved through callbacks: `Single` in → `f64`-shaped
      `Single` out; `Range` in → `Range` out with the same length.
- [ ] Re-rendering with changed unrelated props does not reset uncontrolled
      values, focus, or drag state unless the keyed `.id(...)` changes.

### Value / step / clamp math

- [ ] Port `roundValueToStep`: nearest step multiple with `min` as origin,
      rounded to `max(decimal_precision(step), decimal_precision(min))`
      decimal places.
- [ ] Port `getDecimalPrecision`, including the small-magnitude
      (`|x| < 1`, exponential notation) branch.
- [ ] Port `valueToPercent` (`(value - min) * 100 / (max - min)`) or its
      fraction equivalent used for `relative()` lengths.
- [ ] Port `getSliderValue` neighbor clamping: keyboard/programmatic
      per-thumb changes clamp to `[min, max]` and, in range mode, to the
      hard bounds of the neighboring values (no push/swap on this path).
- [ ] Port `validateMinimumDistance`: a proposed range is rejected when any
      adjacent pair is closer than `step * min_steps_between_values`.
- [ ] Keyboard stepping first rounds the current value to the step grid,
      then adds/subtracts the increment, then cleans decimal precision
      (`getNewValue`: precision = max of value/increment/min precisions),
      then clamps.
- [ ] Floating-point noise is cleaned for common cases (`0.1 + 0.2`,
      `toFixed(12)`-style cleanup in collision/push results).
- [ ] Pointer position→value: fraction of measured control main-axis size,
      clamped to `[0, 1]`, scaled to `[min, max]`, step-rounded, clamped —
      including the pressed-thumb-center offset subtraction and, for edge
      alignment, the half-thumb inset on both ends.
- [ ] All math helpers have focused unit tests translated from
      `roundValueToStep.test.ts`, `getSliderValue.test.ts`, and the
      collision test suites (see Tests section).

### Pointer drag / closest-thumb / capture behavior

- [ ] Left-button press only; presses on a disabled slider or a disabled
      thumb do nothing (disabled thumb press also resets pressed-thumb
      state).
- [ ] Press on the control (not a thumb): the nearest enabled thumb by
      midpoint distance on the main axis is selected (ties go to the later
      index), the value applies immediately with reason `TrackPress`,
      dragging state begins, and the selected thumb receives focus.
- [ ] Press on a thumb: pressed index and pointer-to-thumb-center offset are
      recorded; no value change is applied until movement; the thumb receives
      focus.
- [ ] Pressing a thumb whose value equals `max` while stacked on other
      max-valued thumbs re-targets to the first thumb of that stack so the
      group can move back down (Base UI `startPressing` walk-back rule).
- [ ] Drag-start values are captured as the baseline for push/swap
      restoration for the duration of the interaction.
- [ ] Moves convert position→value with the recorded center offset so the
      value does not jump on the first move.
- [ ] `dragging` style state turns true only after more than 2 move events
      for a thumb press (INTENTIONAL_DRAG threshold), immediate for a track
      press.
- [ ] Moves that violate minimum distance are ignored (no change, no
      callback).
- [ ] Drag changes fire `on_value_change` with reason `Drag` and the active
      thumb index each time the resolved value differs.
- [ ] The drag continues receiving moves when the pointer leaves the control
      bounds (GPUI capture-equivalent handlers replace DOM pointer capture +
      document listeners).
- [ ] If button state is observed released without an up event, the drag
      ends as if released (Base UI `buttons === 0` guard) — or document why
      GPUI cannot observe this and that `on_mouse_up_out` covers it.
- [ ] Release: clears active/dragging/pressed state and fires
      `on_value_committed` (with the last change reason) only if a change
      was applied during the interaction.
- [ ] Disabling the slider mid-interaction stops listening and clears
      transient drag state; active index resets to none while disabled.

### Keyboard step behavior

- [ ] `slider/actions.rs` defines actions bound in a slider key context set
      on the focused thumb: Up, Down, Left, Right, Shift+Up/Down/Left/Right,
      PageUp, PageDown, Home, End (binding names per
      `number_field/actions.rs` precedent).
- [ ] ArrowUp increments and ArrowDown decrements by `step`, regardless of
      orientation.
- [ ] ArrowRight/ArrowLeft increment/decrement by `step` in LTR and are
      flipped in RTL (via `utils/direction.rs`), for both orientations.
- [ ] Shift+Arrow uses `large_step` in place of `step`, same direction rules.
- [ ] PageUp/PageDown increment/decrement by `large_step`.
- [ ] Home sets the thumb to `min`; End sets it to `max`.
- [ ] In range mode, Home/End clamp to the neighbor boundary:
      End → `next_neighbor - step * min_steps_between_values` when a next
      neighbor exists; Home → `previous_neighbor + step * min_steps_between_values`
      when a previous neighbor exists.
- [ ] Keyboard changes go through the neighbor-clamped path
      (`getSliderValue` port) — keyboard never pushes or swaps thumbs.
- [ ] Keyboard changes that violate minimum distance are ignored.
- [ ] Applied keyboard changes fire `on_value_change` (reason `Keyboard`),
      mark the field touched, and commit immediately via
      `on_value_committed` (reason `Keyboard`).
- [ ] Keyboard on a disabled slider or disabled thumb does nothing.
- [ ] Each thumb is a tab stop via its `FocusHandle`; focusing a thumb sets
      it active; blurring clears active and marks touched.

### Thumb collision behaviors

- [ ] Port `resolveThumbCollision` and `getPushedThumbValues` as pure
      functions in `math.rs`, translating Base UI's unit tests
      (`resolveThumbCollision.test.ts`, `getPushedThumbValues.test.ts`).
- [ ] Single-value sliders bypass collision resolution entirely.
- [ ] `Push` (default): dragging a thumb past a neighbor pushes the neighbor
      along, respecting `step * min_steps_between_values` spacing and
      per-index min/max budgets; pushed neighbors restore toward their
      drag-start values when the pressed thumb retreats, without overshooting.
- [ ] `Swap`: dragging past a neighbor (epsilon `1e-7`) swaps the pressed
      thumb to the neighbor's index; the swapped-out neighbor is restored to
      the pressed thumb's clamped value within its bounds; the active/pressed
      index and focus follow the swap; `didSwap` outcomes drive refocusing.
- [ ] `None`: the pressed thumb clamps to
      `[prev + min_distance, next - min_distance]`; excess movement is
      ignored.
- [ ] Collision results are precision-cleaned (`toFixed(12)` equivalent).
- [ ] Collision behavior applies only to pointer interactions; keyboard uses
      neighbor clamping regardless of the configured behavior.

### Field / Form integration

- [ ] Slider inside `FieldRoot` merges Field disabled state
      (`field_disabled || slider_disabled`).
- [ ] Slider inside a disabled `FieldItem` is disabled.
- [ ] `SliderRoot` registers one `FieldControlRegistration` with a stable
      key.
- [ ] Registration name uses Field name when present, else the slider's
      `.name(...)` (Base UI: `fieldName ?? nameProp`).
- [ ] Registration includes a value representation compatible with existing
      `FieldValue` variants (formatted text or `Present`); add a numeric
      variant only if it stays shallow for existing controls.
- [ ] Registration includes merged disabled, focused state, and the first
      thumb's focus handle so `FieldLabel` click focuses the slider.
- [ ] A slider always registers as filled (its value always exists).
- [ ] Field becomes dirty when values differ from the initially registered
      values (element-wise for ranges).
- [ ] Field becomes focused while any thumb is focused; touched when a thumb
      blurs or a value-changing interaction occurs.
- [ ] `FieldValidationMode::OnChange` validates on applied value changes;
      `FieldValidationMode::OnBlur` validates on thumb blur.
- [ ] Applied value changes clear matching Form errors (Base UI
      `clearErrors(name)`).
- [ ] `SliderLabel` click focuses the thumb when the slider has exactly one
      thumb; with multiple thumbs it is a no-op (Base UI only falls back when
      exactly one input exists).

### Orientation / RTL behavior

- [ ] Horizontal sliders map the main axis to x; vertical sliders map to y
      with values increasing upward (position measured from the bottom
      edge).
- [ ] RTL horizontal sliders (via `DirectionProvider`) invert
      position→value mapping (measured from the right edge) and thumb /
      indicator placement.
- [ ] RTL flips ArrowLeft/ArrowRight stepping; ArrowUp/ArrowDown and
      PageUp/PageDown are unaffected.
- [ ] Vertical sliders keep drag, track-press, closest-thumb, and indicator
      math correct on the y axis.
- [ ] Orientation is exposed in style state for all parts.

### Positioning / thumb alignment

- [ ] Thumbs are absolutely positioned along the main axis using
      `relative()` fractions from the value percent (precedent:
      `tabs/layers/tabs_indicator.rs` style-state-driven placement); the
      cross-axis centering translation is GPUI styling, not CSS variables.
- [ ] `SliderIndicator` (center alignment): single → spans from the start
      edge to `percent(value)`; range → spans from `percent(first)` to
      `percent(last)`.
- [ ] Active-thumb stacking: in range mode the active thumb renders above
      others and the last-used thumb above the rest (Base UI z-index 2/1);
      single mode raises the active thumb.
- [ ] `SliderThumbAlignment::Edge` computes inset positions from measured
      control and thumb sizes each prepaint (travel = control size − thumb
      size; position percent includes the half-thumb offset), for thumbs and
      the indicator; no ResizeObserver — prepaint measurement is free every
      frame.
- [ ] Edge alignment also insets the pointer position→value conversion by
      half the thumb size on each end.
- [ ] Before first measurement, edge-aligned thumbs/indicator expose a
      not-yet-positioned state (Base UI hides them) rather than flashing at
      a wrong position.

### Styling / state exposure

- [ ] Add `SliderRootStyleState`, `SliderControlStyleState`,
      `SliderTrackStyleState`, `SliderIndicatorStyleState`,
      `SliderThumbStyleState`, `SliderValueStyleState`, and
      `SliderLabelStyleState` in `style_state.rs`, one per drawing part.
- [ ] Style states expose (where relevant): `values`, `min`, `max`, `step`,
      `orientation`, `disabled`, `dragging`, `active_thumb_index`, and Field
      validity/dirty/touched/focused facts, mirroring Base UI's
      `SliderRootState` + data attributes as typed fields.
- [ ] `SliderThumbStyleState` additionally exposes the thumb's index, value,
      formatted value, focused/active flags, per-thumb disabled, and its
      resolved main-axis position fraction.
- [ ] `SliderValueStyleState` exposes formatted values and raw values.
- [ ] Every public layer supports `style_with_state(...)` receiving its
      typed style state.
- [ ] Queries are part-shaped ("this thumb's state"), not runtime-internal
      ("the active index"), per the architecture doc.
- [ ] Do not expose DOM data attributes or CSS variable names as styling
      API.

### Docs / demo

- [ ] Update `crates/base_gpui/src/main.rs` (or an example) with a single
      slider and a two-thumb range slider demo, including a
      `FieldRoot` + `FieldLabel` + slider + `FieldError` composition.
- [ ] Document the supported phase-1 prop subset and explicitly list dropped
      or deferred Base UI props (`format`/`locale` Intl options, ARIA
      options, `edge-client-only`, `form`, `inputRef`, `tabIndex`).

### Tests / verification

Runtime/math unit tests (no window) plus rendered behavior tests under
`crates/base_gpui/src/slider/tests/`:

- [ ] `round_value_to_step` cases translated from `roundValueToStep.test.ts`
      (decimal steps, decimal min origin, small-magnitude precision).
- [ ] `get_slider_value` neighbor-clamp cases from `getSliderValue.test.ts`.
- [ ] `resolve_thumb_collision` push/swap/none cases from
      `resolveThumbCollision.test.ts` (including canceled-swap index rules,
      epsilon boundaries, min-distance spacing).
- [ ] `get_pushed_thumb_values` cases from `getPushedThumbValues.test.ts`
      (push budgets at min/max, restoration toward initial values).
- [ ] `validate_minimum_distance` accepts/rejects correctly for 2 and 3+
      thumbs.
- [ ] Position→value conversion: horizontal LTR, horizontal RTL, vertical,
      center offset subtraction, and edge-alignment inset.
- [ ] Uncontrolled default value (`Single(min)` when absent; provided
      default otherwise).
- [ ] Controlled value is reflected; interaction fires `on_value_change`
      without self-mutation; unsorted controlled ranges render sorted.
- [ ] `details.cancel()` prevents the change and the commit.
- [ ] Equal-value proposal fires no callbacks.
- [ ] Track press applies the closest-thumb value with reason `TrackPress`
      and focuses that thumb; tie goes to the later thumb.
- [ ] Thumb press applies no change until movement; the first move does not
      jump (center offset honored).
- [ ] Max-stacked thumb press re-targets to the first thumb of the stack.
- [ ] Dragging past a neighbor pushes (default), swaps with
      `Swap` (focus follows), and clamps with `None`.
- [ ] `dragging` style state becomes true only after the intentional-drag
      threshold for a thumb press.
- [ ] Release fires `on_value_committed` exactly once, only when a change
      was applied.
- [ ] Keyboard: Arrow/Shift+Arrow/PageUp/PageDown/Home/End step and clamp
      correctly; RTL flips Left/Right; range Home/End clamp to neighbor
      boundaries; each applied keyboard change commits immediately.
- [ ] Decimal-step keyboard stepping stays precision-clean
      (e.g. `0.1` steps do not accumulate noise).
- [ ] Disabled slider ignores pointer and keyboard; a per-thumb disabled
      thumb is skipped by closest-thumb selection and cannot be dragged.
- [ ] `min_steps_between_values` blocks drag and keyboard changes that
      violate spacing.
- [ ] Indicator geometry matches value percents for single and range, both
      orientations.
- [ ] Edge alignment positions thumbs/indicator from measured sizes and
      updates when sizes change.
- [ ] `SliderValue` renders the default joined formatted display and a
      custom display closure.
- [ ] Field integration: label click focuses a single-thumb slider; filled
      always true; dirty/touched/focused update; `OnChange`/`OnBlur`
      validation timing; Form errors cleared on change.
- [ ] Style states expose dragging/active/disabled/orientation/values for
      every part.
- [ ] `cargo test -p base_gpui slider` passes.

## AccessKit accessibility follow-up

Base UI exposes the slider through per-thumb `<input type="range">` semantics:
`aria-valuenow`, `aria-valuetext` (with range start/end phrasing),
`aria-orientation`, `aria-labelledby` from `SliderLabel`, and `role="group"`
on the root. GPUI does not currently emit accessibility attributes, so this
is deferred, consistent with prior ports. When AccessKit support lands:

- [ ] Expose each thumb as an AccessKit slider node with numeric
      value/min/max/step and formatted value text.
- [ ] Wire `SliderLabel` as the accessible label relation.
- [ ] Revisit `getAriaLabel` / `getAriaValueText` equivalents as typed
      closures.

## Uncertain items needing confirmation

- **`SliderValues` naming.** The display part keeps Base UI's `SliderValue`
  name, so the value enum cannot also be `SliderValue`. Proposed:
  `SliderValues` (mirrors Base UI's internal plural `values`). Confirm or
  pick an alternative before implementation.
- **Step-precision helper sharing.** Whether `number_field/number.rs`
  helpers are lifted to a shared location or the slider ports its own copies
  in `slider/math.rs` (Base UI's `roundValueToStep` grid rule differs
  slightly from Number Field's snap semantics). Default: slider-local ports
  in `math.rs`, with a comment cross-linking the Number Field equivalents.
- **`FieldValue` representation for sliders.** Formatted `Text`, `Present`,
  or a new shallow numeric/list variant. Default: formatted `Text` of the
  joined values, matching current Field infrastructure.
- **Wheel support.** Base UI Slider has no wheel interaction; confirm we do
  not add one (Number Field's wheel scrub is a different component contract).
- **`min_steps_between_values` numeric type.** Base UI allows arbitrary
  numbers; a `usize` step count is the more honest type unless fractional
  step counts are wanted. Default: `f64` to match Base UI mathematics
  exactly.
