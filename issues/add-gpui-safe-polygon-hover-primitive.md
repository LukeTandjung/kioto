# Add GPUI-native Safe Polygon Hover Primitive

## Problem

Hover-opened popups need to survive the pointer's diagonal journey from the trigger to the popup. When a submenu opens to the right of its trigger, the natural pointer path cuts across neighboring menu items; when a navigation menu panel opens below its trigger, the pointer momentarily hovers neither the trigger nor the panel. Without hover-intent logic, the trigger's unhover schedules a close (or a sibling trigger's hover steals activation) and the popup disappears before the user reaches it.

Base UI solves this with `safePolygon` from its vendored floating-ui-react: on leaving the trigger, it records the pointer's exit point and constructs a quadrilateral between that point and the popup's near edge. While subsequent mouse moves stay inside that region, the close is withheld; the moment the pointer exits the region, or stalls without intent, the close runs. Three of our planned ports depend on this behavior:

- Menu submenus (`issues/port-baseui-menu.md`) — `menu/submenu-trigger/MenuSubmenuTrigger.tsx` uses `safePolygon({ blockPointerEvents: true })`;
- Menubar hover-switch (`issues/port-baseui-menubar.md`) — shares the Menu machinery;
- Navigation Menu (`issues/port-baseui-navigation-menu.md`) — `navigation-menu/trigger/NavigationMenuTrigger.tsx` uses `safePolygon(...)` with scoped elements.

Preview Card and hover-openable Popover triggers use it too, so this will keep recurring.

GPUI has no analog. `gpui-component`'s `popup_menu.rs` implements submenus without any safe-polygon logic — submenus flicker closed on diagonal traversal — so that is the baseline to beat, not a reference to port. Our own tooltip already ships a crude rectangular approximation (`point_in_safe_gap` in `crates/base_gpui/src/tooltip/runtime.rs`), which only extends the close delay when the pointer is in the axis-aligned gap between trigger and popup; it has no exit-point triangle, no intent detection, and closes on any diagonal path that leaves the gap band.

The goal is one reusable, unit-testable hover-intent primitive: pure point/bounds geometry plus a small arm/evaluate lifecycle, consumed by hover triggers and composed with the existing hover-delay timer substrate (`schedule_hover` / `cancel_hover` / `take_scheduled_hover` generations in `tooltip/runtime.rs`, `spawn_delayed_hover` in `tooltip/layers/tooltip_trigger.rs`, `spawn_popup_close` in `tooltip/layers/tooltip_popup.rs`). The timers decide *when* a pending close fires; safe polygon adds the geometric gate that decides *whether* it should still fire.

This is a GPUI-native primitive, not a DOM port: no `mouseleave`/`relatedTarget`/`event.target` containment, no `getBoundingClientRect`, no `pointer-events` CSS. Inputs are GPUI `Point<Pixels>` and `Bounds<Pixels>`; element containment falls out of bounds hit-testing.

## Scope

Add a reusable safe-polygon hover-intent primitive under `base_gpui` primitives:

- a public tracker type (working name `SafePolygon`) with an arm/evaluate/disarm lifecycle;
- a public verdict enum describing what the consumer should do with its pending close;
- a public side enum (`Top` / `Bottom` / `Left` / `Right`) describing where the popup sits relative to the trigger; consumers map their own side types (e.g. `TooltipSide`, future `MenuSide`) into it;
- pure geometry: side-dependent quadrilateral construction from exit point + popup bounds, point-in-quadrilateral test, trough-rectangle test, opposite-side exit test;
- intent detection: cursor-velocity check (slow-moving cursor inside the polygon loses the grace) and the landed/not-landed lifecycle;
- a documented integration contract for feeding it mouse-move positions and composing its verdicts with generation-based close timers;
- unit tests over pure point/bounds math, no window required.

The primitive renders nothing. It is not an `Element`, has no layers, and owns no GPUI entities — it is a plain struct consumed inside hover triggers' event handlers, the same tier as the geometry helpers already living inside `tooltip/runtime.rs`.

Primary references:

- `/home/luke/Projects/base-ui/packages/react/src/floating-ui-react/safePolygon.ts`
- `/home/luke/Projects/base-ui/packages/react/src/floating-ui-react/safePolygon.test.ts`
- `/home/luke/Projects/base-ui/packages/react/src/floating-ui-react/hooks/useHover.ts`
- `/home/luke/Projects/base-ui/packages/react/src/menu/submenu-trigger/MenuSubmenuTrigger.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/navigation-menu/trigger/NavigationMenuTrigger.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/trigger/PreviewCardTrigger.tsx`
- `/home/luke/Projects/kioto/crates/base_gpui/src/tooltip/runtime.rs` (hover generations, `point_in_safe_gap`)
- `/home/luke/Projects/kioto/crates/base_gpui/src/tooltip/layers/tooltip_trigger.rs` (`spawn_delayed_hover`)
- `/home/luke/Projects/kioto/crates/base_gpui/src/tooltip/layers/tooltip_popup.rs` (`spawn_popup_close`, hoverable popup)

## Initial design decisions

### Why this is a primitive and not per-component plumbing

`docs/base-gpui-component-architecture.md` warns against shared generic primitives for tiny plumbing. This is not that. The knowledge here is real and entangled: side-dependent quadrilateral corner selection, the trough rectangle between the two boxes, opposite-side exit rejection, cursor-velocity intent, and the landed lifecycle are one body of geometric hover-intent knowledge (Floating UI's implementation is ~450 lines of it), and it is needed verbatim by at least three component families. Duplicating it per component would scatter the exact kind of decision the architecture doc says to hide behind one deep module. The interface is tiny (arm, evaluate, disarm) relative to the knowledge inside — the right interface-to-knowledge ratio for a primitive.

### Placement

```text
crates/base_gpui/src/primitives/
  mod.rs
  safe_polygon/
```

Alongside `primitives/input/`. Do not put it in `utils/`: it owns state (armed region, landed flag, last cursor sample) and deep behavior, so it is a primitive, not a flat helper. Do not put it in `tooltip/`: tooltip is one prospective consumer among several, and the dependency must point from components to the primitive, never back.

### Public API shape

A plain struct with an explicit lifecycle, no GPUI entity types:

```rust
let mut tracker = SafePolygon::new(SafePolygonConfig::default());
tracker.arm(exit_point, trigger_bounds, popup_bounds, SafePolygonSide::Right);
match tracker.evaluate(pointer, now) {
    SafePolygonVerdict::Inside => { /* keep the pending close cancelled; re-arm the short grace timer */ }
    SafePolygonVerdict::Outside => { /* let the close run now */ }
    SafePolygonVerdict::LandedPopup => { /* disarm; popup-hover keep-open takes over */ }
    SafePolygonVerdict::LandedTrigger => { /* disarm; cancel the close entirely */ }
}
```

- `arm(...)` captures the pointer's exit point, both bounds, and the side. Arming replaces any previous armed region.
- `evaluate(pointer, now)` is called on every mouse-move frame while armed. `now` is an injected timestamp (`std::time::Instant` or a `Duration` since an arbitrary epoch) so velocity logic is unit-testable without real time.
- `disarm()` clears the armed region; an unarmed tracker always answers as if the gate is open (consumers fall back to plain close-delay behavior).
- Consumers own the tracker wherever their hover state already lives — for the planned Menu/Navigation Menu ports that is a field on the component runtime, which keeps it reachable from event handlers through the existing context `update` path.

### Geometry model (from Base UI, translated to `Bounds<Pixels>`)

Follow `safePolygon.ts` behavior, expressed in GPUI geometry:

- **Quadrilateral**: two points near the recorded exit point (offset by a small buffer, spread depending on whether the popup is wider/taller than the trigger and which half the cursor exited from) plus the two corners of the popup's near edge for the given side. Point containment uses the standard ray-casting edge-crossing test.
- **Trough rectangle**: the axis-aligned band strictly between the trigger's near edge and the popup's near edge (clamped to the narrower of the two boxes on the cross axis, with a 1px inset for rounding error). Inside the trough is always `Inside`, without a velocity check, so oscillating between trigger and popup never closes.
- **Opposite-side exit**: if the recorded exit point is on the far side of the trigger from the popup (e.g. popup on the right, pointer exited past the trigger's left edge), the region never applies — `Outside` immediately.
- **Landing**: a pointer inside the popup bounds yields `LandedPopup`; inside the trigger bounds yields `LandedTrigger`. After landing on the popup, leaving it again is the popup's unhover story (existing `set_popup_hovered` + close-delay machinery), not the polygon's.
- Constants start at Floating UI's values (`0.5px` polygon buffer, `0.1 px/ms` cursor speed threshold, `40ms` inside-polygon grace re-arm) and live in `SafePolygonConfig` with these defaults.

### Intent model

Inside the quadrilateral is not an unconditional pass. Matching Base UI:

- A cursor moving slower than the speed threshold between consecutive samples is treated as lost intent — `Outside` (the user parked mid-flight; hover-intent has expired).
- An `Inside` verdict is only a stay of execution: the consumer must re-schedule a short grace close (the 40ms re-arm) through the generation timer, so a pointer that stops moving inside the polygon still closes. The polygon never keeps a popup open forever on its own.

### Composition with the hover timer substrate

The primitive deliberately does not own timers. The tooltip's generation pattern is the substrate, and the same pattern is what Menu/Menubar/Navigation Menu runtimes will replicate:

1. On trigger unhover, the consumer records the exit point (`window.mouse_position()` — the same source `close_delay_for_trigger_unhover` uses today), arms the tracker with the trigger bounds, popup bounds, and effective side (from the runtime's measured `set_bounds` data and `effective_side`), and schedules a delayed close via `schedule_hover`-style generations.
2. On every mouse move during the grace window, the consumer feeds `evaluate(...)`:
   - `Inside` → cancel the pending generation and schedule a fresh short-grace close (Floating UI's 40ms), keeping the popup open only while movement continues;
   - `Outside` → let the pending close fire now: bump/consume the generation and run the close path immediately (still through the generation check so a concurrent re-hover stays cancel-safe);
   - `LandedPopup` / `LandedTrigger` → disarm; existing popup-hover / trigger-hover handlers cancel or keep the close as they already do.
3. Re-hovering the trigger or popup cancels everything via the existing `cancel_hover` path and disarms the tracker.

The safe region is therefore only ever active during the grace window between trigger-unhover and close-timer expiry — it gates the timer, it does not replace it.

### Feeding mouse moves

The pointer spends the interesting part of its journey over elements that belong to neither the trigger nor the popup, so per-element `on_mouse_move` on those two elements is not sufficient. The consuming component's root (or portal root) must observe moves across its whole area — a root-level `on_mouse_move` in the consumer, or a window-scoped mouse listener while armed — and forward positions to the tracker. Which mechanism to use is a consumer-side decision recorded in each consumer issue; this primitive only defines the contract: "call `evaluate` with every observed pointer position while armed." Settle the exact GPUI listener choice (root-element capture vs `window`-level observation) during the first consumer implementation and document it in the primitive's module docs.

### Fallback baseline

The tooltip's rectangular `point_in_safe_gap` + extended-close-delay approximation remains the documented cheap fallback: if a consumer decides full polygon fidelity is unnecessary, it can use the trough-rectangle test alone with a longer close delay. The primitive should expose the trough test on its own for exactly this downgrade path. Migrating tooltip itself onto the primitive is a follow-up, not part of this issue.

## Expected implementation files

```text
crates/base_gpui/src/primitives/mod.rs           (updated exports)
crates/base_gpui/src/primitives/safe_polygon/mod.rs
crates/base_gpui/src/primitives/safe_polygon/config.rs
crates/base_gpui/src/primitives/safe_polygon/tracker.rs
crates/base_gpui/src/primitives/safe_polygon/geometry.rs
crates/base_gpui/src/primitives/safe_polygon/tests/
```

Alternative filenames are fine if they preserve the same architecture: deep geometry + tracker state in the primitive, thin verdict-driven wiring in consumers, no rendering layer.

## Out of scope

- Do not implement any consumer wiring in this issue (Menu, Menubar, Navigation Menu, Tooltip adoption are their own issues).
- Do not implement touch or pen pointer handling; this is mouse hover-intent only.
- Do not port `blockPointerEvents` (Floating UI inserts a DOM overlay and toggles `pointer-events` CSS; a GPUI occlusion analog is a follow-up once a consumer proves the need).
- Do not port Floating UI's floating-tree awareness (`hasOpenChildNode` / `getNodeChildren`); whether a nested child popup is open is consumer runtime knowledge — the consumer simply refrains from closing (or from calling `evaluate`) while a child is open.
- Do not port DOM event mechanics: `mouseleave` vs `mousemove` distinction, `relatedTarget`, `event.target` containment. Containment is bounds hit-testing on `Bounds<Pixels>`.
- Do not chase exact Floating UI parity on buffer spreading heuristics beyond the documented constants; tune only if a consumer shows a concrete traversal failure.
- Do not add an `Element`, layers, style states, or `style_with_state` — nothing here draws.
- Do not add scroll-invalidation handling (bounds going stale mid-flight due to scrolling); consumers re-arm with fresh bounds from their measurement pass.
- Do not own timers, executors, or GPUI entities inside the primitive; scheduling stays in consumer runtimes via the generation pattern.

## Acceptance Criteria

### Module/API surface

- [x] Add `crates/base_gpui/src/primitives/safe_polygon/` and export it from `crates/base_gpui/src/primitives/mod.rs`.
- [x] Add a public `SafePolygon` tracker type with `new(config)`, `arm(...)`, `evaluate(...)`, `disarm()`, and an `is_armed()` query.
- [x] Add a public `SafePolygonVerdict` enum with `Inside`, `Outside`, `LandedPopup`, and `LandedTrigger` variants.
- [x] Add a public `SafePolygonSide` enum (`Top`, `Bottom`, `Left`, `Right`) local to the primitive; no dependency on `tooltip` types.
- [x] Add a public `SafePolygonConfig` with polygon buffer, cursor speed threshold, and inside-polygon grace duration, defaulting to Floating UI's constants (0.5px, 0.1 px/ms, 40ms).
- [x] Expose the trough-rectangle test as a standalone public function usable as the documented cheap fallback.
- [x] `evaluate` takes an injected timestamp so no test needs real time or a window.
- [x] The primitive uses only `gpui::{Point, Pixels, Bounds}` (and std); no entities, no `Window`, no `App`, no timers.

### Geometry behavior

- [x] Point-in-quadrilateral uses an edge-crossing containment test and is correct for non-convex vertex orderings produced by the corner selection.
- [x] For each side (`Top`, `Bottom`, `Left`, `Right`), the quadrilateral connects two buffer-offset points at the exit position to the popup's near-edge corners, matching Base UI's corner selection including the wider/taller-popup and exit-half adjustments.
- [x] A pointer inside the trough rectangle between trigger and popup near edges yields `Inside` regardless of cursor speed.
- [x] An exit point on the opposite side of the trigger from the popup yields `Outside` on the next evaluation (with the 1px rounding tolerance).
- [x] A pointer inside the popup bounds yields `LandedPopup`.
- [x] A pointer inside the trigger bounds yields `LandedTrigger`.
- [x] A pointer outside the quadrilateral, trough, trigger, and popup yields `Outside`.
- [x] A straight diagonal path from a trigger's edge to the near corner of a popup positioned on any of the four sides evaluates `Inside` at every sampled point.
- [x] Popup bounds that do not overlap the trigger on either axis (diagonal placements) degrade sanely: the side parameter picks the governing axis and the region still covers the direct path.

### Intent/lifecycle behavior

- [x] Consecutive samples moving slower than the configured speed threshold while inside the quadrilateral (and not in the trough) yield `Outside`.
- [x] The first sample after arming never fails the velocity check (no previous sample to compare).
- [x] `Inside` is documented and tested as requiring the consumer to re-schedule the short grace close; the tracker itself never suppresses a close indefinitely.
- [x] After `LandedPopup` or `LandedTrigger`, the tracker is disarmed and subsequent `evaluate` calls report the unarmed behavior until re-armed.
- [x] `arm` while already armed replaces the previous region and resets the velocity sample history.
- [x] `disarm` clears all state; an unarmed tracker's `evaluate` is side-effect free.
- [x] Zero-duration or degenerate inputs (empty bounds, exit point inside the popup) do not panic and produce a defined verdict.

### Integration contract (documented, and exercised by consumer issues)

- [x] Module docs specify the arm-on-unhover / evaluate-per-move / verdict-to-generation-timer protocol, referencing the `schedule_hover` / `cancel_hover` / `take_scheduled_hover` generation pattern from `crates/base_gpui/src/tooltip/runtime.rs`.
- [x] Module docs specify that `Inside` maps to cancel-and-reschedule of a short grace close, `Outside` maps to letting/forcing the pending close through its generation check, and `Landed*` maps to disarm plus the consumer's existing hover keep-open paths.
- [x] Module docs specify that mouse moves must be observed at the consumer's root (or window) scope while armed, since the pointer traverses space owned by neither trigger nor popup, and record the chosen GPUI listener mechanism once the first consumer lands.
- [x] Module docs name the consumers and cross-link `issues/port-baseui-menu.md`, `issues/port-baseui-menubar.md`, and `issues/port-baseui-navigation-menu.md`.
- [x] Module docs record the trough-only + extended close delay approximation (tooltip's current `point_in_safe_gap` behavior) as the sanctioned lower-fidelity fallback.
- [x] Side/align coverage: consumers with align offsets (e.g. submenu aligned to item top) only need side; verify the region is align-agnostic because it is built from popup bounds, not anchor math.

### Architecture / implementation model

- [x] All geometric and intent knowledge lives in the primitive; consumers see only `arm`/`evaluate`/`disarm` and verdicts.
- [x] No getter/setter pairs exposing internal region state; queries are verdict- or fact-shaped (`is_armed`).
- [x] No dependency from `primitives/safe_polygon` on any component module (`tooltip`, future `menu`, etc.); dependencies point inward only.
- [x] `mod.rs` files are barrel exports only.
- [x] No new external crate dependencies; the math is std + gpui geometry types.

### Docs/demo

- [x] Add a short demo or doc example showing a hover popup that stays open across a diagonal traversal (can live behind the existing `crates/base_gpui/src/main.rs` demo shell), or explicitly defer the visual demo to the first consumer issue with a note in this issue. *Note: visual demo deferred to the first consumer issue (Menu submenus, `issues/port-baseui-menu.md`) — the primitive renders nothing, so a meaningful demo requires a hover-popup consumer.*
- [x] Document the configuration defaults and what each knob trades off.

### Tests / verification

Add unit tests under `crates/base_gpui/src/primitives/safe_polygon/tests/`; all pure math, no window.

- [x] Quadrilateral containment: interior point inside, exterior point outside, for all four sides.
- [x] Trough containment for all four sides, including the 1px insets.
- [x] Opposite-side exit yields `Outside` for all four sides.
- [x] Diagonal traversal sample paths (trigger corner to popup near corner) stay `Inside` for all four sides.
- [x] Perpendicular exit away from the popup yields `Outside`.
- [x] Slow-cursor samples inside the polygon yield `Outside`; the same positions at high speed yield `Inside`.
- [x] Trough position with slow cursor still yields `Inside`.
- [x] Landing transitions: entering popup bounds yields `LandedPopup` and disarms; entering trigger bounds yields `LandedTrigger` and disarms.
- [x] Re-arm replaces region and resets velocity history.
- [x] Unarmed `evaluate` behavior is stable and documented.
- [x] Degenerate bounds (zero-size popup, exit point already inside popup) do not panic.
- [x] Wider-popup and narrower-popup corner-selection variants both tested against expected containment.
- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui safe_polygon` passes.
- [x] `cargo test -p base_gpui` passes.

## Follow-ups

- Wire the primitive into Menu submenu triggers, Menubar hover-switch, and Navigation Menu triggers as those ports land (their issues own the wiring criteria).
- Migrate tooltip's `point_in_safe_gap` extended-close-delay approximation onto the primitive (or its trough fallback) and delete the private duplicate.
- Evaluate a GPUI analog of `blockPointerEvents` (occluding the traversal region so underlying siblings don't react mid-flight) once a consumer demonstrates sibling-hover interference.
- Expose buffer/threshold tuning per consumer if defaults prove wrong for dense menus.
- Consider touch/pen semantics if any consumer needs press-drag traversal.
- Consider a nested-floating-tree helper if consumer-side "child popup open" suppression turns out to be repeated boilerplate across Menu and Navigation Menu.
