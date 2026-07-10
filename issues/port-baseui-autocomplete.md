# Port Base UI Autocomplete to GPUI

## Problem

Base UI's Autocomplete is **not a new interaction model** — it is the Combobox
(`combobox/root/AriaCombobox.tsx`) with a different configuration of the same core:

- `selectionMode` is always `'none'` — there is no selected value; the component's
  "value" **is the input text**,
- `fillInputOnItemPress` is always `true` — pressing an item writes its label into
  the input and closes the popup,
- `openOnInputClick` defaults to `false` (Combobox defaults to `true`),
- a new `mode` prop (`'list' | 'both' | 'inline' | 'none'`, default `'list'`)
  controls the two remaining behavioral axes: whether the item list **filters**
  against the typed query (`list`/`both`) or stays static (`inline`/`none`), and
  whether keyboard-highlighting an item **inline-autocompletes** the input — i.e.
  temporarily overwrites the visible input text with the highlighted item's label
  and restores the typed text on unhighlight (`both`/`inline`),
- the value axis is renamed: Autocomplete `value`/`defaultValue`/`onValueChange`
  map to Combobox `inputValue`/`defaultInputValue`/`onInputValueChange`, and
  Autocomplete `itemToStringValue` maps to Combobox `itemToStringLabel`.

In Base UI this is expressed by reuse, not reimplementation. Of the ~20 parts in
`autocomplete/index.parts.ts`, **15 are literal re-exports of Combobox parts**
(Input, Icon, Clear, List, Status, Portal, Backdrop, Positioner, Popup, Arrow,
Group, GroupLabel, Row, Collection, Empty), 3 are type-cast aliases of Combobox
parts with autocomplete-flavored prop/state types (Trigger, InputGroup, Item), and
only `Root` and `Value` contain new code. `AutocompleteRoot` is ~60 lines of logic
around the `AriaCombobox` core (mode resolution, inline-overlay state, filter
wrapping so `both` filters on the **typed** query rather than the inline-overwritten
display text, pointer-vs-keyboard highlight handling). `AutocompleteValue` exposes
the current **input** value (not a selected value) with an optional formatter.

The GPUI port must therefore be **thin**, the way `alert_dialog` layers on `dialog`
(`issues/port-baseui-alert-dialog.md`): compose the `ComboboxRuntime<T>` /
`ComboboxContext<T>` from `issues/port-baseui-combobox.md` with
`selection_mode = None` and the Autocomplete defaults, re-export the shared parts
under `Autocomplete*` names, and add only the inline-autocompletion logic and the
`AutocompleteValue` part. The Combobox issue was written to keep the needed runtime
knobs (`selection_mode` incl. `None`, `fill_input_on_item_press`, `auto_highlight`,
`keep_highlight`, `highlight_item_on_hover`, `open_on_input_click`,
`submit_on_item_click`, custom/disabled `.filter(...)`, and an inline-autocomplete
hook point) public and configurable precisely so this port stays small — see its
"Selection modes and the Autocomplete seam" section, and the required additions to
that issue listed at the bottom of this one.

**Hard dependency:** this issue cannot start until the Combobox port
(`issues/port-baseui-combobox.md`) lands. Its "small" complexity is true only
because Combobox is sequenced first.

Item values stay Rust-native generics constrained as `T: Clone + Eq + 'static`.
The Autocomplete value axis (the input text) is `SharedString`. No new primitive is
needed: text editing is the existing `crates/base_gpui/src/primitives/input/`
runtime, and its `sync_props(controlled_value, ...)` path
(`crates/base_gpui/src/primitives/input/runtime.rs`) is the programmatic set-text
mechanism the inline overlay drives — no selection-range API is required (Base UI's
inline autocompletion overwrites the whole visible value; it never selects a
completed suffix).

## Scope

Add a GPUI-native Autocomplete that composes the Combobox implementation.

New autocomplete-specific parts (the only new logic):

- `AutocompleteRoot<T>` — configures the Combobox runtime with
  `selection_mode = None`, `fill_input_on_item_press = true`,
  `open_on_input_click` default `false`; exposes `.mode(...)`; remaps the value
  axis to the input value; owns/configures the inline-autocompletion behavior. No
  selection-value props, no `.multiple`, no fill-on-press knob exposed.
- `AutocompleteValue` — displays the current input value (including a live inline
  overlay), with an optional formatter closure.

Reused Combobox parts, re-exported under Autocomplete names (no reimplementation;
alert-dialog precedent — Base UI re-exports/type-casts these):

- `AutocompleteInput<T>` = `ComboboxInput<T>` (which wraps `primitives/input`)
- `AutocompleteInputGroup<T>` = `ComboboxInputGroup<T>`
- `AutocompleteTrigger<T>` = `ComboboxTrigger<T>`
- `AutocompleteIcon` = `ComboboxIcon`
- `AutocompleteClear` = `ComboboxClear`
- `AutocompleteList<T>` = `ComboboxList<T>`
- `AutocompleteStatus` = `ComboboxStatus`
- `AutocompletePortal<T>` = `ComboboxPortal<T>`
- `AutocompleteBackdrop` = `ComboboxBackdrop`
- `AutocompletePositioner<T>` = `ComboboxPositioner<T>`
- `AutocompletePopup<T>` = `ComboboxPopup<T>`
- `AutocompleteArrow` = `ComboboxArrow`
- `AutocompleteGroup` = `ComboboxGroup`
- `AutocompleteGroupLabel` = `ComboboxGroupLabel`
- `AutocompleteItem<T>` = `ComboboxItem<T>`
- `AutocompleteRow` = `ComboboxRow` — only if the Combobox grid stretch lands;
  otherwise deferred with it
