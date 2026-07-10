# Port Base UI Toggle to GPUI

## Problem

Base UI Toggle is a two-state pressable button that can be on (pressed) or off (unpressed). It provides controlled and uncontrolled pressed state, pointer and keyboard activation, disabled handling, cancelable `onPressedChange` notifications, and a `value` identity so the same component can also participate in a Toggle Group: standalone it is a plain pressable button, inside a `ToggleGroup` it becomes a roving-focus composite item whose pressed state is derived from membership in the group's value array and whose activation is routed through the group.

`crates/base_gpui` currently has no Toggle component. The goal is to port Toggle behavior into GPUI-native components using the current `base_gpui` runtime/context/layers architecture, not to copy React, DOM, `useButton`, composite, or ARIA implementation details.

Toggle state is boolean. Controlled state should use `Option<bool>` in the GPUI API, where `None` means uncontrolled and `Some(value)` means the caller owns the pressed value. The `value` identity used for group membership is a generic `T: Clone + Eq + 'static`, so the public type is `Toggle<T>`. This matches the Tabs/Radio Group precedent and is required so the Toggle and Toggle Group value types are identical — `issues/port-baseui-toggle-group.md` owns `ToggleGroup<T>` with a `Vec<T>` value. Base UI's `Value extends string` constraint is not carried over. Standalone use may default the parameter (e.g. `Toggle<T = SharedString>`) so callers who never group need not spell out `T`.

This issue covers the standalone Toggle contract plus the seams a Toggle Group needs. The grouped behaviors listed here (pressed derivation, commit routing, veto ordering) are specified from the Toggle side and are verified end-to-end together with `issues/port-baseui-toggle-group.md`, which owns the composite-item roving-focus mode, orientation, and group-level disabled cascade.

## Scope

Port the Toggle component from Base UI into a GPUI-native component:

- `Toggle<T>` (single part; Base UI renders one `<button>`), generic over the group-membership value type `T: Clone + Eq + 'static`

Base UI reference files:

