# Port Base UI Select to GPUI

## Problem

Base UI Select provides a dropdown/listbox form control with controlled and uncontrolled selected value, controlled and uncontrolled popup open state, item registration, highlighted vs selected item state, keyboard navigation, typeahead, disabled/read-only/required behavior, optional multiple selection, field/form integration, item/value label resolution, grouping, indicators, scroll arrows, and special popup positioning that can align the selected item text with the trigger value text.

`crates/base_gpui` currently has the lower-level form stack needed for this port (`Field`, `Form`, `Input`, `NumberField`, Checkbox/Switch/Radio Group, Fieldset, and direction utilities), but no Select component family and no shared popup/positioner primitive. The goal is to port Select behavior into GPUI-native components using the established `base_gpui` runtime/context/layers architecture, not to copy React hooks, DOM refs, hidden inputs, Floating UI internals, ARIA attributes, CSS variables, or browser form mechanics literally.

Select item values should use Rust-native generic values, constrained around clone/equality semantics such as `T: Clone + Eq + 'static`. Multiple selection should use an ordered `Vec<T>` model rather than Base UI's JavaScript `Value[] | Value | null` union.

## Scope

Port the Select component family from Base UI into GPUI-native components:

- `SelectRoot<T>`
- `SelectLabel`
- `SelectTrigger<T>`
- `SelectValue<T>`
- `SelectIcon`
- `SelectPortal`
- `SelectBackdrop`
- `SelectPositioner<T>`
- `SelectPopup<T>`
- `SelectList<T>`
- `SelectItem<T>`
- `SelectItemText`
- `SelectItemIndicator<T>`
- `SelectArrow`
- `SelectScrollUpArrow`
- `SelectScrollDownArrow`
- `SelectGroup`
- `SelectGroupLabel`
- `SelectSeparator` backed by the shared Base UI Separator behavior

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/select/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/root/SelectRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/root/SelectRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/root/SelectRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/root/SelectRoot.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/store.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/label/SelectLabel.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/trigger/SelectTrigger.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/trigger/SelectTriggerDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/value/SelectValue.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/value/SelectValueDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/icon/SelectIcon.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/icon/SelectIconDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/portal/SelectPortal.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/portal/SelectPortalContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/backdrop/SelectBackdrop.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/backdrop/SelectBackdropDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/positioner/SelectPositioner.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/positioner/SelectPositionerContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/positioner/SelectPositionerDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/positioner/SelectPositionerCssVars.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/popup/SelectPopup.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/popup/SelectPopupDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/popup/utils.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/list/SelectList.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/item/SelectItem.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/item/SelectItemContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/item/SelectItemDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/item-text/SelectItemText.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/item-indicator/SelectItemIndicator.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/item-indicator/SelectItemIndicatorDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/arrow/SelectArrow.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/arrow/SelectArrowDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/scroll-arrow/SelectScrollArrow.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/scroll-up-arrow/SelectScrollUpArrow.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/scroll-up-arrow/SelectScrollUpArrowDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/scroll-down-arrow/SelectScrollDownArrow.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/scroll-down-arrow/SelectScrollDownArrowDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/group/SelectGroup.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/group/SelectGroupContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/select/group-label/SelectGroupLabel.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/select/*.test.tsx` and `select/*/*.test.tsx` for behavior coverage
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/select/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/select/types.md`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/select/demos/**/index.tsx`

Current GPUI implementation:

- No `crates/base_gpui/src/select/` module exists yet.
- No shared popup/positioner/portal primitive exists in `crates/base_gpui` yet.
- Shared `crates/base_gpui/src/separator/` exists and should back `SelectSeparator` / `Select.Separator` behavior.
- GPUI/Zed local precedents for overlays exist and should inform the implementation:
  - `/home/luke/Projects/zed/crates/gpui/examples/popover.rs`
  - `/home/luke/Projects/zed/crates/ui/src/components/popover_menu.rs`
  - `/home/luke/Projects/zed/crates/ui/src/components/dropdown_menu.rs`
  - `/home/luke/Projects/zed/crates/ui/src/components/right_click_menu.rs`
  - `/home/luke/Projects/gpui-component/crates/ui/src/popover.rs`
  - `/home/luke/Projects/gpui-component/crates/ui/src/select.rs`
- `Field`, `Form`, `Fieldset`, `RadioGroup`, `CheckboxGroup`, `Tabs`, `NumberField`, and `utils::direction` provide useful local precedents.

Expected GPUI implementation files:

```text
crates/base_gpui/src/select/mod.rs
crates/base_gpui/src/select/actions.rs
crates/base_gpui/src/select/child.rs
crates/base_gpui/src/select/child_wiring.rs
crates/base_gpui/src/select/context.rs
crates/base_gpui/src/select/props.rs
crates/base_gpui/src/select/style_state.rs
crates/base_gpui/src/select/runtime.rs
crates/base_gpui/src/select/value.rs                  # optional helper types for single/multiple selection
crates/base_gpui/src/select/layers/mod.rs
crates/base_gpui/src/select/layers/select_root.rs
crates/base_gpui/src/select/layers/select_label.rs
crates/base_gpui/src/select/layers/select_trigger.rs
crates/base_gpui/src/select/layers/select_value.rs
crates/base_gpui/src/select/layers/select_icon.rs
crates/base_gpui/src/select/layers/select_portal.rs
crates/base_gpui/src/select/layers/select_backdrop.rs
crates/base_gpui/src/select/layers/select_positioner.rs
crates/base_gpui/src/select/layers/select_popup.rs
crates/base_gpui/src/select/layers/select_list.rs
crates/base_gpui/src/select/layers/select_item.rs
crates/base_gpui/src/select/layers/select_item_text.rs
crates/base_gpui/src/select/layers/select_item_indicator.rs
crates/base_gpui/src/select/layers/select_arrow.rs
crates/base_gpui/src/select/layers/select_scroll_up_arrow.rs
crates/base_gpui/src/select/layers/select_scroll_down_arrow.rs
crates/base_gpui/src/select/layers/select_group.rs
crates/base_gpui/src/select/layers/select_group_label.rs
crates/base_gpui/src/select/layers/select_separator.rs # or crates/base_gpui/src/separator/ if made shared first
crates/base_gpui/src/select/tests/
```

Alternative filenames are fine if they preserve the established architecture: one deep `SelectRuntime<T>`, thin `SelectContext<T>`, typed child wiring before `AnyElement` erasure, typed style-state structs, and renderable layers only under `layers/`.

## Initial design decisions

### Public value model

Use `SelectRoot<T>::multiple(bool)` to mirror Base UI's `multiple` prop. The Rust-specific API decision is not whether Select supports a multiple prop; it is how the statically typed value builders expose the two value shapes cleanly:

- single-select value: `Option<T>`;
- multiple-select value: `Vec<T>`.

Use the pragmatic first-port API:

- `.default_value(Option<T>)`
- `.value(Option<T>)`
- `.on_value_change(...)`
- `.multiple(bool)`
- `.default_values(Vec<T>)`
- `.values(Vec<T>)`
- `.on_values_change(...)`

Runtime state may use an internal enum such as `SelectSelection<T>` so the root can reconcile both modes in one `SelectRuntime<T>`. If `.multiple(true)` is set, the list-valued builders/callback are authoritative; if `.multiple(false)`, the single-valued builders/callback are authoritative. Document deterministic behavior when callers provide both single and multiple value props.

### Equality and labels

Use `T: Clone + Eq + 'static` as the default item-value constraint. Do not port arbitrary JavaScript equality semantics. If custom equality is needed later, add a Rust-native comparator intentionally; do not include it just to mirror `isItemEqualToValue`.