- `AutocompleteCollection<T>` = `ComboboxCollection<T>`
- `AutocompleteEmpty` = `ComboboxEmpty`
- `AutocompleteSeparator` = `ComboboxSeparator` (itself backed by
  `base_gpui::separator::Separator`)

Not part of the Autocomplete surface (Combobox-only parts; Base UI does not export
them from `autocomplete/index.parts.ts`): `ComboboxLabel`, `ComboboxValue<T>`
(selection-shaped — Autocomplete has its own input-shaped Value),
`ComboboxItemIndicator<T>`, `ComboboxChips/Chip/ChipRemove` (no selection, no
chips).

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/autocomplete/index.parts.ts`
  (the re-export map — the authoritative part list)
- `/home/luke/Projects/base-ui/packages/react/src/autocomplete/root/AutocompleteRoot.tsx`
  (all new root behavior: mode resolution, inline overlay, filter wrapping,
  highlight handler)
- `/home/luke/Projects/base-ui/packages/react/src/autocomplete/value/AutocompleteValue.tsx`
  (reads `useComboboxInputValueContext`)
- `/home/luke/Projects/base-ui/packages/react/src/autocomplete/trigger/AutocompleteTrigger.tsx`,
  `.../input-group/AutocompleteInputGroup.tsx`,
  `.../item/AutocompleteItem.tsx` (type-cast aliases of the Combobox parts)
- `/home/luke/Projects/base-ui/packages/react/src/combobox/root/AriaCombobox.tsx`
  (the shared core; `autoComplete` = mode handling, `selectionMode: 'none'` paths)
- `/home/luke/Projects/base-ui/packages/react/src/autocomplete/root/AutocompleteRoot.test.tsx`
  (behavioral reference: `prop: mode`, `prop: autoHighlight`, `prop: keepHighlight`,
  `prop: filter`, `prop: submitOnItemClick`, Field/Form suites)
- `/home/luke/Projects/base-ui/packages/react/src/autocomplete/value/AutocompleteValue.test.tsx`,
  `.../item/AutocompleteItem.test.tsx`

Existing GPUI implementation to reuse (do not fork/duplicate):

- `crates/base_gpui/src/combobox/` (entire module, once
  `issues/port-baseui-combobox.md` lands) — runtime, context, props, style states,
  child wiring, all layers, actions/key context.
- `crates/base_gpui/src/primitives/input/` — text editing; the controlled-value
  path in `crates/base_gpui/src/primitives/input/runtime.rs` (`sync_props`) is how
  the inline overlay reaches the screen.
- `crates/base_gpui/src/field/`, `crates/base_gpui/src/form/`,
  `crates/base_gpui/src/separator/` — inherited through the Combobox parts.

Current GPUI implementation: none. No `crates/base_gpui/src/autocomplete/` module
exists.

Expected new GPUI files (flat layout per
`docs/base-gpui-component-architecture.md`; deliberately small — no new
runtime.rs/context.rs/props.rs/style_state.rs/actions.rs unless implementation
proves one necessary):

```text
crates/base_gpui/src/autocomplete/mod.rs                          # barrel: new parts + Combobox re-exports under Autocomplete names
crates/base_gpui/src/autocomplete/layers/mod.rs
crates/base_gpui/src/autocomplete/layers/autocomplete_root.rs     # AutocompleteRoot<T>
crates/base_gpui/src/autocomplete/layers/autocomplete_value.rs    # AutocompleteValue
crates/base_gpui/src/autocomplete/tests/
```

Plus `pub mod autocomplete;` in `crates/base_gpui/src/lib.rs`. Key bindings are the
Combobox ones (`combobox::init(cx)` already registered from `base_gpui::init(cx)`);
add an `autocomplete::init(cx)` only if a genuinely new action appears.

Complexity: **small** — ~60 lines of new root logic plus re-exports and tests, and
only because Combobox is sequenced first.

## Initial design decisions

### Composition seam

`ComboboxRoot<T>` deliberately hides the Autocomplete knobs from its builder (Base
UI parity), so `AutocompleteRoot<T>` does **not** wrap `ComboboxRoot<T>`. It builds
the `ComboboxContext<T>` / `ComboboxRuntime<T>` directly — the same way
`ComboboxRoot`'s render does — passing `selection_mode = None`,
`fill_input_on_item_press = true`, the resolved filter, and the inline-autocomplete
configuration, then performs the same wire-children / `sync_children` / `reconcile`
sequence. This mirrors Base UI, where `AutocompleteRoot` renders `AriaCombobox`
directly rather than `ComboboxRoot`. All runtime knobs are public (no `pub(...)`),
so no visibility games are needed.

Children: the re-exported parts are the Combobox part types, so `AutocompleteRoot`
routes children through the existing `ComboboxChild<T>` enum and `child_wiring.rs`.
Default: reuse `ComboboxChild<T>` directly (plus a variant for
`AutocompleteValue`, or wire Value separately) rather than mint a parallel
`AutocompleteChild<T>` enum that duplicates every variant — see Uncertain items.

### Value axis remap

Autocomplete's public value axis is the **input text** (`SharedString`):

- `.default_value(...)` → Combobox `default_input_value`
- `.value(...)` (controlled) → Combobox `input_value`
- `.on_value_change(...)` → Combobox `on_input_value_change`
- `.item_to_string_value(...)` → Combobox `item_to_string_label` (display,
  filtering, fill-on-press) and Field serialization

The Combobox selection axis (`value`/`values`/`on_value_change` over `T`) is not
exposed at all — `selection_mode = None` means there is nothing to select, only an
item press that fills the input.

### Mode model

```text
mode      filtering                  inline autocompletion
list      filter items by query      no
both      filter items by query      yes
inline    static items (no filter)   yes
none      static items (no filter)   no
```

- `list`/`both`: the runtime filter runs (caller `.filter(...)` or the default
  case-insensitive contains).
- `inline`/`none`: internal filtering is disabled (the Combobox issue's
  externally-filtered/`filter_none` knob) — items are static.
- `both` has one subtlety Base UI encodes by wrapping the filter: the query is
  always the **typed** value, never the inline-overwritten display value, so the
  filtered list does not collapse to the highlighted item while the user arrows
  through it.

### Inline overlay ownership

The overlay ("temporarily overwrite the visible input text with the keyboard-
highlighted item's label; restore the typed text on unhighlight") is **state**, and
per the architecture doc state belongs in the deep runtime, not in a part. Default
decision: the overlay lives in `ComboboxRuntime<T>` as the concrete form of the
inline-autocomplete hook point the Combobox issue already reserves — an
`inline_overlay: Option<SharedString>` that the highlight-transition path sets or
clears when inline autocompletion is enabled, with the input layer displaying
"overlay if present, else committed input value" and the query derivation always
using the committed value. `AutocompleteRoot` then only *configures* this. The
alternative (AutocompleteRoot-owned overlay driving a controlled `input_value`,
literally mirroring Base UI's `inlineInputValue` React state) is acceptable if the
runtime-owned form proves invasive, but it duplicates the controlled/uncontrolled
rule outside the context and makes "overlay display must not count as typing"
harder to guarantee. Either way this is the ~60 lines of genuinely new logic. See
"Required additions to the Combobox issue" below.

## Out of scope / drop from Base UI

- The `useFilter` / `useFilteredItems` hook re-exports in `index.parts.ts` — React
  hooks; GPUI filtering is a runtime concern plus the `.filter(...)` callback.
- React context/hooks/store internals (`useComboboxInputValueContext`,
  `useCoreFilter`, `React.useState` inline-value mirroring, effects) — GPUI runtime
  state + queries.
- `render` props, including `AutocompleteValue` function children — replaced by an
  optional formatter closure.
- `className`, web `style` props, CSS variable APIs, DOM data attributes
  (`AutocompleteClearDataAttributes.ts`, `AutocompleteTriggerDataAttributes.ts`,
  etc.) — typed style-state structs, inherited from the Combobox port.
- ARIA (`aria-autocomplete`, `aria-activedescendant`, roles) — deferred to the
  AccessKit follow-up, consistent with Combobox/Select.
- Browser autofill hidden-input heuristics, `autoComplete`/`formAutoComplete`
  attribute pass-through, native `requestSubmit` — Field/Form integration is
  GPUI-native; `submit_on_item_click` remains the Combobox issue's documented hook.
- SSR/hydration (the server-render FormData tests in `AutocompleteRoot.test.tsx`).
- `actionsRef.unmount` and transition-aware unmount — deferred with the shared
  presence/transition follow-up, as in Combobox.
- The grouped-`items` data-prop overload (`items: Array<{ items: ... }>`) —
  compose `AutocompleteGroup` + `AutocompleteCollection` instead; revisit only if
  real callers need the data-driven grouped shape.
- Everything already dropped by `issues/port-baseui-combobox.md` (IME heuristics,
  Floating UI, collator/locale, scroll locking, input-inside-popup topology, etc.)
  is inherited as dropped — do not re-litigate it here.
- No new primitive; no forked copy of any Combobox layer; no `utils/`.
- No Rust scoped visibility (`pub(...)`); `ast-grep scan` must stay clean.

## Acceptance Criteria

New issue — all items unchecked. Items marked **(inherited)** are Combobox behavior
that must keep working through the Autocomplete configuration; verify, do not
reimplement.

### Module / API surface

- [x] `crates/base_gpui/src/autocomplete/` module exists and is registered in
      `crates/base_gpui/src/lib.rs` (`pub mod autocomplete;`); key bindings come
      from the existing `combobox::init(cx)` (add `autocomplete::init(cx)` only if
      a new action is introduced, and register it from `base_gpui::init(cx)`).
- [x] `AutocompleteRoot<T>` builder exists with `.id(...)` keyed identity and
      `T: Clone + Eq + 'static`.
