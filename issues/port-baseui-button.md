# Port Base UI Button to GPUI

## Problem

Base UI Button is a single-part pressable control. It renders a `<button>` (or a
non-native element via `nativeButton={false}`), delegates all pressable behavior to
the shared `useButton` hook, and exposes exactly one piece of state:
`{ disabled }` (surfaced as `data-disabled`). Its behavioral contract is:

- pointer click activates the button when enabled,
- Space and Enter activate the button from the keyboard when enabled and focused,
- `disabled` makes every interaction inert (click, mousedown, pointerdown, keydown
  handlers all suppressed) and removes the button from the tab order,
- `focusableWhenDisabled` keeps a disabled button **in** the tab order and focusable
  while all activation stays suppressed (Base UI: `tabindex="0"` +
  `aria-disabled="true"` instead of the `disabled` attribute),
- `tabIndex` defaults to `0`.

`crates/base_gpui` currently has no Button component. GPUI also has no built-in
Button element under `crates/gpui/src/elements` — interactive controls in this repo
are built from `div()` plus a keyed `FocusHandle`, `track_focus` with
`tab_stop`/`tab_index`, a `key_context`, GPUI actions for Space/Enter, and
`on_click`. That pressable recipe is already established by
`crates/base_gpui/src/switch/layers/switch_root.rs` (lines 96-146) and
`switch/actions.rs` / `checkbox/actions.rs`; `ButtonRoot` is that recipe with no
value state.

The goal is behavioral parity with Base UI Button using GPUI-native architecture.
There is no controlled/uncontrolled value, no compound subparts, and no state
beyond `disabled` (plus GPUI-native focus for styling). Complexity: trivial.

Sibling note: Base UI's Toggle reuses only the `useButton` **hook**, not the Button
component (`toggle/Toggle.tsx` line 11/73). The future Toggle port
(`issues/port-baseui-toggle.md`) should reuse this same GPUI pressable recipe
(keyed focus handle + tab stop + key context + actions + `on_click`) inline, not a
shared generic primitive — per `docs/base-gpui-component-architecture.md`, do not
introduce shared primitives for tiny plumbing.

## Scope

Port the Button component from Base UI into a GPUI-native component:

- `ButtonRoot`

Base UI reference files:

- `/home/luke/Projects/base-ui/packages/react/src/button/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/button/Button.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/button/ButtonDataAttributes.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/button/Button.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/button/Button.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/internals/use-button/useButton.ts`
- `/home/luke/Projects/base-ui/packages/react/src/internals/use-button/useButton.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/utils/useFocusableWhenDisabled.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/button/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/button/types.md`

GPUI pressable-recipe precedent (mirror, do not abstract):

- `crates/base_gpui/src/switch/layers/switch_root.rs` (keyed `FocusHandle` via
  `window.use_keyed_state`, `.track_focus(&focus_handle.tab_stop(!disabled).tab_index(...))`,
  `.key_context(...)`, `.on_action(...)`, `.on_click(...)` filtered to
  `ClickEvent::Mouse`)
- `crates/base_gpui/src/switch/actions.rs` and `crates/base_gpui/src/checkbox/actions.rs`
  (Space/Enter bound to one action under a root key context)

GPUI focus/tab APIs (available out of the box in the pinned gpui revision):

- `FocusHandle::tab_stop(bool)` / `FocusHandle::tab_index(isize)` —
  `~/.cargo/git/checkouts/zed-a70e2ad075855582/f7ca86e/crates/gpui/src/window.rs`
  (lines ~397-412)
- Examples: `.../f7ca86e/crates/gpui/examples/tab_stop.rs`,
  `.../f7ca86e/crates/gpui/examples/focus_visible.rs` (keyboard-only focus styling
  via the `.focus_visible(|style| ...)` builder)

Styling analog for reference only (do not copy its API):

- `/home/luke/Projects/gpui-component/crates/ui/src/button/button.rs`

No current GPUI implementation exists. Expected new files (flat layout per
`docs/base-gpui-component-architecture.md`):

- `crates/base_gpui/src/button/mod.rs` — barrel exports + module declarations only.
- `crates/base_gpui/src/button/actions.rs` — `BUTTON_ROOT_KEY_CONTEXT`,
  `ButtonActivate` action, `init(cx)` binding Space and Enter.
- `crates/base_gpui/src/button/style_state.rs` — `ButtonRootStyleState`.
- `crates/base_gpui/src/button/layers/mod.rs`
- `crates/base_gpui/src/button/layers/button_root.rs` — `ButtonRoot`.
- `crates/base_gpui/src/button/tests/` — one behavior per file.
- Register `pub mod button;` and `button::init(cx);` in `crates/base_gpui/src/lib.rs`.

