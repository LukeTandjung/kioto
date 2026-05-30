---
name: create-baseui-port-issue
description: Create a behavior-focused issue checklist for porting one Base UI component family into base_gpui. Use when asked to plan or create an issue for a Base UI component port. The issue should follow issues/port-baseui-tabs.md as the reference shape and emphasize GPUI-native behavior over literal React/DOM code translation.
user-invocable: true
---

# Create a Base UI → GPUI Port Issue

Use this skill when Luke T asks to create an issue for porting a Base UI component or component family into `crates/base_gpui`.

The output is a single Markdown issue under:

```text
issues/port-baseui-<component>.md
```

Use `issues/port-baseui-tabs.md` as the canonical reference for structure, tone, and acceptance-criteria depth.

## Core principle

Port **behavior and contracts**, not source code.

Base UI's React/DOM implementation is the behavioral reference, but GPUI code should be GPUI-native. Drop or translate web-only details instead of copying them literally.

Examples:

- React context/hooks → GPUI keyed state/entities + component context wrappers.
- DOM registration/refs → typed child routing + runtime metadata registration.
- DOM measurement / `ResizeObserver` → GPUI layout/prepaint measurement when needed.
- CSS variables / data attributes → GPUI render-state structs and `style_with_state(...)`.
- DOM focus APIs → GPUI `FocusHandle`, key contexts, actions, and tab stops.

## Standard Base UI API translation decisions

Use these decisions for all Base UI component-port issues unless Luke T explicitly overrides them:

- Do **not** port React `render` props. They conflict with GPUI's builder/element composition style. Prefer typed GPUI components and builder methods.
- Do **not** port `className`. GPUI has no CSS class system.
- Do **not** port web `style` props. GPUI components should use normal styling builder methods. For state-dependent styling, expose `style_with_state(...)` with component-specific render-state structs.
- Do **not** port `nativeButton` / native DOM element switches. GPUI does not currently expose a built-in Button element under `crates/gpui/src/elements`; interactive controls are typically built from `div()` plus focus/click/action/accessibility behavior.
- Do **not** port SSR/hydration/prehydration APIs.
- Do **not** port CSS variable APIs. Expose typed GPUI render-state values and let users style through builder methods.
- Do **not** port arbitrary JavaScript value semantics. Use Rust type parameters and trait bounds appropriate for the component, e.g. `T: Clone + Eq + 'static`.
- Do **not** port DOM data attributes as attributes. Map state/data attributes into typed render-state structs.
- Revisit ARIA/accessibility only through GPUI-native AccessKit APIs when the target GPUI revision supports them; do not write DOM ARIA attributes.

## Required inputs

Before creating the issue, make sure the user provides or confirms:

1. The component family to port, e.g. `Accordion`, `Dialog`, `Select`, `Checkbox`.
2. The local Base UI checkout path.
   - Default to `/home/luke/Projects/base-ui` only if it exists.
   - If it does not exist or the component cannot be found, ask the user for the local Base UI path before continuing.
   - Do not rely only on web docs; the local source, tests, and docs are the behavioral reference.
3. The target GPUI crate/path.
   - Default to `crates/base_gpui` for this repo.
4. Whether an existing GPUI implementation should be treated as a partial port or ignored/replaced.

## Before writing the issue

1. Identify the component family name and GPUI target names.
   - Prefer explicit component names like `TabsRoot`, `TabsList`, etc.
   - Do not invent clever aliases unless the user asks.

2. Read the Base UI source for every part of the component family from the confirmed local checkout.
   - Typical path pattern:
     ```text
     <base-ui-root>/packages/react/src/<component>/...
     ```
   - Include tests and docs when useful:
     ```text
     <base-ui-root>/packages/react/src/<component>/**/*.test.tsx
     <base-ui-root>/docs/src/app/(docs)/react/components/<component>/...
     ```
   - Follow exports/index files to discover the complete component family.

3. Inspect the current GPUI implementation, if any.
   - Typical target path:
     ```text
     crates/base_gpui/src/<component>/
     ```
   - If no implementation exists, say so and list expected new files.

4. Read the architecture reference when the issue concerns `base_gpui` internals:
   ```text
   docs/base-gpui-component-architecture.md
   ```

5. Use `issues/port-baseui-tabs.md` as the issue template, but do not blindly copy Tabs-specific criteria. Adapt sections to the component's real behavior.

## Issue structure

Create the issue with these sections, in this order.

### Title

```md
# Port Base UI <Component> to GPUI
```

### Problem

Explain:

- what behavior Base UI provides,
- what the current GPUI version lacks,
- that the goal is behavioral parity using GPUI-native architecture,
- any important value/type constraints.

Keep this specific to the component. Do not write generic filler.

### Scope

List:

- public GPUI components to port,
- Base UI source files used as references,
- current or expected GPUI implementation files.

For compound components, include every part. Example shape:

```md
Port the <Component> component family from Base UI into GPUI-native components:

- `<Component>Root<T>`
- `<Component>Trigger<T>`
- `<Component>Content<T>`
```