- [x] `AutocompleteRoot<T>` supports `.default_value(...)`, `.value(...)`,
      `.on_value_change(...)` — the **input text** axis (`SharedString`), mapped to
      the Combobox input-value axis.
- [x] `AutocompleteRoot<T>` supports
      `.mode(AutocompleteMode::{List, Both, Inline, None})`, default `List`.
- [x] `AutocompleteRoot<T>` supports `.default_open(bool)`, `.open(bool)`,
      `.on_open_change(...)`.
- [x] `AutocompleteRoot<T>` supports `.disabled(bool)`, `.read_only(bool)`,
      `.required(bool)`, `.name(...)`.
- [x] `AutocompleteRoot<T>` passes through `.auto_highlight(...)` (off /
      on-input-change / always; default off), `.keep_highlight(bool)` (default
      false), `.highlight_item_on_hover(bool)` (default true),
      `.open_on_input_click(bool)` (**default false** — Combobox defaults true),
      `.submit_on_item_click(bool)` (default false; documented hook),
      `.limit(...)`, `.filter(...)`, `.loop_focus(bool)`, and
      `.on_item_highlighted(...)` (receives `Option<T>` + reason
      Keyboard/Pointer/None).
- [x] `AutocompleteRoot<T>` supports `.item_to_string_value(...)` feeding display,
      filtering, fill-on-press, and Field serialization; never requires
      `T: Display`.