Not expected (single stateless part — keep it minimal): no `runtime.rs`, no
`context.rs`, no `props.rs`, no `child.rs`/`child_wiring.rs`. Button has no value
state to reconcile and no subparts to route; children are arbitrary content
(`AnyElement` via `ParentElement`-style `child`/`children`), matching Base UI where
Button children are plain React children with no compound parts.

## Out of scope / drop from Base UI

- `nativeButton` and the native/non-native DOM element switch (including the
  dev-mode tag-mismatch warnings and `type="button"` vs `role="button"` selection).
  GPUI has no built-in Button element; `ButtonRoot` is always the `div()`-based
  pressable recipe.
- React `render` prop.
- `className` and web `style` props; expose GPUI builder styling (`Styled`) plus
  `style_with_state(...)`.
- ARIA attributes (`role="button"`, `aria-disabled`) and the DOM `disabled`
  attribute — track under the AccessKit follow-up; do not write DOM ARIA.
- `data-disabled` as a DOM attribute — map into `ButtonRootStyleState.disabled`.
- Native form participation (`type="submit"`, form association). Only revisit if a
  GPUI `Form` integration ever needs a submit control.
- `useButton`'s `composite` parameter and composite-item Space handling
  (Space-on-keydown vs keyup, `CompositeRootContext` inference, the
  `updateDisabled` composite-button DOM patching). That belongs to future composite
  widgets (Toolbar/Menu), not Button itself.
- `useButton`'s link detection (`isValidLinkElement`) and keyup-Space synthetic
  click ordering — DOM-only mechanics; GPUI keyboard activation goes through one
  `ButtonActivate` action.
- `mousedown`/`pointerdown` `preventDefault` suppression details — GPUI event flow
  differs; the contract to keep is "disabled means no activation and no user
  callbacks", not the per-DOM-event suppression.
- CSS `:focus-visible` — GPUI already provides keyboard-only focus styling via the
  native `.focus_visible(|style| ...)` builder; do not invent a parallel API.
- SSR/hydration, CSS variables, arbitrary DOM event objects.

## Acceptance Criteria

New issue — all items unchecked.

### Module/API surface

- [x] Add a `button` module and export it from `crates/base_gpui/src/lib.rs`
      (`pub mod button;` + `button::init(cx);`).
- [x] Add a public `ButtonRoot` layer type constructed with `ButtonRoot::new()`.
- [x] `ButtonRoot` supports `.id(impl Into<ElementId>)` with a stable default id,
      following `SwitchRoot`.
- [x] `ButtonRoot` supports `.disabled(bool)`, defaulting to `false`.
- [x] `ButtonRoot` supports `.focusable_when_disabled(bool)`, defaulting to `false`.
- [x] `ButtonRoot` supports `.on_click(...)` taking a Rust-native activation
      handler, e.g. `impl Fn(&ClickEvent, &mut Window, &mut App) + 'static`; both
      pointer and keyboard activation route through this one handler.
- [x] `ButtonRoot` supports `.style_with_state(impl Fn(ButtonRootStyleState, Div) -> Div + 'static)`.
- [x] `ButtonRoot` implements `Styled` for plain builder styling and accepts
      arbitrary element children via `child(...)` / `children(...)` (no typed child
      enum — Button has no compound subparts in Base UI).
