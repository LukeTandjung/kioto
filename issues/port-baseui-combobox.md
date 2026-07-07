# Port Base UI Combobox to GPUI

## Problem

Base UI Combobox is an input-driven filtering listbox: the user types a query into a text
input, the item list filters live against that query, and items can be selected in single or
multiple mode. It provides controlled and uncontrolled selected value, input value, and popup
open state; query-based filtering with single-selection browse semantics; highlight (active
item) state that is separate from both selection and DOM focus (focus stays on the input while
arrow keys move a virtual highlight); auto-highlight of the first match; multiple selection
rendered as removable chips with their own keyboard navigation; a clear button; an
empty-list part; grouping; optional grid rows; optional virtualization; and the same
portal/positioner/popup/backdrop/arrow overlay stack as Select.

Unlike Select, Combobox has **no open-list typeahead** — printable keys edit the input query
instead. (Base UI has one narrow typeahead: the closed trigger in single mode, which is
optional stretch scope here.) The core logic lives in
`root/AriaCombobox.tsx` (~1644 lines) with a `selectionMode` of `single | multiple | none`;
`ComboboxRoot` is a thin wrapper that maps `multiple` to `selectionMode` and hides the
internal knobs (`fillInputOnItemPress`, `keepHighlight`, `submitOnItemClick`,
`autoComplete`/inline-autocomplete). Base UI's Autocomplete component is the *same*
`AriaCombobox` core with `selectionMode: 'none'` — so the GPUI `ComboboxRuntime` must keep
those knobs public and configurable so a future thin Autocomplete port
(`issues/port-baseui-autocomplete.md`) can reuse it the way `alert_dialog`
reuses `dialog` (see `issues/port-baseui-alert-dialog.md`).

`crates/base_gpui` already has everything below the popup line: the text-editing primitive
`primitives/input` (which `ComboboxInput` must wrap, not reimplement), `field`, `form`,
`separator`, and a complete Select port whose positioner/portal/list/runtime are the closest
per-component precedent for the overlay and highlight machinery. Per
`docs/base-gpui-component-architecture.md`, that popup infrastructure is **not** extracted
into a shared primitive — Combobox implements its own local equivalents, citing the Select
and Popover files as references.

Item values use Rust-native generics constrained as `T: Clone + Eq + 'static`. Multiple
selection is an ordered `Vec<T>`. Labels come from item registration or an explicit
`item_to_string_label` resolver — never `T: Display` by default. Do not port JavaScript
value/equality semantics (`Object.is`, `isItemEqualToValue`) — `Eq` is the default; a custom
comparator is a deliberate later addition only if real callers need it (same decision as
Select).

## Scope

Port the Combobox component family from Base UI into GPUI-native components (26 public
parts):

- `ComboboxRoot<T>`
- `ComboboxLabel`
- `ComboboxValue<T>`
- `ComboboxInput<T>` — wraps the existing `primitives/input` `Input`
- `ComboboxInputGroup<T>`
- `ComboboxTrigger<T>`
- `ComboboxList<T>`
- `ComboboxStatus`
- `ComboboxPortal<T>`
- `ComboboxBackdrop`
- `ComboboxPositioner<T>`
- `ComboboxPopup<T>`
- `ComboboxArrow`
- `ComboboxIcon`
- `ComboboxGroup`
- `ComboboxGroupLabel`
- `ComboboxItem<T>`
- `ComboboxItemIndicator<T>`
- `ComboboxChips<T>`
- `ComboboxChip<T>`
- `ComboboxChipRemove<T>`
- `ComboboxRow` — grid mode, **stretch scope**
- `ComboboxCollection<T>` — data-driven item rendering from a `Vec<T>` + per-item builder closure
- `ComboboxEmpty`
- `ComboboxClear`
- `ComboboxSeparator` backed by the shared `base_gpui::separator::Separator`

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/combobox/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/root/ComboboxRoot.tsx` (thin wrapper; prop surface)
- `/home/luke/Projects/base-ui/packages/react/src/combobox/root/AriaCombobox.tsx` (core behavior, ~1644 lines)
- `/home/luke/Projects/base-ui/packages/react/src/combobox/root/ComboboxRootContext.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/store.ts`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/root/utils/useFilter.ts`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/root/utils/useFilteredItems.ts`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/root/utils/index.ts` (collator filters; single-selection browse filter)
- `/home/luke/Projects/base-ui/packages/react/src/combobox/root/utils/constants.ts`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/input/ComboboxInput.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/input-group/ComboboxInputGroup.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/trigger/ComboboxTrigger.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/list/ComboboxList.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/value/ComboboxValue.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/label/ComboboxLabel.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/icon/ComboboxIcon.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/status/ComboboxStatus.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/empty/ComboboxEmpty.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/clear/ComboboxClear.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/portal/ComboboxPortal.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/backdrop/ComboboxBackdrop.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/positioner/ComboboxPositioner.tsx` (+ `ComboboxPositionerContext.tsx`)
- `/home/luke/Projects/base-ui/packages/react/src/combobox/popup/ComboboxPopup.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/arrow/ComboboxArrow.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/group/ComboboxGroup.tsx` (+ `ComboboxGroupContext.ts`)
- `/home/luke/Projects/base-ui/packages/react/src/combobox/group-label/ComboboxGroupLabel.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/item/ComboboxItem.tsx` (+ `ComboboxItemContext.ts`)
- `/home/luke/Projects/base-ui/packages/react/src/combobox/item-indicator/ComboboxItemIndicator.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/chips/ComboboxChips.tsx` (+ `ComboboxChipsContext.ts`)
- `/home/luke/Projects/base-ui/packages/react/src/combobox/chip/ComboboxChip.tsx` (+ `ComboboxChipContext.ts`)
- `/home/luke/Projects/base-ui/packages/react/src/combobox/chip-remove/ComboboxChipRemove.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/row/ComboboxRow.tsx` (+ `ComboboxRowContext.ts`)
- `/home/luke/Projects/base-ui/packages/react/src/combobox/collection/ComboboxCollection.tsx` (+ `GroupCollectionContext.tsx`)
- `/home/luke/Projects/base-ui/packages/react/src/combobox/utils/handleInputPress.ts`
- `/home/luke/Projects/base-ui/packages/react/src/combobox/**/*.test.tsx` for behavior coverage (especially `root/ComboboxRoot.test.tsx`, `input/ComboboxInput.test.tsx`, `chip/ComboboxChip.test.tsx`, `clear/ComboboxClear.test.tsx`)
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/combobox/page.mdx` and demos