- [x] `AutocompleteRoot<T>` does **not** expose selection props (`value`/`values`
      over `T`, `default_value(Option<T>)`, `on_value_change` over `T`,
      `.multiple`), `selection_mode`, `fill_input_on_item_press`, or a custom
      item-equality comparator.
- [x] `AutocompleteValue` exists: renders the current input value, with an optional
      formatter closure over the value (`Fn(&str) -> ...` shape) and optional
      static children taking precedence over the raw value, mirroring Base UI's
      children fallback order.
- [x] `autocomplete/mod.rs` re-exports the reused Combobox parts under Autocomplete
      names: `AutocompleteInput`, `AutocompleteInputGroup`, `AutocompleteTrigger`,
      `AutocompleteIcon`, `AutocompleteClear`, `AutocompleteList`,
      `AutocompleteStatus`, `AutocompletePortal`, `AutocompleteBackdrop`,
      `AutocompletePositioner`, `AutocompletePopup`, `AutocompleteArrow`,
      `AutocompleteGroup`, `AutocompleteGroupLabel`, `AutocompleteItem`,
      `AutocompleteCollection`, `AutocompleteEmpty`, `AutocompleteSeparator`
      (and `AutocompleteRow` iff the Combobox grid stretch landed).
- [x] `ComboboxLabel`, `ComboboxValue`, `ComboboxItemIndicator`, and the chips
      parts are **not** re-exported under Autocomplete names.
