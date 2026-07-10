# Port Base UI Fieldset to GPUI

## Problem

Base UI Fieldset groups related fields and propagates disabled state to descendant controls. HTML Fieldset also has legend semantics and disabled cascading behavior; GPUI should model the useful behavior natively without porting DOM fieldset quirks literally.

This is a follow-up prerequisite for full Form parity: Form should respect Fieldset-disabled fields when validating and collecting values once Fieldset exists.

## Scope

- Add a `fieldset` component module under `crates/base_gpui/src/fieldset/`.
- Provide GPUI-native `FieldsetRoot` / `FieldsetLegend` or equivalent public API after inspecting Base UI source.
- Provide ambient Fieldset context for descendant `FieldRoot` / controls.
- Propagate disabled state to descendant fields and controls.
- Expose typed style state and `style_with_state(...)` APIs.
- Do not port DOM `<fieldset>`, `<legend>`, browser form association, or `aria-*` literally.

## Acceptance Criteria

- [x] Inspect Base UI Fieldset source, docs, and tests before implementation.
- [x] Create `crates/base_gpui/src/fieldset/` following `docs/base-gpui-component-architecture.md`.
- [x] Export and initialize Fieldset from `crates/base_gpui/src/lib.rs`.
- [x] Descendant fields/controls inherit disabled state from Fieldset.
- [x] Nested Fieldsets combine disabled state deterministically.
- [x] Legend/group label behavior is represented with GPUI-native composition.
- [x] Form validation and value collection skip fields disabled by Fieldset.
- [x] Add Fieldset tests for disabled propagation, nested behavior, and Form integration.
- [x] Add demo coverage.
- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui fieldset` passes.
- [x] `cargo test -p base_gpui` passes.

## AccessKit accessibility follow-up

Base UI's Fieldset is semantically thin: `FieldsetRoot.tsx` renders a native `<fieldset>` (implicit `role="group"`) with `aria-labelledby` pointing at the legend's id, plus the native `disabled` attribute (which cascades `aria-disabled` to descendant controls, per `FieldsetRoot.test.tsx`). `FieldsetLegend` is a plain `div` that only contributes its id. Map this onto gpui as follows.

### Per accessible part

- **`FieldsetRoot`** (`layers/fieldset_root.rs`): the a11y node lives on the `Div` in `base`, which already carries a stable `self.id` (`ElementId::from("fieldset")` by default, overridable via `.id(...)`). In `render`, apply `.role(Role::Group)` to `base` before wrapping it in `FieldsetScopeElement`. Note the id must be set on the `Div` itself (currently `self.id` is only threaded into `FieldsetContext::new`), since a node needs both `.id(...)` and `.role(...)` to appear in the tree.
- **`FieldsetLegend`** (`layers/fieldset_legend.rs`): no role in Base UI (plain `div`); it exists to *label* the group. Do not give it a role — leave it out of the a11y tree and surface its text via the root's `.aria_label(...)` instead (see Labels). If a standalone announceable heading is ever wanted, `Role::Heading` + `.aria_level(...)` is available, but that diverges from Base UI and should stay opt-in.

### Actions

- None. Fieldset is non-interactive: there is no `on_click`, no `track_focus`, and no runtime transition to route. No `.on_a11y_action(...)` handlers are needed. (For components that do wire `.on_click`/`.track_focus`, `Action::Click`/`Action::Focus` are auto-registered — irrelevant here, noted for consistency.)

### Labels

- Add an `.aria_label(impl Into<SharedString>)`-style builder prop on `FieldsetRoot` (e.g. `label: Option<SharedString>`) and forward it to the root `Div` via `.aria_label(...)`. This substitutes for Base UI's `aria-labelledby` → legend-id wiring, which gpui cannot express (see Gaps).
- When the caller supplies both a visible `FieldsetLegend` and the root `.aria_label`, the legend's visible text should be built with `Text::new_inaccessible(...)` rather than `text!(...)` so the label is not announced twice. Document this on `FieldsetLegend::new()` / in the demo.

### Gaps (no gpui builder in the pinned revision)

- **`aria-labelledby`** (root → legend id): no relationship builders in this revision. Fallback: literal-string `.aria_label(...)` on `FieldsetRoot` duplicating the legend text; document that it must be kept in sync manually.
- **`disabled` / `aria-disabled`**: `FieldsetRoot.disabled` and the cascaded `FieldsetProps::new(disabled)` state (also `current_fieldset_disabled()` for nesting) have no `.aria_disabled(...)` builder and `write_a11y_info` never sets a disabled flag. Fallback: the disabled cascade already suppresses interaction on descendant Field controls (their own issues cover omitting actions while disabled); the group node itself omits the attribute — documented limitation, revisit if a `set_disabled` builder is upstreamed to gpui.

### Checklist

- [ ] Set `.role(Role::Group)` and a stable `.id(...)` on the `FieldsetRoot` base `Div`.
- [ ] Add an `aria_label` builder prop on `FieldsetRoot` mapped to `.aria_label(...)`.
- [ ] Keep `FieldsetLegend` out of the a11y tree (no role); render its visible text with `Text::new_inaccessible(...)` when the root `aria_label` duplicates it.
- [ ] Document the `aria-labelledby` and `aria-disabled` gaps (literal label fallback; disabled flag omitted pending gpui upstream).
- [ ] Add an a11y test asserting the root node reports `Role::Group` with the expected label, and that no legend node is emitted.