Local GPUI reuse and precedent (reuse the first group, reference the second):

Reused directly:

- `crates/base_gpui/src/primitives/input/` — the text-field primitive backing
  `ComboboxInput` (`Input` builder with `value`, `default_value`, `placeholder`, `disabled`,
  `read_only`, `on_value_change_with_context`, `on_enter`, `on_home`/`on_end`,
  `focus_handle`, `style_with_state`).
- `crates/base_gpui/src/field/` and `crates/base_gpui/src/form/` — Field/Form registration.
- `crates/base_gpui/src/separator/` — backs `ComboboxSeparator`.
- gpui `uniform_list` — for the virtualized stretch item; no new primitive.

Per-component reference only (do **not** extract shared popup primitives — the architecture
doc forbids premature extraction; implement Combobox-local equivalents citing these):

- `crates/base_gpui/src/select/layers/select_positioner.rs` — anchored/deferred positioning,
  trigger-bounds measurement, side/align mapping.
- `crates/base_gpui/src/select/layers/select_portal.rs` — mounted/force-mounted deferred overlay.
- `crates/base_gpui/src/select/layers/select_list.rs` — list scroll + highlight-into-view.
- `crates/base_gpui/src/select/runtime.rs` — highlight navigation (`move_highlight`),
  list scroll handle, `scroll_highlighted_into_view`, style-state query shape.
- `crates/base_gpui/src/popover/layers/popover_backdrop.rs` and
  `crates/base_gpui/src/popover/layers/popover_arrow.rs` — backdrop and arrow layers.
- `/home/luke/Projects/gpui-component/crates/ui/src/combobox.rs` and
  `/home/luke/Projects/gpui-component/crates/ui/src/searchable_list/` — nearest Rust analog
  for query filtering of a registered item list.

Current GPUI implementation: none. No `crates/base_gpui/src/combobox/` module exists.

Expected new GPUI files (flat layout per `docs/base-gpui-component-architecture.md`):

```text
crates/base_gpui/src/combobox/mod.rs
crates/base_gpui/src/combobox/actions.rs
crates/base_gpui/src/combobox/runtime.rs
crates/base_gpui/src/combobox/context.rs
crates/base_gpui/src/combobox/props.rs
crates/base_gpui/src/combobox/style_state.rs
crates/base_gpui/src/combobox/child.rs
crates/base_gpui/src/combobox/child_wiring.rs
crates/base_gpui/src/combobox/layers/mod.rs
crates/base_gpui/src/combobox/layers/combobox_root.rs
crates/base_gpui/src/combobox/layers/combobox_label.rs
crates/base_gpui/src/combobox/layers/combobox_value.rs
crates/base_gpui/src/combobox/layers/combobox_input.rs
crates/base_gpui/src/combobox/layers/combobox_input_group.rs
crates/base_gpui/src/combobox/layers/combobox_trigger.rs
crates/base_gpui/src/combobox/layers/combobox_list.rs
crates/base_gpui/src/combobox/layers/combobox_status.rs
crates/base_gpui/src/combobox/layers/combobox_portal.rs
crates/base_gpui/src/combobox/layers/combobox_backdrop.rs
crates/base_gpui/src/combobox/layers/combobox_positioner.rs
crates/base_gpui/src/combobox/layers/combobox_popup.rs
crates/base_gpui/src/combobox/layers/combobox_arrow.rs
crates/base_gpui/src/combobox/layers/combobox_icon.rs
crates/base_gpui/src/combobox/layers/combobox_group.rs
crates/base_gpui/src/combobox/layers/combobox_group_label.rs
crates/base_gpui/src/combobox/layers/combobox_item.rs
crates/base_gpui/src/combobox/layers/combobox_item_indicator.rs
crates/base_gpui/src/combobox/layers/combobox_chips.rs
crates/base_gpui/src/combobox/layers/combobox_chip.rs
crates/base_gpui/src/combobox/layers/combobox_chip_remove.rs
crates/base_gpui/src/combobox/layers/combobox_row.rs          # stretch (grid mode)
crates/base_gpui/src/combobox/layers/combobox_collection.rs
crates/base_gpui/src/combobox/layers/combobox_empty.rs
crates/base_gpui/src/combobox/layers/combobox_clear.rs
crates/base_gpui/src/combobox/layers/combobox_separator.rs
crates/base_gpui/src/combobox/tests/
```

Complexity: **large** — comparable to Select plus input-value plumbing and filtering. Grid
`Row` mode and virtualized rendering are clearly-marked stretch scope; do not block the port
on them.

## Initial design decisions

### Selection modes and the Autocomplete seam

