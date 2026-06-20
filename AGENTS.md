# Kioto Repository Guidelines

## Rust module layout

`mod.rs` files are barrel export files only.

Allowed in `mod.rs`:
- `mod ...;`
- `pub mod ...;`
- `pub use ...;`
- `#[cfg(test)] mod tests;`

Disallowed in `mod.rs`:
- structs, enums, traits, type aliases
- functions
- constants/statics
- impl blocks
- macro definitions

Move implementation logic into a named sibling file and re-export it from `mod.rs`.
Before finishing Rust module work, run:

```sh
ast-grep scan crates/base_gpui/src
```

The `barrel-only-mod-rs` ast-grep rule enforces this policy.

## `base_gpui` component architecture

Follow `docs/base-gpui-component-architecture.md` for compound components.

Use this file shape for component modules:

```text
crates/base_gpui/src/<component>/
  mod.rs            # barrel only
  actions.rs        # optional GPUI key dispatch actions/bindings
  runtime.rs        # component business state and behavior
  context.rs        # thin entity/props/controlled-state injection
  props.rs          # public configuration and callbacks
  style_state.rs    # typed state passed to style_with_state(...)
  child.rs          # typed child enums
  child_wiring.rs   # private child traversal/indexing/context attachment, when needed
  layers/           # renderable GPUI parts only
  tests/
```

Use `*StyleState` type names and `style_state.rs` files. Do not introduce new
`*RenderState` names or `render_state.rs` files.

Do not add `runtime_control.rs` or similar trait-boundary files for normal
component behavior. Runtime behavior should be inherent methods on
`<Component>Runtime`, with controlled/uncontrolled mediation in `<Component>Context`.

## Shared utilities

Keep component-specific helpers inside the component folder. Truly shared,
repeated cross-component helpers may live under `crates/base_gpui/src/utils/` as
flat named files, re-exported by `utils/mod.rs`.
