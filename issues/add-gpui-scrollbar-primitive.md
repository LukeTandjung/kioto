# Add GPUI-native Scrollbar Primitive

## Problem

GPUI ships native overflow scrolling and virtualization, but no scrollbar element. `Div` supports `.overflow_scroll()` / `.overflow_x_scroll()` / `.overflow_y_scroll()` plus `.track_scroll(&ScrollHandle)`, `uniform_list` exposes `UniformListScrollHandle`, and `list` exposes `ListState` — all of them scroll content without ever drawing a track or thumb. Base UI ports keep running into this because the browser gives scrollbars away for free and Base UI's Scroll Area assumes a paintable, draggable scrollbar exists.

In this repo the gap is already visible: `crates/base_gpui/src/select/layers/select_list.rs` scrolls its dropdown with `overflow_y_scroll().track_scroll(...)` and compensates with scroll-arrow layers driven by `refresh_scroll_arrow_visibility` in `select/runtime.rs`, because there is nothing to render as a scrollbar.

GPUI does expose the lower-level bridge needed to build one:

- `ScrollHandle` with `offset()`, `max_offset()`, `set_offset(...)`, and `bounds()` (offsets are zero-or-negative; `max_offset()` is positive; content size is `max_offset() + bounds().size`);
- `UniformListScrollHandle` and `ListState` with equivalent offset/size access for virtualized content;
- custom `Element` implementations with `request_layout` / `prepaint` / `paint`, hitboxes, content masks, and `window.on_mouse_event(...)` for `MouseDownEvent` / `MouseMoveEvent` / `MouseUpEvent` / `ScrollWheelEvent`.

The goal is to add a reusable `scrollbar()` primitive — a custom GPUI `Element` that paints a track and a draggable thumb over any scroll handle. This primitive should be shared by a future Base UI Scroll Area port, Select's dropdown list, and any other scrollable surface in `base_gpui`. `gpui-component`'s `Scrollbar` element is the primary behavioral reference for the geometry and event model, but the implementation must follow this repo's deep-runtime architecture rather than copying its monolithic prepaint.

## Scope

Add a reusable scrollbar primitive under `base_gpui` primitives:

- public `scrollbar()` constructor function;
- public `Scrollbar` builder/element type implementing GPUI `Element`;
- a `ScrollTarget` trait abstracting over `ScrollHandle`, `UniformListScrollHandle`, and `ListState`, so the same scrollbar works over plain scrollable divs and virtualized lists;
- a deep runtime/geometry module owning thumb math, drag mapping, visibility mode, and fade timing, unit-testable without a window;
- vertical and horizontal orientations, plus a corner spacer where both meet;
- typed visibility modes (show while scrolling with idle fade-out, show on hover, always show);
- typed style state exposed through `style_with_state(...)`.

Primary references:

- `/home/luke/.cargo/git/checkouts/zed-a70e2ad075855582/f7ca86e/crates/gpui/src/elements/div.rs` (`ScrollHandle`, `.overflow_scroll()`, `.track_scroll(...)`)
- `/home/luke/.cargo/git/checkouts/zed-a70e2ad075855582/f7ca86e/crates/gpui/src/elements/uniform_list.rs` (`UniformListScrollHandle`)
- `/home/luke/.cargo/git/checkouts/zed-a70e2ad075855582/f7ca86e/crates/gpui/src/elements/list.rs` (`ListState`)
- `/home/luke/.cargo/git/checkouts/zed-a70e2ad075855582/f7ca86e/crates/gpui/examples/scrollable.rs`
- `/home/luke/Projects/gpui-component/crates/ui/src/scroll/scrollbar.rs` (`Scrollbar` element, `ScrollbarShow`, `ScrollbarHandle` trait, fade-out timing, prepaint hitbox + mouse event model)
- `/home/luke/Projects/gpui-component/crates/ui/src/scroll/scrollable.rs` (wrapper composition pattern)
- `/home/luke/Projects/kioto/crates/base_gpui/src/select/layers/select_list.rs` (current in-repo scroll usage to stay compatible with)
- `/home/luke/Projects/kioto/crates/base_gpui/src/select/runtime.rs` (`list_scroll_handle`, `refresh_scroll_arrow_visibility`)
- `/home/luke/Projects/kioto/docs/base-gpui-component-architecture.md`

Consumers, cross-linked:

- `issues/port-baseui-scroll-area.md` (to be written) — the Base UI Scroll Area port is the first consumer; its Root/Viewport/Scrollbar/Thumb/Corner anatomy composes this primitive rather than reimplementing thumb math.
- `issues/port-baseui-select.md` — Select's dropdown could adopt this primitive later as an alternative or complement to its scroll arrows; do not rework Select in this issue.

## Initial design decisions

### Placement

Put the primitive under:

```text
crates/base_gpui/src/primitives/scroll/
```

Do not put this in `select/`: scrollbar painting is shared infrastructure, not Select-specific behavior. Do not put it in `utils/`: a scrollbar owns deep geometry, drag, and visibility state, so it is a primitive, not a helper. Do not move it into a shared UI crate yet; incubate it under `primitives/` alongside `primitives/input/`, then split/move later only if multiple crates need the same primitive boundary.

### Public constructor shape

Expose an ergonomic primitive constructor over any scroll target:

```rust
scrollbar(&scroll_handle)
    .id("editor-scroll")
    .axis(ScrollbarAxis::Vertical)
    .visibility(ScrollbarVisibility::Scrolling)
```

The public type can be `Scrollbar`, but consumers should usually reach it through `scrollbar()` the same way they reach GPUI primitives through functions like `div()`. The constructor accepts any `&H where H: ScrollTarget + Clone` and stores it as `Rc<dyn ScrollTarget>`. The element positions itself absolutely and fills its parent, so consumers overlay it on a `relative()` container wrapping the scrollable content — the same composition `gpui-component`'s `scrollable.rs` uses.

### The `ScrollTarget` trait

Follow `gpui-component`'s `ScrollbarHandle` precedent: a small trait so the scrollbar is agnostic to how content scrolls.