### Out of scope / drop from Base UI

Explicitly list web-only details to drop or translate. Apply the standard translation decisions above. Consider:

- React context/hooks implementation details,
- render prop support,
- `className`,
- web `style` props,
- native DOM element options such as `nativeButton`,
- SSR/hydration/prehydration scripts,
- CSS variable API,
- DOM data attributes as attributes,
- DOM `ResizeObserver`, `MutationObserver`, `getBoundingClientRect`,
- DOM transition attributes,
- arbitrary JS value semantics,
- ARIA roles/attributes and DOM id linking **unless** GPUI AccessKit support is available and the issue includes a GPUI-native accessibility plan.

Be precise: do not mark something out of scope if GPUI has a meaningful equivalent we should implement.

### Acceptance Criteria

Use checkbox subsections. Start unchecked (`[ ]`) for new issues unless the repo already satisfies the item.

Recommended subsections:

1. `Module/API surface`
2. `Correctness / compile readiness`
3. `Architecture / internal primitives`
4. `Stateful/stateless behavior`
5. `Shared state model`
6. Component-specific behavior sections, e.g.
   - `Selection behavior`
   - `Keyboard/focus behavior`
   - `Pointer interaction behavior`
   - `Popup/positioning behavior`
   - `Panel/content behavior`
   - `Indicator behavior`
   - `Validation behavior`
7. `Styling/state exposure`
8. `Tests / verification`

Only include sections that make sense, but keep the checklist detailed enough to implement from.

## Architecture checklist rules

When the component is compound or stateful, prefer the established `base_gpui` ontology:

```text
crates/base_gpui/src/api/
crates/base_gpui/src/<component>/
  actions.rs                 # optional GPUI key dispatch actions/bindings
  child/
    <component>_child.rs      # typed child routing enum
    context/
      <component>_context.rs  # component-specific context wrapper
      props/
      runtime/
      state/                  # mod.rs only declares/exports state modules; each state struct gets its own file
  layers/                     # renderable GPUI elements only
```

Checklist items should enforce:

- typed children before `AnyElement` erasure,
- child-routing enums should only include child layer types shown in Base UI documentation/examples; add an `AnyElement` escape hatch only when Base UI examples show arbitrary root children are expected,
- component-specific context wrapper around `GenericContext<S, P, R>`,
- `GenericContext` limited to generic state/props/runtime mechanics,
- component behavior on `<Component>Context`, not inherent impls on generic context types,
- component-specific runtime registration through context methods,
- runtime metadata shapes kept component-specific,
- render-state structs kept in separate files under `context/state/`, not defined inline in `state/mod.rs`,
- `GenericChild<C>` used only for context injection,
- renderable layers under `layers/`,
- child-routing enums under `child/`, not `layers/`,
- no `utils/` folder for new primitives.

## Behavioral checklist rules

Make behavior explicit enough to drive implementation.

For controlled/uncontrolled components:

- define controlled vs uncontrolled precedence,
- define default value semantics,
- define empty/null/none semantics when relevant,
- define callback timing,
- define whether callbacks can cancel changes,
- define automatic fallback behavior,
- define externally controlled invalid value behavior.

For keyboard/focus components:

- prefer GPUI actions and key contexts over raw key-down handlers,
- list orientation-specific navigation,
- list Home/End/Page/etc. behavior when applicable,
- list wrapping/clamping behavior,
- list activation-on-focus/manual activation behavior,
- distinguish selected, highlighted, focused, and active states when relevant,
- call out known GPUI-native focus expectations.

For measurement/positioning components:

- state which bounds must be measured,
- state which render-state values expose measurements,
- use GPUI layout/prepaint mechanisms instead of DOM APIs,
- avoid porting Base UI CSS variable names unless useful in GPUI.

For styling:

- require component-specific render-state structs,
- require `style_with_state(...)` APIs when state-aware styling is expected,
- map Base UI state/data attributes into GPUI render state rather than DOM attributes,
- do not include `className`, web `style`, or CSS variable API in the GPUI public surface.

## Tests / verification checklist

Always include a test/examples section. Tailor it to the component.

Prefer behavior-level items such as:

- uncontrolled initial state,
- controlled state,
- disabled/read-only behavior,
- fallback behavior,
- pointer activation,
- keyboard navigation,
- focus behavior,
- mounting/unmounting behavior,
- measurement/positioning updates,
- state-aware styling exposure.

If GPUI test support is blocked or unavailable, still write the checklist as desired future verification.

## Writing style

- Be concrete and exhaustive, but avoid implementation code in the issue unless a small signature clarifies the API.
- Use checked boxes only for behavior already implemented and verified.
- Prefer exact file paths.
- Avoid overgeneralized abstractions.
- Keep the issue as a living checklist that can be updated during implementation.

## Deliverable

Write the Markdown issue file to:

```text
issues/port-baseui-<component>.md
```

Then summarize:

- issue path,
- Base UI files referenced,
- major behavior areas covered,
- any uncertain items that need human confirmation.