`ComboboxRuntime<T>` implements the `AriaCombobox` core with a
`ComboboxSelectionMode { Single, Multiple, None }` — including `None`, even though
`ComboboxRoot<T>` only exposes `.multiple(bool)` (mapping to Single/Multiple), because the
future Autocomplete port is `selection_mode = None` plus different defaults. Keep these
runtime/props knobs public and configurable rather than hard-coding Combobox defaults:

- `selection_mode` (`Single`/`Multiple`/`None`)
- `fill_input_on_item_press` (None mode: whether pressing an item writes its label into the input)
- `auto_highlight` (`Off` / `OnInputChange` / `Always` — Base UI `autoHighlight: boolean | 'always'`)
- `keep_highlight` (retain highlight when the pointer leaves the list)
- `highlight_item_on_hover`
- `open_on_input_click`
- `submit_on_item_click` (may be a documented no-op hook until Form exposes programmatic submit)
- `inline_overlay` state (Base UI `autoComplete: 'both' | 'inline'`) — concrete runtime
  state, not merely a hook point. When a highlight change should preview the highlighted
  item's label in the input, the runtime holds an `inline_overlay: Option<..>` and exposes a
  **display-value query** (the string the input layer paints = the overlay when present, else
  the real input value). Two guarantees the Autocomplete port depends on: (1) the overlay is a
  *whole-value* replacement of the visible text, so it needs **no** input-primitive
  selection-range support — the existing `sync_props` controlled-value path in
  `crates/base_gpui/src/primitives/input/runtime.rs` suffices; (2) applying or clearing the
  overlay through `reconcile` is **not** treated as user typing and never fires
  `on_input_value_change`.
- a public **input-value query** so a consumer part can read the current input value (Base UI
  `AutocompleteValue` renders the input value, distinct from `ComboboxValue` which renders the
  selected value); the runtime already owns input value, so this is a read-only exposure.

`ComboboxRoot<T>` hides the Autocomplete-only knobs from its public builder exactly as
Base UI's `ComboboxRoot` omits them from `AriaCombobox.Props`. This decision is consumed by
`issues/port-baseui-autocomplete.md` (already written) — keep the cross-link in sync.

### Value model

Same pragmatic split as Select (`issues/port-baseui-select.md`):

- `.default_value(Option<T>)` / `.value(Option<T>)` / `.on_value_change(...)` for single mode;
- `.multiple(bool)` plus `.default_values(Vec<T>)` / `.values(Vec<T>)` / `.on_values_change(...)`
  for multiple mode;
- `T: Clone + Eq + 'static`; ordered `Vec<T>` for multiple; no JS union types.

Additionally Combobox has a third controlled/uncontrolled axis absent from Select — the
**input value**:

- `.default_input_value(...)` / `.input_value(...)` / `.on_input_value_change(...)` using
  `SharedString`.
- When neither is provided and mode is single, the initial input value derives from the
  selected value's label (Base UI `initialDefaultInputValue`).

Labels/serialization: `.item_to_string_label(...)` for display/filtering and
`.item_to_string_value(...)` for Field/Form serialization, defaulting to the item's
registered label (from `ComboboxItem::label(...)` or item text children). Never require
`T: Display`; never use `Debug` formatting silently.

### Filtering model

Base UI filters either a data `items` prop or the DOM-registered items. The GPUI port
filters **registered item metadata** in the runtime (nearest Rust analog:
`gpui-component/crates/ui/src/searchable_list/`), with `ComboboxCollection<T>` as the
data-driven alternative (root/list receives `Vec<T>` + `Fn(&T, usize) -> ComboboxItem<T>`).

- Query = trimmed input value (runtime-owned derivation; a frozen `close_query` keeps
  filtering stable while the popup is closing).
- Default filter: case-insensitive substring match on the item label. Do not port
  `Intl.Collator`/locale machinery; document the simplification and accept a custom
  `.filter(...)` callback (`Fn(&T, &str) -> bool` shape, label resolver available) for
  callers who need more. `.filter_none()`/equivalent disables internal filtering for
  externally filtered lists (Base UI `filter={null}` + `filteredItems`).
- Single-selection browse semantics: when the popup opens with a selected value and the
  query still exactly matches the selected label (`query_changed_after_open == false`), show
  **all** items so the user can browse (Base UI `shouldBypassFiltering` +
  `createSingleSelectionCollatorFilter`).
- `.limit(usize)` caps the filtered list.

### Popup/positioner scope

Same GPUI-native subset as Select: deferred `anchored()` overlay, prepaint-measured anchor
bounds, `side`/`align`/`side_offset`/`align_offset`, `snap_to_window_with_margin`, outside
mouse-down dismissal, next-frame focus handoff. The anchor is the **input group if present,
else the input** (Base UI `resolvedAnchor`), not a trigger button. Combobox has no
`align_item_with_trigger` mode and no scroll arrows — this port is simpler than Select on the
positioning axis. Keep all positioning code Combobox-local under
`crates/base_gpui/src/combobox/`.

Base UI also supports an input-inside-popup topology (button trigger opens a popup that
contains the input, detected via a positioner-parent context). That topology is **deferred**
(follow-up) — the first port supports the canonical input-outside-popup composition, and the
wiring/props leave room to add `input_inside_popup` later without API breakage.

## Out of scope / drop from Base UI

- React hooks/context/store internals (`useControlled`, `Store`, refs, `useStableCallback`,
  `flushSync`, `queueMicrotask`) — GPUI keyed entity state + `ComboboxContext<T>`.
- `render` props (including `ComboboxValue`/`ComboboxList` function children — replaced by
  typed builder closures on `ComboboxCollection`/`ComboboxValue`).
