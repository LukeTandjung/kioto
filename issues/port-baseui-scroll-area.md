# Port Base UI Scroll Area to GPUI

## Problem

Base UI's Scroll Area is a custom-styled scroll container: a `Root` that groups
everything and tracks interaction facts, a `Viewport` that is the actual scrollable
element (native overflow scrolling with the OS scrollbar visually disabled), an
optional `Content` wrapper so horizontal overflow measures correctly, per-axis
`Scrollbar` tracks with a draggable `Thumb`, and a `Corner` square where the two
scrollbars meet. Its behavioral contract, per part:

- **Root** (`root/ScrollAreaRoot.tsx`) owns `hovering`, per-axis `scrollingX` /
  `scrollingY` flags cleared 500ms after the last scroll (`SCROLL_TIMEOUT`,
  `constants.ts`), overflow-edge flags (`overflowXStart/XEnd/YStart/YEnd`) gated by
  a configurable `overflowEdgeThreshold`, thumb/corner sizes, and hosts the shared
  thumb-drag pointer handlers.
- **Viewport** (`viewport/ScrollAreaViewport.tsx`) is the scroll container. It
  computes thumb size/position on every scroll/resize (`computeThumbPosition`),
  derives the per-axis hidden state (`clientSize >= scrollSize`), derives overflow
  edges, and joins/leaves the tab order depending on whether anything is scrollable.
- **Content** (`content/ScrollAreaContent.tsx`) is a `min-width: fit-content`
  wrapper so horizontal overflow is measurable, re-triggering thumb recompute when
  its size changes.
- **Scrollbar** (`scrollbar/ScrollAreaScrollbar.tsx`) is a vertical or horizontal
  track: unmounted when its axis has no overflow unless `keepMounted`, wheel events
  over the track scroll the viewport (chaining to the parent at the edges), and
  pointer-down on the track outside the thumb jumps to the clicked position.
- **Thumb** (`thumb/ScrollAreaThumb.tsx`) drags to scroll; dragging keeps the axis
  marked as scrolling.
- **Corner** (`corner/ScrollAreaCorner.tsx`) renders the intersection square sized
  from the two scrollbars, hidden unless both axes overflow.

Everything Base UI implements by hand on top of the DOM splits into two halves in
GPUI, and this issue only owns the first half:

1. **Scroll Area behavior** (this issue): the compound part anatomy, scroll and
   hover activity state, overflow/edge state, mount/visibility policy, and corner
   sizing — built over GPUI's native `ScrollHandle`
   (`div().id(...).overflow_scroll().track_scroll(&handle)`, with `offset()`,
   `max_offset()`, `bounds()`).
2. **Scrollbar rendering and drag** (`issues/add-gpui-scrollbar-primitive.md`): the
   custom `Element` that paints a track and thumb, thumb length/offset math, drag
   mapping, track-click jump, and wheel-over-track forwarding. Scroll Area
   **composes** that primitive; it must not reimplement thumb geometry, drag
   anchoring, or `MIN_THUMB_SIZE` clamping. That issue lists this port as its first
   consumer; keep the two cross-linked.

The in-repo precedent for deriving edge state from a `ScrollHandle` is Select:
`refresh_scroll_arrow_visibility` in `crates/base_gpui/src/select/runtime.rs`
(offset is zero-or-negative, `max_offset()` positive, at-end when
`-offset.y >= max_offset.y`) consumed by
`crates/base_gpui/src/select/layers/select_list.rs`. Scroll Area generalizes that
pattern to both axes plus thresholds.

There is no existing `crates/base_gpui/src/scroll_area/` implementation; this is a
fresh port. Scroll Area carries no value semantics, so no `T` type parameter is
needed — parts are plain (non-generic) types.

Complexity: medium — no controlled/uncontrolled value, no keyboard model of its
own, but real timer-driven state, per-axis mount policy, and a composition boundary
with the scrollbar primitive that must stay clean.

## Scope

Port the Scroll Area component family from Base UI into GPUI-native components:

- `ScrollAreaRoot`
- `ScrollAreaViewport`
- `ScrollAreaContent`
- `ScrollAreaScrollbar` (one part, `orientation` vertical/horizontal)
- `ScrollAreaThumb`
- `ScrollAreaCorner`

Base UI source references:

- `/home/luke/Projects/base-ui/packages/react/src/scroll-area/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/scroll-area/constants.ts` (`SCROLL_TIMEOUT = 500`, `MIN_THUMB_SIZE = 16`)
- `/home/luke/Projects/base-ui/packages/react/src/scroll-area/root/ScrollAreaRoot.tsx` (+ `ScrollAreaRootContext.ts`, `stateAttributes.ts`)
- `/home/luke/Projects/base-ui/packages/react/src/scroll-area/viewport/ScrollAreaViewport.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/scroll-area/content/ScrollAreaContent.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/scroll-area/scrollbar/ScrollAreaScrollbar.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/scroll-area/thumb/ScrollAreaThumb.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/scroll-area/corner/ScrollAreaCorner.tsx`
- Behavioral test references: `root/ScrollAreaRoot.test.tsx`,
  `viewport/ScrollAreaViewport.test.tsx`, `scrollbar/ScrollAreaScrollbar.test.tsx`,
  `thumb/ScrollAreaThumb.test.tsx`, `corner/ScrollAreaCorner.test.tsx`,
  `content/ScrollAreaContent.test.tsx`

GPUI dependency (consumed, not implemented here):

- `issues/add-gpui-scrollbar-primitive.md` — `scrollbar()` /
  `crates/base_gpui/src/primitives/scroll/`: the custom scrollbar `Element`, the
  `ScrollTarget` trait, thumb geometry, drag mapping, track-click jump,
  wheel-over-track. Implement that issue first (or in lockstep); this port is its
  first consumer.

In-repo precedents:

- `crates/base_gpui/src/select/runtime.rs` (`refresh_scroll_arrow_visibility` —
  ScrollHandle-driven edge state)
- `crates/base_gpui/src/select/layers/select_list.rs`
  (`overflow_y_scroll().track_scroll(...)` usage)
- `docs/base-gpui-component-architecture.md` (flat module layout, deep runtime,
  thin context/parts)

Expected new GPUI files (flat layout, no nested taxonomies):

- `crates/base_gpui/src/scroll_area/mod.rs` — barrel exports only.
- `crates/base_gpui/src/scroll_area/runtime.rs` — `ScrollAreaRuntime`: owns the
  `ScrollHandle`, hover/scrolling facts, overflow-edge derivation, hidden state,
  corner facts.
- `crates/base_gpui/src/scroll_area/context.rs` — `ScrollAreaContext` (thin
  injection: `read` / `update`).
- `crates/base_gpui/src/scroll_area/props.rs` — `overflow_edge_threshold`, timing
  configuration if any.
- `crates/base_gpui/src/scroll_area/style_state.rs` — one style-state struct per
  drawing part (root/viewport/content share a shape; scrollbar and thumb extend it;
  corner has its own).
- `crates/base_gpui/src/scroll_area/child.rs` — typed child enums
  (`ScrollAreaChild`, `ScrollAreaScrollbarChild`).
- `crates/base_gpui/src/scroll_area/layers/mod.rs`
- `crates/base_gpui/src/scroll_area/layers/scroll_area_root.rs`
- `crates/base_gpui/src/scroll_area/layers/scroll_area_viewport.rs`
- `crates/base_gpui/src/scroll_area/layers/scroll_area_content.rs`
- `crates/base_gpui/src/scroll_area/layers/scroll_area_scrollbar.rs`
- `crates/base_gpui/src/scroll_area/layers/scroll_area_thumb.rs`
- `crates/base_gpui/src/scroll_area/layers/scroll_area_corner.rs`
- `crates/base_gpui/src/scroll_area/tests/`
- Register `pub mod scroll_area;` in `crates/base_gpui/src/lib.rs` (plus
  `scroll_area::init(cx);` only if actions turn out to be needed — none are
  expected; Scroll Area has no keyboard model of its own).

## Out of scope / drop from Base UI

- **Thumb geometry, drag mapping, track-click jump, wheel-over-track,
  `MIN_THUMB_SIZE` clamping** — these belong to the scrollbar primitive
  (`issues/add-gpui-scrollbar-primitive.md`). Scroll Area must not duplicate
  `computeThumbPosition`'s thumb-size/thumb-offset math, Root's
  `handlePointerDown/Move/Up` drag handlers, or Scrollbar's wheel/track-press
  logic. If the primitive's behavior needs a change to serve Scroll Area, amend the
  primitive issue rather than forking the math here.
