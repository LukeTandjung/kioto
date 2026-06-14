# Port Base UI Form to GPUI

## Problem

`base_gpui` has Field-level state and validation, but no GPUI-native Form primitive for submit orchestration, form value collection, external/server errors, or field-array semantics.

## Scope

- Add a GPUI-native `form` component module when needed.
- Provide a `FormRoot`-style owner for submit/reset actions, registered fields, and form-level validation.
- Integrate with `field` through Field registration/context APIs, not DOM forms or hidden inputs.

## Initial Field integration decisions

- `FieldRoot::name(...)` should be the form field name override. If absent, Form can fall back to the registered control name from `FieldControlRegistration`.
- External form errors should be represented as Rust-native error/validity data and injected into Field state, not as DOM `setCustomValidity` or browser `ValidityState`.
- Multiple registered controls under one Field should use the deterministic Field aggregation rules: representative value prefers the first enabled registered control, while filled/dirty/focused aggregate across registered controls.
- Field arrays should be modeled explicitly by the future Form value model rather than inferred from hidden DOM input names.

## Non-goals

- Do not implement hidden DOM input submission behavior.
- Do not depend on browser `FormData`.
- Do not port React hooks/context literally.

## Acceptance Criteria

- [ ] Add a `form` module and export it from `crates/base_gpui/src/lib.rs`.
- [ ] Add GPUI-native submit/reset actions.
- [ ] Collect values from registered Field controls by Field name/control fallback name.
- [ ] Surface external form errors to Field via Rust-native validity/error data.
- [ ] Add tests for submit validation, external errors, and multiple fields.
