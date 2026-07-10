# Port Base UI Toggle Group to GPUI

## Problem

Base UI Toggle Group provides shared pressed state for a series of Toggle buttons. It owns a group value (an array of the values of all pressed toggles), controlled/uncontrolled value state, single-select (`multiple=false`, default) versus multi-select (`multiple=true`) commit semantics, cancelable `onValueChange` notifications that share one details object with the item's `onPressedChange` (either callback can veto), a group-level disabled cascade, and orientation-aware roving focus across its toggles (arrow keys per axis, Home/End, focus looping, RTL-aware horizontal arrows) via Base UI's internal `CompositeRoot`.

`crates/base_gpui` currently has no Toggle Group component. The goal is to port Toggle Group behavior into GPUI-native components using the current `base_gpui` runtime/context/layers architecture, not to copy React, DOM, `CompositeRoot`/`CompositeItem`, or ARIA implementation details.

This issue is the group side of a two-issue pair with `issues/port-baseui-toggle.md`. The Toggle issue owns the standalone pressable contract; this issue owns the group value model, the grouped-Toggle composite-item mode, roving focus, the disabled cascade, and end-to-end verification of the shared-details veto ordering.

**Value type decision (resolves the open question in both issues):** the group value is generic, `ToggleGroup<T>` with `T: Clone + Eq + 'static`, matching `radio_group`, `tabs`, and `docs/base-gpui-component-architecture.md`. Base UI constrains toggle values to strings only because DOM identity needs strings; GPUI has no such constraint, and a `SharedString` special case would make Toggle Group the one value-carrying component with a non-generic value. The group state is `Vec<T>` (Base UI `Value[]`), extending the radio group's single `Option<T>` model to Vec semantics. **Required reconciliation of `issues/port-baseui-toggle.md`:** that issue currently specifies `SharedString` as the Toggle membership value and flags the generic alternative as uncertain. Toggle must adopt the same type as the group: `Toggle<T>` with `.value(T)`, `T: Clone + Eq + 'static`. A consequence: Base UI's auto-generated string identity (`useBaseUiId`) for a `value`-less grouped Toggle cannot exist for generic `T`, so a grouped Toggle without an explicit `value` is inert for group membership and produces a debug-time warning (this also settles the Toggle issue's third Uncertain item in favor of "require `value` in grouped mode, keep only the warning").

## Scope

Port the Toggle Group component from Base UI into a GPUI-native component:

- `ToggleGroup<T>` (single part; Base UI renders one `role="group"` container via `CompositeRoot`)

The grouped item is `Toggle<T>` from `issues/port-baseui-toggle.md`; this issue wires it as a composite child but does not re-specify its standalone contract.

Base UI reference files:

- `/home/luke/Projects/base-ui/packages/react/src/toggle-group/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toggle-group/ToggleGroup.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toggle-group/ToggleGroupContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toggle-group/ToggleGroupDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/toggle-group/ToggleGroup.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toggle-group/ToggleGroup.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/toggle/Toggle.tsx` (grouped branch: membership-derived pressed, `setGroupValue` routing, `CompositeItem` mode)
- `/home/luke/Projects/base-ui/packages/react/src/internals/composite/root/CompositeRoot.tsx` (behavioral reference only: roving focus, `loopFocus`, `enableHomeAndEndKeys`, orientation, direction)
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/toggle-group/page.mdx`

Current GPUI implementation:

- No `crates/base_gpui/src/toggle_group/` implementation exists yet.

Expected GPUI implementation files:

- `crates/base_gpui/src/toggle_group/mod.rs`
- `crates/base_gpui/src/toggle_group/actions.rs`
- `crates/base_gpui/src/toggle_group/child.rs`
- `crates/base_gpui/src/toggle_group/child_wiring.rs`
- `crates/base_gpui/src/toggle_group/context.rs`
- `crates/base_gpui/src/toggle_group/props.rs`
- `crates/base_gpui/src/toggle_group/style_state.rs`
- `crates/base_gpui/src/toggle_group/runtime.rs`
- `crates/base_gpui/src/toggle_group/layers/mod.rs`
- `crates/base_gpui/src/toggle_group/layers/toggle_group.rs`
- `crates/base_gpui/src/toggle_group/tests/`

