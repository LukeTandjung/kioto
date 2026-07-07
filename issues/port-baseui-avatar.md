# Port Base UI Avatar to GPUI

## Problem

Base UI Avatar is a small compound display component with three parts:

- `Avatar.Root` — owns a single piece of shared state, the image loading status
  (`idle | loading | loaded | error`), and exposes it to every part as render
  state (`AvatarRootState.imageLoadingStatus`).
- `Avatar.Image` — renders the image **only while the status is `loaded`**. It
  probes the image load itself (`useImageLoadingStatus`: `new window.Image()` +
  `onload`/`onerror`, `error` when `src` is missing, a fast path for
  cached/decoded images) and reports every non-`idle` status change both to the
  root context and to the user callback `onLoadingStatusChange`.
- `Avatar.Fallback` — user-supplied content (initials, icon) rendered whenever
  the status is **not** `loaded`, with an optional `delay` (ms) before it first
  appears so a fast-loading image never flashes the fallback.

`crates/base_gpui` has no `avatar` module. The goal is behavioral parity with
Base UI's contract — Root-owned loading status, Image visible only when loaded,
Fallback as a *peer part* of Image with show-delay — using GPUI-native
architecture, not a literal translation of the DOM image-probe or React
context.

Important GPUI fact that changes the port: **no new primitive is needed for the
image machinery.** GPUI's built-in `img` element already accepts an
`ImageSource` (URI, path, `SharedString`, render image, custom fn), shows a
`.with_loading(...)` element after a fixed `LOADING_DELAY` (200 ms), shows a
`.with_fallback(...)` element on load error, and caches images automatically
(`crates/gpui/src/elements/img.rs`, `image_cache.rs` in the pinned checkout
`/home/luke/.cargo/git/checkouts/zed-a70e2ad075855582/f7ca86e/`). The design
question for this port is how much of Base UI's part contract to keep on top of
that — see "Loading-status ownership" below.

Avatar is stateless from the caller's perspective: no controlled/uncontrolled
value, no keyboard behavior, no positioning. Complexity: small.

## Scope

Port the Avatar component family from Base UI into GPUI-native components:

- `AvatarRoot`
- `AvatarImage`
- `AvatarFallback`
- `AvatarImageLoadingStatus` (`Idle`, `Loading`, `Loaded`, `Error`)
- `AvatarRootStyleState`, `AvatarImageStyleState`, `AvatarFallbackStyleState`
  (all expose the loading status, mirroring Base UI where every part's render
  state includes `imageLoadingStatus`)

Base UI source reference:

- `/home/luke/Projects/base-ui/packages/react/src/avatar/index.parts.ts`
- `/home/luke/Projects/base-ui/packages/react/src/avatar/index.ts`
- `/home/luke/Projects/base-ui/packages/react/src/avatar/root/AvatarRoot.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/avatar/root/AvatarRootContext.ts`
- `/home/luke/Projects/base-ui/packages/react/src/avatar/root/stateAttributesMapping.ts`
- `/home/luke/Projects/base-ui/packages/react/src/avatar/root/AvatarRoot.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/avatar/image/AvatarImage.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/avatar/image/useImageLoadingStatus.ts`
- `/home/luke/Projects/base-ui/packages/react/src/avatar/image/AvatarImageDataAttributes.ts`
- `/home/luke/Projects/base-ui/packages/react/src/avatar/image/AvatarImage.test.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/avatar/fallback/AvatarFallback.tsx`
- `/home/luke/Projects/base-ui/packages/react/src/avatar/fallback/AvatarFallback.test.tsx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/avatar/page.mdx`
- `/home/luke/Projects/base-ui/docs/src/app/(docs)/react/components/avatar/types.md`

GPUI references (behavioral building blocks, not to be copied literally):

- `/home/luke/.cargo/git/checkouts/zed-a70e2ad075855582/f7ca86e/crates/gpui/src/elements/img.rs`
  (`img`, `ImageSource`, `.with_loading(...)`, `.with_fallback(...)`,
  `LOADING_DELAY`, `ImageSource::get_data` / `window.use_asset` for observing
  load status)
- `/home/luke/.cargo/git/checkouts/zed-a70e2ad075855582/f7ca86e/crates/gpui/examples/image_loading.rs`
- `/home/luke/Projects/gpui-component/crates/ui/src/avatar/avatar.rs`
  (styling/initials reference only — it has no error fallback or status
  contract; do not copy its single-widget shape)

Current GPUI implementation:

- No `crates/base_gpui/src/avatar/` module exists yet.