- `className`, web `style` props, CSS variable APIs (`ComboboxPositionerCssVars`).
- DOM data attributes (`stateAttributesMapping`, all `*DataAttributes.ts`) — mapped into
  typed style-state structs.
- ARIA roles/attributes (`role="combobox"`, `aria-activedescendant`, `aria-autocomplete`,
  `aria-expanded`, listbox/option/grid roles, labelled-by wiring) — deferred to the AccessKit
  follow-up; no DOM ARIA strings in GPUI.
- Live-region announcement mechanics of `Status`/`Empty` (`role="status"`, `aria-live`,
  `useInitialLiveRegionTextMutation`, the iOS `Status` test file) — GPUI keeps the parts but
  they only render (conditional) children.
- `nativeButton` switches and `useButton` internals.
- SSR/hydration, `suppressHydrationWarning`, prehydration.
- Hidden DOM inputs, browser autofill handling (`insertReplacementText` heuristics),
  `autoComplete`/`formAutoComplete` hints, `form` attribute, native `requestSubmit` — Field/Form
  integration is GPUI-native; `submit_on_item_click` becomes a documented hook, not a DOM form
  submit.
- IME composition-event heuristics (`onCompositionStart/End`, `event.which === 229`, Android
  workarounds) — text editing is owned by the `primitives/input` runtime; revisit IME there.
- DOM event objects, `allowPropagation`, `preventBaseUIHandler`, event bubbling control —
  Rust-native details structs with reason/source/cancel state (Select precedent).
- `Intl.Collator` / `locale` prop — simple case-insensitive contains default + custom filter
  callback.
- Floating UI middleware, `ResizeObserver`, `getBoundingClientRect`,
  `getPseudoElementBounds`, scroll locking, `InternalBackdrop` inertness, touch heuristics
  (`touchOpenDelay`, `outsidePressEvent` mouse/touch modes), `ComboboxInternalDismissButton`
  focus-trap sentinels.
- Links inside items (`closest('a')` href handling).
- `isItemEqualToValue` custom equality (use `T: Eq`; comparator only as a later deliberate
  addition), arbitrary JS value semantics.
- `actionsRef.unmount` imperative handle and transition-aware unmount sequencing — GPUI has no
  shared transition/presence infrastructure yet; mount/unmount is immediate, transition
  status fields are deferred with the shared follow-up (as in Select/Popover).
- The input-inside-popup topology (first port), `inline` list mode (no popup), and
  `modal` — explicit follow-ups.
- Do not extract a generic popup/overlay/collection primitive; keep everything
  Combobox-local.
- No Rust scoped visibility (`pub(...)`); `ast-grep scan` must stay clean.

## Acceptance Criteria

New issue — all items unchecked.

### Module/API surface

- [x] Add a top-level `combobox` module and export it from `crates/base_gpui/src/lib.rs`.
- [x] Register Combobox key bindings from `base_gpui::init(cx)` (`combobox::init(cx)`).
- [x] Add public `ComboboxRoot<T>` with `ComboboxRoot::new()` and `.id(...)` keyed identity.
- [x] `ComboboxRoot<T>` supports `.default_value(Option<T>)`, `.value(Option<T>)`,
      `.on_value_change(...)` (single mode).
- [x] `ComboboxRoot<T>` supports `.multiple(bool)`, `.default_values(Vec<T>)`,
      `.values(Vec<T>)`, `.on_values_change(...)` (multiple mode), with documented
      deterministic behavior when both single and multiple value props are supplied.
- [x] `ComboboxRoot<T>` supports `.default_input_value(...)`, `.input_value(...)`,
      `.on_input_value_change(...)`.
- [x] `ComboboxRoot<T>` supports `.default_open(bool)`, `.open(bool)`, `.on_open_change(...)`.
- [x] `ComboboxRoot<T>` supports `.disabled(bool)`, `.read_only(bool)`, `.required(bool)`,
      `.name(...)`, each defaulting like Base UI.
- [x] `ComboboxRoot<T>` supports `.open_on_input_click(bool)` (default `true`),
      `.auto_highlight(...)` (off / on-input-change / always; default off),
      `.highlight_item_on_hover(bool)` (default `true`), `.loop_focus(bool)` (default `true`),
      `.limit(...)`, `.filter(...)`, and disabling internal filtering for externally
      filtered lists.
- [x] `ComboboxRoot<T>` supports `.item_to_string_label(...)` and `.item_to_string_value(...)`
      without requiring `T: Display`.
- [x] `ComboboxRoot<T>` supports `.on_item_highlighted(...)` receiving `Option<T>` plus a
      details struct with reason (`Keyboard`/`Pointer`/`None`) and index.
- [x] Runtime/props keep `selection_mode` (incl. `None`), `fill_input_on_item_press`,
      `keep_highlight`, `submit_on_item_click`, the `inline_overlay` state + display-value
      query, and the public input-value query public and configurable for the Autocomplete
      port, while `ComboboxRoot<T>`'s builder hides them (Base UI parity).
- [x] Add public `ComboboxLabel` (focuses the input without opening the popup).
- [x] Add public `ComboboxValue<T>` with `.placeholder(...)` and an optional Rust-native
      formatter closure over the current selection.
- [x] Add public `ComboboxInput<T>` that internally composes `primitives/input::Input`
      (no second text-editing implementation) and supports `.disabled(bool)`.
- [x] Add public `ComboboxInputGroup<T>` wrapping input + adjacent controls (clear, trigger,
      chips); pressing its non-interactive area focuses the input and opens the popup when
      `open_on_input_click` (Base UI `handleInputPress`).