`SelectItem<T>` should register both its value and a text label. The label can come from `SelectItem::label(...)`, `SelectItemText`, or a root-level label resolver. `SelectValue<T>` should display the selected item's registered/resolved label, not require `T: Display` by default.

For form values, add an explicit serializer such as `.item_to_string_value(...)` / `.value_serializer(...)`. Do not use `Debug` formatting as hidden behavior.

### Popup/positioner scope

Base UI Select has a large Floating UI positioning surface. GPUI and local component libraries already provide overlay precedents through `deferred(anchored().child(...))`, `snap_to_window_with_margin(...)`, trigger-bound measurement, outside mouse-down dismissal, and next-frame focus handoff. The first Select port should use those GPUI-native patterns rather than treating `SelectPortal` as a no-op.

The first GPUI port should implement a useful subset and keep the API open for future parity:

- render popup/list content through a deferred anchored layer;
- capture trigger bounds through `on_prepaint` or custom element prepaint state;
- position against the trigger with GPUI `anchored()` and `Anchor` where possible;
- support `side`, `align`, `side_offset`, and `align_offset` by mapping them to anchor/attach/offset semantics;
- use `.snap_to_window_with_margin(...)` for first-pass viewport clamping;
- use `.on_mouse_down_out(...)` or window mouse-event handling for outside dismissal;
- focus popup/list content after the deferred layer is linked, using a next-frame focus handoff when needed;
- expose style-state values for side, align, anchor bounds, available size, and whether align-item mode is active.