Expected new GPUI files (flat layout per
`docs/base-gpui-component-architecture.md`):

```text
crates/base_gpui/src/avatar/mod.rs
crates/base_gpui/src/avatar/runtime.rs        # AvatarRuntime: loading status + fallback-delay state
crates/base_gpui/src/avatar/context.rs        # AvatarContext: entity plumbing only
crates/base_gpui/src/avatar/props.rs          # on_loading_status_change callback (if kept out of the Image builder)
crates/base_gpui/src/avatar/style_state.rs    # AvatarImageLoadingStatus + the three style-state structs
crates/base_gpui/src/avatar/child.rs          # AvatarChild typed child enum
crates/base_gpui/src/avatar/layers/mod.rs
crates/base_gpui/src/avatar/layers/avatar_root.rs
crates/base_gpui/src/avatar/layers/avatar_image.rs
crates/base_gpui/src/avatar/layers/avatar_fallback.rs
crates/base_gpui/src/avatar/tests/
```

Register `pub mod avatar;` in `crates/base_gpui/src/lib.rs`. No `actions.rs`
(no keyboard behavior) and no `child_wiring.rs` unless child traversal turns
out to need it — Avatar has no indexing, so plain context attachment in the
root should suffice. If option (a) below is chosen, `runtime.rs` /
`context.rs` / `props.rs` collapse away and the file list shrinks accordingly.

## Loading-status ownership (decide before implementing)

Base UI's contract is: Root owns the status; Image reports it; Fallback and
every other part read it. GPUI's `img` element internally already implements
the loading/error state machine. Two GPUI-native options — pick one and record
the decision in the PR:

1. **Lean on `img`'s closures (simplest).** `AvatarImage` wraps gpui `img`
   and `AvatarFallback` content is threaded into `.with_fallback(...)` (error)
   and `.with_loading(...)` (shown after gpui's fixed 200 ms `LOADING_DELAY`).
   No runtime, no context; Avatar becomes a stateless styling wrapper like
   Separator. Costs: Fallback is no longer a real sibling part (it must be
   nested/moved inside the Image), the fallback `delay` is pinned to gpui's
   200 ms instead of caller-controlled, `on_loading_status_change` has no
   natural home, and no part can style itself off the shared status — all
   departures from Base UI's public contract.

2. **Track status in an `AvatarRuntime` (preferred — parity).** A small
   runtime (`Entity<AvatarRuntime>` behind a thin `AvatarContext`) owns
   `AvatarImageLoadingStatus` plus the fallback-delay state. `AvatarImage`
   still renders through gpui `img` for decoding/caching, but derives the
   status GPUI-natively by observing the `ImageSource` asset state during
   render (`window.use_asset` / `ImageSource::get_data`: `None` → `Loading`,
   `Some(Ok)` → `Loaded`, `Some(Err)` → `Error`; no source → `Error`, matching
   Base UI's missing-`src` rule) and issues a runtime command when it changes.
   `AvatarFallback` stays a true peer part that queries the runtime and
   renders only when status is not `Loaded` and its own `delay` has elapsed
   (timer via the executor + notify). This preserves Base UI's part contract:
   sibling Fallback, caller-controlled `delay`, `on_loading_status_change`,
   and status exposed in every part's style state.

Recommend **option 2**. It follows the standard runtime/context/parts shape
from `docs/base-gpui-component-architecture.md` (a tiny runtime is expected
and fine — depth is interface-to-knowledge ratio, not line count). Whichever
option is chosen, the runtime must be the single place status transitions are
computed — no shadow previous-status fields in parts, per the architecture
invariants. The acceptance criteria below assume option 2; if option 1 is
chosen after discussion, rewrite the Architecture and Loading-status sections
before implementing.

## Out of scope / drop from Base UI

- Do not port React `render` props.
- Do not port `className` or web `style` props; use GPUI styling builders and
  `style_with_state(...)`.
- Do not port `referrerPolicy` / `crossOrigin` — web network semantics with no
  GPUI equivalent; gpui asset loading handles fetching.
- Do not port the DOM image probe (`new window.Image()` + `onload`/`onerror`
  in `useImageLoadingStatus`); derive status from gpui `ImageSource` asset
  state instead.
- Do not port `transitionStatus` / `useTransitionStatus` /
  `useOpenChangeComplete` and the `data-starting-style` / `data-ending-style`
  appear/disappear CSS transition machinery (`AvatarImageDataAttributes`).
  This is a web CSS-transition idiom. If an appear animation is wanted later,
  map it to gpui `animation.rs` (`with_animation`) as a follow-up — do not
  expose transition status in the initial style state.
- Do not port DOM data attributes; expose status through the style-state
  structs. (Note Base UI itself maps `imageLoadingStatus` to *no* data
  attribute — it is render-state only, which is exactly what
  `style_with_state` models.)
- Do not port SSR/hydration behavior (e.g. the cached-image hydration test).
  gpui's automatic image caching covers the "cached image resolves without a
  fallback flash" concern; test that path natively instead.
- No ARIA: Base UI Avatar renders plain `<span>`/`<img>` with no roles or
  aria attributes, so there is no AccessKit follow-up beyond ordinary image
  alt-text, which GPUI does not currently expose. Nothing to track.
- Typed children only: `AvatarChild` routes `Image` and `Fallback`; include an
  `AnyElement` escape hatch only because Base UI demos show arbitrary root
  children (initials text directly inside `Fallback`, not `Root`) — confirm
  against the demos before adding it.
- Do not create a component-local `utils/` folder.
- Do not use Rust scoped visibility syntax (`pub(...)`); `ast-grep scan` must
  remain clean.
- `mod.rs` files are barrel exports only.

## Acceptance Criteria

All items implemented and verified.

### Module/API surface

- [x] Add a top-level `avatar` module and register `pub mod avatar;` in
      `crates/base_gpui/src/lib.rs` (no `init(cx)` needed — no actions).
- [x] `AvatarRoot::new()` builder exists and accepts typed children
      (`AvatarChild`: `Image`, `Fallback`, plus escape hatch per scope note).
- [x] `AvatarImage::new(source)` builder exists, taking `impl Into<ImageSource>`
      so URIs, paths, `SharedString`s, and render images all work, mirroring
      Base UI `src` while staying GPUI-native.
- [x] `AvatarImage` supports `.on_loading_status_change(...)` receiving
      `AvatarImageLoadingStatus`.
- [x] `AvatarFallback::new()` builder exists, accepts arbitrary children
      (initials text, icons), and supports `.delay(Duration)` (or ms) for the
      show-delay; no delay means show immediately, matching Base UI's
      `delayPassed` default.
- [x] `AvatarImageLoadingStatus` enum has exactly `Idle`, `Loading`, `Loaded`,
      `Error`.
- [x] All three parts support normal GPUI styling builders through `Styled`
      and `.style_with_state(...)`.
- [x] `avatar/mod.rs` exposes ergonomic barrel exports.

### Correctness / compile readiness

- [x] `cargo check -p base_gpui` passes.
- [x] `cargo test -p base_gpui avatar` passes.
- [x] `cargo test -p base_gpui` passes.
- [x] `ast-grep scan crates/base_gpui/src/avatar` reports no scoped-visibility
      violations.
- [x] Add a small Avatar demo to `crates/base_gpui/src/main.rs`: one avatar
      with a valid image, one with a broken/missing source showing an initials
      fallback, one with `delay` set.

### Architecture

- [x] Record the loading-status ownership decision (option 1 vs option 2 above)
      in the PR; criteria below assume option 2.
      Decision: option 2 implemented — `AvatarRuntime` owns the status; the
      image derives it from gpui asset state (`use_asset` for resources,
      `Image::use_render_image` for in-memory images, custom loaders called
      directly, `Render` sources treated as loaded) and reports via
      `report_image_status`. Known limitation: `ImageSource::Image` decode
      failures surface as `Loading` rather than `Error`, because gpui's
      `ImageDecoder` asset is private and `use_render_image` folds errors into
      `None`.
- [x] `AvatarRuntime` (plain struct, no GPUI entity types inside) owns the
      loading status and fallback-delay state; unit-testable without a window.
- [x] Status transitions are computed only inside the runtime via commands
      (e.g. `report_image_status(...)`, `fallback_delay_elapsed()`); parts
      never diff previous status themselves.
- [x] Runtime queries are part-shaped: `root_state()`, `image_state()`,
      `fallback_state()` returning the style-state structs, plus visibility
      answers ("should the image render?", "should the fallback render?") —
      no raw status getter/setter pairs.
- [x] `AvatarContext` is a thin injection vehicle (entity + props), with only
      `read` / `update` shapes; no avatar vocabulary on the context.
- [x] The `on_loading_status_change` callback fires from the part/context
      based on a runtime outcome — the runtime never calls user callbacks.
- [x] `AvatarImage` renders through gpui's `img` element (decoding, caching,
      sizing) rather than reimplementing image loading; it does not use
      `.with_loading(...)` / `.with_fallback(...)` when option 2 is chosen,
      since the sibling `AvatarFallback` covers those states.
- [x] Renderable parts live under `layers/`; typed child enum under `child.rs`;
      no nested `child/context/{...}` taxonomy; no `utils/`.
- [x] No DOM concepts leak into the public API.

### Loading-status behavior

- [x] Initial status is `Idle` before any image part has reported.
- [x] A mounted `AvatarImage` with an unresolved source drives status to
      `Loading`.
- [x] Successful decode drives status to `Loaded`; failed load drives `Error`.
- [x] A missing/empty image source is reported as `Error` (Base UI: no `src`
      → `error`), not left at `Idle`.
- [x] `AvatarImage` content is visible only while status is `Loaded`; in
      `Idle`/`Loading`/`Error` it renders nothing (Base UI unmounts the
      `<img>`).
- [x] `AvatarFallback` renders whenever status is not `Loaded` (idle, loading,
      and error) — it is the error fallback *and* the loading placeholder.
- [x] With `.delay(d)`, the fallback stays hidden until `d` has elapsed since
      the fallback mounted, then shows if status is still not `Loaded`; an
      image that loads before `d` never flashes the fallback.
- [x] Without `delay`, the fallback shows immediately while not `Loaded`.
- [x] Image and fallback are never both visible in the same frame (Base UI
      test: "keeps only one of image or fallback mounted when switching").
- [x] `on_loading_status_change` fires on every status transition except the
      initial `Idle` (Base UI only reports non-`idle` statuses), and fires at
      most once per distinct transition.
- [x] Changing the image source resets the machine: status re-derives for the
      new source instead of keeping the old result.
- [x] A cached/already-decoded image resolves to `Loaded` without an
      intermediate visible fallback flash (gpui image cache fast path,
      mirroring Base UI's `image.complete` fast path).
- [x] A root with only a fallback and no image part shows the fallback (status
      stays `Idle`).
- [x] Avatar is inert: no focus, no activation, no app-state mutation.

### Styling/state exposure

- [x] `AvatarRootStyleState`, `AvatarImageStyleState`, and
      `AvatarFallbackStyleState` each expose
      `image_loading_status: AvatarImageLoadingStatus` (Base UI: every part's
      render state extends `AvatarRootState`).
- [x] `.style_with_state(...)` on each part receives its current style state
      during render.
- [x] No transition-status field in the initial style state (see out of
      scope); note gpui `with_animation` as the follow-up path if an appear
      animation is requested.
- [x] Style-state structs live in `style_state.rs`, one file, no CSS-variable
      or data-attribute surface.

### Tests / verification

Runtime unit tests (no window):

- [x] Status transition sequence `Idle → Loading → Loaded` and
      `Idle → Loading → Error`.
- [x] Missing source reports `Error`.
- [x] Fallback visibility query: hidden while `Loaded`, shown otherwise once
      delay has elapsed, hidden before delay elapses.
- [x] Report of an unchanged status produces no outcome (no duplicate
      callbacks).
- [x] Source change resets status derivation.

Rendered behavior tests under `avatar/tests/`:

- [x] Fallback visible while image is loading; image swaps in on `Loaded` and
      fallback unmounts.
- [x] Fallback visible after image load error.
- [x] Fallback with `delay` is absent before the delay and present after it
      when the image has not loaded (Base UI: "shows the fallback when the
      delay has elapsed").
- [x] Fallback with `delay` never appears when the image loads first.
- [x] Only one of image/fallback is mounted when switching to a loaded image.
- [x] `on_loading_status_change` observed firing for `Loading`/`Loaded`/`Error`
      and not for initial `Idle`.
- [x] `style_with_state` on each part observes the correct
      `image_loading_status`.
- [x] Avatar renders inside arbitrary containers without affecting siblings.

## Uncertain items needing confirmation

- Option 1 vs option 2 in "Loading-status ownership" — the issue assumes
  option 2 (runtime parity with Base UI's part contract); confirm before
  implementing.
- Whether `Idle` is reachable in practice under option 2 (an image part
  reports on first render). Keep the variant for Base UI parity and for the
  fallback-only composition, but confirm the exact first-frame semantics
  against `window.use_asset` behavior in the pinned gpui revision.
- Whether the `AvatarChild` enum needs the `AnyElement` escape hatch — check
  the Base UI avatar demos for arbitrary direct children of `Root` before
  adding it.
- `delay` parameter type: `Duration` (GPUI-idiomatic) vs `u64` ms (Base UI
  literal). Prefer `Duration`.