- `/home/luke/Projects/base-ui/packages/react/src/toggle/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toggle/Toggle.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toggle/ToggleDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toggle/Toggle.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/internals/use-button/useButton.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toggle-group/ToggleGroupContext.ts` (consumed seam only)
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/toggle/page.mdx`

Current GPUI implementation:

- No `crates/base_gpui/src/toggle/` implementation exists yet.

Expected GPUI implementation files:

- `crates/base_gpui/src/toggle/mod.rs`
- `crates/base_gpui/src/toggle/actions.rs`
- `crates/base_gpui/src/toggle/context.rs`
- `crates/base_gpui/src/toggle/props.rs`
- `crates/base_gpui/src/toggle/style_state.rs`
- `crates/base_gpui/src/toggle/runtime.rs`
- `crates/base_gpui/src/toggle/layers/mod.rs`
- `crates/base_gpui/src/toggle/layers/toggle.rs`
- `crates/base_gpui/src/toggle/tests/`

Implementation precedents:

- `crates/base_gpui/src/switch/layers/switch_root.rs` and `crates/base_gpui/src/switch/actions.rs` — the pressable-button recipe: `div()` + `track_focus` + tab stop + `key_context` + `on_click` + Space/Enter actions. Toggle needs no new primitive.
- `crates/base_gpui/src/switch/runtime.rs` and `crates/base_gpui/src/switch/props.rs` — boolean controlled/uncontrolled resolution and the cancelable Rust-native change-details shape (`SwitchCheckedChangeDetails` with `reason()`/`source()`/`cancelable()`/`cancel()`/`is_canceled()`).
- `crates/base_gpui/src/radio_group/context.rs` and `crates/base_gpui/src/radio_group/layers/radio_group_radio.rs` — an item consuming an optional group context: deriving its own on-state from the group value and routing its commit through the group runtime.
- `/home/luke/Projects/gpui-component/crates/ui/src/button/toggle.rs` — gpui-component analog for the visual/interaction shape (reference only; do not copy its API).

## Out of scope / drop from Base UI

- Do not port React context/hooks directly; use GPUI keyed state/entities plus a thin `ToggleContext` wrapper, and consume the Toggle Group context through GPUI context plumbing when it exists.
- Do not port `render` props.
- Do not port `className`.
- Do not port web `style` props; expose GPUI builder styling and `style_with_state(...)`.
- Do not port `nativeButton` / `type`; GPUI does not expose a native button element, so the pressable is built from `div()` plus focus/click/action behavior (the Switch recipe).
- Do not port `useButton`'s DOM tag validation, `tabIndex` juggling, link/anchor special-casing, or keydown-vs-keyup Space dispatch distinctions; GPUI actions and key contexts replace them.
- Do not port the `form` prop; Base UI Toggle never participates in form validation.
- Do not port arbitrary HTML attributes or DOM event props.
- Do not port SSR/hydration APIs.
- Do not port CSS variable APIs.
- Do not port DOM data attributes (`data-pressed`, `data-disabled`) as attributes; map them into typed style-state fields.
- Do not port arbitrary DOM event objects. Use Rust-native change details for cancellation, reason, and source information.
- Do not write DOM ARIA attributes (`aria-pressed`); map accessibility through GPUI-native AccessKit APIs once available.
- Toolbar integration is out of scope: Base UI Toggle publishes `ToolbarRoot.ItemMetadata` (`{ disabled, focusableWhenDisabled: false }`) so a Toolbar can compute its disabled roving-focus indices. Do not build this now, but keep the runtime's disabled fact queryable so a future Toolbar port can host toggles without reworking this component. Track the actual wiring in the Toolbar port issue.
- The Toggle Group component itself (`ToggleGroup` root, orientation, multiple-pressed policy, roving focus composite root) is out of scope; only the Toggle-side seam is specified here. See `issues/port-baseui-toggle-group.md`.

## Acceptance Criteria

### Module/API surface

- [x] Add a `toggle` module and export it from `crates/base_gpui/src/lib.rs`.
- [x] Register Toggle key bindings from `base_gpui::init(cx)`.
- [x] Add a public `Toggle` layer type that accepts arbitrary content children (label/icon), since Base UI Toggle renders a button with arbitrary children; no typed child enum is needed for a single-part component.
- [x] Support uncontrolled construction with `.default_pressed(bool)`, defaulting to `false`.
- [x] Support controlled construction with `.pressed(Option<bool>)`; controlled state takes precedence over internal state.
- [x] Support `.disabled(bool)`, defaulting to `false`.
- [x] Support `.value(T)` with `T: Clone + Eq + 'static` as the identity used for Toggle Group membership; it has no standalone behavior. The public type is `Toggle<T>`, matching `ToggleGroup<T>` in `issues/port-baseui-toggle-group.md`.
- [x] Support `.on_pressed_change(...)` with a Rust-native cancelable change-details API, e.g. `Fn(bool, &mut TogglePressedChangeDetails, &mut Window, &mut App)`.
- [x] Add `TogglePressedChangeDetails` with `reason()`, `source()`, `cancelable()`, `cancel()`, and `is_canceled()` APIs, matching the Switch details shape.
- [x] Add `TogglePressedChangeReason::None`, matching Base UI's current single change reason.
- [x] Add a source enum such as `TogglePressedChangeSource::{Pointer, Keyboard}` without exposing DOM event objects.
- [x] Support `.style_with_state(...)` taking `ToggleStyleState`.
- [x] `toggle/mod.rs` exposes ergonomic barrel exports for the component name, style state, context, props, runtime, actions, and change-details types; `mod.rs` is barrel-only.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui toggle` passes.
- [x] The component compiles without adding web/React-specific concepts to public APIs.
- [x] The implementation follows `docs/base-gpui-component-architecture.md`: flat module layout (no `child/context/{props,runtime,state}` taxonomy), no `pub(...)` visibility qualifiers, and clean under the repo's ast-grep rules.
- [x] Add a small example/demo in `crates/base_gpui/src/main.rs` or a dedicated example that renders a standalone Toggle.

### Architecture / internal primitives

- [x] Add `ToggleRuntime` as the single owner of Toggle business state: uncontrolled pressed value, focused state, and the resolved disabled fact; commands return outcomes and the runtime never calls user callbacks.
- [x] Add `ToggleProps` for stable props and callbacks.
- [x] Add `ToggleContext` as a thin injection/plumbing type with only `read(...)`, `update(...)`, and `toggle(...)`-style methods; the controlled/uncontrolled rule and callback firing live in `ToggleContext::toggle(...)`, not in layers.
- [x] Keep Toggle behavior on `ToggleRuntime`; do not grow component vocabulary on `ToggleContext` beyond the value-changing method.
- [x] Reuse the Switch pressable recipe in `layers/toggle.rs` (`div()` + `track_focus` + tab stop + `key_context` + `on_click` + Space/Enter actions); do not introduce a new shared button primitive.
- [x] Add `ToggleStyleState` in `style_state.rs` as the query result the layer feeds to `style_with_state`.
- [x] Do not add a `utils/` folder for Toggle.
- [x] Keep the disabled fact (own prop OR group disabled) resolved inside the runtime so a future Toolbar port can read it as item metadata without new API.

### Controlled/uncontrolled pressed + cancelable change details

- [x] Uncontrolled Toggle initializes pressed state from `default_pressed`, defaulting to unpressed.
- [x] Uncontrolled Toggle mutates internal pressed state on accepted user activation.
- [x] Controlled Toggle reflects the external `pressed` value; external changes update style state.
- [x] Controlled Toggle calls `on_pressed_change` on valid user activation without mutating internal pressed state as the source of truth.
- [x] User activation computes the next pressed value as `!current_pressed`.
- [x] `on_pressed_change` is called exactly once per accepted user activation, with the next pressed value.
- [x] `on_pressed_change` is not called for disabled activation attempts.
- [x] User-initiated change details are cancelable; uncontrolled mode calls the callback before mutating internal state and skips mutation when canceled.
- [x] Controlled mode calls the callback but never mutates internal state; cancellation only confirms that Base GPUI performs no internal state change.
- [x] Expose pointer vs keyboard source and `TogglePressedChangeReason::None`; do not expose Base UI's native `event`/`trigger`/propagation APIs.
- [x] Re-rendering with changed props does not reset uncontrolled state except when the keyed instance id changes.

### Standalone pressable behavior

- [x] Clicking an enabled Toggle flips unpressed to pressed and pressed to unpressed.
- [x] Clicking a disabled Toggle does not toggle and does not call `on_pressed_change`.
- [x] `Toggle` owns a stable keyed `FocusHandle` and is focusable when enabled.
- [x] Toggle uses GPUI actions/key dispatch and a Toggle key context instead of raw key handlers; Space and Enter toggle an enabled, focused Toggle.
- [x] Disabled Toggle does not toggle from keyboard activation.
- [x] Space/Enter activation does not double-fire if GPUI exposes both key-down and key-up style dispatch.
- [x] Pointer activation and keyboard activation share the same runtime toggle command.
- [x] Focused state is synced into `ToggleRuntime` and exposed through style state if the Switch precedent exposes focused styling (match Switch here).
- [x] Match the Switch decision on whether disabled Toggle is removed from tab order, revisiting with AccessKit work later.

### Grouped mode (seam; verified with `issues/port-baseui-toggle-group.md`)

- [x] When a Toggle Group context is present, the Toggle's pressed state is controlled by group membership: pressed iff the group value contains this Toggle's `value`. `pressed`/`default_pressed` props do not drive state inside a group (Base UI discards `defaultPressed` and derives the controlled value from membership).
- [x] Inside a group, a Toggle requires an explicit `value`; Base UI's `useBaseUiId` auto-identity fallback is not ported (per `issues/port-baseui-toggle-group.md`). A `value`-less grouped Toggle emits a debug-time warning mirroring Base UI's dev error and does not participate in group membership.
  - Note: verified end-to-end — ToggleGroup child wiring emits the debug warning and `toggle_group/tests/valueless_toggle.rs` covers membership inertness.
- [x] Toggle's effective disabled state is its own `disabled` prop OR the group's disabled state.
- [x] Veto ordering, toggle-before-group: on activation, `on_pressed_change(next, details)` fires first; if canceled, neither the Toggle nor the group value changes. Otherwise the commit is routed to the group runtime (group value add/remove for this Toggle's `value`), and the group's own change callback receives the same shared details object; if the group cancels, the local pressed commit is also skipped.
  - Note: verified end-to-end via `toggle_group/tests/{toggle_cancellation,group_cancellation}.rs`; the group details type is an alias of `TogglePressedChangeDetails`, so one object is shared per activation.
- [x] `on_pressed_change` fires exactly once per accepted activation in grouped mode too; the group commit does not re-enter the Toggle callback.
  - Note: verified via `toggle_group/tests/value_change_payload.rs` (one pressed-change per accepted activation, no re-entry).
- [x] Inside a group, the Toggle participates as a composite/roving-focus item rather than an independent tab stop; the composite-item mechanics (tab-stop assignment, arrow navigation, orientation) are owned and verified by the Toggle Group issue.
  - Note: implemented — grouped Toggle renders under the ToggleGroup key context with a roving tab stop; verified by the toggle_group roving-focus tests.
- [x] Standalone behavior is fully functional without any group context present; the group seam adds no cost or API burden to standalone use.

### Styling/state exposure

- [x] `ToggleStyleState` includes at least `pressed` and `disabled` (Base UI's `data-pressed`/`data-disabled`), plus `focused` if the Switch precedent exposes it.
- [x] Expose state-aware styling through `style_with_state(...)`.
- [x] Map Base UI state/data attributes into typed style-state fields, not DOM attributes; do not expose CSS variable names.
- [x] The docs styling pattern is recreatable with GPUI builder methods: background/foreground change when pressed, muted styling when disabled.

### Tests / verification

Add one behavior per file under `crates/base_gpui/src/toggle/tests/`.

- [x] Uncontrolled initial unpressed state.
- [x] Uncontrolled `default_pressed(true)` initial pressed state.
- [x] Uncontrolled click flips unpressed to pressed and pressed to unpressed.
- [x] Controlled pressed state reflects external value; external changes update style state.
- [x] Controlled activation calls `on_pressed_change` without mutating internal state.
- [x] `on_pressed_change` is called exactly once with the next pressed value.
- [x] Disabled click does not toggle and does not call the change handler.
- [x] Space and Enter toggle when focused; disabled keyboard activation does not toggle.
- [x] Canceled uncontrolled pointer activation does not mutate pressed state.
- [x] Canceled uncontrolled keyboard activation does not mutate pressed state.
- [x] Canceled controlled activation still calls the handler but does not mutate internal state.
- [x] Pointer vs keyboard source is reported correctly in change details.
- [x] `style_with_state(...)` receives correct pressed/disabled state.
- [x] Grouped derivation: with a (real or stubbed) group context whose value contains the Toggle's `value`, the Toggle reports pressed; when membership is removed, it reports unpressed. Full grouped interaction tests (commit routing, veto ordering across Toggle and group callbacks, roving focus) live with `issues/port-baseui-toggle-group.md`.

## AccessKit accessibility follow-up

Base UI Toggle renders a single `<button aria-pressed={pressed} disabled?>` (see `Toggle.tsx`: `'aria-pressed': pressed` plus `useButton`'s disabled handling; `ToggleDataAttributes.ts` adds only `data-pressed`/`data-disabled`, already mapped into `ToggleStyleState`). The port has one accessible part: the `Toggle<T>` layer in `crates/base_gpui/src/toggle/layers/toggle.rs`, in both its standalone render path and `render_grouped(...)`.

### Per accessible part

- **`Toggle<T>` (standalone path, `RenderOnce::render`)** — the element already has a stable `.id(self.id)`, so it only needs a role and state props on the same builder chain:
  - `.role(Role::Button)` — accesskit 0.24 has no dedicated "toggle button" role; `Role::Button` + toggled state is the correct mapping for `aria-pressed`.
  - `.aria_toggled(...)` from the resolved pressed value: the layer already computes `style_state = context.read(cx, |runtime, _props| runtime.state())`, so map `style_state.pressed` → `Toggled::True` / `Toggled::False`. This is the AccessKit equivalent of Base UI's `aria-pressed`.
  - `.aria_label(...)` from a new optional `.aria_label(impl Into<SharedString>)` builder prop on `Toggle<T>` (see Labels).
- **`Toggle<T>` (grouped path, `render_grouped`)** — same `.role(Role::Button)` and `.aria_toggled(...)`, sourced from the locally computed `pressed` (`runtime.toggle_pressed(value.as_ref())`) and the same `aria_label` prop. Optionally add `.aria_position_in_set(index + 1)` from `toggle.group_index` and `.aria_size_of_set(...)` if the `ToggleGroupContext` runtime exposes the item count; Base UI does not emit `aria-posinset` here, so treat this as optional polish, not parity. The group container's own role/orientation belongs to `issues/port-baseui-toggle-group.md`.

### Actions

- No new `.on_a11y_action(...)` handlers are required. `Action::Click` is auto-registered by the existing `.on_click(...)` in both paths, and `Action::Focus` is auto-registered by the existing `.track_focus(&focus_handle...)`. Both already route into the same runtime transition as pointer/keyboard (`ToggleContext::toggle(...)` standalone, `grouped_activate(...)` grouped) — but note the standalone `.on_click` closure early-returns unless `matches!(event, ClickEvent::Mouse(_))`. Verify how an AT-dispatched `Action::Click` surfaces through gpui's synthesized `ClickEvent`; if it is not a `ClickEvent::Mouse`, either accept the synthesized variant in the guard or add an explicit `.on_a11y_action(AccessibleAction::Click, ...)` that calls the same `context.toggle(TogglePressedChangeSource::Keyboard, ...)` / `grouped_activate(...)` path. Do not double-register otherwise.

### Labels

- Add `.aria_label(...)` as a builder prop on `Toggle<T>` (stored alongside `disabled`/`pressed`, applied in both render paths). Toggles are frequently icon-only (e.g. a bold "B" glyph), so an explicit label is the primary mechanism; Base UI relies on the button's text content or the caller's `aria-label`.
- When the caller supplies both an `aria_label` and visible text children rendered via `text!(...)`, the demo/gallery usage should switch that visible text to `Text::new_inaccessible(...)` so the label is not announced twice. Since `Toggle` accepts arbitrary `AnyElement` children, document this caller-side convention in the module docs rather than enforcing it.

### Gaps (no gpui builder in the pinned revision)

- **`disabled` / `aria-disabled`**: no `.aria_disabled(...)` builder exists and `write_a11y_info` never sets a disabled flag. Fallback: the layer already removes the disabled toggle from the tab order (`tab_stop(!disabled)` / `tab_index(-1)`) and the runtime rejects activation, so AT-dispatched Click/Focus are inert; document that AT will not *announce* the disabled state — blocked pending a gpui upstream `set_disabled` addition.
- **`aria-pressed` as a distinct property**: AccessKit models it via `set_toggled`; `.aria_toggled(...)` is the accepted stand-in. Document that screen readers may announce "toggled"/"checked" phrasing rather than "pressed"; no further fallback available.
- **Relationship props** (`aria-labelledby`, `aria-describedby`, etc.): Base UI Toggle itself emits none, but callers who would have used `aria-labelledby` must use the literal-string `.aria_label(...)` instead — there is no id-reference wiring in this revision. Omit + document.
- **Toolbar item metadata** (`ToolbarRoot.ItemMetadata`): already out of scope above; no a11y impact until the Toolbar port.

### Checklist

- [ ] Add `.role(Role::Button)` to the standalone render path in `layers/toggle.rs`.
- [ ] Add `.aria_toggled(Toggled::True/False)` from `style_state.pressed` in the standalone path.
- [ ] Add `.role(Role::Button)` and `.aria_toggled(...)` from `pressed` in `render_grouped(...)`.
- [ ] Add an `.aria_label(impl Into<SharedString>)` builder prop on `Toggle<T>` and apply it in both render paths.
- [ ] Verify AT-dispatched `Action::Click` reaches the runtime through the `ClickEvent::Mouse(_)` guard; if not, wire an explicit `.on_a11y_action(AccessibleAction::Click, ...)` into `ToggleContext::toggle(...)` / `grouped_activate(...)`. Do not re-add `Action::Focus` (auto-registered by `track_focus`).
- [ ] Document in module docs that icon-only toggles must set `.aria_label(...)`, and that visible text duplicating the label should use `Text::new_inaccessible(...)`.
- [ ] Document the disabled-announcement gap (no `.aria_disabled` builder; tab-order removal + inert activation is the interim behavior, blocked pending gpui upstream).
- [ ] Update the gallery/demo toggle to set `.aria_label(...)`.

### Uncertain / needs confirmation

- [x] Value type — RESOLVED: `Toggle<T>` with `T: Clone + Eq + 'static`, matching `ToggleGroup<T>` (`issues/port-baseui-toggle-group.md`) and the Tabs/Radio Group precedent. Base UI's `Value extends string` constraint is dropped; standalone use may default the parameter (e.g. `Toggle<T = SharedString>`). (Reconciled with the Toggle Group port.)
- [x] Grouped-mode plumbing: whether the group context reaches the Toggle through child wiring owned by the ToggleGroup root (Radio Group precedent) or through an ambient context. Decide in the Toggle Group issue; this issue only requires that standalone Toggle works with no group present.
  - Note: RESOLVED — child wiring owned by the ToggleGroup root (Radio Group precedent) attaches the context, index, and focus handle via `Toggle::with_toggle_group`.
- [x] Auto-generated group identity — RESOLVED: not ported; grouped mode requires an explicit `value`, and a `value`-less grouped Toggle only emits a debug warning (per `issues/port-baseui-toggle-group.md`).