Do not block the entire Select port on perfect Floating UI collision middleware, browser scroll lock, or exact Base UI touch heuristics.

## Out of scope / drop from Base UI

- Do not port React hooks/context/store implementation details; use GPUI keyed state/entities plus `SelectContext<T>`.
- Do not port `render` props.
- Do not port `className`.
- Do not port web `style` props; use normal GPUI styling builders and `style_with_state(...)`.
- Do not port `nativeButton` options.
- Do not port SSR, hydration, CSP nonce, or prehydration behavior.
- Do not port hidden DOM inputs, browser autofill, `autoComplete`, `form` attributes, or native `FormData` mechanics literally.
- Do not port DOM event objects, `allowPropagation`, or browser propagation APIs. Use Rust-native details structs with reason/source/cancel state.
- Do not port DOM `id`, `aria-*`, `role`, `aria-controls`, `aria-labelledby`, or `aria-activedescendant` literally. Revisit GPUI AccessKit support separately.
- Do not port CSS variable APIs such as `--anchor-width`, `--available-height`, or `--transform-origin`; expose typed style-state values instead.
- Do not port DOM data attributes as attributes; map Base UI state attributes into style-state structs.
- Do not port `ResizeObserver`, `getBoundingClientRect`, or Floating UI middleware directly; use GPUI layout/prepaint measurement.
- Do not port browser scroll locking, inert outside DOM trees, or touch-specific viewport heuristics literally.
- Do not port arbitrary JavaScript value/object semantics; use Rust values and explicit label/serializer callbacks.
- Do not introduce a generic popup utility before Select reveals a stable repeated concept. Keep Select-local positioning code under `crates/base_gpui/src/select/` unless a later component justifies extraction.
- Do not use Rust scoped visibility syntax (`pub(...)`); `ast-grep scan` must remain clean.

## Acceptance Criteria

### Module/API surface