- `offset() -> Point<Pixels>` — current scroll offset (zero-or-negative in GPUI's convention);
- `set_offset(Point<Pixels>)` — move the scroll position;
- `content_size() -> Size<Pixels>` — full scrollable content size (`max_offset() + bounds().size` for `ScrollHandle`);
- `viewport_size() -> Size<Pixels>` — visible container size;
- optional drag lifecycle hooks (`ListState` needs `scrollbar_drag_started` / `scrollbar_drag_ended` so virtualized measurement stays stable during drag).

Implement it for `ScrollHandle`, `UniformListScrollHandle`, and `ListState`. Keep the trait in the primitive module; consumers implementing custom scroll containers can implement it too.

### State ownership

Hide interaction state (hover, drag anchor, last scroll offset/time, fade timer) behind keyed window state, following the input primitive's precedent. Consumers should not need to create an entity just to render a scrollbar.

Use `.id(...)` as the stable identity for preserving hover/drag/fade state across renders, with a code-location default when omitted (per `gpui-component`'s `ElementId::CodeLocation` precedent). The scroll position itself is never owned by the scrollbar — it always lives in the consumer's scroll handle; the scrollbar reads and writes through `ScrollTarget` only.

### Thumb geometry

All geometry lives in the deep runtime/geometry module as pure functions/methods over plain numbers, computed once per axis in `prepaint`:

- thumb length = `max(MIN_THUMB_SIZE, track_len * viewport_len / content_len)`, where `MIN_THUMB_SIZE` is a small constant (`gpui-component` uses 48px; pick and document a default in that range, do not go below ~16px);
- thumb offset = `scroll_ratio * (track_len - thumb_len)`, where `scroll_ratio = -offset / (content_len - viewport_len)` (note the sign flip from GPUI's negative offsets);
- drag: on thumb mouse-down record the grab point inside the thumb; on move, map `(pointer - grab_point - track_origin) / (track_len - thumb_len)` back to a clamped scroll offset via `set_offset(...)`;
- track click: jump so the thumb centers on the click position, clamped to the valid offset range;
- the scrollbar hides itself (or renders nothing for that axis) when `content_len <= viewport_len`;
- when both axes render, the horizontal track reserves an end margin equal to the scrollbar thickness so it does not overlap the vertical track, and the leftover square is the corner.

### Visibility model

Expose a typed visibility mode; do not hardcode one policy:

```rust
pub enum ScrollbarVisibility {
    Scrolling, // show while scrolling, fade out after idle (default)
    Hover,     // show while the pointer is over the scroll container/track
    Always,    // always visible
}
```

For `Scrolling`, follow `gpui-component`'s `FADE_OUT_DELAY` / `FADE_OUT_DURATION` precedent: detect offset changes by comparing against the last observed offset, keep the bar fully visible for an idle delay, then fade opacity to zero over a duration, driven by `window.request_animation_frame()` plus one scheduled idle timer (do not busy-poll). Hovering the track while visible resets the idle clock. Dragging always keeps the bar visible. Make the delay/duration constants module-level and documented; configurability can be a follow-up.

Base UI's Scroll Area drives visibility through CSS and `data-hovering` / `data-scrolling` attributes; translate that into typed style-state fields instead (see below).

### Pointer interaction

Wire all interaction through the custom element's `prepaint` hitboxes and `window.on_mouse_event(...)` during `paint`, following `gpui-component`'s model:

- `MouseDownEvent` on the thumb starts a drag (record grab point, call the target's drag-start hook, stop propagation);
- `MouseDownEvent` on the track (outside the thumb) jumps to the clicked position;
- `MouseMoveEvent` updates hover facts (over track, over thumb) and, while dragging, maps the pointer to a new scroll offset with a frame-rate cap on `set_offset` calls;
- `MouseUpEvent` ends the drag and calls the target's drag-end hook;
- `ScrollWheelEvent` over the track forwards to the scroll handle (the underlying scroll container already handles the wheel; the scrollbar only refreshes its last-scroll facts so visibility wakes up).

Content-mask the track and thumb to the element bounds so an overlay scrollbar never paints outside its container.

### Attribute mapping

Start with the surface needed by Scroll Area and Select:

| Concept | GPUI builder |
|---|---|
| stable identity | `.id(...)` |
| axis (vertical / horizontal / both) | `.axis(ScrollbarAxis::...)`, plus `scrollbar_vertical(...)` / `scrollbar_horizontal(...)` shorthands if useful |
| visibility policy | `.visibility(ScrollbarVisibility::...)` |
| explicit content size override | `.content_size(Size<Pixels>)` (defaults to the target's `content_size()`) |
| state-aware styling | `.style_with_state(...)` |

Do not attempt full Base UI Scroll Area anatomy (Root/Viewport/Content parts) in this issue — that is the Scroll Area port's job, composing this primitive.

### Accessibility

Accessibility is intentionally out of scope for this issue. Do not add AccessKit roles or scrollbar value semantics until the GPUI dependency is updated and that work is explicitly revisited.

## Expected implementation files

```text
crates/base_gpui/src/primitives/mod.rs
crates/base_gpui/src/primitives/scroll/mod.rs
crates/base_gpui/src/primitives/scroll/target.rs        # ScrollTarget trait + impls for ScrollHandle / UniformListScrollHandle / ListState
crates/base_gpui/src/primitives/scroll/runtime.rs       # geometry + interaction state: thumb math, drag mapping, visibility/fade facts
crates/base_gpui/src/primitives/scroll/props.rs         # axis, visibility mode, overrides
crates/base_gpui/src/primitives/scroll/style_state.rs   # ScrollbarStyleState
crates/base_gpui/src/primitives/scroll/layers/mod.rs
crates/base_gpui/src/primitives/scroll/layers/scrollbar_element.rs  # the custom Element: request_layout/prepaint/paint
crates/base_gpui/src/primitives/scroll/tests/
```

Alternative filenames are fine if they preserve the same architecture: deep runtime/geometry, thin builder, custom element, and a target trait boundary.

## Out of scope

- Do not implement touch/gesture events; GPUI input is pointer/trackpad.
- Do not port RTL negative-`scrollLeft` normalization; revisit RTL when a direction provider reaches scrolling.
- Do not port `CSS.registerProperty`, CSS variables, or any CSS-driven visibility/animation from Base UI Scroll Area.
- Do not port DOM data attributes (`data-hovering`, `data-scrolling`, `data-orientation`) as attributes; map them into the typed style state.
- Do not implement the Base UI Scroll Area component family (Root/Viewport/Content/Scrollbar/Thumb/Corner) here; that is `issues/port-baseui-scroll-area.md`.
- Do not rework Select's scroll arrows or migrate Select to this primitive in this issue.
- Do not implement keyboard scrolling on the scrollbar itself (arrow/page keys belong to the scrollable container or Scroll Area port).
- Do not implement gutter-reserving (non-overlay) layout; the primitive is an absolutely positioned overlay. Layout reservation is a consumer/Scroll Area concern.
- Do not implement scroll physics, momentum, smooth-scroll animation, or auto-scroll on drag-past-edge.
- Do not depend on `gpui-component`'s scroll implementation directly; it is a reference, not a dependency.
- Do not implement AccessKit/accessibility wiring in this issue.

## Acceptance Criteria

### Module/API surface

- [ ] Add `crates/base_gpui/src/primitives/scroll/` and export it from `crates/base_gpui/src/primitives/mod.rs`.
- [ ] Add a public `scrollbar()` constructor function taking `&H where H: ScrollTarget + Clone`.
- [ ] Add a public `Scrollbar` builder/element type implementing GPUI `Element` and `IntoElement`.
- [ ] Support `.id(...)` as the stable identity for keyed interaction state, with a code-location default.
- [ ] Support `.axis(...)` with vertical, horizontal, and both.
- [ ] Support `.visibility(...)` with `Scrolling`, `Hover`, and `Always`, defaulting to `Scrolling`.
- [ ] Support `.content_size(...)` as an explicit content-size override.
- [ ] Support `style_with_state(...)` with a typed style state.
- [ ] Add a public `ScrollTarget` trait with `offset`, `set_offset`, `content_size`, `viewport_size`, and drag lifecycle hooks.
- [ ] Implement `ScrollTarget` for `ScrollHandle`.
- [ ] Implement `ScrollTarget` for `UniformListScrollHandle`.
- [ ] Implement `ScrollTarget` for `ListState`, wiring its drag start/end hooks.
- [ ] Re-export ergonomic names from `primitives/scroll/mod.rs` and `primitives/mod.rs`.

### Geometry / thumb math

- [ ] Thumb length is `track_len * viewport_len / content_len`, clamped to a documented `MIN_THUMB_SIZE` minimum.
- [ ] Thumb offset maps `scroll_ratio` onto `track_len - thumb_len`, accounting for GPUI's zero-or-negative offset convention.
- [ ] Scrolling to content start places the thumb at the track start; scrolling to content end places it at the track end.
- [ ] A clamped thumb (at `MIN_THUMB_SIZE`) still reaches both track ends at the scroll extremes.
- [ ] The axis renders nothing when content does not overflow the viewport on that axis.
- [ ] When both axes are visible, the horizontal track reserves an end margin equal to the scrollbar thickness and a corner region is defined where the tracks meet.
- [ ] Geometry is computed in `prepaint` from the target's current offset/content/viewport sizes; no cached copies of scroll position live outside the runtime's last-observed facts.
- [ ] Drag mapping converts pointer delta over `track_len - thumb_len` back into a clamped `set_offset(...)`, preserving the other axis' offset.
- [ ] Track click jumps so the thumb centers on the click position, clamped to the valid range.
- [ ] All geometry functions are pure and unit-testable without a window.

### Pointer interaction behavior

- [ ] Mouse down on the thumb starts a drag, records the grab point inside the thumb, and stops propagation.
- [ ] Mouse move while dragging updates the scroll offset so the grab point stays under the pointer.
- [ ] Dragging past the track ends clamps at the scroll extremes without jumping.
- [ ] Mouse up anywhere ends the drag, including when released outside the scrollbar bounds.
- [ ] Drag start/end call the target's drag lifecycle hooks (required for `ListState`).
- [ ] Mouse down on the track outside the thumb jumps to the clicked position.
- [ ] Scroll-wheel events over the track keep working: the underlying container scrolls and the scrollbar refreshes its visibility facts.
- [ ] `set_offset` calls during drag are rate-limited to a documented maximum update rate.
- [ ] Hover facts distinguish pointer-over-track from pointer-over-thumb, per axis.
- [ ] Painting is clipped to the element bounds via content mask.
- [ ] Interaction only occurs while the bar is interactable (visible, or in hover mode); a fully faded-out bar does not swallow clicks.

### Visibility / fade behavior

- [ ] `Always` keeps the scrollbar visible whenever content overflows.
- [ ] `Hover` shows the scrollbar while the pointer is over the scrollbar region and hides it otherwise.
- [ ] `Scrolling` shows the scrollbar when the observed scroll offset changes.
- [ ] `Scrolling` keeps the scrollbar fully visible for the idle delay after the last scroll/hover activity.
- [ ] `Scrolling` fades the scrollbar out over the fade duration after the idle delay, then stops painting.
- [ ] Hovering the visible bar resets the idle clock; dragging keeps the bar visible regardless of timing.
- [ ] Fade is driven by `request_animation_frame` during the fade window plus at most one scheduled idle timer; no continuous polling while idle.
- [ ] Fade timing constants are module-level and documented.

### Architecture / implementation model

- [ ] Implement `Scrollbar` as a custom GPUI `Element`: absolute-positioned, filling its parent in `request_layout`; hitboxes and per-axis geometry in `prepaint`; painting and `window.on_mouse_event(...)` registration in `paint`.
- [ ] Keep thumb math, drag mapping, and visibility/fade decisions in the deep runtime/geometry module; the element translates events into runtime commands and paints from runtime queries.
- [ ] Keep interaction state in keyed window state addressed by `.id(...)`; the scrollbar never owns the scroll position.
- [ ] Access the scroll target only through the `ScrollTarget` trait; no downcasting or handle-specific branches in the element.
- [ ] Keep the primitive consumer-agnostic: no imports from `select/` or any component module.
- [ ] Avoid adding generic helpers unless they represent a repeated deep primitive concept.
- [ ] Add no new dependencies beyond what `base_gpui` already carries.

### Styling/state exposure

- [ ] Add a `ScrollbarStyleState` or equivalent typed style-state struct.
- [ ] Style state exposes `axis`/orientation, `hovering` (track and thumb), `scrolling` (offset recently changed), `dragging`, `has_overflow`, `at_start`, and `at_end` per axis.
- [ ] Style state exposes the current fade opacity or an equivalent visibility fact so consumers can style the fade.
- [ ] `style_with_state(...)` receives the current style state and controls track and thumb appearance (colors, thickness, inset, corner radius) with sensible defaults when omitted.
- [ ] Do not expose DOM data attributes as the styling API.
- [ ] Do not expose CSS variable names as the styling API.

### Docs/demo

- [ ] Update `crates/base_gpui/src/main.rs` or add an example rendering a scrollable container with a vertical scrollbar over a `ScrollHandle`.
- [ ] Add a demo of both axes plus the corner over an `.overflow_scroll()` container.
- [ ] Add a demo or documented example over a `uniform_list` via `UniformListScrollHandle`.
- [ ] Document the visibility modes and fade timing.
- [ ] Document the overlay composition pattern (relative container, absolute scrollbar layer) and cross-link the Scroll Area port issue as the intended compound consumer.

### Tests / verification

Add behavior tests under `crates/base_gpui/src/primitives/scroll/tests/` where practical; geometry tests need no window.

- [ ] Thumb length is proportional to viewport/content ratio.
- [ ] Thumb length clamps at `MIN_THUMB_SIZE` for very long content.
- [ ] Thumb offset is 0 at scroll start and `track_len - thumb_len` at scroll end, including when clamped.
- [ ] No-overflow content yields no thumb for that axis.
- [ ] Drag delta maps to the expected scroll offset and clamps at both extremes.
- [ ] Drag preserves the cross-axis offset.
- [ ] Track click centers the thumb on the click position, clamped at the ends.
- [ ] Both-axes layout reserves the horizontal end margin and defines the corner.
- [ ] Visibility: offset change marks scrolling; idle past delay begins fade; fade completes to hidden.
- [ ] Hover during the visible window resets the idle clock; drag forces visibility.
- [ ] Style state reports `has_overflow`, `at_start`, `at_end`, `hovering`, `dragging` correctly from runtime facts.
- [ ] `ScrollTarget` impl for `ScrollHandle` derives `content_size` as `max_offset() + bounds().size`.
- [ ] Rendered test (where practical): dragging the thumb scrolls a tracked container; releasing ends the drag.
- [ ] `cargo check -p base_gpui` passes.
- [ ] `cargo test -p base_gpui scroll` passes.
- [ ] `cargo test -p base_gpui` passes.

## Follow-ups

- Write and implement `issues/port-baseui-scroll-area.md`: Base UI Scroll Area anatomy (Root/Viewport/Content/Scrollbar/Thumb/Corner) composing this primitive.
- Adopt the primitive in Select's dropdown list, replacing or complementing the scroll arrows.
- Make fade delay/duration configurable per scrollbar if a consumer needs it.
- Auto-scroll while dragging past the container edge, and page-jump-on-track-press-and-hold semantics.
- Keyboard scrolling actions on the scrollable container (arrow/page/home/end) as a container-side concern.
- RTL/horizontal direction awareness once a direction provider exists.
- Revisit AccessKit scrollbar semantics when the GPUI dependency supports them.
