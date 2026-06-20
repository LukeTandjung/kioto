# Port Base UI Fieldset to GPUI

## Problem

Base UI Fieldset groups related fields and propagates disabled state to descendant controls. HTML Fieldset also has legend semantics and disabled cascading behavior; GPUI should model the useful behavior natively without porting DOM fieldset quirks literally.

This is a follow-up prerequisite for full Form parity: Form should respect Fieldset-disabled fields when validating and collecting values once Fieldset exists.

## Scope

- Add a `fieldset` component module under `crates/base_gpui/src/fieldset/`.
- Provide GPUI-native `FieldsetRoot` / `FieldsetLegend` or equivalent public API after inspecting Base UI source.
- Provide ambient Fieldset context for descendant `FieldRoot` / controls.
- Propagate disabled state to descendant fields and controls.
- Expose typed render state and `style_with_state(...)` APIs.
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