- React context/hooks (`ScrollAreaRootContext`, `ScrollAreaViewportContext`,
  `ScrollAreaScrollbarContext`, `useStableCallback`, `useTimeout`,
  `useIsoLayoutEffect`) — replaced by the runtime/context/parts architecture.
- `render` prop, `className`, web `style` props.
- CSS variable APIs (`ScrollAreaRootCssVars` corner width/height,
  `ScrollAreaScrollbarCssVars` thumb width/height,
  `ScrollAreaViewportCssVars` overflow px vars) and the
  `CSS.registerProperty` inheritance optimization — corner size and overflow
  distances become runtime facts exposed through style-state structs and layout,
  not variables.
- DOM data attributes (`data-hovering`, `data-scrolling`, `data-has-overflow-x/y`,
  `data-overflow-x/y-start/end`, `data-orientation`, `data-corner-hidden`,
  `data-id`) — mapped into typed style-state fields.
- `styleDisableScrollbar` / CSP nonce handling — GPUI's `overflow_scroll` draws no
  native scrollbar, so there is nothing to visually disable.
- `ResizeObserver`, `getAnimations`/animation settling, `queueMicrotask`
  scheduling, `getBoundingClientRect`, `getOffset` padding/margin probing —
  replaced by GPUI layout/prepaint measurement (`ScrollHandle::bounds()` /
  `max_offset()`, `on_children_prepainted` where needed).
- `translate3d` thumb transforms — the primitive paints the thumb at a computed
  offset.
- SSR/hydration and the `hasMeasuredScrollbar` first-paint-hiding machinery in its
  DOM form. GPUI has no hydration flash; keep only whatever first-frame guard the
  measurement flow genuinely needs (see Content measurement criteria).
- Touch modality (`touchModality`, `pointerType === 'touch'`, `setPointerCapture`,
  `pointercancel`) — GPUI input is pointer/trackpad; the primitive handles capture
  semantics via `window.on_mouse_event`.
- RTL / negative-`scrollLeft` normalization and `useDirection` — revisit when a
  direction provider (`issues/port-baseui-direction-provider.md`) reaches
  scrolling.
- ARIA (`role="presentation"`, scrollable-region-focusable tabIndex as a DOM
  attribute) — no DOM ARIA. The tabIndex *behavior* (viewport focusable only when
  scrollable) is translated to GPUI focus (see Viewport criteria); the AccessKit
  role work is deferred to the standard follow-up.

## Composition with the scrollbar primitive (decide before implementing)

The primitive is a self-contained overlay `Element` with its own visibility policy
(`Scrolling`/`Hover`/`Always`) and its own hover/drag state keyed by `.id(...)`.
Base UI's Scroll Area instead makes visibility a *styling* concern: the parts
always render (when their axis overflows), expose `hovering`/`scrolling` state, and
user CSS decides opacity/transition. Preserve Base UI's contract:

- `ScrollAreaScrollbar` renders the primitive with its visibility policy pinned to
  `Always` (the primitive never hides itself); showing/hiding is driven by the
  consumer's `style_with_state` over the Scroll Area's `hovering`/`scrolling`
  fields, matching Base UI's CSS-driven pattern.
- `ScrollAreaThumb` supplies thumb styling to the primitive; it does not own drag
  handlers. If the primitive's `style_with_state` granularity cannot express
  separate track vs thumb styling closures from two different parts, resolve it on
  the primitive side (e.g. separate track/thumb style hooks) and record the
  decision in both issues.
- Scroll offset state lives in the runtime's `ScrollHandle` only; the primitive
  reads/writes it through its `ScrollTarget` implementation for `ScrollHandle`.
  The runtime never mirrors the offset except as last-observed facts for
  scrolling-flag detection.
- Drag/track/wheel interactions performed by the primitive must still feed the
  Scroll Area's activity clock: the scrolling flags are derived from observed
  offset changes (see below), so primitive-driven scrolling marks the axis as
  scrolling without any callback plumbing between the two.

## Acceptance Criteria

### Module / API surface

- [x] `crates/base_gpui/src/scroll_area/` exists with the flat layout above and is
      registered in `crates/base_gpui/src/lib.rs`.
- [x] `ScrollAreaRoot` builder exists with `id(...)`,
      `overflow_edge_threshold(...)` (uniform value or per-edge
      `xStart`/`xEnd`/`yStart`/`yEnd` struct, negatives clamped to zero, default
      0), `child`/`children`, and `style_with_state(...)`.