Implementation precedents (no new shared primitive is needed):

- `crates/base_gpui/src/radio_group/runtime.rs`, `context.rs`, `child_wiring.rs` — the single-select group precedent: runtime-owned item metadata registration in source order (value, disabled, index, focus handle), roving-focus tab-stop bookkeeping, controlled/uncontrolled resolution in the context's value-changing method, and the cancelable Rust-native change-details shape. Toggle Group extends the single `Option<T>` group value to `Vec<T>` with press/unpress semantics.
- `crates/base_gpui/src/tabs/runtime.rs` (`move_highlight`) and `crates/base_gpui/src/tabs/child_wiring.rs` — roving-focus highlight movement with looping and child wiring/index assignment.
- `crates/base_gpui/src/utils/direction.rs` (`current_direction`, `HorizontalArrowKey`, `HorizontalDirection`) — ambient RTL-aware horizontal arrow resolution, already consumed by `crates/base_gpui/src/radio_group/layers/radio_group_radio.rs`.
- `/home/luke/Projects/gpui-component/crates/ui/src/button/toggle.rs` — gpui-component analog for visual styling only (reference; do not copy its API).

## Out of scope / drop from Base UI

- Do not port React context/hooks directly; use GPUI keyed state/entities plus a thin `ToggleGroupContext<T>` wrapper reached through child wiring (the Radio Group precedent), not an ambient provider.
- Do not port Base UI's internal `CompositeRoot`/`CompositeItem`/`CompositeList` implementation literally; translate roving focus into GPUI runtime registration, focus handles, key contexts, and dispatch actions. Do not introduce a new shared composite primitive.
- Do not port `render` props.
- Do not port `className`.
- Do not port web `style` props; expose GPUI builder styling and `style_with_state(...)`.
- Do not port `nativeButton` or DOM tag switching; the grouped item is the GPUI `Toggle` from `issues/port-baseui-toggle.md`.
- Do not port arbitrary HTML attributes or DOM event props.
- Do not port SSR/hydration/prehydration APIs.
- Do not port CSS variable APIs.
- Do not port DOM data attributes (`data-disabled`, `data-orientation`, `data-multiple`) as attributes; map them into typed style-state fields.
- Do not port arbitrary DOM event objects; use Rust-native change details for cancellation, reason, and source information.
- Do not write DOM ARIA attributes (`role="group"`, `aria-pressed`); map accessibility through GPUI-native AccessKit APIs once available. Note Base UI deliberately renders no `aria-orientation` on the group.
- Do not port `useBaseUiId` auto-generated string identities for `value`-less grouped Toggles; with generic `T` this is impossible. Require an explicit `value` for group membership and emit a debug-time warning otherwise (mirrors Base UI's dev error).
- Toolbar integration is out of scope: in Base UI, a ToggleGroup nested inside a Toolbar renders a plain `role="group"` div and defers all roving focus to the Toolbar's composite root, and toggles publish `{ disabled, focusableWhenDisabled: false }` item metadata to the Toolbar. Do not build this now, but keep the runtime's per-item disabled facts and the group's "owns roving focus" assumption localized so a future Toolbar port can take over focus management without reworking this component. Track the actual wiring in the future `issues/port-baseui-toolbar.md` (not yet written).

## Acceptance Criteria

### Module/API surface

- [x] Add a `toggle_group` module and export it from `crates/base_gpui/src/lib.rs`.
- [x] Register Toggle Group key bindings from `base_gpui::init(cx)`.
- [x] Add a public `ToggleGroup<T>` layer type with `T: Clone + Eq + 'static`; the same `T` is `Toggle<T>`'s membership value type.
- [x] Add a typed `ToggleGroupChild<T>` enum in `child.rs` that routes `Toggle<T>` children before `AnyElement` erasure; do not add an `AnyElement` escape hatch unless Base UI examples require arbitrary group children.
- [x] Support uncontrolled construction with `.default_value(Vec<T>)`, defaulting to an empty `Vec` when not called.
- [x] Support controlled construction with `.value(Vec<T>)`; calling the builder marks the group controlled even when the supplied `Vec` is empty.
- [x] Support `.on_value_change(...)` with a Rust-native cancelable change-details API, e.g. `Fn(&[T], &mut ToggleGroupValueChangeDetails, &mut Window, &mut App)`, called with the proposed next group value.
- [x] The group change details are the same details object the activating Toggle's `on_pressed_change` receives (shared cancellation state), with `reason()`, `source()`, `cancelable()`, `cancel()`, and `is_canceled()` APIs matching the Switch/Radio Group details shape; reason is the single `None` reason and source is `{Pointer, Keyboard}`.
- [x] Support `.disabled(bool)`, defaulting to `false`.
- [x] Support `.orientation(...)` with a horizontal/vertical orientation type, defaulting to horizontal.
- [x] Support `.loop_focus(bool)`, defaulting to `true`.
- [x] Support `.multiple(bool)`, defaulting to `false`.
- [x] Support `.style_with_state(...)` taking `ToggleGroupStyleState`.
- [x] Consume the shared ambient direction primitive (`crates/base_gpui/src/utils/direction.rs`) instead of adding a one-off `.direction(...)` builder.
- [x] `toggle_group/mod.rs` exposes ergonomic barrel exports for the component name, style state, context, props, runtime, actions, child, and change-details types; `mod.rs` is barrel-only.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui toggle_group` passes.
- [x] The component compiles without adding web/React-specific concepts to public APIs.
- [x] The implementation follows `docs/base-gpui-component-architecture.md`: flat module layout (no `child/context/{props,runtime,state}` taxonomy), no `pub(...)` visibility qualifiers, and clean under the repo's ast-grep rules.
- [x] `issues/port-baseui-toggle.md` is reconciled to the generic value type: `Toggle<T>` with `.value(T)` where `T: Clone + Eq + 'static`, replacing that issue's `SharedString` membership value, before or together with implementation.
- [x] Add a small example/demo in `crates/base_gpui/src/main.rs` or a dedicated example that renders a Toggle Group of toggles.

### Architecture / internal primitives

- [x] Add `ToggleGroupRuntime<T>` as the single owner of Toggle Group business state: uncontrolled group value `Vec<T>`, registered toggle metadata in source order (value `Option<T>`, disabled, index, focus handle), the highlighted/roving tab-stop index, and group-level facts (disabled, orientation, multiple, loop_focus as needed by commands); commands return outcomes and the runtime never calls user callbacks.
- [x] The group-commit command computes the next `Vec<T>` inside the runtime: `multiple=false` replaces the whole value with `[value]` on press and `[]` on unpress; `multiple=true` appends the value on press and removes its first occurrence on unpress.
- [x] Add `ToggleGroupProps<T>` for stable props and callbacks.
- [x] Add `ToggleGroupContext<T>` as a thin injection/plumbing type with only `read(...)`, `update(...)`, and a single value-changing method (e.g. `commit_toggle(value, next_pressed, details, window, cx)`); controlled/uncontrolled resolution and group-callback firing live there, not in layers.
- [x] Keep Toggle Group behavior on `ToggleGroupRuntime<T>`; do not grow component vocabulary on `ToggleGroupContext<T>` beyond the value-changing method.
- [x] Grouped toggles receive the group context through child wiring owned by the `ToggleGroup` root (`child_wiring.rs`, the Radio Group precedent), which assigns indices and collects metadata/focus handles for `sync_children`; no index bookkeeping in layers or public helpers on `Toggle`.
- [x] Add `ToggleGroupStyleState` in `style_state.rs` as the query result the group layer feeds to `style_with_state`; grouped-toggle style state stays `ToggleStyleState` from the Toggle issue.
- [x] Reuse `crates/base_gpui/src/utils/direction.rs` for RTL-aware horizontal arrows; do not add a `utils/` folder or new shared primitives for Toggle Group.

### Controlled/uncontrolled group value

- [x] Uncontrolled group initializes the group value from `default_value`, defaulting to empty (no toggle pressed).
- [x] Uncontrolled group mutates the internal group value on accepted user activation.
- [x] Controlled group reflects the external `value`; external changes update every grouped toggle's pressed style state.
- [x] Controlled group calls `on_value_change` on valid user activation without mutating internal group value as the source of truth.
- [x] A grouped `Toggle<T>` derives pressed purely from membership: pressed iff the group value contains its `value`. Its `pressed`/`default_pressed` props do not drive state inside a group.
- [x] A controlled or default group value containing an entry that matches no mounted toggle leaves all toggles consistent (unmatched entries are preserved in the value, no toggle presses).
- [x] A grouped toggle without an explicit `value` never joins the group value; when the group value was initialized (via `value` or `default_value`), a debug-time warning mirrors Base UI's dev error recommending an explicit `value`.
- [x] Re-rendering with changed props does not reset uncontrolled group value except when the keyed instance id changes.

### Selection semantics (single vs multiple)

- [x] `multiple=false` (default): pressing an unpressed toggle replaces the group value with exactly that toggle's value; all other toggles unpress.
- [x] `multiple=false`: pressing the already-pressed toggle empties the group value (deselect-to-empty is allowed; the group may have no pressed toggle).
- [x] `multiple=true`: pressing an unpressed toggle appends its value; other pressed toggles stay pressed.
- [x] `multiple=true`: pressing a pressed toggle removes the first occurrence of its value from the group value.
- [x] `on_value_change` receives the full next group value (e.g. `["one"]`, then `["one", "two"]` in multiple mode), not a delta.

### Cancelable shared change details + veto ordering

- [x] On grouped activation, the Toggle's `on_pressed_change(next_pressed, details, ...)` fires first; if it cancels, neither the group value nor the toggle changes and `on_value_change` is not called.
- [x] If the toggle callback does not cancel, the commit routes to the group: the runtime computes the next group value and the group's `on_value_change(next_value, details, ...)` fires with the same shared details object; if the group cancels, the group value does not change and the local pressed commit is also skipped.
- [x] Each callback fires at most once per activation; the group commit does not re-enter the toggle callback.
- [x] Uncontrolled mode mutates the group value only after both callbacks have run uncanceled; controlled mode never mutates internal group value.
- [x] Disabled activation attempts (group-disabled or toggle-disabled) call neither callback.
- [x] Details expose the `None` reason and pointer-vs-keyboard source; no DOM `event`/`trigger`/propagation APIs.

### Keyboard/focus behavior (roving focus)

- [x] The group owns roving focus over its registered toggles; grouped toggles are composite items, not independent tab stops. Exactly one enabled toggle holds the tab stop at a time.
- [x] The initial tab stop is the first enabled toggle (Base UI composite starts at index 0); after tabbing away and back, focus returns to the current roving tab stop rather than resetting.
- [x] Toggle Group uses GPUI actions/key dispatch and a Toggle Group key context instead of raw key handlers; bindings register from `base_gpui::init(cx)`.
- [x] Horizontal orientation (default): ArrowRight/ArrowLeft move focus to the next/previous enabled toggle in ambient LTR; ambient RTL flips them (via `current_direction()` + `HorizontalArrowKey`). ArrowUp/ArrowDown are ignored.
- [x] Vertical orientation: ArrowDown/ArrowUp move focus to the next/previous enabled toggle; ArrowLeft/ArrowRight are ignored. RTL does not affect the vertical axis.
- [x] Arrow navigation moves focus/tab stop only; it never changes pressed state (unlike Radio Group's select-on-navigate).
- [x] With `loop_focus=true` (default), arrow navigation wraps past either end; with `loop_focus=false`, it clamps at the ends.
- [x] Home moves focus to the first enabled toggle; End moves focus to the last enabled toggle (Base UI passes `enableHomeAndEndKeys` for Toggle Group — the opposite of Radio Group).
- [x] Disabled toggles are skipped by arrow navigation, Home, and End, and never hold the tab stop.
- [x] Space and Enter activate the focused enabled toggle through the Toggle's own activation path (which routes the commit to the group); activation does not double-fire if GPUI exposes both key-down and key-up dispatch.
- [x] Pointer activation and keyboard activation share the same runtime commit command; source is reported in change details.

### Item registration + disabled cascade

- [x] The group root's child wiring registers every `Toggle<T>` descendant in source order with its value, resolved disabled fact, index, and focus handle via a runtime `sync_children`-style command before reconciliation and style queries, so initial pressed/tab-stop states are correct on first render.
- [x] Group `disabled=true` cascades: every grouped toggle is effectively disabled (a toggle's effective disabled is its own prop OR the group's), does not activate from pointer or keyboard, fires no callbacks, and reports disabled in its style state.
- [x] Per-toggle `disabled=true` disables only that toggle; the rest of the group interacts normally.
- [x] Per-item disabled facts remain queryable from the runtime so a future Toolbar port can consume them as item metadata without new API.
- [x] Toggles keep working standalone with no group present; grouping adds no cost or API burden to standalone `Toggle` use.

### Styling/state exposure

- [x] `ToggleGroupStyleState` includes at least `disabled`, `orientation`, and `multiple` (Base UI's `data-disabled`/`data-orientation`/`data-multiple`).
- [x] Expose state-aware styling through `style_with_state(...)` on the group; grouped toggles style through the Toggle issue's `ToggleStyleState` (`pressed`/`disabled`), which must reflect membership-derived pressed state.
- [x] Map Base UI state/data attributes into typed style-state fields, not DOM attributes; do not expose CSS variable names.
- [x] The docs styling pattern is recreatable with GPUI builder methods: a bordered group container laying out toggles along its orientation, with pressed/disabled toggle styling from the Toggle side.

### Tests / verification

Add one behavior per file under `crates/base_gpui/src/toggle_group/tests/`.

- [x] Uncontrolled initial state: empty default value, no toggle pressed.
- [x] Uncontrolled `default_value` presses the matching toggle initially.
- [x] Controlled group reflects the external value; external value changes flip toggles' pressed style state.
- [x] Controlled activation calls `on_value_change` without mutating internal group value.
- [x] Single-select: clicking an unpressed toggle presses it and unpresses the previously pressed toggle.
- [x] Single-select deselect: clicking the pressed toggle empties the group value.
- [x] Multiple: clicking a second toggle presses it while the first stays pressed; group value contains both.
- [x] Multiple deselect: clicking a pressed toggle removes only its value.
- [x] `on_value_change` receives the full next group value exactly once per accepted activation.
- [x] Toggle-side cancellation: canceling `on_pressed_change` prevents the group commit and `on_value_change` is never called.
- [x] Group-side cancellation: canceling `on_value_change` leaves the group value and the toggle's pressed state unchanged (uncontrolled), and both callbacks saw the same shared details object.
- [x] Canceled controlled activation still calls both callbacks but mutates no internal state.
- [x] Keyboard activation: Space and Enter toggle the focused toggle and fire `on_value_change` with the keyboard source.
- [x] Horizontal LTR roving focus: ArrowRight/ArrowLeft move focus, wrap with `loop_focus=true`, and ArrowUp/ArrowDown are ignored.
- [x] Horizontal RTL roving focus (DirectionProvider-wrapped): ArrowLeft/ArrowRight are flipped.
- [x] Vertical roving focus: ArrowDown/ArrowUp move focus and horizontal arrows are ignored.
- [x] `loop_focus=false` clamps at both ends.
- [x] Home focuses the first enabled toggle; End focuses the last enabled toggle.
- [x] Arrow navigation alone never changes pressed state or fires callbacks.
- [x] Group `disabled=true` cascade: no toggle activates from pointer or keyboard, no callbacks fire, all toggles report disabled style state.
- [x] Per-toggle disabled: the disabled toggle is skipped by roving focus and cannot activate; siblings work.
- [x] Grouped toggle without `value` never affects the group value; debug warning fires when the group value is initialized.
- [x] Group `style_with_state(...)` receives correct `disabled`/`orientation`/`multiple` state.
- [x] Membership-derived pressed: grouped toggle `style_with_state(...)` reports pressed iff the group value contains its value (closes the seam test deferred from `issues/port-baseui-toggle.md`).

### Uncertain / needs confirmation

- [x] Value type is decided here as `T: Clone + Eq + 'static` (shared by `Toggle<T>` and `ToggleGroup<T>`); confirm and update `issues/port-baseui-toggle.md`'s Module/API surface (`.value(T)` instead of `SharedString`), its grouped-mode auto-identity item (dropped: explicit `value` required, warning only), and its Uncertain section before implementation.
- [x] Shared details naming: whether `ToggleGroupValueChangeDetails` is a distinct type or an alias of the Toggle issue's `TogglePressedChangeDetails`. Behaviorally they must be one object per activation with shared cancellation state; pick the simplest representation during implementation.
- [x] Whether standalone `Toggle` becomes generic (`Toggle<T>`) with a defaulted parameter or the group wraps toggles in a way that keeps standalone use monomorphic; decide during implementation so standalone Toggle stays ergonomic without turbofish noise.
- [x] Duplicate values in `multiple` mode: Base UI removes only the first occurrence on unpress and never deduplicates on press. Mirror that literally or debug-warn on duplicate registration; confirm during implementation.
- [ ] Toolbar seam: when a Toolbar port exists, a nested Toggle Group must hand roving focus to the Toolbar (Base UI renders a plain group container inside a Toolbar). Out of scope here; revisit in the future `issues/port-baseui-toolbar.md`. (Update: the Toolbar port now exists with the shared `ToolbarItemMetadata` registration channel and documented flattening contract; the actual `ToolbarChild::ToggleGroup(...)` wiring is still pending.)

## AccessKit accessibility follow-up

Base UI's accessible surface for this component is small and fully documented in the source: `ToggleGroup.tsx` renders one container with `role: 'group'` (deliberately no `aria-orientation`), and the grouped `Toggle.tsx` renders a button with `'aria-pressed': pressed` plus native `disabled`. Map that onto the AccessKit builders available in the pinned gpui revision (`docs/accesskit-gpui-reference.md`) as follows. All work happens in `crates/base_gpui/src/toggle_group/layers/toggle_group.rs` (`ToggleGroup<T>`) and `crates/base_gpui/src/toggle/layers/toggle.rs` (`Toggle<T>`, both the standalone `render` path and `render_grouped`).

### Per accessible part

- **`ToggleGroup<T>` root (`layers/toggle_group.rs`)** — the root `Div` currently renders as `base.children(children)` with **no `.id(...)`**, so it cannot enter the a11y tree. Give it `.id(self.id.clone())` (the keyed id already exists on the struct and is stable across frames) plus:
  - `.role(Role::Group)` — Base UI's `role="group"`.
  - `.aria_label(...)` from a new `.aria_label(impl Into<SharedString>)` builder prop on `ToggleGroup<T>` (Base UI users pass `aria-label` through to the group div; we need an explicit prop since arbitrary attributes are out of scope).
  - Do **not** set `.aria_orientation(...)`: Base UI deliberately renders no `aria-orientation` on the group (already noted in "Out of scope"); mirror that.
- **Grouped `Toggle<T>` (`render_grouped` in `toggle/layers/toggle.rs`)** — already has `.id(toggle.id)`; add:
  - `.role(Role::ToggleButton)` (verify the exact accesskit 0.24 variant name at implementation time; fall back to `Role::Button` + toggled state if absent).
  - `.aria_toggled(if pressed { Toggled::True } else { Toggled::False })` — `pressed` is the membership-derived local already computed via `runtime.toggle_pressed(value.as_ref())`; this is the AccessKit expression of Base UI's `aria-pressed`.
- **Standalone `Toggle<T>` (`render` path)** — same `Role::ToggleButton` + `.aria_toggled(...)` from `style_state.pressed` (the runtime-owned pressed state from `runtime.state()`). Track it in the Toggle issue too, but wire both paths in one pass since they share the file.

### Actions

- No new `.on_a11y_action(...)` handlers are needed. `Action::Click` is auto-registered by the existing `.on_click(...)` in both `render` and `render_grouped`, and `Action::Focus` is auto-registered by the existing `.track_focus(&focus_handle...)`. AT-dispatched Click therefore already routes through `grouped_activate` → `ToggleGroupContext::commit_toggle` (the same shared-details veto path pointer and keyboard use).
- Caveat: the `.on_click` closures early-return unless `matches!(event, ClickEvent::Mouse(_))`. Verify what `ClickEvent` an AT-dispatched `Action::Click` synthesizes in this gpui revision; if it is not a mouse click, the guard silently swallows AT activation and must be adjusted (e.g. also accept the a11y-synthesized variant) so the same `grouped_activate` transition runs.
- Roving focus needs no extra a11y actions: AT focus lands on the current tab stop via the auto-registered `Focus` action, and arrow/Home/End movement stays keyboard-driven through the existing `ToggleGroupFocus*` actions.

### Labels

- Group label: from the new `ToggleGroup::aria_label(...)` prop (literal string; there is no `aria-labelledby` wiring in this revision — see Gaps).
- Toggle label: add `.aria_label(impl Into<SharedString>)` to `Toggle<T>` and apply it in both render paths. Toggles are typically icon-only (the Base UI docs example uses SVG icons), so an explicit label is the primary path.
- Where a toggle's child is visible text created with `text!(...)`, either rely on that text node and skip `.aria_label`, or set `.aria_label` and switch the child to `Text::new_inaccessible(...)` so the name is not announced twice. Pick one convention and apply it in the demo in `crates/base_gpui/src/main.rs`.

### Gaps (no gpui builder in this revision)

- **`disabled` / `aria-disabled`**: Base UI renders the toggle natively `disabled`; gpui has no `.aria_disabled(...)` and `write_a11y_info` sets no disabled flag. Fallback: omit and document. The interaction guards already exist (`grouped_activate` returns early on `own_disabled || group_disabled`, and disabled toggles get `tab_stop(false)` / `tab_index(-1)`), so a disabled toggle is inert to AT actions even though it is not *announced* as disabled. Revisit when gpui grows a `set_disabled` mapping (candidate upstream contribution).
- **`aria-pressed` as a distinct attribute**: no dedicated builder; `aria_toggled(Toggled::…)` is the documented closest equivalent and is what we use. No further fallback needed.
- **Relationship props** (`aria-labelledby`, `aria-describedby`, `aria-controls`, `aria-activedescendant`, `aria-haspopup`): Base UI Toggle Group emits none of these itself, but group labelling via `aria-labelledby` is a common consumer pattern; it has no builder, so the literal `.aria_label(...)` string is the only labelling channel. Omit and document.
- **Live regions / announcements**: not needed by Toggle Group; no gap to track here.

### Checklist

- [ ] Give the `ToggleGroup<T>` root `Div` a stable `.id(self.id.clone())` and `.role(Role::Group)` in `layers/toggle_group.rs`.
- [ ] Add `.aria_label(...)` builder prop to `ToggleGroup<T>` and forward it to the root element; do not set `aria_orientation`.
- [ ] Set `.role(Role::ToggleButton)` (or `Role::Button` if the variant is absent in accesskit 0.24) plus `.aria_toggled(...)` from membership-derived `pressed` in `render_grouped`.
- [ ] Set the same role + `.aria_toggled(...)` from `runtime.state().pressed` in the standalone `Toggle` render path.
- [ ] Add `.aria_label(...)` builder prop to `Toggle<T>`, applied in both render paths; use `Text::new_inaccessible(...)` for visible toggle text when `.aria_label` is set.
- [ ] Verify AT-dispatched `Action::Click` is not swallowed by the `ClickEvent::Mouse(_)` guard in the `on_click` handlers; adjust the guard if it is.
- [ ] Do not add manual `on_a11y_action(Action::Click | Action::Focus, ...)` handlers — they are auto-registered by `.on_click`/`.track_focus`.
- [ ] Document the missing disabled announcement (no `.aria_disabled` in this gpui revision) in the module docs, noting that disabled toggles are still action-inert.
- [ ] Windowed test: pressed/unpressed grouped toggles report the correct `Toggled` state, and the group node carries `Role::Group` with its label.
