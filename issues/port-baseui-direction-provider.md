# Port Base UI Direction Provider to GPUI

## Problem

Base UI Direction Provider exposes an ambient text-direction value for descendant components. `useDirection()` returns the nearest provider's direction and defaults to `ltr` outside a provider. Components such as Radio Group use that direction to flip horizontal keyboard behavior in RTL contexts.

`crates/base_gpui` currently has no shared direction primitive. Radio Group could add a component-local `.direction(...)` builder, but that would duplicate a cross-cutting concern that future components also need. The goal is to add a GPUI-native, reusable direction utility under a shared module, not inside a single component folder.

This should port the behavior and contract, not React context implementation details.

## Scope

Port the Direction Provider utility from Base UI into a shared `base_gpui` utility module:

- `TextDirection`
- `DirectionProvider`
- a GPUI-native way for descendants to read the current direction, e.g. `current_direction(...)` / `DirectionContext` / equivalent helper

Base UI reference files:

- `/home/luke/Projects/base-ui/packages/react/src/direction-provider/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/direction-provider/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/direction-provider/DirectionProvider.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/direction-provider/DirectionProvider.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/direction-provider/DirectionProvider.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/internals/direction-context/DirectionContext.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/internals/direction-context/index.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/utils/direction-provider/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/utils/direction-provider/types.md`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/utils/direction-provider/types.ts`

Current GPUI implementation:

- No shared `crates/base_gpui/src/utils/` module exists yet.
- No ambient direction provider exists yet.

Expected GPUI implementation files:

- `crates/base_gpui/src/utils/mod.rs`
- `crates/base_gpui/src/utils/direction.rs` or `crates/base_gpui/src/utils/direction/mod.rs`
- `crates/base_gpui/src/utils/direction/tests/` or colocated module tests

Radio Group integration should happen in `issues/port-baseui-radio-group.md`, but this issue should provide the shared direction primitive that Radio Group consumes.

## Out of scope / drop from Base UI

- Do not port React context/hooks directly.
- Do not add a component-specific direction prop to Radio Group as the long-term API.
- Do not use a simple app-global direction as the only mechanism; Base UI supports nested providers, so GPUI needs scoped provider semantics or an explicitly documented equivalent.
- Do not make Direction Provider a visual styling primitive.
- Do not automatically change text layout, text shaping, CSS direction, or any GPUI style. Base UI's Direction Provider only provides behavior context; app code remains responsible for visual/text direction styling.
- Do not port `children?: React.ReactNode` literally; use GPUI element composition.
- Do not port DOM `dir` attributes.
- Do not port web docs/demo styling directly.

## Acceptance Criteria

### Module/API surface

- [x] Add `crates/base_gpui/src/utils/` as a shared module peer to component folders.
- [x] Export the shared utilities from `crates/base_gpui/src/lib.rs` via `pub mod utils;`.
- [x] Add a `TextDirection` enum with `Ltr` and `Rtl` variants.
- [x] `TextDirection` implements `Clone`, `Copy`, `Debug`, `Default`, `Eq`, and `PartialEq`.
- [x] `TextDirection::default()` is `TextDirection::Ltr`.
- [x] Add ergonomic helpers such as `is_ltr()`, `is_rtl()`, and direction-aware horizontal navigation mapping if useful for consumers.
- [x] Add a public `DirectionProvider` GPUI builder/element.
- [x] `DirectionProvider::new()` defaults to `TextDirection::Ltr`.
- [x] `DirectionProvider` has `.direction(TextDirection)`.
- [x] `DirectionProvider` supports `.child(...)` and `.children(...)` composition.
- [x] Add a public function or context wrapper for descendants to read the current direction, defaulting to `TextDirection::Ltr` outside any provider.
- [x] Re-export direction names ergonomically from `base_gpui::utils::direction` and, if desired, from `base_gpui::utils`.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui direction` passes.
- [x] The implementation compiles without adding web/React-specific concepts to public APIs.
- [x] Add a small example/demo in `crates/base_gpui/src/main.rs` or a dedicated example showing an RTL provider wrapping direction-aware content.

### Architecture / implementation model

- [x] Keep Direction Provider under `crates/base_gpui/src/utils/`, not under `radio_group/`.
- [x] Document why this shared `utils` module is allowed: direction is cross-cutting component infrastructure, not component-local helper code.
- [x] Preserve nested provider semantics: nearest provider wins for descendants.
- [x] Provider scope does not leak to siblings or ancestors.
- [x] Direction changes across renders update descendants.
- [x] Avoid app-wide `Global` as the only source of truth because it cannot represent nested provider scopes.
- [x] If GPUI requires a custom wrapper element to model ambient scoped values, keep it layout-neutral when possible.
- [x] If a layout-neutral wrapper is not possible with current GPUI APIs, document the fallback behavior and keep it as visually/layout-minimal as possible.
- [x] Direction reads are available to `RenderOnce` components during their render path, not only during paint, so components can choose key bindings/behavior from direction.
- [x] Direction reads are cheap and deterministic within a render pass.
- [x] Do not couple the direction provider to Radio Group internals.

### Behavioral parity

- [x] Reading direction outside a provider returns `TextDirection::Ltr`.
- [x] A provider configured with `TextDirection::Rtl` returns RTL to descendants.
- [x] A provider configured with `TextDirection::Ltr` returns LTR to descendants.
- [x] Nested providers override outer providers only for their own descendants.
- [x] Updating a provider's direction from RTL to LTR updates descendants on the next render.
- [x] Direction Provider itself has no checked/selected/focused state and no state-aware styling API.
- [x] Direction Provider does not affect visual/text layout unless the caller styles visual/text layout separately.

### Radio Group integration requirements

- [x] Radio Group should consume the shared direction primitive instead of adding a one-off `.direction(...)` builder as its main API.
- [x] Radio Group horizontal key behavior should use `TextDirection`: LTR maps Right to next and Left to previous; RTL maps Left to next and Right to previous.
- [x] Radio Group tests should include provider-wrapped LTR and RTL behavior once Radio Group is implemented.
- [x] The Radio Group issue references this issue as the source for ambient direction behavior.

### Portal/window follow-up

- [x] Decide how direction should behave for future portal/overlay components if they render outside the provider's normal element subtree.
- [x] If GPUI overlay/portal primitives cannot inherit provider scope automatically, document the explicit bridging API that future portal-like components should use.
- [x] Do not block initial Radio Group work on portal inheritance unless Radio Group itself needs portals.

### Tests / verification

Add tests under the shared direction utility test module.

- [x] Default direction outside provider is LTR.
- [x] Provider supplies RTL to direct descendants.
- [x] Provider supplies LTR to direct descendants.
- [x] Nested RTL inside LTR overrides to RTL.
- [x] Nested LTR inside RTL overrides to LTR.
- [x] Direction does not leak to siblings after a provider subtree.
- [x] Updating provider direction updates descendant observations.
- [x] Direction can be read by a descendant `RenderOnce` component during render.
- [x] Direction Provider can wrap multiple children.
- [x] Direction Provider does not introduce unexpected visual/layout changes, or any required wrapper behavior is documented and tested.