- [x] Add a top-level `select` module and export it from `crates/base_gpui/src/lib.rs`.
- [x] Register Select key bindings from `base_gpui::init(cx)`.
- [x] Add public `SelectRoot<T>` with `SelectRoot::new()`.
- [x] Add public `SelectLabel`.
- [x] Add public `SelectTrigger<T>`.
- [x] Add public `SelectValue<T>`.
- [x] Add public `SelectIcon`.
- [x] Add public `SelectPortal` backed by GPUI deferred/anchored overlay rendering, following local Zed/gpui-component precedents.
- [x] Add public `SelectBackdrop`.
- [x] Add public `SelectPositioner<T>`.
- [x] Add public `SelectPopup<T>`.
- [x] Add public `SelectList<T>`.
- [x] Add public `SelectItem<T>`.
- [x] Add public `SelectItemText`.
- [x] Add public `SelectItemIndicator<T>`.
- [x] Add public `SelectArrow`.
- [x] Add public `SelectScrollUpArrow`.
- [x] Add public `SelectScrollDownArrow`.
- [x] Add public `SelectGroup`.
- [x] Add public `SelectGroupLabel`.
- [x] Add `SelectSeparator` by reusing `base_gpui::separator::Separator`.
- [x] `SelectRoot<T>` supports `.id(...)` as stable keyed identity.
- [x] `SelectRoot<T>` supports uncontrolled `.default_value(Option<T>)` for single select.
- [x] `SelectRoot<T>` supports controlled `.value(Option<T>)` for single select.
- [x] `SelectRoot<T>` supports `.on_value_change(...)` with Rust-native cancelable details.
- [x] `SelectRoot<T>` supports uncontrolled `.default_open(bool)`.
- [x] `SelectRoot<T>` supports controlled `.open(bool)`.
- [x] `SelectRoot<T>` supports `.on_open_change(...)` with Rust-native cancelable details.
- [x] `SelectRoot<T>` supports `.disabled(bool)`, defaulting to `false`.
- [x] `SelectRoot<T>` supports `.read_only(bool)`, defaulting to `false`.
- [x] `SelectRoot<T>` supports `.required(bool)`, defaulting to `false`.
- [x] `SelectRoot<T>` supports `.name(...)` metadata for Field/Form integration.
- [x] `SelectRoot<T>` supports `.multiple(bool)` or a clearer Rust-native multiple-select API.
- [x] `SelectRoot<T>` supports `.highlight_item_on_hover(bool)`, defaulting to `true`.
- [x] `SelectRoot<T>` supports a label resolver and value serializer without requiring `T: Display`.
- [x] `SelectValue<T>` supports `.placeholder(...)`.
- [x] `SelectItem<T>` supports `.value(T)`, `.label(...)`, and `.disabled(bool)`.
- [x] `SelectItemIndicator<T>` supports `.keep_mounted(bool)` or explicitly defers transition/presence support.
- [x] Positioner exposes at least `side`, `align`, `side_offset`, `align_offset`, and `align_item_with_trigger`, mapping the first four to GPUI anchor/attach/offset behavior where possible.
- [x] `select/mod.rs` exposes ergonomic barrel exports for all public names.

### Correctness / compile readiness

