# Port Base UI Accordion to GPUI

## Problem

Base UI's Accordion provides a set of related collapsible panels with headings. A root owns the open item values, each item groups a header/trigger with a panel, triggers toggle item open state, panels mount or hide based on open state, and root configuration controls single-item vs multiple-item expansion.

`crates/base_gpui` currently has no Accordion component family. The goal is to port Accordion behavior into GPUI-native components using the current `base_gpui` runtime/context/layers architecture, not to copy React, DOM, Collapsible hook internals, CSS transitions, or ARIA attributes literally.

Accordion item values should be generic, using a Rust type parameter constrained around clone/equality semantics, e.g. `T: Clone + Eq + 'static`. The root value should be an ordered `Vec<T>` of open item values. Missing/no open items should be represented as an empty vector.

## Scope

Port the Accordion component family from Base UI into GPUI-native components:

- `AccordionRoot<T>`
- `AccordionItem<T>`
- `AccordionHeader<T>`
- `AccordionTrigger<T>`
- `AccordionPanel<T>`

Base UI reference files:

- `/home/luke/Projects/base-ui/packages/react/src/accordion/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/root/AccordionRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/root/AccordionRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/root/AccordionRootDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/root/AccordionRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/root/AccordionRoot.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/item/AccordionItem.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/item/AccordionItemContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/item/AccordionItemDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/item/AccordionItem.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/item/stateAttributesMapping.ts`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/header/AccordionHeader.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/header/AccordionHeaderDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/header/AccordionHeader.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/trigger/AccordionTrigger.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/trigger/AccordionTriggerDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/trigger/AccordionTrigger.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/panel/AccordionPanel.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/panel/AccordionPanelCssVars.ts`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/panel/AccordionPanelDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/accordion/panel/AccordionPanel.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/collapsible/root/useCollapsibleRoot.ts`
- `/home/luke/Projects/base-ui/packages/react/src/collapsible/panel/useCollapsiblePanel.ts`
- `/home/luke/Projects/base-ui/packages/react/src/utils/collapsibleOpenStateMapping.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/accordion/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/accordion/types.md`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/accordion/types.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/accordion/demos/hero/css-modules/index.tsx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/accordion/demos/multiple/css-modules/index.tsx`

Current GPUI implementation:

- None. Add a new `crates/base_gpui/src/accordion/` module.

Expected GPUI implementation files:

- `crates/base_gpui/src/accordion/mod.rs`
- `crates/base_gpui/src/accordion/actions.rs`
- `crates/base_gpui/src/accordion/child.rs`
- `crates/base_gpui/src/accordion/child_wiring.rs`
- `crates/base_gpui/src/accordion/context.rs`
- `crates/base_gpui/src/accordion/props.rs`
- `crates/base_gpui/src/accordion/runtime.rs`
- `crates/base_gpui/src/accordion/style_state.rs`
- `crates/base_gpui/src/accordion/layers/mod.rs`
- `crates/base_gpui/src/accordion/layers/accordion_root.rs`
- `crates/base_gpui/src/accordion/layers/accordion_item.rs`
- `crates/base_gpui/src/accordion/layers/accordion_header.rs`
- `crates/base_gpui/src/accordion/layers/accordion_trigger.rs`
- `crates/base_gpui/src/accordion/layers/accordion_panel.rs`
- `crates/base_gpui/src/accordion/tests/`

Use `crates/base_gpui/src/collapsible/` as the closest behavior precedent for trigger activation, cancelable open-change details, focus handling, and panel `keep_mounted` behavior. Use `crates/base_gpui/src/tabs/` as the closest precedent for generic values, child registration, child wiring, and per-item metadata. Do not implement Accordion as a shallow wrapper around public Collapsible parts if doing so splits Accordion's root value knowledge across multiple component runtimes.

## Value model decision

Base UI permits value-less items by generating internal IDs. GPUI Accordion intentionally drops that fallback.

Decision:

- `AccordionItem<T>` must be constructed with an explicit `T` value.
- Do not expose a value-less `AccordionItem::new()` constructor.
- Prefer an API shape such as `AccordionItem::new(value: T)` so the requirement is enforced at compile time.
- Do not generate private item values and do not leak generated values through `Vec<T>` callbacks.

## Out of scope / drop from Base UI

- Do not port React context/hooks directly; use GPUI keyed state/entities plus a thin `AccordionContext<T>` wrapper.
- Do not port Base UI's internal `CompositeList` / `useCompositeListItem` implementation literally; translate item registration into GPUI child wiring and runtime metadata.
- Do not port Base UI's nested `useCollapsibleRoot` / `useCollapsiblePanel` hook structure literally; Accordion should have one Accordion runtime that owns value and item state.
- Do not port `render` props.
- Do not port `className`.
- Do not port web `style` props; expose GPUI builder styling and state-aware styling APIs.
- Do not port `nativeButton`; GPUI does not currently expose a native button element in `crates/gpui/src/elements`.
- Do not port arbitrary HTML attributes or DOM event props.
- Do not port SSR, hydration, CSP nonce, or prehydration behavior.
- Do not port CSS variable APIs literally. If panel dimensions are useful, expose them as typed GPUI style-state fields.
- Do not port DOM data attributes as attributes; map them into typed style-state structs.
- Do not port arbitrary DOM event objects. Use Rust-native change details when cancellation/source information is needed.
- Do not port browser `hiddenUntilFound` / `beforematch`; GPUI has no browser find-in-page equivalent. Revisit only if GPUI gains a native searchable hidden-content mechanism.
- Do not port deprecated roving-focus behavior from `orientation` / `loopFocus`; Base UI now marks these as deprecated and no longer uses them for keyboard focus behavior.
- Do not write DOM ARIA attributes. Map accessibility through GPUI-native AccessKit APIs once the target GPUI revision supports the needed roles/states.

## Acceptance Criteria

### Module/API surface

- [x] Add an `accordion` module and export it from `crates/base_gpui/src/lib.rs`.
- [x] Register Accordion key bindings from `base_gpui::init(cx)`.
- [x] Add public `AccordionRoot<T>`, `AccordionItem<T>`, `AccordionHeader<T>`, `AccordionTrigger<T>`, and `AccordionPanel<T>` layer types.
- [x] Add typed child enums for root/item/header composition before `AnyElement` erasure.
- [x] `AccordionRoot<T>` accepts only `AccordionItem<T>` children unless Base UI examples force an `AnyElement` escape hatch.
- [x] `AccordionItem<T>` routes `AccordionHeader<T>` and `AccordionPanel<T>` children before erasure.
- [x] `AccordionHeader<T>` routes `AccordionTrigger<T>` children before erasure.
- [x] Support uncontrolled construction with `.default_value(Vec<T>)`, defaulting to an empty vector.
- [x] Support controlled construction with `.value(Vec<T>)`; calling the builder marks the root controlled even when the supplied vector is empty.
- [x] Support `.multiple(bool)`, defaulting to `false`.
- [x] Support `.disabled(bool)` on `AccordionRoot<T>`, defaulting to `false`.
- [x] Support `.disabled(bool)` on `AccordionItem<T>`, defaulting to `false`.
- [x] Require an explicit item value at construction time, e.g. `AccordionItem::new(value: T)`; values are generic and constrained as `T: Clone + Eq + 'static`.
- [x] Do not expose a value-less `AccordionItem::new()` constructor or generated-value fallback.
- [x] Support `.on_value_change(...)` on `AccordionRoot<T>` with a Rust-native cancelable change-details API, e.g. `Fn(&[T], &mut AccordionValueChangeDetails, &mut Window, &mut App)`.
- [x] Support `.on_open_change(...)` on `AccordionItem<T>` with a Rust-native cancelable change-details API, e.g. `Fn(bool, &mut AccordionItemOpenChangeDetails, &mut Window, &mut App)`.
- [x] Add change-details types with `reason()`, `source()`, `cancelable()`, `cancel()`, and `is_canceled()` APIs.
- [x] Add `AccordionChangeReason::TriggerPress`, matching trigger activation from Base UI.
- [x] Add a source enum such as `AccordionChangeSource::{Pointer, Keyboard}` without exposing DOM event objects.
- [x] Support root-level `.keep_mounted(bool)`, defaulting to `false`.
- [x] Support panel-level `.keep_mounted(bool)`, overriding the root default when set.
- [x] Support `.orientation(AccordionOrientation)`, defaulting to vertical, only as style state / metadata; do not use it to reintroduce deprecated roving focus.
- [x] Do not add `.loop_focus(...)` unless a specific GPUI behavior requires it; Base UI deprecates it and no longer uses it.
- [x] `accordion/mod.rs` remains barrel-only and exposes ergonomic exports for component names, style states, context, props, runtime, actions, and child types.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui accordion` passes.
- [x] The component compiles without adding web/React-specific concepts to public APIs.
- [x] The implementation follows `docs/base-gpui-component-architecture.md` and does not introduce the old `child/context/{props,runtime,state}` taxonomy.
- [x] `ast-grep scan crates/base_gpui/src` passes.
- [x] Add a small gallery demo in `crates/base_gpui/src/main.rs` that renders an Accordion with at least two items.
- [x] Add a multiple-open gallery example or a visible toggle in the demo if it stays compact.

### Architecture / internal primitives

- [x] Add `AccordionRuntime<T>` as the single owner of Accordion business state: open values, item metadata, disabled state, item/panel presence, trigger focus state, and any derived transition/dimension state that is implemented.
- [x] Add `AccordionProps<T>` for stable root props and callbacks.
- [x] Add `AccordionContext<T>` as a thin injection/plumbing type with only `read(...)`, `update(...)`, and value-changing methods such as `toggle_item(...)`.
- [x] Keep controlled/uncontrolled resolution in `AccordionContext<T>`, not in layers.
- [x] Keep Accordion behavior on `AccordionRuntime<T>`; do not grow component vocabulary on `AccordionContext<T>` beyond value-changing mediation.
- [x] Add `AccordionItemMetadata<T>` or equivalent runtime-owned metadata for item value, index/order, disabled state, and focus handle.
- [x] Add `AccordionRootStyleState<T>`, `AccordionItemStyleState<T>`, `AccordionHeaderStyleState<T>`, `AccordionTriggerStyleState<T>`, and `AccordionPanelStyleState<T>` in `style_state.rs`.
- [x] Add renderable GPUI elements only under `accordion/layers/`.
- [x] Add typed child routing in `accordion/child.rs` and private indexing/context attachment in `accordion/child_wiring.rs`.
- [x] Child indexing lives only in `child_wiring.rs`; do not recompute item indices in layer render paths.
- [x] Do not add a `utils/` folder for Accordion.
- [x] Do not add new generic primitives unless they hide a repeated deep concept across components.

### Stateful/stateless behavior

- [x] Uncontrolled Accordion initializes open values from `default_value`, defaulting to no open items.
- [x] Uncontrolled Accordion toggles internal open values on valid user activation.
- [x] Controlled Accordion reflects the external `value` vector.
- [x] Controlled Accordion calls `on_value_change` on valid user activation without mutating internal open values as the source of truth.
- [x] External controlled value changes update item, header, trigger, and panel style state.
- [x] Root disabled state disables all items and prevents all user toggles.
- [x] Item disabled state disables that item and prevents its user toggles without disabling siblings.
- [x] Re-rendering with changed props does not reset uncontrolled state except when the keyed root id changes.
- [x] Open/closed state is modeled as normal runtime state, not as an error/special path spread across layers.
- [x] Controlled values that do not match any mounted item do not invent fallback items or panic.

### Single vs multiple open behavior

- [x] With `multiple(false)`, opening a closed item emits and commits exactly that item value as the only open value.
- [x] With `multiple(false)`, toggling the currently open item closes it and leaves the open value vector empty.
- [x] With `multiple(false)`, opening a second item closes the previously open item.
- [x] With `multiple(true)`, opening a closed item appends it to the open value vector without closing currently open items.
- [x] With `multiple(true)`, closing an open item removes it from the open value vector without changing other open items.
- [x] Emitted value vectors preserve deterministic order. Prefer existing open-value order plus newly opened item appended, matching Base UI.
- [x] Emitted value vectors do not contain duplicate item values.
- [x] Duplicate item values in the rendered tree are handled deterministically, preferably by treating matching values as the same open group and documenting the behavior.

### Change event behavior

- [x] User activation computes the target item's next open state from current runtime membership.
- [x] Item `on_open_change` is called before root `on_value_change` for a valid activation.
- [x] Canceling item `on_open_change` prevents root `on_value_change` and prevents uncontrolled mutation.
- [x] Root `on_value_change` is called exactly once per accepted user activation.
- [x] Root `on_value_change` receives the next full open value vector.
- [x] Canceling root `on_value_change` prevents uncontrolled mutation.
- [x] Controlled mode calls root/item callbacks but never mutates internal open state as source of truth.
- [x] Disabled root/item activation calls neither item nor root callbacks.
- [x] User-initiated change details use `AccordionChangeReason::TriggerPress`.
- [x] User-initiated change details expose pointer vs keyboard source without exposing DOM event objects.
- [x] User-initiated change details are cancelable.
- [x] Do not expose Base UI's native `event`, `trigger`, `allowPropagation`, or propagation APIs literally.

### Pointer interaction behavior

- [x] Clicking an enabled trigger opens its closed item.
- [x] Clicking an enabled trigger closes its open item.
- [x] Clicking a disabled-root trigger does not toggle and does not call change handlers.
- [x] Clicking a disabled-item trigger does not toggle and does not call change handlers.
- [x] Pointer activation and keyboard activation share the same runtime value-changing command.
- [x] Pointer activation uses GPUI click events only as an input source; no browser event object leaks into the public API.

### Keyboard/focus behavior

- [x] `AccordionTrigger<T>` owns a stable keyed `FocusHandle`.
- [x] `AccordionTrigger<T>` is focusable when enabled.
- [x] Trigger focused state is synced into `AccordionRuntime<T>` and exposed through trigger style state.
- [x] Accordion uses GPUI actions/key dispatch and an Accordion-specific key context instead of raw DOM-style key handlers.
- [x] Space toggles an enabled Accordion item when its trigger is focused.
- [x] Enter toggles an enabled Accordion item when its trigger is focused.
- [x] Disabled root/item activation does not toggle from keyboard activation.
- [x] Do not add Arrow/Home/End roving focus behavior in the initial port; Base UI has deprecated this behavior for Accordion.
- [x] Disabled triggers are removed from tab order initially, matching current GPUI control behavior.

### Item/header/trigger behavior

- [x] `AccordionItem<T>` receives root context and registers value, index/order, disabled state, and trigger focus handle through private child wiring/runtime sync.
- [x] `AccordionItem<T>` exposes `open`, `closed`/`hidden`, `disabled`, `index`, `value`, and root orientation in style state where useful.
- [x] `AccordionHeader<T>` receives the containing item state for styling.
- [x] `AccordionTrigger<T>` receives the containing item state for styling.
- [x] `AccordionTrigger<T>` can be rendered inside `AccordionHeader<T>` following Base UI anatomy.
- [x] `AccordionHeader<T>` and `AccordionTrigger<T>` can be omitted without panicking; omitted trigger means the item cannot be user-toggled.
- [x] Root disabled state wins over item/trigger-local enabled settings.
- [x] Item disabled state wins over trigger-local enabled settings.

### Panel/content behavior

- [x] `AccordionPanel<T>` renders its children when the corresponding item is open.
- [x] `AccordionPanel<T>` returns `gpui::Empty` when the corresponding item is closed and effective `keep_mounted` is false.
- [x] `AccordionPanel<T>` remains in the element tree when the corresponding item is closed and effective `keep_mounted` is true.
- [x] Root `.keep_mounted(true)` applies to all panels by default.
- [x] Panel `.keep_mounted(...)` overrides the root default for that panel.
- [x] A kept-mounted closed panel is visually hidden using a GPUI-native mechanism such as `invisible()`, not by DOM `hidden` attributes.
- [x] A kept-mounted closed panel exposes closed/hidden state to `style_with_state(...)`.
- [x] Opening a kept-mounted panel makes it visible again and exposes open state to `style_with_state(...)`.
- [x] Closing a non-kept-mounted panel removes it immediately unless GPUI-native transition state is implemented.
- [x] Switching items in single-open mode hides/removes the previously open panel according to its `keep_mounted` setting.
- [x] The panel can be omitted without breaking item/trigger behavior.

### Transition and measurement behavior

- [x] Expose a typed transition-like state only if GPUI can make it meaningful, e.g. `AccordionTransitionStatus::{Starting, Ending, Idle}` or a simpler presence state.
- [x] Do not copy Base UI DOM transition attributes (`data-starting-style`, `data-ending-style`) as attributes.
- [x] No GPUI-native transition sequencing is implemented in this pass, so there is no transition sequencing outside `AccordionRuntime<T>`.
- [x] No panel dimensions are implemented in this pass, so there is no DOM-style measurement to replace.
- [x] No panel dimensions are implemented in this pass, so no CSS variable API is exposed.
- [x] Do not port the Base UI `hiddenUntilFound` instant-open and animation-suppression paths.

### Styling/state exposure

- [x] `AccordionRootStyleState<T>` includes at least open values, disabled, multiple, and orientation.
- [x] `AccordionItemStyleState<T>` includes at least open, hidden/closed, disabled, index, value, and orientation.
- [x] `AccordionHeaderStyleState<T>` includes the item state or equivalent fields needed by Base UI header state callbacks.
- [x] `AccordionTriggerStyleState<T>` includes at least open/panel-open, closed, disabled, focused, index, value, and orientation.
- [x] `AccordionPanelStyleState<T>` includes at least open, hidden/closed, mounted/present, disabled, index, value, orientation, and any implemented transition/dimension fields.
- [x] Expose state-aware styling through `style_with_state(...)` on root, item, header, trigger, and panel.
- [x] Map Base UI state/data attributes (`data-disabled`, `data-index`, `data-open`, `data-panel-open`, `data-orientation`, and transition status if implemented) into typed style-state fields, not DOM attributes.
- [x] Do not expose CSS variable names as the styling API.
- [x] The docs hero styling pattern can be recreated with GPUI builder methods: item border/state styling, trigger styling changes when open, panel visibility changes when open/closed, and focus styling can use trigger focused style state.

### Accessibility follow-up

- [ ] Once the target GPUI revision supports the needed AccessKit APIs, map trigger semantics to a GPUI/AccessKit button or disclosure trigger role.
- [ ] Once available, expose expanded/collapsed state through GPUI-native accessibility APIs.
- [ ] Once available, expose disabled state through GPUI-native accessibility APIs.
- [ ] Once available, map `AccordionHeader<T>` to an appropriate heading role/level or provide a builder to configure heading level.
- [ ] Decide how GPUI Accordion links a trigger to its panel through AccessKit relationships if GPUI supports relationships; do not port DOM `aria-controls` / `aria-labelledby` / `id` linking literally.
- [ ] Add accessibility tests if GPUI exposes test helpers for AccessKit state.

### Tests / verification

Add behavior-level tests under `crates/base_gpui/src/accordion/tests/`.

- [x] Uncontrolled initial state defaults to no open items.
- [x] Uncontrolled `default_value` opens matching item panels.
- [x] Controlled `value` opens matching item panels.
- [x] External controlled value changes update item, header, trigger, and panel style state.
- [x] Click opens a closed item.
- [x] Click closes an open item.
- [x] Disabled root click does not toggle and does not call handlers.
- [x] Disabled item click does not toggle and does not call handlers.
- [x] Space toggles when the trigger is focused.
- [x] Enter toggles when the trigger is focused.
- [x] Disabled root/item keyboard activation does not toggle.
- [x] Trigger focused state appears in `AccordionTriggerStyleState` when focused and clears on blur.
- [x] Single-open mode opens only one item at a time.
- [x] Single-open mode can close the currently open item to an empty value.
- [x] Multiple-open mode allows more than one item open.
- [x] Multiple-open mode closes one open item without closing siblings.
- [x] Root `on_value_change` receives the next full value vector.
- [x] Item `on_open_change` receives the next item open bool.
- [x] Item `on_open_change` cancellation prevents root callback and uncontrolled mutation.
- [x] Root `on_value_change` cancellation prevents uncontrolled mutation.
- [x] Controlled cancellation still calls handlers but does not mutate internal open state.
- [x] Change details expose trigger-press reason, pointer source, keyboard source, and cancelability.
- [x] Root `keep_mounted(true)` keeps closed panels rendered and hidden.
- [x] Panel `keep_mounted(false)` can override root `keep_mounted(true)` if panel overrides are implemented.
- [x] Panel renders children while open.
- [x] Panel is omitted while closed by default.
- [x] Kept-mounted closed panel receives closed/hidden style state.
- [x] Header receives item state for styling.
- [x] Trigger receives item state for styling.
- [x] Panel receives item state for styling.
- [x] Item indices are deterministic and update when items are inserted/removed.
- [x] Missing controlled values do not panic and do not render phantom panels.
- [x] Value-less items cannot be constructed through the public API.
- [x] Duplicate item values behave deterministically.
- [x] `style_with_state(...)` receives correct root state.
- [x] `style_with_state(...)` receives correct item state.
- [x] `style_with_state(...)` receives correct header state.
- [x] `style_with_state(...)` receives correct trigger state.
- [x] `style_with_state(...)` receives correct panel state.
- [x] Transition/dimension tests are not applicable in this pass because GPUI-native transition and measurement behavior were not implemented.