- [x] Add public `ComboboxTrigger<T>` (button that opens/focuses; `.disabled(bool)`).
- [x] Add public `ComboboxList<T>`.
- [x] Add public `ComboboxStatus` (plain container; renders children; no live region).
- [x] Add public `ComboboxEmpty` (renders children only when the filtered list is empty).
- [x] Add public `ComboboxClear` with `.keep_mounted(bool)` or documented presence deferral.
- [x] Add public `ComboboxPortal<T>`, `ComboboxBackdrop`, `ComboboxPositioner<T>`,
      `ComboboxPopup<T>`, `ComboboxArrow` backed by GPUI deferred/anchored overlay rendering,
      implemented Combobox-locally with the Select/Popover layers as references.
- [x] Positioner exposes at least `anchor` override, `side`, `align`, `side_offset`,
      `align_offset`, defaulting the anchor to input-group-else-input.
- [x] Add public `ComboboxIcon`.
- [x] Add public `ComboboxGroup` and `ComboboxGroupLabel`.
- [x] Add public `ComboboxItem<T>` with `.value(T)`, `.label(...)`, `.disabled(bool)`.
- [x] Add public `ComboboxItemIndicator<T>` with `.keep_mounted(bool)`.
- [x] Add public `ComboboxChips<T>`, `ComboboxChip<T>`, `ComboboxChipRemove<T>`.
- [x] Add public `ComboboxCollection<T>` taking items + `Fn(&T, usize) -> ComboboxItem<T>`.
- [x] Add `ComboboxSeparator` reusing `base_gpui::separator::Separator`.
- [x] Stretch: add public `ComboboxRow` and a `.grid(bool)` root flag, or explicitly defer
      grid mode in the issue checklist when the port lands without it.
      *(Grid mode explicitly deferred: no `ComboboxRow`, no `.grid(bool)` in this port.)*
