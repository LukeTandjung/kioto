# Port Base UI Separator to GPUI

## Problem

Base UI Separator is a small stateless component that renders a visual divider with horizontal/vertical orientation state. Select re-exports Base UI's shared `Separator` as `Select.Separator`, so porting Separator first gives `base_gpui` a shared primitive that Select, Menu, Dropdown, Toolbar, and future list/menu components can reuse instead of inventing a Select-only separator.

`crates/base_gpui` currently has no shared `separator` module. The goal is to port the useful behavior into a GPUI-native component with typed orientation and style state, not to copy DOM attributes, React render props, CSS classes, or browser accessibility attributes literally.

## Scope

Port the Base UI Separator component into a shared GPUI component:

- `Separator`
- `SeparatorOrientation`
- `SeparatorStyleState`

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/separator/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/separator/Separator.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/separator/SeparatorDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/separator/Separator.test.tsx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/separator/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/separator/types.md`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/separator/types.ts`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/separator/demos/hero/css-modules/index.tsx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/separator/demos/hero/tailwind/index.tsx`

Current GPUI implementation:

- No `crates/base_gpui/src/separator/` module exists yet.
- `issues/port-baseui-select.md` should depend on this shared component for `SelectSeparator` / `Select.Separator` behavior.

Expected GPUI implementation files:

```text
crates/base_gpui/src/separator/mod.rs
crates/base_gpui/src/separator/style_state.rs
crates/base_gpui/src/separator/layers/mod.rs
crates/base_gpui/src/separator/layers/separator.rs
crates/base_gpui/src/separator/tests/
```

Because Separator is stateless and non-compound, it should not need `runtime.rs`, `context.rs`, `actions.rs`, or `child_wiring.rs`.

## Out of scope / drop from Base UI

- Do not port React `render` props.
- Do not port `className`.
- Do not port web `style` props; use normal GPUI styling builders and `style_with_state(...)`.
- Do not port DOM data attributes such as `data-orientation`; expose `orientation` in `SeparatorStyleState`.
- Do not port DOM `role="separator"` or `aria-orientation` literally. If the current GPUI dependency exposes appropriate AccessKit APIs, wire them GPUI-natively; otherwise track accessibility as a follow-up.
- Do not create a component-local `utils/` folder.
- Do not use Rust scoped visibility syntax (`pub(...)`); `ast-grep scan` must remain clean.

## Acceptance Criteria

### Module/API surface

- [x] Add a top-level `separator` module and export it from `crates/base_gpui/src/lib.rs`.
- [x] Add public `Separator` layer type with `Separator::new()`.
- [x] Add public `SeparatorOrientation` enum with `Horizontal` and `Vertical` variants.
- [x] `SeparatorOrientation::Horizontal` is the default.
- [x] `Separator` supports `.orientation(SeparatorOrientation)`.
- [x] Optional convenience builders `.horizontal()` and `.vertical()` exist or are explicitly skipped as unnecessary.
- [x] `Separator` supports normal GPUI styling builders through `Styled`.
- [x] `Separator` supports children only if there is a clear GPUI composition reason; otherwise keep it childless like a divider.
- [x] `Separator` supports `.style_with_state(...)`.
- [x] `separator/mod.rs` exposes ergonomic barrel exports.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui separator` passes.
- [x] `cargo test -p base_gpui` passes.
- [x] `ast-grep scan crates/base_gpui/src/separator` passes or produces no scoped-visibility violations.
- [x] Add a small Separator demo to `crates/base_gpui/src/main.rs`, ideally showing vertical separator usage similar to the Base UI hero demo.

### Architecture

- [x] Keep Separator simple and stateless; do not add runtime/context/entity plumbing.
- [x] Keep the renderable GPUI element under `crates/base_gpui/src/separator/layers/separator.rs`.
- [x] Keep `SeparatorStyleState` in `style_state.rs`.
- [x] Avoid generic abstractions; Separator should be a small shared component, not a utility dump.
- [x] Do not leak DOM concepts into the public API.

### Behavior

- [x] Default orientation is horizontal.
- [x] Horizontal orientation is reflected in style state.
- [x] Vertical orientation is reflected in style state.
- [x] Orientation can be changed between renders without stale state.
- [x] Separator is inert: it does not take focus, handle activation, or mutate application state.
- [x] Separator can be styled as a horizontal rule, e.g. full width with `h(px(1.0))`.
- [x] Separator can be styled as a vertical rule, e.g. full height with `w(px(1.0))`.

### Styling/state exposure

- [x] Add `SeparatorStyleState { orientation: SeparatorOrientation }`.
- [x] `.style_with_state(...)` receives the current `SeparatorStyleState`.
- [x] Do not expose `data-orientation`; callers use `style_with_state(...)` instead.
- [x] Documentation/example shows state-aware styling for horizontal vs vertical orientation if useful.

### Accessibility follow-up

The pinned workspace `gpui` dependency is `https://github.com/zed-industries/zed#f7ca86e6`. This revision does not expose the newer AccessKit role/orientation helpers on `Div`, so Separator intentionally remains a visual GPUI component for now. GPUI-native separator accessibility should be wired after this repo updates to a GPUI revision with the needed AccessKit APIs.

- [x] Check whether the pinned GPUI dependency exposes AccessKit role/orientation APIs.
- [x] Document AccessKit separator semantics as a follow-up rather than adding DOM-like attributes.

### Tests / verification

- [x] Unit or render tests cover default horizontal orientation.
- [x] Unit or render tests cover explicit vertical orientation.
- [x] Tests cover `style_with_state(...)` observing horizontal orientation.
- [x] Tests cover `style_with_state(...)` observing vertical orientation.
- [x] Tests verify the component renders without a surrounding field/form context.
- [x] Tests verify Separator can be used as a child inside arbitrary containers without affecting layout state of siblings.

## Select integration follow-up

After Separator is implemented:

- [x] Update `issues/port-baseui-select.md` implementation plan to reuse `base_gpui::separator::Separator` for `SelectSeparator`.
- [x] Track in the Select issue that separator children do not participate in item registration, highlight movement, selected-index derivation, or typeahead.