- [x] `autocomplete/mod.rs` is barrel exports only.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui autocomplete` passes.
- [x] `cargo test -p base_gpui` passes.
- [x] `cargo clippy -p base_gpui --all-targets` passes or has documented
      pre-existing warnings only.
- [x] `ast-grep scan crates/base_gpui/src/autocomplete` reports no
      scoped-visibility (`pub(...)`) violations.
- [x] Demo in `crates/base_gpui/src/main.rs`: an Autocomplete with input group,
      input, clear, trigger, portal/positioner/popup/list, items, and empty state —
      one `mode = List` (filtering) and one `mode = Both` (inline autocompletion)
      variant.

### Configures the Combobox runtime (thin-variant invariants)

- [x] `AutocompleteRoot<T>` composes the existing `ComboboxRuntime<T>` /
      `ComboboxContext<T>` / `ComboboxChild<T>` wiring — no forked runtime, no
      duplicated filtering/highlight/positioning/dismissal logic anywhere under
      `crates/base_gpui/src/autocomplete/`.
- [x] Regardless of caller input, `selection_mode` is `None` — no selection state
      is ever produced; item activation only fills the input.
- [x] Regardless of caller input, `fill_input_on_item_press` is `true`.
- [x] `open_on_input_click` defaults to `false`; typing a non-empty query still
      opens the popup. **(inherited open-on-type)**
- [x] `mode` resolves to the runtime knobs: `List`/`Both` enable filtering (caller
      `.filter(...)` or default contains); `Inline`/`None` disable internal
      filtering (static items); `Both`/`Inline` enable inline autocompletion.
- [x] In `Both` mode, filtering always uses the **typed** query, never the
      inline-overlay display value (Base UI's filter-wrapping behavior): arrowing
      through items must not re-filter the list down to the highlighted item.
- [x] `.item_to_string_value(...)` is wired as the runtime's label resolver (the
      Combobox `item_to_string_label` slot) and the Field serialization resolver.
- [x] The pass-through knobs (`auto_highlight`, `keep_highlight`,
      `highlight_item_on_hover`, `submit_on_item_click`, `limit`, `loop_focus`)
      reach the runtime unchanged and behave per the Combobox issue's criteria.
      **(inherited)**
- [x] Controlled/uncontrolled resolution for the input-value and open axes reuses
      the Combobox context rule (callbacks with reason details, cancelation,
      controlled source-of-truth). **(inherited)**
- [x] Disabled/read-only roots ignore open/typing/fill interactions. **(inherited)**

### Inline autocompletion behavior (modes `Both`/`Inline` — the new logic)

- [x] Keyboard-highlighting an item (reason Keyboard) temporarily overwrites the
      **displayed** input text with the highlighted item's label (via
      `item_to_string_value`/registered label).
- [x] Programmatic highlight (reason None, e.g. `auto_highlight`) also writes the
      overlay — only reason **Pointer** is excluded: hovering items never changes
      the displayed input text (Base UI preserves the temporary value on hover).
- [x] Unhighlighting (highlight cleared, including popup close and Escape)
      removes the overlay and restores the typed input text exactly.
- [x] The overlay is display-only: while it is active, `on_value_change` does not
      fire, the committed/typed value and query are unchanged, no open-on-type or
      highlight reset is triggered, and Field dirty/filled facts key off the
      committed value.
- [x] Typing clears any active overlay, commits the new typed value, and (in
      `Both` mode) re-filters against it.
- [x] A committed value change from any source (typing, item press, clear,
      controlled `.value(...)` change) clears the overlay.
- [x] Item press (pointer or Enter on the highlighted item) commits the item's
      label as a real input-value change (item-press reason), clears the overlay,
      and closes the popup — this is fill-on-press, not the overlay.
      **(inherited fill-on-press, verified in None selection mode)**
- [x] In modes `List`/`None`, highlight changes never alter the displayed input
      text.
- [x] The overlay reaches the screen through the existing `primitives/input`
      controlled-value path (`sync_props` in
      `crates/base_gpui/src/primitives/input/runtime.rs`); no new input-primitive
      API and no selection-range/suffix-selection behavior (Base UI has none).
- [x] The caret/selection state of the input remains valid when the overlay swaps
      text in and out (the input runtime already clamps selection on controlled
      value change — verify through the overlay path).

### Input-value exposure via `AutocompleteValue`

- [x] `AutocompleteValue` renders the current **displayed** input value — including
      an active inline overlay — matching Base UI, where the Value part reads the
      resolved input value context.
- [x] With a formatter closure, the closure receives the current value and its
      result is rendered; with static children, children win; otherwise the raw
      value renders.
- [x] `AutocompleteValue` re-renders live as the value changes (typing, overlay
      set/clear, fill-on-press, clear button).

### Reuse of Combobox behavior (inherited — verify through Autocomplete, do not reimplement)

- [x] Default filtering (case-insensitive contains on labels), custom
      `.filter(...)`, `.limit(...)`, and static-items mode behave per the Combobox
      issue.
- [x] `AutocompleteEmpty` renders children only when the filtered list is empty;
      highlight resets on empty list.
- [x] Highlight/keyboard navigation: ArrowDown/ArrowUp open + move highlight,
      focus stays on the input (virtual highlight), `loop_focus` input-position
      semantics, disabled items highlightable but inert, highlighted item scrolled
      into view.
- [x] `auto_highlight` modes and `keep_highlight`/`highlight_item_on_hover`
      pointer semantics work as specified in the Combobox issue.
- [x] Enter with no highlight closes the popup; Escape while open closes; Escape
      while closed with a non-empty input clears it (escape-key reason; None-mode
      variant: input only, no selection to clear).
- [x] Positioner anchors to input-group-else-input; outside press dismisses;
      trigger/clear/input-group presses do not count as outside; focus-out closes
      and marks the Field touched.
- [x] `AutocompleteClear` is visible when the input is non-empty (the Combobox
      issue's None-mode visibility rule); pressing it clears the input (clear-press
      reason) and refocuses the input.
- [x] Field/Form: filled = input non-empty; dirty compares the committed input
      value against initial; validation modes; the submitted value is the **input
      text** (via `item_to_string_value` only for item-derived text), never a
      selection; invalid submit focuses the input.
- [x] Style states are the Combobox style-state structs, reported through the
      re-exported parts with `selection_mode = None` facts (root reports selection
      mode; input/trigger/input-group report open/popup-side/list-empty/Field
      facts); `style_with_state(...)` works on every re-exported part.

### Tests / verification

Runtime tests (no window) under `crates/base_gpui/src/autocomplete/tests/` (or as
Combobox runtime tests where the knowledge lives there — do not duplicate suites):

- [x] Mode matrix: `List` filters + no overlay; `Both` filters + overlay; `Inline`
      static + overlay; `None` static + no overlay.
- [x] `Both` filters on the typed query while an overlay is active (list does not
      collapse to the highlighted item).
- [x] Overlay lifecycle: keyboard highlight sets it, pointer highlight does not,
      unhighlight/close restores the typed text, typing clears and commits,
      committed change from clear/controlled update clears it, no
      `on_value_change` fires for overlay-only changes.
- [x] Item press in `selection_mode = None`: fills the input with the label
      (item-press reason), closes the popup, produces no selection.
- [x] Controlled vs uncontrolled input value through the Autocomplete axis names,
      including cancelation and Escape-clear.
- [x] `open_on_input_click = false` default: input click does not open; typing
      opens.

Rendered tests (not implemented in this pass — no windowed rendered-test
harness exists yet for Combobox either; runtime coverage above plus the
main.rs demos stand in):

- [ ] Typing filters visible items (`List`/`Both`) vs static items
      (`Inline`/`None`).
- [ ] ArrowDown highlights and (in `Both`/`Inline`) the input displays the
      highlighted label; ArrowUp/Escape restores the typed text.
- [ ] Hovering items never changes the input text in any mode.
- [ ] Enter on a highlighted item fills the input and closes.
- [ ] `AutocompleteValue` tracks typing, overlay, and fill-on-press; formatter
      closure output renders.
- [ ] Clear button visibility/behavior with a non-empty input.
- [ ] Field/Form: submitted value is the typed input text; filled/dirty/touched
      facts; external error clearing on change.
- [ ] Demos render in `crates/base_gpui/src/main.rs` without panics.

## Required additions/clarifications to `issues/port-baseui-combobox.md`

The Combobox issue anticipated this port well; these three small deltas are needed
(add them there — do not compensate by duplicating runtime logic here):

- [x] **Input-value query for the Value part.** Autocomplete's `Value` reads the
      current (displayed) input value; the `ComboboxRuntime<T>` needs a part-shaped
      query exposing it (Base UI's `useComboboxInputValueContext` analog). The
      runtime owns the value already — this is only a query addition.
- [x] **Upgrade the inline-autocomplete "hook point" into overlay state.** The
      Combobox issue reserves a highlight-outcome hook; this port needs it to be
      concrete: `inline_overlay: Option<SharedString>` set/cleared on highlight
      transitions when enabled, a display-value query ("overlay if present, else
      committed input value") consumed by the input layer, and the guarantee that
      overlay display changes are never treated as typing (no
      `on_input_value_change`, no query recompute, no open-on-type/highlight
      reset). Equivalently: reconciling a controlled input value that differs only
      by overlay must not count as a user edit.
- [x] **Correct the follow-up wording about the input primitive.** The Combobox
      issue's follow-up says inline autocompletion "requires ... input-primitive
      selection-range support" — it does not. Base UI's `AutocompleteRoot`
      overwrites the whole visible value and never selects a completed suffix; the
      existing controlled-value path (`sync_props` in
      `crates/base_gpui/src/primitives/input/runtime.rs`) suffices.
- [x] **None-mode Field serialization.** State explicitly that in
      `selection_mode = None` the registered Field value is
      `FieldValue::Text(input value)` (the Combobox issue covers filled/dirty for
      None mode but not serialization).
- [x] Cross-link: the Combobox issue's Autocomplete follow-up item should point at
      this file as written (it currently says "to be written").

## AccessKit accessibility follow-up

Autocomplete is almost entirely re-exported Combobox parts, so nearly all of the
role/aria work belongs to the Combobox AccessKit follow-up
(`issues/port-baseui-combobox.md`) and is **inherited** here — implement it once
on the Combobox layers, then verify it through the Autocomplete configuration
(`selection_mode = None`, `mode` matrix). This section records the full per-part
mapping as it must behave *for Autocomplete*, plus the two autocomplete-only
parts. Everything below uses only APIs from
`docs/accesskit-gpui-reference.md` — no invented builders.

Base UI ARIA source of truth: `combobox/root/AriaCombobox.tsx` (reference gets
`role="combobox"`, `aria-expanded`, `aria-haspopup="listbox"`, `aria-controls`,
`aria-autocomplete` = the `mode`), `combobox/input/ComboboxInput.tsx`
(`aria-readonly`, `aria-required`, `aria-labelledby`),
`combobox/list/ComboboxList.tsx` (`role="listbox"`),
`combobox/item/ComboboxItem.tsx` (`role="option"`, `aria-selected` only when
selectable — never for Autocomplete), `combobox/trigger/ComboboxTrigger.tsx`
(`aria-expanded`, `aria-haspopup`, `aria-controls`, `aria-labelledby`),
`combobox/popup/ComboboxPopup.tsx` (`role="presentation"` when the input is
outside the popup — always, for us), `combobox/group/ComboboxGroup.tsx`
(`role="group"`, `aria-labelledby`), and `combobox/status/ComboboxStatus.tsx` /
`combobox/empty/ComboboxEmpty.tsx` (`aria-live="polite"`, `aria-atomic`). The
autocomplete-specific parts (`AutocompleteRoot`, `AutocompleteValue`) emit no
ARIA of their own in Base UI.

### Per accessible part (all on the Combobox layers; verified through the Autocomplete re-exports)

- **`AutocompleteInput` = `ComboboxInput<T>`**
  (`crates/base_gpui/src/combobox/layers/combobox_input.rs`, wrapping
  `primitives/input`): `.role(Role::TextInput)` on the id'd input element —
  accesskit has no combined text-field-combobox role in this revision;
  `Role::TextInput` plus `.aria_expanded(...)` is the closest honest encoding of
  Base UI's `role="combobox"` input. Props:
  `.aria_expanded(context.read(cx, |runtime, _| runtime.open_value()))`.
  Focus is already wired through `input_focus_handle` / the input primitive's
  focus handle, so `Action::Focus` is auto-registered — do not re-add.
- **`AutocompleteTrigger` = `ComboboxTrigger<T>`**
  (`combobox_trigger.rs`): `.role(Role::Button)`,
  `.aria_expanded(runtime.open_value())`, `.aria_label(...)` from a new
  builder prop (default e.g. `"Open popup"`). Its `.on_click` already toggles
  open via `ComboboxContext::toggle_open`, so `Action::Click` is auto-registered.
- **`AutocompleteClear` = `ComboboxClear`** (`combobox_clear.rs`):
  `.role(Role::Button)`, `.aria_label(...)` builder prop (default `"Clear"`).
  `.on_click` (→ `ComboboxContext::clear_all`) auto-registers `Action::Click`.
- **`AutocompleteList` = `ComboboxList<T>`** (`combobox_list.rs`):
  `.role(Role::ListBox)`. Base UI's `aria-multiselectable` never applies —
  Autocomplete has no `.multiple`.
- **`AutocompleteItem` = `ComboboxItem<T>`** (`combobox_item.rs`):
  `.role(Role::ListBoxOption)`,
  `.aria_position_in_set(...)` / `.aria_size_of_set(...)` from
  `ComboboxItemMetadata::index()` mapped through
  `ComboboxRuntime::filtered_indices()` (position among *visible* items, size =
  `filtered_indices().len()`). Do **not** set `.aria_selected(...)` — in
  `selection_mode = None` Base UI omits `aria-selected` entirely (`selectable ?
  selected : undefined`); verify the Combobox layer's selected-mode
  `.aria_selected` wiring is skipped under the Autocomplete configuration.
  Express keyboard highlight as `.aria_selected(true)` **only if** the Combobox
  follow-up chooses that encoding for virtual focus — otherwise leave highlight
  unexpressed (see Gaps: `aria-activedescendant`). `.on_click` (→
  `ComboboxContext::select_item`, i.e. fill-on-press here) auto-registers
  `Action::Click`.
- **`AutocompleteGroup` = `ComboboxGroup`** (`combobox_group.rs`):
  `.role(Role::Group)` with `.aria_label(...)` resolved from
  `ComboboxGroupMetadata::label()` (the literal-string stand-in for Base UI's
  `aria-labelledby` → GroupLabel id wiring).
- **`AutocompleteGroupLabel` = `ComboboxGroupLabel`**: visible text only; render
  it with `Text::new_inaccessible(...)` since the group already carries the same
  string as `.aria_label` — avoids double-announcing.
- **No role (decorative / presentation — leave out of the a11y tree by giving
  them no `.role(...)`)**: `AutocompletePopup` (Base UI `role="presentation"` in
  the input-outside-popup topology, which is our only topology),
  `AutocompletePositioner`, `AutocompletePortal`, `AutocompleteBackdrop`,
  `AutocompleteArrow`, `AutocompleteIcon`, `AutocompleteInputGroup`,
  `AutocompleteRow`, `AutocompleteCollection`, `AutocompleteSeparator` (the
  shared `separator::Separator` follow-up owns `Role::Separator` if wanted).
- **`AutocompleteRoot<T>`**
  (`crates/base_gpui/src/autocomplete/layers/autocomplete_root.rs`): container
  only — no role (and `Role::GenericContainer` is filtered/asserts, so simply
  set none).
- **`AutocompleteValue<T>`**
  (`crates/base_gpui/src/autocomplete/layers/autocomplete_value.rs`): display
  mirror of `ComboboxRuntime::display_value()`. No role. Render the resolved
  string via `Text::new_inaccessible(...)` so the value is not announced twice
  (the input itself is the accessible value surface). This is the only
  autocomplete-module code change in this follow-up.

### Actions

- `Action::Click` and `Action::Focus` are auto-registered by the existing
  `.on_click` / focus-handle wiring on trigger, clear, item, and input — do
  **not** re-add handlers for them.
- Add `.on_a11y_action(AccessibleAction::Expand, ...)` /
  `(AccessibleAction::Collapse, ...)` on the input (or trigger) routed into the
  **same** transition the pointer/keyboard path uses:
  `ComboboxContext::set_open(true/false, ...)` — never a parallel open path.
- Do not add `SetValue` on the input in this pass; programmatic text entry is
  the input primitive's follow-up, not Autocomplete's.

### Labels

- Input accessible name: Base UI uses Field-label `aria-labelledby`; with no
  id-reference wiring, take a literal `.aria_label(...)` builder prop on
  `ComboboxInput` (Field integration may forward the Field label string later).
- Trigger/Clear: `.aria_label(...)` builder props with sensible defaults (they
  render icon-only in the demos).
- Group label text and `AutocompleteValue` output: `Text::new_inaccessible(...)`
  as above. Any other visible `text!(...)` inside a part that already sets
  `.aria_label` on its id'd parent gets the same treatment.

### Gaps (Base UI ARIA with no gpui builder in this revision)

- **`aria-autocomplete`** (`'list' | 'both' | 'inline' | 'none'` — literally the
  Autocomplete `mode`, the one ARIA attribute this component *adds* over
  Combobox): no builder, and accesskit exposure isn't surfaced by gpui. Omit +
  document; note in `AutocompleteRoot` docs that the `mode` is not conveyed to
  AT. Blocked pending gpui upstream.
- **`aria-activedescendant`** (virtual highlight while focus stays on the
  input): no relationship builders. Fallback: omit; optionally encode highlight
  as `.aria_selected(true)` on the highlighted `ComboboxItem` if the Combobox
  follow-up adopts that convention — decide there, once, not per variant.
- **`aria-controls`**, **`aria-labelledby`**, **`aria-describedby`**,
  **`aria-owns`**: no id-reference wiring. Fallback: literal `.aria_label`
  strings only; omit the relationships.
- **`aria-haspopup="listbox"`**: no builder. Fallback: omit; `.aria_expanded`
  plus `Role::ListBox` on the popup list carries the intent.
- **`disabled` / `aria-disabled`, `aria-readonly`, `aria-required`**: no
  builders (`write_a11y_info` sets no disabled flag). Fallback: while
  `.disabled(true)` / `.read_only(true)` the runtime already ignores
  interactions; additionally skip registering `Expand`/`Collapse` handlers on a
  disabled root and document that the disabled state itself is not announced.
  Blocked pending gpui upstream for the announced state.
- **Live regions** — `AutocompleteStatus` / `AutocompleteEmpty`
  (`aria-live="polite"`, `aria-atomic`): no announcement API in gpui. Omit +
  document on both parts; blocked pending gpui upstream. The inline overlay
  text swap (modes `Both`/`Inline`) is likewise silent to AT for now.

### Checklist

- [ ] `ComboboxInput`: `Role::TextInput` + `.aria_expanded(open)` +
      `.aria_label(...)` prop; verify `Action::Focus` comes free from the input
      focus handle. **(inherited — implement in Combobox follow-up)**
- [ ] `ComboboxTrigger`: `Role::Button` + `.aria_expanded(open)` +
      `.aria_label(...)`; Click auto-registered. **(inherited)**
- [ ] `ComboboxClear`: `Role::Button` + `.aria_label(...)`; Click
      auto-registered. **(inherited)**
- [ ] `ComboboxList`: `Role::ListBox`. **(inherited)**
- [ ] `ComboboxItem`: `Role::ListBoxOption` + position/size-of-set from
      `filtered_indices()`; **no** `aria_selected` under
      `selection_mode = None`. **(inherited — verify through Autocomplete)**
- [ ] `ComboboxGroup`/`ComboboxGroupLabel`: `Role::Group` + `.aria_label` from
      `ComboboxGroupMetadata::label()`; label text `Text::new_inaccessible`.
      **(inherited)**
- [ ] `Expand`/`Collapse` a11y actions route through
      `ComboboxContext::set_open`; skipped while disabled/read-only.
      **(inherited)**
- [ ] `AutocompleteValue` renders its string via `Text::new_inaccessible(...)`
      (this issue's only code change).
- [ ] Popup/positioner/portal/backdrop/arrow/icon/input-group have no role and
      stay out of the a11y tree.
- [ ] Gaps documented in rustdoc where they bite: `aria-autocomplete` (mode) on
      `AutocompleteRoot`, disabled/read-only announcement, live-region silence
      on `AutocompleteStatus`/`AutocompleteEmpty`, no
      activedescendant/controls/labelledby/haspopup wiring.
- [ ] a11y verification pass with the Autocomplete demos in
      `crates/base_gpui/src/main.rs` (ids stable across frames so nodes are not
      remove+add churned while typing/filtering).

## Uncertain items needing confirmation

- Child routing: reuse `ComboboxChild<T>` directly for `AutocompleteRoot` children
  (default — the parts are the same types) vs a narrowed `AutocompleteChild<T>`
  enum excluding chips/label/indicator variants. Reuse is less code; the narrow
  enum rejects non-Autocomplete parts at compile time. Also where
  `AutocompleteValue` routes (new variant vs separate wiring).
- Overlay ownership: runtime-owned (default, per the architecture doc) vs
  AutocompleteRoot-owned controlled input value (literal Base UI mirror). Decide
  during the Combobox implementation of the hook point.
- Naming: `.value`/`.on_value_change` on `AutocompleteRoot` mean the **input
  text**, deliberately diverging from `ComboboxRoot`'s selection-typed `.value` —
  matches Base UI naming, but confirm the same-crate inconsistency is acceptable.
- Whether `AutocompleteValue` is worth porting in the first pass (Base UI renders
  no element; it exists for display composition). Default: port it as the thin
  display part scoped above.