Implementation pass 1 completed a functional first GPUI Select port with the shared Separator integration, keyed runtime/context/child-wiring architecture, deferred/anchored popup positioning, single and multiple value models, Field/Form registration, demos, runtime tests, and core rendered interaction tests. Remaining unchecked items are mostly exact positioning/measurement parity, typeahead key-event integration, scroll-arrow behavior, Select-specific Field/Form rendered tests, and advanced follow-ups.

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui select` passes.
- [x] `cargo test -p base_gpui field form` passes after Select Field/Form integration. Verified as `cargo test -p base_gpui field` plus `cargo test -p base_gpui form` because Cargo accepts one test filter.
- [x] `cargo test -p base_gpui` passes.
- [x] `cargo clippy -p base_gpui --all-targets` passes or has documented pre-existing warnings only. The command exits successfully; remaining warnings are pre-existing outside `select`.
- [x] `ast-grep scan crates/base_gpui/src/select crates/base_gpui/src/field crates/base_gpui/src/form` passes or produces no scoped-visibility violations.
- [x] Add a small Select demo to `crates/base_gpui/src/main.rs` showing trigger, value, popup/list, items, and item indicator.
- [x] Add a Field + Select demo.
- [x] Add a multiple Select demo if multiple selection is implemented in the first port.

### Architecture / internal model

- [x] Follow `docs/base-gpui-component-architecture.md`: one deep `SelectRuntime<T>`, thin `SelectContext<T>`, thin render layers.
- [x] Keep selected value(s), open state, mounted/force-mounted state, item metadata, label metadata, highlight state, focus handles, measured bounds, scroll-arrow visibility, and fallback behavior in `SelectRuntime<T>`.
- [x] Keep `SelectContext<T>` as read/update plus value/open-changing commands; do not leak runtime internals to layers.
- [x] `SelectRoot<T>` is the single non-event mutation site for child synchronization, controlled/uncontrolled reconciliation, and Field registration.
- [x] Use typed child routing before `AnyElement` erasure.
- [x] Preserve arbitrary visual children inside parts where Base UI examples rely on it, e.g. trigger contents, item contents, labels, and icons.
- [x] Keep child indexing and metadata extraction in `child_wiring.rs`, not in public layer APIs.
- [x] Runtime methods are commands/queries in Select domain language, not getter/setter pairs.
- [x] Runtime has direct unit tests that do not require a GPUI window.
- [x] Layers translate GPUI events/actions into runtime/context commands and style from style-state structs only.
- [x] Do not add generic popup/collection utilities unless Select plus another component prove the abstraction is deep and repeated.

### Open state behavior

- [x] Uncontrolled Select initializes popup open state from `.default_open(false)`.
- [x] Controlled Select reflects `.open(...)` as the source of truth.
- [x] Trigger pointer activation opens a closed popup when not disabled/read-only.
- [x] Trigger activation closes an open popup when appropriate.
- [x] Disabled Select ignores open/close user interaction.
- [x] Read-only Select does not open from trigger pointer or keyboard interaction.
- [x] `on_open_change` fires before uncontrolled open-state mutation.
- [x] Canceling `on_open_change` prevents uncontrolled open-state mutation.
- [x] Controlled open state calls `on_open_change` but never mutates internal open state as the source of truth.
- [x] Closing by item selection uses reason/source equivalent to Base UI `item-press`.
- [x] Closing by Escape uses reason/source equivalent to `escape-key`.
- [x] Closing by focus leaving the Select uses reason/source equivalent to `focus-out`.
- [x] Closing by outside pointer press uses reason/source equivalent to `outside-press` if GPUI can observe outside presses.
- [x] Closing by window resize/layout invalidation is supported or explicitly deferred. Deferred to the Select positioning follow-up because current GPUI anchored/deferred positioning does not expose the required layout invalidation hook.
- [x] Open transition/mounted state supports popup rendering during close animation if shared transition infrastructure exists; otherwise document transition status as a follow-up. Deferred to the shared transition/presence follow-up below.

### Single selection behavior

- [x] Uncontrolled single Select initializes selected value from `.default_value(...)`, defaulting to `None`.
- [x] Controlled single Select reflects `.value(...)` as the source of truth.
- [x] `None` means no selected value; callers can model a selectable null-like item with `T = Option<U>` if needed.
- [x] Clicking/activating an enabled item selects that item.
- [x] Selecting an item in single-select mode closes the popup.
- [x] Selecting the already selected item is deterministic and does not duplicate callbacks unnecessarily.
- [x] Disabled items are never selected by pointer, keyboard, or typeahead commit.
- [x] Read-only root prevents value changes.
- [x] Disabled root prevents value changes.
- [x] `on_value_change` fires before uncontrolled selected-value mutation.
- [x] Canceling `on_value_change` prevents uncontrolled selected-value mutation.
- [x] Controlled value changes call `on_value_change` but never mutate internal selected value as the source of truth.
- [x] Value change details include at least reason, source, cancelability, `cancel()`, and `is_canceled()`.
- [x] Value change sources distinguish pointer, keyboard, typeahead/list navigation, programmatic/automatic fallback, and unknown/none where useful.
- [x] Do not expose browser DOM event objects in value-change details.

### Multiple selection behavior

- [x] Multiple Select initializes uncontrolled selected values from `.default_values(...)`, defaulting to an empty ordered list.
- [x] Controlled multiple Select reflects `.values(...)` or `SelectSelection::Multiple(...)` as the source of truth.
- [x] Clicking/activating an unselected enabled item appends its value to the ordered selected list.
- [x] Clicking/activating a selected enabled item removes its value from the selected list.
- [x] Multiple Select does not close the popup when selecting/deselecting an item.
- [x] Re-selecting an already selected value does not create duplicates.
- [x] Selection order remains deterministic and matches user selection order unless a different order is documented.
- [x] Disabled selected items remain selected unless explicitly removed by controlled value changes or dynamic item cleanup rules.
- [x] Multiple value changes support cancelable callbacks before uncontrolled mutation.
- [x] Multiple selected values are serialized to `FieldValue::List(Vec<SharedString>)` / `FormValue::List(Vec<SharedString>)` when a serializer is supplied.
- [x] Required validation treats an empty selected list as missing and a non-empty list as filled.

### Item registration, dynamic items, and fallback

- [x] Root/positioner/list child wiring registers item metadata in render order.
- [x] Metadata includes value, label, disabled state, group association, index, focus handle, item text bounds, and item bounds where needed.
- [x] Highlighted index is separate from selected value.
- [x] Selected index is derived from selected value and current item registry.
- [x] Opening the popup highlights/focuses the selected item when present.
- [x] Opening with no selected item highlights the first enabled/selectable item when appropriate.
- [x] Dynamic item removal reconciles uncontrolled single selection: if the selected value disappears, fall back to the initial default value when still present, otherwise `None`.
- [x] Dynamic item removal in controlled single mode calls the change handler with the fallback candidate but does not override the controlled value.
- [x] Dynamic item removal in multiple mode removes selected values that no longer have registered items, unless controlled mode preserves external values by design.
- [x] Re-adding items after removal restores correct selected/highlighted state.
- [x] Duplicate item values behave deterministically and are documented.
- [x] Items with missing values either fail clearly at construction/test time or behave as a documented null-like value model.

### Keyboard, focus, and typeahead behavior

- [x] Select uses GPUI key dispatch actions and a Select key context for arrow keys, Enter, Space, Escape, Home, and End where possible.
- [x] Trigger is focusable when enabled and skipped when disabled.
- [x] Label click/focus behavior focuses the trigger without opening the popup.
- [x] Field label integration focuses the trigger.
- [x] Opening from keyboard focuses/highlights the selected item or the first selectable item.
- [x] ArrowDown/ArrowUp move highlight through registered items while the popup is open.
- [x] Home moves highlight to the first item.
- [x] End moves highlight to the last item.
- [x] Keyboard navigation skips non-item children such as groups, labels, separators, and null placeholders that are not selectable.
- [x] Disabled items can receive highlight/focus if matching Base UI's roving behavior, but activation remains a no-op.
- [x] Enter selects the highlighted item.
- [x] Space selects the highlighted item unless it is part of an active typeahead sequence.
- [x] Escape closes the popup and returns focus according to popup focus rules.
- [x] Focus leaving the popup/trigger marks the Field as touched and closes when appropriate.
- [x] Typeahead while the popup is open moves highlight to the next matching enabled item.
- [x] Typeahead while the popup is closed commits the matching item in single-select mode.
- [x] Typeahead is disabled or carefully defined in multiple-select mode.
- [x] Typeahead uses item labels, not serialized values, and skips disabled items.
- [x] Repeated typeahead characters cycle through matching enabled items.
- [x] Typeahead state resets after a timeout and after explicit value reset.
- [x] Keyboard highlight movement scrolls the highlighted item into view if popup scrolling is implemented.

### Pointer behavior

- [x] Pointer hover highlights items when `highlight_item_on_hover == true`.
- [x] Pointer hover does not change highlight when `highlight_item_on_hover == false`.
- [x] Pointer activation selects enabled items.
- [x] Pointer activation of disabled items is a no-op.
- [x] Pointer activation of items respects root disabled/read-only state.
- [x] Opening the popup under the pointer does not immediately select the item under the cursor by accident.
- [x] Drag-to-select / mouse-up selection behavior is either implemented to match Base UI or explicitly deferred. Deferred until GPUI pointer-drag semantics are normalized across collection widgets.
- [x] Touch-specific behavior is either implemented through GPUI pointer metadata or explicitly deferred. GPUI has no pointer-type metadata on mouse press events, so the port exposes touch open/change sources and disables align-item/scroll-arrow behavior for touch-driven opens when that source is used.

### Popup, positioning, and scrolling behavior

- [x] `SelectPortal` renders popup content only while mounted/open/force-mounted through GPUI deferred/anchored overlay rendering.
- [x] `SelectPositioner` measures trigger bounds and popup/list bounds through GPUI-native layout/prepaint hooks.
- [x] `SelectPositioner` stores measured anchor width/height and available width/height in runtime/style state.
- [x] `SelectPopup` renders only when mounted/open unless force-mounted for measurement.
- [x] Basic side/align positioning works relative to the trigger.
- [x] Side and align offsets are supported if included in the public API.
- [x] Collision avoidance is implemented for the practical subset or documented as a follow-up.
- [x] `align_item_with_trigger` aligns selected item text with trigger value text when practical.
- [x] `align_item_with_trigger` automatically falls back when there is not enough vertical space or when unsupported.
- [x] Direction-aware alignment uses `utils::direction` for LTR/RTL inline start/end behavior.
- [x] Popup style state exposes `open`, `side`, `align`, and transition status if supported.
- [x] Positioner style state exposes `open`, `side`, `align`, anchor-hidden/anchor-available state if supported, and measured sizes.
- [x] Backdrop style state exposes open/closed and transition status if supported.
- [x] Arrow style state exposes open, side, align, and uncentered status; arrow is omitted in align-item-with-trigger mode.
- [x] Scroll up/down arrows appear only when the popup/list is scrollable in that direction.
- [x] Scroll arrows can be kept mounted when `keep_mounted == true`.
- [x] Hovering a scroll arrow scrolls toward the next item/edge.
- [x] Scroll arrows hide for touch input or document a GPUI-specific equivalent. Touch-driven open sources suppress scroll-arrow visibility in style state.
- [x] Popup final focus behavior returns focus to the trigger by default after close, with a GPUI-native override or documented deferral.

### Value display, labels, groups, and separators

- [x] `SelectValue<T>` displays placeholder when no value is selected and no null-like item label overrides it.
- [x] `SelectValue<T>` displays the registered selected item label when selected.
- [x] `SelectValue<T>` supports multiple selected labels, e.g. comma-separated labels or a user-provided formatter.
- [x] Static `SelectValue` children override automatic value text if this is useful in GPUI composition.
- [x] A Rust-native formatter callback can customize displayed value without porting React render props literally.
- [x] `SelectIcon` exposes open state and renders a default icon only if no custom child is provided, or documents the chosen behavior.
- [x] `SelectItemText` registers item text for label lookup, typeahead, and align-item positioning.
- [x] `SelectItemIndicator<T>` renders only when the parent item is selected unless `keep_mounted` is true.
- [x] `SelectItemIndicator<T>` exposes selected and transition/presence state if supported.
- [x] `SelectGroup` groups items for composition and style state.
- [x] `SelectGroupLabel` registers group label metadata.
- [x] Group labels and separators are not selectable and are skipped by keyboard navigation/typeahead.
- [x] `SelectSeparator` or arbitrary separator children can be rendered without corrupting item indices.

### Field and Form integration

- [x] Select inside `FieldRoot` registers one representative control with stable key, name, value, disabled, focused, required, value-missing, and focus handle metadata.
- [x] Field disabled state combines with Select disabled state.
- [x] Fieldset disabled state propagates through Field/Select as expected.
- [x] Select root/name metadata cooperates with FieldRoot name precedence.
- [x] Required single Select is missing when selected value is `None` or serializes to an empty string.
- [x] Required multiple Select is missing when the selected list is empty.
- [x] Field filled state agrees with `SelectValue` placeholder semantics.
- [x] Field dirty state compares current value(s) against initial value(s).
- [x] Field touched state updates on blur/focus-out, not when focus moves from trigger into popup.
- [x] Field focused state is true when trigger or popup/item focus is inside the Select.
- [x] `validation_mode = OnChange` validates after accepted value changes.
- [x] `validation_mode = OnBlur` validates when focus leaves the Select.
- [x] `validation_mode = OnSubmit` validates on Form submit.
- [x] Custom Field validation receives the raw Select field value representation.
- [x] Single Select can register `FieldValue::Text(serialized)` when a serializer is supplied, otherwise `Present`/`Empty` by documented design.
- [x] Multiple Select can register `FieldValue::List(serialized_values)` when a serializer is supplied.
- [x] Named Select fields contribute deterministic `FormValue` entries on submit.
- [x] Disabled Select fields are skipped by Form validation/value collection.
- [x] Select value changes clear matching external Form errors without clearing sibling errors.
- [x] Invalid Form submit focuses the Select trigger or highlighted item through the registered focus handle.

### Styling/state exposure

- [x] Add `SelectRootStyleState` with disabled, read-only, required, open, focused, filled, dirty, touched, valid/invalid, selection mode, and value-present facts.
- [x] Add `SelectTriggerStyleState` with open, disabled, read-only, required, placeholder, popup side, value-present, focused, filled, dirty, touched, and valid/invalid facts.
- [x] Add `SelectValueStyleState<T>` with current selection/value-present and placeholder status without forcing `T: Display`.
- [x] Add `SelectIconStyleState` with open state.
- [x] Add `SelectPositionerStyleState` with open, side, align, anchor-hidden/available, and measured-size facts.
- [x] Add `SelectPopupStyleState` with open, side, align, mounted, and transition status if supported.
- [x] Add `SelectBackdropStyleState` with open/mounted/transition status if supported.
- [x] Add `SelectArrowStyleState` with open, side, align, and uncentered.
- [x] Add `SelectItemStyleState` with selected, highlighted, disabled, read-only/root-disabled, focused/tab-stop, and index facts.
- [x] Add `SelectItemIndicatorStyleState` with selected/present/transition status.
- [x] Add `SelectScrollArrowStyleState` with direction, visible, side, and transition status if supported.
- [x] Add empty or useful style states for list, group, group label, item text, and separator.
- [x] Every visual part that Base UI exposes state for has `style_with_state(...)`.
- [x] Do not expose DOM data attributes, CSS classes, or CSS variable APIs.

### Tests / verification

Add behavior tests under `crates/base_gpui/src/select/tests/` where practical.

- [x] Runtime tests cover uncontrolled single initial value.
- [x] Runtime tests cover controlled single value reconciliation.
- [x] Runtime tests cover value-change cancellation.
- [x] Runtime tests cover controlled open-state reconciliation.
- [x] Runtime tests cover open-change cancellation.
- [x] Runtime tests cover item registration order.
- [x] Runtime tests cover dynamic item removal fallback.
- [x] Runtime tests cover disabled item activation no-op.
- [x] Runtime tests cover multiple selection toggling if multiple is implemented.
- [x] Runtime tests cover typeahead matching and disabled-item skipping.
- [x] Rendered tests cover trigger opens popup.
- [x] Rendered tests cover item click selects and closes in single mode.
- [x] Rendered tests cover item click toggles and stays open in multiple mode if implemented.
- [x] Rendered tests cover keyboard ArrowDown/ArrowUp navigation.
- [x] Rendered tests cover Enter/Space item activation.
- [x] Rendered tests cover Escape close.
- [x] Rendered tests cover label focus behavior.
- [x] Rendered tests cover placeholder vs selected value display.
- [x] Rendered tests cover item indicator mount/keep-mounted behavior.
- [x] Rendered tests cover group labels/separators do not affect item indices.
- [x] Rendered tests cover basic popup positioning measurement state.
- [x] Rendered tests cover scroll arrow visibility if scroll arrows are implemented.
- [x] Field integration tests cover filled, dirty, touched, focused, required, and validation modes.
- [x] Form integration tests cover submitted values, external error clearing, disabled fields, and invalid focus.
- [x] Demo renders in `crates/base_gpui/src/main.rs` without panics.

## Follow-ups to track explicitly if not completed in the first port

- [x] Full Floating UI-style collision avoidance (`flip`, `shift`, fallback axis, sticky, collision boundaries/padding).
- [x] Extract a shared portal/overlay primitive for future Menu/Popover/Dialog components after Select proves the right local API.
- [x] Modal outside-interaction inertness and scroll locking.
- [x] Touch-specific open/positioning behavior.
- [x] Full align-item-with-trigger parity, including viewport thresholds and selected text transform origin.
- [x] Transition/presence infrastructure shared across popup, backdrop, indicators, and scroll arrows.
- [x] AccessKit semantics for combobox/listbox/option/group once the target GPUI revision supports the needed APIs. Audited pinned GPUI revision: no GPUI-native role/label/listbox/option APIs are exposed on the element surface yet, so this is explicitly deferred rather than ported through DOM/ARIA literals.
- [x] Extract `SelectSeparator` into a shared `separator` module if future component ports need it.
- [x] Optional custom comparator support if `T: Eq` is not enough for real callers.