- [x] `ScrollAreaViewport` builder exists with `child`/`children` and
      `style_with_state(...)`.
- [x] `ScrollAreaContent` builder exists with `child`/`children` and
      `style_with_state(...)`.
- [x] `ScrollAreaScrollbar` builder exists with
      `orientation(ScrollAreaOrientation::Vertical | Horizontal)` (default
      vertical), `keep_mounted(bool)` (default false), typed thumb child, and
      `style_with_state(...)`.
- [x] `ScrollAreaThumb` builder exists with `style_with_state(...)`.
- [x] `ScrollAreaCorner` builder exists with `style_with_state(...)`.
- [x] Typed child enums: `ScrollAreaChild` routes
      `Viewport`/`Scrollbar`/`Corner` (plus an `AnyElement` escape hatch only if
      Base UI examples show arbitrary root children); `ScrollAreaScrollbarChild`
      routes `Thumb`. `Content` is a typed child of `Viewport`.
- [x] No `pub(crate)`/`pub(super)` visibility modifiers; module boundaries are
      public API or private.
- [x] `mod.rs` files are barrel exports only.
- [x] `sg scan` (repo `sgconfig.yml` rules) reports no violations in the new
      module.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui` passes.
- [x] A demo in `crates/base_gpui/src/main.rs` (or an example) mounts Root >
      Viewport > Content with overflowing content, both scrollbars with thumbs,
      and a corner, with hover/scroll-driven scrollbar styling.
- [x] No thumb-geometry or drag-mapping code exists under `scroll_area/`; grep for
      thumb-size math finds it only in `primitives/scroll/`.

### Architecture / composition

- [x] `ScrollAreaRuntime` is the single deep module: it owns the `ScrollHandle`,
      hover fact, per-axis scrolling facts and their deadlines, last-observed
      offset, overflow-edge flags, per-axis hidden state, and corner size. Plain
      `&mut self`/`&self` methods, no GPUI entity types, unit-testable without a
      window.
- [x] Runtime interface is commands + queries only (e.g.
      `observe_scroll(offset, max_offset, now)`, `set_hovering(bool)`,
      `refresh_overflow(offset, max_offset, viewport_bounds)`,
      `expire_scrolling(now)`; `root_state()`, `viewport_state()`,
      `scrollbar_state(orientation)`, `thumb_state(orientation)`,
      `corner_state()`). No getter/setter pairs; parts ask part-shaped questions.
- [x] `ScrollAreaContext` is the thin injection vehicle
      (`Entity<ScrollAreaRuntime>` + `Rc<ScrollAreaProps>`), exposing only
      `read`/`update`. Scroll Area has no value-changing `select`-analogue; the
      context grows no component vocabulary.
      *(One addition: `refresh`, an `update` variant that notifies only when
      the command reports a change, used by render-top reconciliation and
      layout observation so repaints settle. No component vocabulary.)*
- [x] `ScrollAreaViewport` renders
      `div().id(...).overflow_scroll().track_scroll(&handle)` with the runtime's
      `ScrollHandle`; per-axis overflow derives from
      `max_offset()`/`bounds()`, not from any hand-measured content size.
- [x] `ScrollAreaScrollbar`/`ScrollAreaThumb` compose the `scrollbar()` primitive
      from `issues/add-gpui-scrollbar-primitive.md` over that same `ScrollHandle`
      via `ScrollTarget`, per the Composition section; the primitive's visibility
      policy is pinned to `Always` and Scroll Area state drives styling.
- [x] Child wiring (which children exist, context attachment) happens once in the
      root render before erasure to `AnyElement`; no index bookkeeping leaks into
      parts.
- [x] State mutates only at the top of root render (reconcile/refresh) and in
      event/timer handlers via context commands.

### Scroll activity / overflow-edge state

- [x] A module-level documented `SCROLL_TIMEOUT` constant of 500ms exists.
- [x] When the observed vertical offset changes, `scrolling_y` becomes true; when
      the horizontal offset changes, `scrolling_x` becomes true. A change on one
      axis does not mark the other axis as scrolling.
- [x] Each axis's scrolling flag clears 500ms after that axis's *last* observed
      scroll activity (deadline extends on continued scrolling), driven by at most
      one scheduled timer per axis (or one shared timer) — no per-frame polling
      while idle. The deadline decision (`expire_scrolling(now)`) lives in the
      runtime so it is unit-testable with injected times.
- [x] Scrolling caused by the primitive (thumb drag, track click, wheel over
      track) marks the axis as scrolling through the same observed-offset path —
      no special-case plumbing.
- [x] Overflow-edge flags per axis: `overflow_y_start` is true when the scrolled
      distance from the start exceeds the `y_start` threshold;
      `overflow_y_end` when the remaining distance to the end exceeds the
      `y_end` threshold; likewise for x. With the default threshold 0 this reduces
      to "not at that edge", matching Select's
      `refresh_scroll_arrow_visibility` sign conventions
      (offset zero-or-negative; at end when `-offset >= max_offset`).
- [x] Edge flags are false for an axis with no overflow.
- [x] Edge flags refresh on scroll and on layout changes that alter
      `max_offset` (content or viewport resize), and refresh commands return
      whether anything changed so parts only notify when needed.

### Scrollbar visibility / mount / corner

- [x] `hovering` becomes true while the pointer is inside the Root's bounds and
      false when it leaves (Root-level hover, not viewport-only, matching Base
      UI's root `onPointerEnter`/`onPointerLeave` + child containment).
- [x] An axis is hidden when its content does not overflow the viewport
      (`max_offset` on that axis is zero, the GPUI analogue of
      `clientSize >= scrollSize`).
- [x] `ScrollAreaScrollbar` renders nothing when its axis is hidden, unless
      `keep_mounted(true)`, in which case it stays in the tree with
      `has_overflow_*` false in its style state.
- [x] A `keep_mounted` scrollbar whose axis starts hidden appears (gains overflow
      state) without user scrolling once content grows to overflow, and a
      non-`keep_mounted` scrollbar mounts under the same condition.
- [x] The corner is hidden unless both axes have overflow
      (`corner_hidden = hidden_x || hidden_y`).
- [x] Corner size derives from the rendered scrollbar thicknesses (vertical
      scrollbar width × horizontal scrollbar height), measured via GPUI
      layout/prepaint, and resets to zero when either axis loses overflow.
- [x] The vertical scrollbar reserves the corner height at its end and the
      horizontal scrollbar reserves the corner width, so tracks never overlap the
      corner (Base UI's `bottom: var(--corner-height)` / `inset-inline-end:
      var(--corner-width)` translated to layout insets; coordinate with the
      primitive's both-axes end-margin criterion rather than double-reserving).

### Viewport / focus behavior

- [x] The Viewport is the single scroll container; wheel/trackpad scrolling over
      the viewport scrolls it natively via `overflow_scroll` (no re-implemented
      wheel handling at the viewport level).
- [x] The Viewport participates in focus (tab stop with a `FocusHandle`) only when
      at least one axis is scrollable, and drops out of the tab order when nothing
      scrolls — the GPUI translation of Base UI's conditional `tabIndex: 0 / -1`.
- [x] Wheel-over-scrollbar-track scrolling is delegated to the primitive and stays
      functional through the composition (see primitive issue; verify, do not
      reimplement).

### Content measurement

- [x] `ScrollAreaContent` wraps children so intrinsic content width is measurable
      for horizontal overflow (the GPUI analogue of `min-width: fit-content`):
      the content must be allowed to exceed the viewport width instead of being
      clamped by it, so `max_offset().x` reflects true overflow.
- [x] Content growth/shrink updates overflow, edge flags, and scrollbar mount
      state on the next frame without user scrolling (Base UI uses a
      `ResizeObserver`; use GPUI layout/prepaint observation, e.g.
      `on_children_prepainted` or `max_offset` refresh during viewport prepaint).
- [x] No first-frame flash of a wrong-sized thumb: whatever first-measurement
      guard is needed lives in the runtime as a fact (measured yet or not), not as
      a visibility hack in parts.
- [x] Scroll Area works without a Content part for vertical-only use (Content is
      optional, as in Base UI).

### Styling / state exposure

- [x] `ScrollAreaRootStyleState` (shared by Root, Viewport, and Content queries)
      exposes: `scrolling` (either axis), `has_overflow_x`, `has_overflow_y`,
      `overflow_x_start`, `overflow_x_end`, `overflow_y_start`, `overflow_y_end`,
      `corner_hidden`.
- [x] `ScrollAreaScrollbarStyleState` extends that with `hovering`, per-axis
      `scrolling` (the scrollbar's own orientation), and `orientation`.
- [x] `ScrollAreaThumbStyleState` exposes `scrolling` (own orientation) and
      `orientation`.
- [x] `ScrollAreaCornerStyleState` exposes the corner size facts needed to style
      it (or the corner is styled purely by layout; decide and document).
- [x] Style-state structs live in `style_state.rs`, are returned by runtime
      queries, and feed `style_with_state(...)`; no DOM data attributes, CSS
      variables, or `className` in the public surface.
- [x] The demo styles scrollbar opacity from `hovering || scrolling`, reproducing
      Base UI's canonical show-on-hover/show-while-scrolling recipe purely through
      `style_with_state`.

### Tests / verification

Runtime tests need no window (inject offsets/bounds/times); rendered tests under
`crates/base_gpui/src/scroll_area/tests/` where practical.

- [x] Observing a vertical offset change sets `scrolling_y` and not `scrolling_x`,
      and vice versa.
- [x] Scrolling flag clears exactly at `SCROLL_TIMEOUT` past the last activity and
      the deadline extends on continued activity.
- [x] Overflow-edge flags: at start only `*_end` is set, at end only `*_start`,
      mid-scroll both, no-overflow neither (default threshold).
- [x] Non-zero thresholds: an edge flag stays false until the scrolled/remaining
      distance exceeds its threshold; negative threshold inputs clamp to zero;
      uniform-number input applies to all four edges.
- [x] Hidden state: `max_offset == 0` on an axis hides that axis;
      `corner_hidden` is true unless both axes overflow.
- [x] `keep_mounted(false)` scrollbar is absent without overflow and present with
      it; `keep_mounted(true)` is always present with correct `has_overflow_*`.
- [x] Corner size equals the scrollbar thicknesses when both axes overflow and
      zero otherwise.
- [x] Content resize (max_offset change) updates overflow/edges/mount state
      without a scroll event.
- [x] Hover enter/leave over the root toggles `hovering` in scrollbar style
      state.
- [x] Viewport is a tab stop only while scrollable.
- [ ] Rendered test (where practical): wheel-scrolling the viewport moves the
      composed primitive's thumb and flips edge state; dragging the primitive
      thumb (covered by the primitive's own tests) marks the Scroll Area as
      scrolling via observed offsets.
      *(Not practical: no scroll-wheel simulation is used in this repo's GPUI
      test harness. Covered instead by rendered tests for overflow/edge
      derivation and mount policy (`tests/rendered_scroll_area.rs`,
      `tests/scrollbar_mounting.rs`) plus runtime tests over injected
      offsets; primitive-driven scrolling flows through the same
      `observe_scroll` path unit-tested in `tests/scrolling_axis_flags.rs`.)*
- [x] Style-state structs report the full field sets above from runtime facts.
- [x] `cargo test -p base_gpui scroll_area` passes.

## AccessKit accessibility follow-up

Base UI marks Root/Viewport/Content as `role="presentation"` and keeps
non-scrollable viewports out of the tab order. GPUI does not emit DOM ARIA; when
the project updates to a GPUI revision with AccessKit support, revisit scrollable
region semantics (scrollable node, scroll offsets/extents) alongside the scrollbar
primitive's AccessKit follow-up. Not part of this issue.

## Cross-links

- `issues/add-gpui-scrollbar-primitive.md` — must land first (or in lockstep);
  this port composes `scrollbar()` and its `ScrollTarget for ScrollHandle` impl,
  and is listed there as the first consumer. Any track/thumb styling-granularity
  changes needed for the Thumb part are recorded in both issues.
- `issues/port-baseui-select.md` — Select's scroll arrows are the current in-repo
  ScrollHandle edge-state precedent; do not rework Select here.
- `issues/port-baseui-direction-provider.md` — RTL scrolling deferred until a
  direction provider reaches scroll components.

## Uncertain items needing confirmation

- Whether the primitive's single `style_with_state` closure suffices for the
  Scrollbar (track) and Thumb parts to contribute styling separately, or the
  primitive should grow distinct track/thumb style hooks. Resolve on the primitive
  side; default assumption is separate hooks.
- Whether the 500ms scrolling-flag timer should live as one shared deferred task
  re-armed on activity or one per axis. Either satisfies the criteria; pick the
  simpler one that keeps `expire_scrolling(now)` unit-testable.
- Whether Corner needs its own style-state struct or pure layout sizing is enough
  (Base UI's Corner exposes an empty state object).
