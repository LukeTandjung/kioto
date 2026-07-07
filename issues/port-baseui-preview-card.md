# Port Base UI Preview Card to GPUI

## Problem

Base UI's React Preview Card is a link-preview popup: hovering or focusing a link-like trigger opens a rich card after a longer-than-tooltip delay (600ms open / 300ms close), the pointer can travel into the card without it closing (safe-polygon-guarded hover close), and the card dismisses on outside press or Escape. It supports controlled and uncontrolled open state, multiple triggers with typed payloads, detached triggers through a handle with imperative `open(triggerId)` / `close()` / `isOpen`, portal mounting, anchored positioning with collision handling, an arrow, a presentation-only backdrop, and a viewport for trigger-to-trigger content transitions.

`base_gpui` currently has no Preview Card component family. Behaviorally it is a thin blend of two already-ported families and introduces no new primitive:

- From Tooltip it takes the hover-open/hover-close delay model, hoverable popup persistence, focus-open, the provider-less root (Preview Card has no delay-group provider), and the viewport.
- From Popover it takes the arrow, the backdrop part (adapted: Preview Card's backdrop is presentation-only and never captures outside clicks), the outside-press/escape dismissal path, the detached handle, and payload-carrying triggers.

Unlike Popover, Preview Card has no title/description/close parts, no modal mode, and no focus trap. Unlike Tooltip, its trigger is a link-like element that opens on mouse hover only (`mouseOnly: true` in Base UI's hover interaction) and on focus, never on press; its delays are longer; and its `instantType` is limited to `focus` and `dismiss`. The gpui-component analog `/home/luke/Projects/gpui-component/crates/ui/src/hover_card.rs` uses the same 600ms/300ms defaults and is useful local precedent for the trigger/content hover persistence loop.

The goal is to port Preview Card behavior and contracts, not React or DOM internals. Web-specific implementation details such as React hooks, stores, Floating UI DOM refs, render props, DOM data attributes, CSS variables, ARIA attributes, and SSR concerns should be dropped or translated into GPUI-native architecture per `docs/base-gpui-component-architecture.md`.

Preview Card payloads are in scope and should be Rust-native typed values using a generic payload parameter such as `P: Clone + 'static`; do not port arbitrary JavaScript value semantics.

## Scope

Port the Preview Card component family from Base UI into GPUI-native components:

- `PreviewCardRoot<P>`
- `PreviewCardTrigger<P>`
- `PreviewCardPortal<P>`
- `PreviewCardPositioner<P>`
- `PreviewCardPopup<P>`
- `PreviewCardArrow<P>`
- `PreviewCardBackdrop<P>`
- `PreviewCardViewport<P>`
- `PreviewCardHandle<P>` / `create_preview_card_handle()` or an equivalent GPUI-native constructor

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/preview-card/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/root/PreviewCardRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/root/PreviewCardContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/root/PreviewCardRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/root/PreviewCardRoot.detached-triggers.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/root/PreviewCardRoot.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/trigger/PreviewCardTrigger.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/trigger/PreviewCardTrigger.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/trigger/PreviewCardTriggerDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/portal/PreviewCardPortal.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/portal/PreviewCardPortalContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/portal/PreviewCardPortal.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/positioner/PreviewCardPositioner.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/positioner/PreviewCardPositionerContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/positioner/PreviewCardPositioner.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/positioner/PreviewCardPositioner.spec.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/positioner/PreviewCardPositionerDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/positioner/PreviewCardPositionerCssVars.ts`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/popup/PreviewCardPopup.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/popup/PreviewCardPopup.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/popup/PreviewCardPopupDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/arrow/PreviewCardArrow.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/arrow/PreviewCardArrow.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/arrow/PreviewCardArrowDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/backdrop/PreviewCardBackdrop.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/backdrop/PreviewCardBackdrop.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/backdrop/PreviewCardBackdropDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/viewport/PreviewCardViewport.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/viewport/PreviewCardViewport.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/viewport/PreviewCardViewportDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/viewport/PreviewCardViewportCssVars.ts`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/store/PreviewCardStore.ts`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/store/PreviewCardHandle.ts`
- `/home/luke/Projects/base-ui/packages/react/src/preview-card/utils/constants.ts` (`OPEN_DELAY = 600`, `CLOSE_DELAY = 300`)

Base UI docs reference:

- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/preview-card/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/preview-card/types.md`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/preview-card/demos/hero/`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/preview-card/demos/detached-triggers-simple/`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/preview-card/demos/detached-triggers-controlled/`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/preview-card/demos/detached-triggers-full/`

Relevant local GPUI precedent (reference per-component; do not extract shared modules from Tooltip/Popover for this port):

- `crates/base_gpui/src/tooltip/runtime.rs` — hover open/close delay timers with generation checks, hoverable popup persistence, focus-open, safe-gap geometry, viewport transition facts.
- `crates/base_gpui/src/tooltip/layers/tooltip_trigger.rs` — `spawn_delayed_hover`, focus/blur wiring, per-trigger delay overrides.
- `crates/base_gpui/src/tooltip/layers/tooltip_popup.rs` — `spawn_popup_close`, popup-hover-cancels-close.
- `crates/base_gpui/src/tooltip/layers/tooltip_viewport.rs` — activation direction / payload content precedent.
- `crates/base_gpui/src/popover/runtime.rs` — outside-press/escape dismissal path, trigger metadata registration, payload storage, arrow/positioning facts.
- `crates/base_gpui/src/popover/context.rs` — thin context with controlled/uncontrolled open resolution.
- `crates/base_gpui/src/popover/layers/popover_arrow.rs` — arrow placement/style state.
- `crates/base_gpui/src/popover/layers/popover_backdrop.rs` — backdrop rendering; ADAPT for Preview Card: remove the `OutsidePress` click capture, the Preview Card backdrop is presentation-only.
- `/home/luke/Projects/gpui-component/crates/ui/src/hover_card.rs` — same 600ms/300ms defaults, trigger/content hover persistence loop.

Cross-linked issue:

- `issues/add-gpui-safe-polygon-hover-primitive.md` — the reusable safe-polygon hover-intent primitive this trigger's hover close should compose with once it lands.

Current GPUI implementation:

- No `crates/base_gpui/src/preview_card/` implementation exists yet.

Expected GPUI implementation files:

```text
crates/base_gpui/src/preview_card/
  mod.rs            # barrel only
  actions.rs        # Escape dismissal dispatch
  child.rs          # typed child enums
  child_wiring.rs   # private traversal/context attachment/trigger registration
  context.rs        # PreviewCardContext<P>
  props.rs          # root/trigger props and callbacks
  runtime.rs        # PreviewCardRuntime<P> + metadata/outcomes/details
  style_state.rs    # PreviewCard*StyleState structs
  layers/
    mod.rs          # barrel only
    preview_card_root.rs
    preview_card_trigger.rs
    preview_card_portal.rs
    preview_card_positioner.rs
    preview_card_popup.rs
    preview_card_arrow.rs
    preview_card_backdrop.rs
    preview_card_viewport.rs
  tests/
```

Also update:

- `crates/base_gpui/src/lib.rs`
- `crates/base_gpui/src/main.rs` with Preview Card smoke scenarios

## Out of scope / drop from Base UI

- React context, hooks, stores, refs, `FloatingTree`, `FloatingNode`, `FloatingPortalLite`, `ReactDOM.flushSync`, and `useDismiss`/`useHoverReferenceInteraction`/`useFocus` implementation details.
- React children render-function API for payload-driven content (`children={({ payload }) => ...}`). Payload storage and payload-driven content are in scope; expose payload through a GPUI-native content builder, runtime query, open-change details, or viewport child routing, following the Tooltip/Popover payload APIs.
- `render` prop support.
- `className` and web `style` props.
- Native DOM element semantics: Base UI's trigger renders an `<a>` element. GPUI has no anchor element; build the trigger from an interactive `div()` with focus/click behavior and document that link navigation is the consumer's concern.
- SSR/hydration/prehydration APIs.
- DOM data attributes (`data-popup-open`, `data-open`, `data-closed`, `data-side`, `data-align`, `data-starting-style`, `data-ending-style`, `data-activation-direction`). Map them into typed `*StyleState` structs.
- CSS variable APIs (`--available-width`, `--available-height`, `--anchor-width`, `--anchor-height`, `--transform-origin`, viewport CSS vars). Expose typed style-state measurements instead.
- Literal DOM `id`, `role="presentation"`, `aria-hidden`, and ARIA relationship attributes unless GPUI-native AccessKit support exists and is explicitly implemented. The pinned GPUI lacks these APIs per the Tooltip/Popover audits.
- Inline-rect anchoring (`createInlineMiddleware` / `inlineRectCoordsRef` / `getInlineRectTriggerProps`): Base UI anchors the card to the hovered line of a wrapped multi-line link using DOM client rects. This is web-specific. Either approximate it with the hover point captured at open time (a virtual anchor derived from the mouse position when the hover open fires, following Tooltip's cursor-anchor precedent) or descope it entirely with a documented note; do not port DOM rect traversal.
- `positionMethod` (`absolute`/`fixed`) — CSS positioning strategy has no GPUI meaning.
- Browser event objects in change details. Use GPUI-native reason/source fields.
- Exact CSS animation semantics and `node.getAnimations()` flush behavior. Preserve mounted/open/transition/instant facts in style state.
- Base UI's `safePolygon` DOM implementation (`pointer-events` CSS, `relatedTarget` containment). The behavior is in scope via the cross-linked GPUI-native primitive or the documented interim fallback; the DOM mechanics are not.
- Provider/delay-group behavior: Preview Card has no provider part in Base UI; do not add one.

## Acceptance Criteria

### Module/API surface

- [x] `crates/base_gpui/src/preview_card/` exists with the expected flat component architecture (no nested `child/context/{props,runtime,state}/` taxonomies).
- [x] `preview_card/mod.rs` is a barrel-only file with module declarations and re-exports only.
- [x] `preview_card/layers/mod.rs` is a barrel-only file with module declarations and re-exports only.
- [x] `base_gpui::preview_card` is exported from `crates/base_gpui/src/lib.rs`.
- [x] `base_gpui::init(cx)` calls `preview_card::init(cx)` if Preview Card registers actions/key bindings.
- [x] `PreviewCardRoot<P>` exists and is publicly exported.
- [x] `PreviewCardTrigger<P>` exists and is publicly exported.
- [x] `PreviewCardPortal<P>` exists and is publicly exported.
- [x] `PreviewCardPositioner<P>` exists and is publicly exported.
- [x] `PreviewCardPopup<P>` exists and is publicly exported.
- [x] `PreviewCardArrow<P>` exists and is publicly exported, following the Popover arrow precedent (unlike Tooltip, which omitted its arrow).
- [x] `PreviewCardBackdrop<P>` exists and is publicly exported.
- [x] `PreviewCardViewport<P>` exists and is publicly exported.
- [x] `PreviewCardHandle<P>` and `create_preview_card_handle()` (or an equivalent GPUI-native constructor) exist, using the late-bound `Rc<RefCell<...>>` handle pattern from Tooltip/Popover.
- [x] No `PreviewCardTitle`, `PreviewCardDescription`, `PreviewCardClose`, or `PreviewCardProvider` parts are added; Base UI Preview Card does not have them.
- [x] Public payload APIs use Rust-native generics such as `P: Clone + 'static`.
- [x] All public style payloads use `*StyleState` names from `style_state.rs`; no `*RenderState` or `render_state.rs` files are introduced.
- [x] No Preview Card code uses Rust scoped visibility syntax such as `pub(...)`.

### Correctness / compile readiness

- [x] `cargo fmt --check` passes.
- [x] `cargo check -p base_gpui` passes.
- [x] `cargo clippy -p base_gpui --all-targets` passes or only reports pre-existing warnings.
- [x] `ast-grep scan crates/base_gpui/src` passes, including the barrel-only `mod.rs` rule.
- [x] `rg -n "pub\(" crates/base_gpui/src/preview_card` returns no scoped visibility.
- [x] `rg -n "RenderState|render_state" crates/base_gpui/src/preview_card` returns no results.

### Architecture / internal primitives

- [x] Follow `docs/base-gpui-component-architecture.md`: one deep `PreviewCardRuntime<P>`, thin `PreviewCardContext<P>`, thin render layers.
- [x] `PreviewCardRuntime<P>` owns all Preview Card state: open/mounted/presence state, active trigger id, trigger metadata and payloads, hover open/close timers, instant state, focus facts, trigger/popup/arrow bounds, dismissal facts, and viewport transition facts.
- [x] The root is a Tooltip-style hover/focus root: provider-less, hover/focus-driven open with delay timers modeled on `tooltip/runtime.rs` and `tooltip/layers/tooltip_trigger.rs`, not a click-driven Popover root.
- [x] Arrow, backdrop, detached handle, and outside-press/escape dismissal follow the Popover shape: arrow placement modeled on `popover/layers/popover_arrow.rs`, backdrop rendering modeled on `popover/layers/popover_backdrop.rs` (minus click capture), dismissal via the runtime path as in `popover/runtime.rs`, handle as in Popover/Tooltip.
- [x] Tooltip/Popover code is used as per-component reference only; no shared modules are extracted from Tooltip or Popover for this port, and no new generic abstractions are introduced unless they hide a deep repeated concept.
- [x] `PreviewCardRuntime<P>` exposes domain commands (for example `sync_children`, `request_open`, `request_close`, `hover_trigger`, `unhover_trigger`, `focus_trigger`, `blur_trigger`, `activate_trigger`, `dismiss_outside`, `close_from_escape`, `set_bounds`, `reconcile`) and part-shaped queries returning `PreviewCard*StyleState` values.
- [x] `PreviewCardContext<P>` remains a thin injection vehicle with `read`, `update`, and one or two value-changing methods for controlled/uncontrolled open resolution; it does not grow vocabulary for child indexing, trigger registration, positioning, or hover timing.
- [x] Child routing uses typed enums in `child.rs` before `AnyElement` erasure; `child_wiring.rs` is the only place that walks typed children, scopes trigger ids, registers trigger metadata, and attaches context.
- [x] Public layer parts do not expose helper methods solely for child traversal or registration.
- [x] Trigger ids are root/handle-scoped internally so independent preview cards can reuse public trigger ids safely; duplicate scoped ids are deterministic and documented.
- [x] Hover timer tasks use generation checks so stale delayed opens/closes cannot mutate current state, following the Tooltip pattern.
- [x] `PreviewCardRuntime<P>` is unit-testable without a GPUI window.
- [x] No `runtime_control.rs` or equivalent trait-boundary file is introduced; no `utils/` folder is added for Preview Card-specific helpers.

### Controlled/uncontrolled open state

- [x] `PreviewCardRoot<P>::new()` exists with stable root identity (`id(...)` or equivalent) for keyed runtime state.
- [x] `PreviewCardRoot<P>::default_open(bool)` initializes uncontrolled open state (default `false`).
- [x] `PreviewCardRoot<P>::open(bool)` supports controlled open state and takes precedence over `default_open(...)`.
- [x] `PreviewCardRoot<P>::on_open_change(...)` fires on open-state changes with typed details.
- [x] `PreviewCardRoot<P>::on_open_change_complete(...)` fires after open/close presence settles, immediately when no transition infrastructure is active, and not on initial closed mount.
- [x] `PreviewCardRoot<P>::trigger_id(...)` supports controlled active-trigger selection, and `default_trigger_id(...)` supports initially open uncontrolled cards associated with a trigger.
- [x] Controlled roots call `on_open_change(...)` without mutating internal open state; uncontrolled roots mutate internal state unless the change is canceled.
- [x] Add `PreviewCardOpenChangeReason` with exactly the Base UI reason set: `TriggerHover`, `TriggerFocus`, `TriggerPress`, `OutsidePress`, `EscapeKey`, `ImperativeAction`, and `None`.
- [x] Add `PreviewCardOpenChangeDetails<P>` with `reason`, GPUI-native source metadata, `trigger_id`, optional typed payload, `cancel()` / `is_canceled()`, and `prevent_unmount_on_close()`, following the Tooltip/Popover details shape.
- [x] Canceling an uncontrolled open prevents opening; canceling an uncontrolled close (including the active-trigger-unmount close) prevents closing.
- [x] Closing can keep the popup mounted via `prevent_unmount_on_close()`, and a later normal close unmounts as expected.
- [x] Root-level imperative actions exist mirroring Base UI `actionsRef` (`close()` and `unmount()`), either on the handle or as a documented GPUI-native equivalent.
- [x] When the card is open with no active trigger, the active payload resolves to none rather than a stale payload.
- [x] When the active trigger (attached or detached) unmounts, the uncontrolled card closes unless the close is canceled; controlled roots do not invent a replacement trigger.
- [x] Instant state is limited to `Focus` and `Dismiss` (Base UI `instantType: 'focus' | 'dismiss'`): focus-open sets `Focus`, close from `TriggerPress`/`EscapeKey` sets `Dismiss`, hover changes clear it. No delay-group instant variant is added.

### Hover-open (mouse-only), focus-open, delays, and safe polygon

- [x] Hovering an enabled trigger opens the card after the effective open delay; the default open delay is `600ms` (`OPEN_DELAY`).
- [x] Unhovering closes the card after the effective close delay; the default close delay is `300ms` (`CLOSE_DELAY`).
- [x] `PreviewCardTrigger<P>::delay(Duration)` and `close_delay(Duration)` override the defaults per trigger; the active trigger's close delay governs popup-hover close scheduling (Base UI's `closeDelayRef`).
- [x] Hover open is mouse-only: hover-like input from touch does not open the card, where GPUI exposes enough pointer information to distinguish them; otherwise the gap is documented following the Tooltip mouse-only audit.
- [x] Hover open/close never opens from trigger press; pressing an open trigger dismisses with reason `TriggerPress` and instant `Dismiss`, matching Base UI's dismiss classification.
- [x] Focusing an enabled trigger opens the card with reason `TriggerFocus` and instant `Focus`; Base UI applies the trigger's open delay to focus-open, so match that (or document immediate focus-open as a deliberate deviation).
- [x] Blurring the active focused trigger closes the card unless focus moves into the popup interaction area.
- [x] Re-hovering the trigger or popup before a pending close fires cancels the pending close.
- [x] Hovering the popup keeps the card open by default (hoverable popup, per `tooltip/layers/tooltip_popup.rs`); leaving the popup schedules a close with the active trigger's close delay.
- [x] The hover close is safe-polygon guarded: either compose with the primitive from `issues/add-gpui-safe-polygon-hover-primitive.md` once it lands, or ship the documented interim fallback — Tooltip's close-delay plus popup-hover-cancels-close persistence (including its rectangular safe-gap grace) — such that a normal pointer path from trigger to popup does not close the card. Either implementation satisfies this criterion; record which one shipped.
- [ ] Re-entering a trigger while its card is in a closing transition reopens immediately without waiting the full open delay (Base UI's `isClosing` check); after the close lifecycle fully finishes, the normal open delay applies again. — not shipped: no close-transition infrastructure exists, so a closing card is fully closed immediately and the normal open delay applies; revisit with GPUI animation support.
- [x] Moving from one enabled trigger to another while open switches the active trigger, payload, and anchor without the full open delay.
- [x] After an Escape close, hovering the trigger again reopens the card.
- [x] Escape closes the open card with reason `EscapeKey` via GPUI actions/key context (`actions.rs`), following the Popover dismissal path.
- [x] Outside press closes the open card with reason `OutsidePress` via the runtime dismissal path (GPUI `on_mouse_down_out` or equivalent), not via the backdrop.
- [ ] Nested preview cards (a trigger inside another card's popup) keep the parent open while hovering/pressing the nested trigger or popup, to the extent GPUI hit-testing allows, following the Tooltip nested-trigger precedent; remaining gaps are documented. — not verified: no nested-preview-card rendered test was added; GPUI hit-testing behavior for nested overlays is untested here.

### Presentation-only backdrop

- [x] `PreviewCardBackdrop<P>::new()` exists and renders beneath the popup when mounted.
- [x] The backdrop is presentation-only: it does not capture pointer events, does not close the card on click, and does not block interaction with content beneath it — outside-press dismissal is handled by the root/runtime path, never the backdrop. This is the key adaptation from `popover/layers/popover_backdrop.rs`, which must NOT be copied with its `OutsidePress` click capture.
- [x] The backdrop is hidden/omitted when the card is not mounted.
- [x] `PreviewCardBackdropStyleState` exposes open and transition-status facts.
- [x] No modal mode, focus trap, scroll lock, or `modal_backdrop` usage exists anywhere in Preview Card.

### Positioning and portal

- [x] `PreviewCardPortal<P>::new()` exists with `keep_mounted(bool)` (default `false`); children are omitted when closed and not keep-mounted, and remain rendered but hidden/non-interactive when closed and keep-mounted, following the Tooltip `invisible()` pattern.
- [x] `PreviewCardPositioner<P>::new()` exists and uses GPUI `deferred(...)` / `anchored()` or the trigger-bounds overlay model; no DOM portals or Floating UI refs.
- [x] Positioner defaults to side `Bottom` and align `Center` (unlike Tooltip's `Top`).
- [x] Positioner supports side `Top`, `Bottom`, `Left`, `Right`, `InlineStart`, `InlineEnd` (via `utils::direction`) and align `Start`, `Center`, `End`.
- [x] Positioner supports `side_offset(...)` (default `0`), `align_offset(...)` (default `0`), and `collision_padding(...)` (default `5`); function-valued offsets are dropped as web/JS-specific.
- [x] Positioner supports `arrow_padding(...)` (default `5`) so the arrow does not exceed popup edges.
- [x] Positioner supports side-axis and alignment-axis collision flip/shift to the same practical extent as the Popover port, records the effective side/align after collision handling, and updates when the active trigger changes or trigger/popup layout changes.
- [x] Positioner anchors to the active trigger by default; the dropped inline-rect anchoring is either approximated with a virtual anchor from the hover point captured at open time or explicitly descoped in the implementation notes.
- [x] Position is refreshed when the card opens while mounted (Base UI re-runs positioning on open).
- [x] `PreviewCardPositionerStyleState` exposes open, effective side/align, anchor-hidden, instant, transform-origin, anchor size, popup size, and available size facts as typed measurements.
- [x] `PreviewCardArrow<P>::new()` exists; arrow placement follows the effective side/align, updates on collision flips, respects `arrow_padding`, and `PreviewCardArrowStyleState` exposes open, side, align, and uncentered facts.
- [x] Sticky positioning, `collision_boundary` element references, `position_method`, and arbitrary external anchor elements are dropped or documented as deferred, matching the Tooltip/Popover decisions.

### Payloads and detached handle

- [x] `PreviewCardTrigger<P>::new()` exists with stable `id(...)` and `payload(P)`.
- [x] Payload-driven popup content is exposed through a GPUI-native content builder (for example `payload_content(...)`) or runtime query, matching the Tooltip/Popover payload APIs; no React render-function children.
- [x] Multiple triggers inside one root can open the same card; switching triggers updates the active payload and reuses the same popup/positioner runtime identity rather than remounting the overlay.
- [x] `create_preview_card_handle::<P>()` creates a late-bound handle connecting detached `PreviewCardTrigger<P>::handle(...)` triggers to a `PreviewCardRoot<P>::handle(...)` root.
- [x] Detached triggers open the card on hover and on focus with the same delays and reasons as attached triggers.
- [x] `PreviewCardHandle<P>::open(trigger_id)` opens the card for a registered trigger with reason `ImperativeAction`; Base UI throws on a missing trigger id — the GPUI port returns a recoverable `bool`/`Result` like the Tooltip/Popover handles, and documents this deviation.
- [x] `PreviewCardHandle<P>::close()` closes with reason `ImperativeAction`; `is_open()` reports current state or `false` when unbound.
- [x] Multiple detached triggers sharing one handle can open the same card; the active detached trigger's unmount closes an uncontrolled card.
- [x] Programmatic controlled sequences (open trigger 1, open trigger 2, close) preserve payload correctness.
- [x] Callback details expose the active trigger id and typed payload when available.

### Viewport

- [x] `PreviewCardViewport<P>::new()` exists and renders the current payload-driven content by default.
- [x] The runtime tracks whether a viewport is present (Base UI `hasViewport`) if positioning/transform-origin behavior depends on it; otherwise document why the fact is unnecessary in GPUI.
- [x] Viewport records previous and current content identity when the active trigger/payload changes and derives activation direction from previous vs current trigger bounds.
- [x] `PreviewCardViewportStyleState` exposes activation direction, transitioning, instant (`Focus`/`Dismiss`), and previous/current popup size facts where available.
- [x] Rapid trigger changes settle on the latest trigger/payload without stale previous content and without panicking.
- [x] Full morphing previous/current content containers follow the Tooltip decision: expose the facts and defer full morphing until GPUI animation infrastructure warrants it.

### Styling/state exposure

- [x] `PreviewCardTriggerStyleState` exposes open-by-this-trigger (Base UI's `data-popup-open`), active-trigger, focused, hovered, and payload-present facts as appropriate; a `disabled` fact is included only if a disabled API is added (Base UI's trigger has none).
- [x] `PreviewCardPortalStyleState` exposes mounted/open facts if the portal exposes styling; otherwise no empty style state is added.
- [x] `PreviewCardPositionerStyleState`, `PreviewCardPopupStyleState` (open, side, align, instant, transition status), `PreviewCardArrowStyleState`, `PreviewCardBackdropStyleState`, and `PreviewCardViewportStyleState` exist per the sections above.
- [x] A root style state is added only if the root renders a styleable element (Base UI's root state is empty); otherwise the root remains injection-only.
- [x] All parts that draw expose `style_with_state(...)` with the component-specific style-state struct.
- [x] Base UI data attributes and CSS variables are represented only as typed style-state fields; no `className`, web `style`, render-prop, or DOM data-attribute public API is introduced.
- [x] No literal DOM ARIA attributes are emitted; AccessKit mapping is deferred per the Tooltip/Popover audits of the pinned GPUI.

### Tests / verification

- [x] Runtime tests cover uncontrolled default-closed, `default_open(true)`, and `default_open(true)` + `default_trigger_id(...)` payload resolution.
- [x] Runtime tests cover controlled open precedence and controlled callbacks without internal mutation.
- [x] Runtime tests cover `on_open_change` cancellation of open and close in uncontrolled mode.
- [x] Runtime tests cover `prevent_unmount_on_close()` and later normal unmount.
- [x] Runtime tests cover hover open delay (600ms default), close delay (300ms default), per-trigger overrides, and re-hover canceling a pending close.
- [ ] Runtime tests cover reopen-during-close-transition bypassing the open delay and normal delay after the lifecycle finishes. — not shipped alongside the unchecked isClosing behavior above.
- [x] Runtime tests cover instant state transitions: focus-open sets `Focus`, press/escape close sets `Dismiss`, hover clears.
- [x] Runtime tests cover multiple triggers, typed payload switching, active-trigger unmount close, and canceled unmount close.
- [x] Runtime tests cover handle open/close/is_open and the missing-trigger-id failure mode.
- [x] Runtime tests cover positioner effective side/align, collision flip, arrow placement facts, and bounds updates on trigger switch.
- [x] Runtime tests cover viewport activation direction and rapid trigger switching.
- [x] Rendered tests cover hover open/close, focus open/blur close, Escape close, and outside-press close.
- [x] Rendered tests cover hoverable popup persistence (pointer travels from trigger into popup without closing) under whichever safe-polygon implementation shipped.
- [x] Rendered tests cover the backdrop not capturing clicks: a click on backdrop-covered outside content reaches that content, and dismissal is attributed to the outside-press path.
- [ ] Rendered tests cover multiple triggers in one root, multiple detached triggers sharing a handle, and controlled programmatic switching. — partial: multiple attached triggers and a single handle-bound detached trigger are tested; multiple detached triggers sharing one handle and controlled programmatic switching are not rendered-tested.
- [ ] Rendered tests cover portal `keep_mounted(true)` hidden/non-interactive behavior and repositioning to a different trigger when reopened with keep-mounted. — partial: keep-mounted hidden/non-interactive behavior is tested; repositioning to a different trigger on keep-mounted reopen is not rendered-tested.
- [ ] Rendered tests cover nested preview card interactions to the extent GPUI hit-testing allows. — not added; see the nested behavior note above.
- [x] Rendered style-state tests cover trigger, positioner, popup, arrow, backdrop, portal, and viewport.
- [x] A `crates/base_gpui/src/main.rs` gallery section includes basic, payload/multi-trigger, and detached-handle Preview Card smoke scenarios, starting closed by default.
- [x] `cargo test -p base_gpui preview_card` passes.
- [x] `cargo test -p base_gpui` passes.

## Uncertain items to confirm during implementation

- Whether the inline-rect approximation (virtual anchor from the hover point at open time) is worth shipping in the first pass or should be descoped entirely.
- Whether GPUI pointer information is sufficient for strict mouse-only hover parity (same open question as the Tooltip port).
- Whether the safe-polygon primitive (`issues/add-gpui-safe-polygon-hover-primitive.md`) lands before or after this port; if after, this port ships the Tooltip-style fallback and swaps in the primitive later.
- Whether Base UI's delayed focus-open (open delay applied to focus) should be matched exactly or simplified to immediate focus-open like the Tooltip port.