- [x] `combobox/mod.rs` is barrel exports only and exposes all public names.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui combobox` passes.
- [x] `cargo test -p base_gpui` passes.
- [x] `cargo clippy -p base_gpui --all-targets` passes or has documented pre-existing
      warnings only.
- [x] `ast-grep scan crates/base_gpui/src/combobox` reports no scoped-visibility (`pub(...)`)
      violations.
- [x] Demo in `crates/base_gpui/src/main.rs`: single-select combobox with label, input group,
      input, clear, trigger, portal/positioner/popup/list, items with indicators, and empty
      state.
- [x] Demo: multiple-select combobox with chips + chip remove.
- [x] Demo: Field + Combobox integration.

### Architecture / internal model

- [x] Follow `docs/base-gpui-component-architecture.md`: one deep `ComboboxRuntime<T>`
      (runtime.rs), thin `ComboboxContext<T>` (context.rs), thin layers under `layers/`,
      flat module layout — no nested `child/context/{props,runtime,state}/` taxonomy.
- [ ] `ComboboxRuntime<T>` owns all state: selected value(s), input value, query and *(NOTE: satisfied except per-item bounds — not tracked; nothing consumes them without Select's align-item mode)*
      `query_changed_after_open` / frozen close-query, open/mounted state, registered item
      metadata (value, label, disabled, group, index, bounds), filtered index set,
      highlighted index, chip highlight index, measured anchor/popup bounds, focus handles,
      and clear-button visibility facts.
- [x] Filtering is a runtime concern: a command/derivation recomputes the filtered item set
      from registered metadata + query; layers never filter.
- [x] Runtime methods are commands/queries in Combobox domain language (e.g.
      `set_input_value`, `sync_children`, `reconcile`, `select_item`, `toggle_value`,
      `move_highlight`, `remove_chip`, `clear`, `item_state(...)`), not getter/setter pairs;
      highlight questions are part-shaped ("is index highlighted") not state dumps.
- [x] `ComboboxContext<T>` stays thin: `read`/`update` plus the value-, input-value-, and
      open-changing commands that resolve controlled vs uncontrolled and fire props
      callbacks from runtime outcomes. The controlled/uncontrolled rule lives only here (and
      in what is passed to `reconcile`). Three controlled axes: selected value(s), input
      value, open.
- [x] `ComboboxRoot<T>` render is the single non-event mutation site: wire children,
      `sync_children`, `reconcile` all three axes, Field registration.
- [x] Typed child routing before `AnyElement` erasure: `ComboboxChild<T>` in `child.rs`
      covering the documented part set (with an `AnyElement` escape hatch only where Base UI
      examples show arbitrary children — e.g. inside input group, popup, items); nested
      constrained enums for list/group/chips children as needed.
- [x] Child indexing, item metadata extraction, and context attachment live in
      `child_wiring.rs` (private traits), never in public layer APIs; separators, group
      labels, status/empty do not corrupt item indices.
- [x] Item registration works for both composition styles: statically declared
      `ComboboxItem<T>` children and `ComboboxCollection<T>` data-driven items; registered
      metadata is the single registry filtering runs against.
- [x] `ComboboxInput<T>` reuses the `primitives/input` runtime/layer for text editing
      (caret, selection, Home/End, clipboard) and only adds combobox wiring: input value ↔
      runtime sync, open-on-type, highlight reset, chip navigation hand-off, and key
      dispatch for list navigation.
- [x] Keyboard behavior uses `actions.rs` + a Combobox key context on the input/list layers
      (arrows, Enter, Escape); compose with, and do not conflict with,
      `INPUT_KEY_CONTEXT` bindings from the input primitive.
- [x] Runtime is directly unit-testable without a GPUI window.
- [x] No generic popup/collection/filter utilities extracted; Combobox-local only.

### Controlled/uncontrolled: selected value, input value, open state

- [x] Uncontrolled single mode initializes from `.default_value(...)`, default `None`;
      multiple from `.default_values(...)`, default empty `Vec`.
- [x] Controlled `.value(...)` / `.values(...)` are the source of truth; interaction fires
      callbacks without mutating internal selection.
- [x] Uncontrolled input value initializes from `.default_input_value(...)`; when absent in
      single mode it derives from the selected value's label; otherwise empty.
- [x] Controlled `.input_value(...)` is the source of truth; typing fires
      `on_input_value_change` without internal mutation.
- [x] Uncontrolled open initializes from `.default_open(false)`; controlled `.open(...)` is
      the source of truth.
- [x] All three change callbacks fire before uncontrolled mutation, receive Rust-native
      details (reason, source, `cancel()`, `is_canceled()`), and canceling prevents the
      uncontrolled mutation.
- [x] Change reasons cover at least: input-change, input-clear, item-press, trigger-press,
      outside-press, escape-key, list-navigation, focus-out, clear-press, chip-remove-press,
      cancel-open, none.
- [x] Programmatic/controlled selected-value changes update the derived selected index and,
      in single mode with input outside the popup, sync the input text to the new label
      (Base UI `useValueChanged(selectedValue)` sync).
- [x] Disabled/read-only root ignores open/close/select/typing interactions.

### Filtering

- [x] Typing recomputes the filtered item set from the trimmed query against registered item
      labels (default case-insensitive contains).
- [x] Custom `.filter(...)` replaces the default; disabling internal filtering shows all
      registered items so callers can filter externally.
- [x] `item_to_string_label` feeds both filtering and display.
- [x] Single-mode browse bypass: opening with a query exactly matching the selected label
      shows the full list; once the user edits the query after open
      (`query_changed_after_open`), normal filtering applies.
- [x] Closing freezes the active query (close-query) so the list does not flash unfiltered
      while closing; the frozen query and flags reset on unmount/close-complete.
- [x] After close, input-value cleanup follows mode: single syncs input to the selected
      label (or clears it when nothing selected); multiple clears the typed filter.
- [x] Clearing the input to empty in single mode clears the selected value with an
      input-clear reason.
- [x] `.limit(n)` caps the filtered list deterministically.
- [x] Empty filtered list: `ComboboxEmpty` children render; item highlight resets so
      auto-highlight cannot point at index 0 of an empty list.
- [x] Filtered set recomputation preserves highlight on the same item when it survives the
      filter, and clears/re-clamps the highlight when it does not (highlight change reported
      through `on_item_highlighted` exactly once per transition — no duplicate callbacks for
      the same item+index).

### Highlight / keyboard list navigation

- [x] Focus stays on the input while navigating; the highlight is runtime state (virtual
      focus), never focus movement between item elements.
- [x] ArrowDown/ArrowUp on the input opens a closed popup (list-navigation reason) and moves
      the highlight through the filtered items when open.
- [x] `loop_focus == true`: navigation loops through an "input position" (no highlight)
      between last→first, matching Base UI's input-in-the-loop behavior; `false`: clamps at
      the ends.
- [x] `auto_highlight` off: typing clears the highlight; on-input-change: typed queries
      highlight the first match and keep the highlight while the query changes; always: first
      item is highlighted whenever the list opens/changes.
- [x] `highlight_item_on_hover == true`: pointer hover highlights (reason pointer); `false`:
      hover does not change highlight; pointer leaving the list clears the highlight unless
      `keep_highlight`.
- [x] Home/End edit the input caret (input primitive behavior), and do not jump the list
      highlight.
- [x] Printable keys always type into the input — no open-list typeahead.
- [x] Enter with a highlighted item selects it; Enter with no highlight closes the popup
      (and remains the future form-submit hook).
- [x] Escape while open closes the popup; Escape while closed (input focused, value or
      input non-empty) clears the input and the selection with an escape-key reason.
- [x] Keyboard highlight movement skips nothing it shouldn't: disabled items match Base UI's
      roving behavior (highlightable, activation no-op); group labels/separators/status/empty
      are never highlighted.
- [x] Highlight movement scrolls the highlighted item into view (Select
      `scroll_highlighted_into_view` precedent).
- [x] `on_item_highlighted` fires with the highlighted value and reason on every highlight
      transition, and with `None` when the highlight clears.

### Item selection: single vs multiple, chips

- [x] Pointer press on an enabled item in single mode selects it, closes the popup
      (item-press reason), and fills the input with the item label when the input is outside
      the popup.
- [x] Selecting in multiple mode toggles membership in the ordered `Vec<T>` (append on
      select, remove on deselect), does not close the popup, and never duplicates values.
- [x] In multiple mode, selecting while a typed filter is active clears the filter query so
      the full list is visible again (Base UI wasFiltering behavior for the first-port
      topology).
- [x] Disabled items and read-only/disabled roots make activation a no-op (pointer and
      Enter).
- [x] The press-selection path is one runtime command shared by pointer and Enter so
      callbacks/reasons stay consistent.
- [x] `ComboboxItemIndicator<T>` renders only when its item is selected unless
      `keep_mounted`.
- [x] `ComboboxChips<T>` renders one `ComboboxChip<T>` per selected value in selection
      order; chips compose as plain layers over runtime `Vec<T>` state.
- [ ] Chip keyboard model: from the input with caret at start, ArrowLeft (RTL-aware: *(NOTE: implemented via input caret-edge hooks and key-down observation; RTL-aware inline-start mapping not implemented — physical Left/Right only)*
      inline-start) highlights the last chip; ArrowLeft/ArrowRight move between chips and
      return to the input past the ends; Backspace/Delete on a highlighted chip removes that
      value and moves the highlight to the adjacent chip (or back to the input); Enter/Space/
      printable characters return focus to the input; ArrowDown/ArrowUp from a chip opens
      the popup.
- [x] Backspace in an empty input with no chip highlighted removes the last selected value.
- [x] `ComboboxChipRemove<T>` removes its chip's value (chip-remove-press reason), refocuses
      the input, and clears the list highlight if the removed item was highlighted.
- [x] `ComboboxClear` is visible only when there is something to clear (single: value
      present; multiple: non-empty; None mode: input non-empty); pressing it clears input +
      selection + highlight (clear-press reason) and refocuses the input without opening the
      popup.
- [x] `ComboboxValue<T>` shows placeholder when nothing is selected, the selected label in
      single mode, and joined labels (or a caller formatter) in multiple mode.
- [x] Dynamic item removal/re-add reconciles selection deterministically (Select fallback
      rules as precedent); duplicate item values behave deterministically and are
      documented.

### Positioning / portal / dismissal

- [x] `ComboboxPortal` renders overlay content only while open/mounted (or force-mounted for
      measurement) through GPUI deferred/anchored rendering.
- [x] Clicking/pressing the input (when `open_on_input_click`) opens the popup without
      toggling it closed; typing a non-empty query opens it; the trigger button toggles
      open/close and focuses the input on non-touch press.
- [x] The positioner anchors to the input group when present, else the input, with an
      explicit `.anchor(...)` override; anchor bounds are measured through GPUI
      layout/prepaint hooks, and anchor width is exposed so the popup can match the input
      width (Base UI CSS-var equivalent as typed style state).
- [x] Side/align + offsets map to GPUI anchored semantics; viewport clamping via
      `snap_to_window_with_margin(...)`; collision subset documented as in Select.
- [ ] Outside press dismisses (outside-press reason) but presses on the trigger, clear *(NOTE: input-group/trigger/clear/chips presses re-open or act after the positioner's mouse-down-out close; strict exclusion not verified in a rendered test)*
      button, chips container, or input group do not count as outside.
- [x] Focus leaving input+popup closes with focus-out reason and marks the Field touched.
- [x] Escape closes and keeps focus on the input.
- [x] While open the input keeps keyboard focus; opening never steals focus into the popup.
- [x] `ComboboxBackdrop` and `ComboboxArrow` render with open/side/align style state
      (popover layer precedents); no scroll-lock/modal inertness in the first port.
- [x] List scrolling uses a runtime-owned scroll handle; long lists remain usable; the
      virtualized stretch item uses gpui `uniform_list` with indices mapped to the filtered
      set, or is explicitly deferred.

### Field / Form integration

- [x] Combobox inside `FieldRoot` registers one representative control (the input) with
      stable key, name, value, disabled, focused, required, value-missing facts, and focus
      handle.
- [x] Field/Fieldset disabled state combines with Combobox disabled state; name precedence
      follows Field rules.
- [x] Filled: single = value selected; multiple = non-empty list (None mode, for
      Autocomplete later: input non-empty).
- [x] Dirty compares current selection (and input value in None mode) against initial;
      touched sets on blur/focus-out, not when focus moves within the combobox; focused is
      true while input or popup interaction is inside the combobox.
- [x] Required: single missing when `None`; multiple missing when empty.
- [ ] Accepted value changes clear matching external Form errors and run `OnChange` *(NOTE: relies on Field's registration-driven validation like Select; not covered by a Combobox rendered test yet)*
      validation; `OnBlur` validates on focus-out; `OnSubmit` validates on Form submit.
- [x] Serialization: single registers `FieldValue::Text(serialized)` via
      `item_to_string_value` (or documented `Present`/`Empty`); multiple registers
      `FieldValue::List(...)`; None mode (for the Autocomplete port) registers
      `FieldValue::Text(input_value)`; named fields contribute deterministic `FormValue`
      entries; disabled fields are skipped.
- [x] Invalid Form submit focuses the Combobox input through the registered focus handle.

### Styling / state exposure

- [x] `ComboboxRootStyleState`: open, disabled, read-only, required, selection mode,
      value-present, input-non-empty, list-empty, plus Field facts (focused, filled, dirty,
      touched, valid).
- [x] `ComboboxInputStyleState`: open, disabled, read-only, popup side, list-empty, plus
      Field facts (Base UI input state = FieldRootState + open/popupSide/listEmpty/readOnly);
      composes with, not replaces, the inner `InputStyleState` styling hook.
- [x] `ComboboxInputGroupStyleState` and `ComboboxTriggerStyleState`: open, disabled,
      read-only, popup side, list-empty, placeholder (no selected value), plus Field facts.
- [x] `ComboboxListStyleState`: empty.
- [x] `ComboboxPositionerStyleState`: open, side, align, anchor width/measured sizes,
      anchor-hidden if supported, empty.
- [x] `ComboboxPopupStyleState`: open, side, align, empty (transition status deferred with
      the shared follow-up).
- [x] `ComboboxBackdropStyleState`: open/mounted.
- [x] `ComboboxArrowStyleState`: open, side, align, uncentered.
- [x] `ComboboxItemStyleState`: selected, highlighted, disabled, index.
- [x] `ComboboxItemIndicatorStyleState`: selected.
- [x] `ComboboxChipStyleState`: highlighted, disabled, read-only, index;
      `ComboboxChipRemoveStyleState`: disabled.
- [x] `ComboboxClearStyleState`: visible, disabled, open.
- [x] `ComboboxIconStyleState`: open.
- [x] Empty-or-useful style states for label, value, status, empty, group, group label,
      separator; every part Base UI exposes state for has `style_with_state(...)`.
- [x] Style-state structs live in `style_state.rs`; no DOM data attributes, CSS classes, or
      CSS variables anywhere in the public surface.

### Tests / verification

Runtime tests (no window) under `crates/base_gpui/src/combobox/tests/`:

- [x] Uncontrolled/controlled reconciliation for all three axes (selected value(s), input
      value, open) including cancellation.
- [x] Default input value derivation from the selected label in single mode.
- [x] Filtering: default contains match, custom filter, filter disabled, limit,
      single-mode browse bypass and `query_changed_after_open` transition, close-query
      freeze/reset.
- [x] Highlight: move next/previous with and without loop, auto-highlight modes, highlight
      preservation/clamping across filter changes, disabled-item behavior, hover vs
      keep-highlight.
- [x] Selection: single select outcome (close + fill input), multiple toggle
      (order, no duplicates, stays open, filter clear), Backspace-removes-last,
      chip-remove highlight cleanup, clear command, Escape-while-closed clear.
- [ ] Item registration order with groups/separators interleaved; collection-driven *(NOTE: order + group registration unit-tested; collection-driven registration requires a rendered test — not written)*
      registration; dynamic removal/re-add reconciliation.
- [x] `on_item_highlighted` fires exactly once per transition with correct reason/index.

Rendered tests: *(none written in this pass — runtime behavior is unit-tested; rendered/window tests deferred)*

- [ ] Typing filters the visible items and opens the popup.
- [ ] ArrowDown opens and highlights; Enter selects; single mode closes and fills the input.
- [ ] Multiple mode: item click toggles, popup stays open, chips render in order; chip
      keyboard navigation and removal; ChipRemove click.
- [ ] Escape closes; Escape while closed clears.
- [ ] Outside press closes without selecting; trigger/clear/chips presses do not count as
      outside.
- [ ] Clear button visibility and behavior.
- [ ] Empty part renders children only when the filtered list is empty.
- [ ] Placeholder vs selected label(s) display through `ComboboxValue`.
- [ ] Item indicator mount/keep-mounted.
- [ ] Group labels/separators do not affect item indices.
- [ ] Basic positioner measurement state (anchor width available to the popup).
- [ ] Field integration: filled, dirty, touched, focused, required, validation modes.
- [ ] Form integration: submitted values (single text, multiple list), external error
      clearing, disabled skip, invalid-submit focus.
- [ ] Demos render in `crates/base_gpui/src/main.rs` without panics. *(NOTE: demos added; not executed headlessly in this pass)*

## Stretch scope (explicitly optional in the first port)

- [ ] Grid mode: `.grid(bool)` + `ComboboxRow`, with row/column highlight navigation
      (ArrowLeft/Right move within rows only when an item is highlighted, never while the
      caret is editing the query).
- [ ] Virtualized long lists via gpui `uniform_list` over the filtered set (`virtualized`
      prop analog); no new primitive.
- [ ] Closed-trigger typeahead in single mode (Base UI `useTypeahead` on the trigger:
      typing while closed selects the matching item).

## Follow-ups to track explicitly if not completed in the first port

- [ ] Autocomplete port as a thin layer over `ComboboxRuntime<T>` with
      *(Seams ready: `ComboboxSelectionMode::None`, `ComboboxProps.fill_input_on_item_press` /
      `keep_highlight` / `submit_on_item_click`, `ComboboxRuntime::set_inline_overlay` +
      `display_value()` (whole-value overlay via input `sync_props`), and the read-only
      `input_value()` query.)*
      `selection_mode = None` (`issues/port-baseui-autocomplete.md`, already written —
      keep the cross-link to the runtime-knob criterion above in sync).
- [ ] Inline autocompletion (`autoComplete: 'both' | 'inline'`): highlighted item label
      temporarily previewed in the input via the `inline_overlay` runtime state and
      display-value query above. This is a **whole-value** overlay written through the
      existing `sync_props` controlled-value path — it does **not** require input-primitive
      selection-range support (Base UI overwrites the entire visible value, not a suffix).
- [ ] Input-inside-popup topology (button-only trigger with the input rendered in the
      popup) and `inline` (popupless) list mode.
- [ ] `modal` open state, scroll locking, and outside-interaction inertness.
- [ ] Transition/presence status for popup/backdrop/clear/indicator (shared infrastructure
      follow-up, same as Select/Popover).
- [ ] Locale-aware filtering (collator analog) if simple case-insensitive contains proves
      insufficient.
- [ ] Custom value comparator if `T: Eq` is not enough for real callers.
- [ ] `submit_on_item_click` wired to a real Form submit command when Form exposes one.
- [ ] AccessKit semantics (combobox/listbox/option/grid roles, active-descendant
      equivalent, live-region status/empty announcements) once the pinned GPUI revision
      exposes the needed APIs — deferred, consistent with Select's audit.

## Uncertain items needing confirmation

- Whether `ComboboxValue<T>` is worth porting in the first pass given GPUI composition
  (Base UI renders no element; it exists mainly for trigger-button display in the
  input-inside-popup topology). Default: port it as a thin display part as scoped above.
- Whether the `loop_focus` "input position in the loop" model should be modeled as
  `highlight = None` between ends (proposed) or as a distinct input-slot index.
- Whether `ComboboxCollection<T>` should live in the first port or be deferred until the
  virtualized stretch item needs it; default: include, since Base UI's `items`-driven mode
  and the Empty part's semantics lean on a known item set.
- Exact composition rule between the Combobox key context and `INPUT_KEY_CONTEXT` (which
  layer claims ArrowUp/ArrowDown/Enter/Escape first) — decide during implementation and
  document in `actions.rs`.