- [x] `button/mod.rs` is barrel exports only: `ButtonRoot`, `ButtonRootStyleState`,
      `ButtonActivate`, `BUTTON_ROOT_KEY_CONTEXT`, `init`.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui button` passes.
- [x] No web/React concepts (`nativeButton`, ARIA, className, render props) appear
      in the public API.
- [x] Flat module layout per `docs/base-gpui-component-architecture.md`; no
      `child/context/{props,runtime,state}` taxonomy, no `utils/` folder, no new
      shared generic pressable primitive.
- [x] No Rust scoped visibility syntax (`pub(...)`);
      `ast-grep scan crates/base_gpui/src/button` produces no violations.
- [x] Add a small example/demo in `crates/base_gpui/src/main.rs` (or the existing
      demo surface) rendering an enabled button, a disabled button, and a
      focusable-when-disabled button.

### Pressable behavior

- [x] Clicking an enabled `ButtonRoot` invokes `on_click` exactly once per click.
- [x] Keyboard activation dispatches a single `ButtonActivate` action (bound to
      Space and Enter under `BUTTON_ROOT_KEY_CONTEXT` in `actions.rs`, registered
      from `base_gpui::init(cx)`), which invokes the same `on_click` handler.
- [x] Pointer and keyboard activation do not double-fire: mirror
      `switch_root.rs` by filtering the `.on_click` handler to
      `ClickEvent::Mouse(_)` and letting keyboard go through the action.
- [x] Disabled `ButtonRoot` ignores pointer activation and never calls `on_click`.
- [x] Disabled `ButtonRoot` ignores keyboard activation (Space and Enter) and never
      calls `on_click`.
- [x] `focusable_when_disabled(true)` does not re-enable activation: pointer and
      keyboard activation remain no-ops while disabled, matching Base UI's
      `focusableWhenDisabled` tests (focus is received, handlers never fire).
- [x] The disabled guard lives in one place (the activation path), not duplicated
      per event source.

### Keyboard/focus behavior

- [x] `ButtonRoot` owns a stable keyed `FocusHandle`
      (`window.use_keyed_state` keyed off the root `ElementId`, per
      `switch_root.rs` lines 96-101).
- [x] The root renders with
      `.track_focus(&focus_handle.tab_stop(...).tab_index(...))`,
      `.key_context(BUTTON_ROOT_KEY_CONTEXT)`, and `.focusable()`.
- [x] Enabled: `tab_stop(true)` and `tab_index(0)` — the button participates in the
      window tab order (Base UI default `tabIndex = 0`).
- [x] Disabled with `focusable_when_disabled == false`: `tab_stop(false)` and
      `tab_index(-1)` — removed from the tab order and not click-focusable into an
      interactive state.
- [x] Disabled with `focusable_when_disabled == true`: `tab_stop(true)` and
      `tab_index(0)` — the button stays in the tab order and can receive focus
      while all activation stays inert (Base UI: `tabindex="0"` retained).
- [x] Keyboard behavior uses GPUI actions/key dispatch only; no raw
      `on_key_down` handlers.

### Styling/state exposure

- [x] Add `ButtonRootStyleState` in `style_state.rs` with at least `disabled` and
      `focused` fields. `disabled` maps Base UI's single `ButtonState`
      field / `data-disabled` attribute; `focused` follows the established
      `SwitchRootStyleState` precedent (`focus_handle.is_focused(window)` computed
      at render — no runtime needed for a stateless part).
- [x] `style_with_state(...)` receives the correct `ButtonRootStyleState` and its
      result styles the root `Div`.
- [x] Keyboard-only focus-visible styling is achievable through gpui's native
      `.focus_visible(|style| ...)` builder on the root (see
      `gpui/examples/focus_visible.rs`); document this in the demo rather than
      adding a `focus_visible` field unless gpui exposes a queryable
      focus-visible fact at render time.
- [x] Do not expose DOM data attributes, CSS variables, or class names.

### Tests / verification

Add one behavior per file under `crates/base_gpui/src/button/tests/`, following the
Switch test layout.

- [x] Click invokes `on_click` exactly once.
- [x] Space invokes `on_click` when focused.
- [x] Enter invokes `on_click` when focused.
- [x] Disabled click is a no-op (`on_click` never called).
- [x] Disabled Space/Enter is a no-op (`on_click` never called).
- [x] Disabled button is not a tab stop (tab traversal skips it).
- [x] `focusable_when_disabled` button remains a tab stop and can receive focus.
- [x] `focusable_when_disabled` button still never calls `on_click` for click,
      Space, or Enter.
- [x] `style_with_state(...)` receives `disabled == true` for a disabled button and
      `disabled == false` otherwise.
- [x] `style_with_state(...)` receives `focused == true` when the root focus handle
      is focused and `focused == false` after blur.
- [x] A single pointer click does not double-fire through both the click handler
      and the keyboard action path.

## AccessKit accessibility follow-up

Base UI Button's ARIA surface is small: `role="button"` (implicit on `<button>`,
explicit when `nativeButton={false}`), the `disabled` attribute when disabled, and
`aria-disabled="true"` + `tabindex="0"` in the `focusableWhenDisabled` case
(`Button.tsx` / `useButton.ts` / `useFocusableWhenDisabled.ts`). The pinned gpui
revision exposes AccessKit through `.role(...)` / `.aria_*(...)` builders on
`.id(...)` elements (see `docs/accesskit-gpui-reference.md`); wire `ButtonRoot`
against those.

### Per accessible part

- **`ButtonRoot`** (`crates/base_gpui/src/button/layers/button_root.rs`): the root
  already has a stable `.id(self.id)` in `RenderOnce::render`, so it only needs
  `.role(Role::Button)` on the same `div` chain to appear in the a11y tree. There
  is no value state, so no `.aria_toggled` / `.aria_selected` / `.aria_expanded` —
  the only state fields (`ButtonRoot.disabled`, `ButtonRoot.focusable_when_disabled`,
  and the render-time `ButtonRootStyleState { disabled, focused }`) map to features
  listed under Gaps, not to any available `aria_*` builder.

### Actions

- No new `.on_a11y_action(...)` handlers are needed. `Action::Click` is
  auto-registered by the existing `.on_click(...)` and `Action::Focus` by
  `.track_focus(...)` / `.focusable()` — do **not** re-add them.
- Verify the AT-dispatched Click reaches the shared `activate(...)` path so the
  single disabled guard applies. The current `.on_click` handler early-returns on
  `!matches!(event, ClickEvent::Mouse(_))`; if an AT-synthesized click does not
  arrive as `ClickEvent::Mouse(_)`, it is silently dropped. Confirm what variant an
  `Action::Click` produces and, if needed, widen the match (keyboard already routes
  through `ButtonActivate`, so the double-fire filter must be kept for keyboard
  clicks specifically, not all non-mouse clicks).
- In the non-focusable disabled case the root skips `.track_focus`/`.focusable()`
  entirely (the `when(tab_stop, ...)` branch), so no `Focus` action is registered —
  correct: a disabled, non-focusable button should not be AT-focusable either.

### Labels

- Add a `.aria_label(impl Into<SharedString>)` builder prop on `ButtonRoot`
  (stored alongside `disabled` etc.) and pass it through to the root element when
  set. Button children are arbitrary `AnyElement`s, so there is no automatic
  text-content label; callers with icon-only buttons must supply `aria_label`.
- When a caller sets `aria_label` and also renders a visible text child, the demo
  and docs should show the child as `Text::new_inaccessible(...)` instead of
  `text!(...)` to avoid double-announcing. When no `aria_label` is given, a plain
  `text!(...)` child remains accessible and serves as the computed name.

### Gaps (no gpui builder in this revision)

- **`disabled` / `aria-disabled`**: no `.aria_disabled(...)` builder exists and
  `write_a11y_info` never sets a disabled flag. This covers both Base UI cases
  (the `disabled` attribute, and `aria-disabled="true"` under
  `focusableWhenDisabled`). Fallback: the behavior is already inert (the
  `activate(...)` guard suppresses Click, and the non-focusable case registers no
  Focus action), but AT will not announce "dimmed/disabled". Document the
  limitation in the issue/demo and track as blocked pending a gpui upstream
  `set_disabled` addition. Do not invent `.aria_disabled`.
- **Relationship props** (`aria-labelledby`, `aria-describedby`, `aria-haspopup`,
  etc.): Base UI Button itself emits none, but consumers commonly add them. No
  builders exist; labels must go through the literal-string `.aria_label(...)`.
  Omit + document.
- **`aria-pressed`**: not a Button concern (that is Toggle); note in the future
  Toggle port that `aria_toggled(Toggled)` is the closest available mapping.

### Checklist

- [ ] Set `.role(Role::Button)` on the `ButtonRoot` root element in
      `button_root.rs` (same chain as `.id(self.id)`).
- [ ] Add an `aria_label` builder prop to `ButtonRoot` mapped to
      `.aria_label(...)` on the root.
- [ ] Confirm AT-dispatched `Action::Click` (auto-registered via `.on_click`)
      flows through `activate(...)` and is not dropped by the
      `ClickEvent::Mouse(_)` filter; adjust the filter if necessary without
      re-introducing keyboard double-fire.
- [ ] Do not add explicit `.on_a11y_action(Action::Click | Action::Focus, ...)` —
      both are auto-registered by `.on_click` / `.track_focus`.
- [ ] Demo: show an icon-only button using `aria_label`, and a labeled button
      whose visible text uses `Text::new_inaccessible(...)` when `aria_label` is
      set.
- [ ] Document the disabled-state announcement gap (no `.aria_disabled` in this
      gpui revision) covering both plain `disabled` and
      `focusable_when_disabled`; mark blocked pending gpui upstream.

## Uncertain items needing confirmation

- Whether `ButtonRoot` should inherit disabled state from surrounding
  `field`/`fieldset` contexts the way `SwitchRoot` does
  (`current_field_item_disabled` / `current_fieldset_disabled`). Base UI Button is
  not a Field control, but a GPUI button inside a disabled `Fieldset` arguably
  should be inert. Default: skip Field registration, but do inherit
  `current_fieldset_disabled()` if that matches the Fieldset issue's contract.
- Whether the no-runtime/no-context minimal shape is acceptable, or whether a tiny
  `ButtonRuntime` should exist purely for symmetry. Default: no runtime — the
  architecture doc optimizes for interface-to-knowledge ratio, and Button has no
  transitions to own.
- Whether `on_click` should receive gpui's `&ClickEvent` directly or a
  Rust-native activation-details struct (pointer vs keyboard source) like
  `SwitchCheckedChangeSource`. Default: `&ClickEvent`, since Base UI Button's
  `onClick` carries no cancelable Base-UI-specific details.
