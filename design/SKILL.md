---
name: concise-design
description: Use this skill to generate well-branded interfaces and assets for Concise — a sleek, near-black "carbon" learning app — either for production or throwaway prototypes/mocks. Contains essential design guidelines, colors, type, fonts, assets, and UI kit components for prototyping.
user-invocable: true
---

Read `README.md` first — it has the full content fundamentals, visual
foundations, and iconography rules. Then explore:

- `colors_and_type.css` — every CSS variable in the system
- `assets/icons/` — Heroicons SVGs used by the product
- `assets/avatar-sample.jpg` — sample avatar for placeholder use
- `ui_kits/web_app/` — JSX components + `index.html` showing the app in use
- `preview/` — Design System cards (palette swatches, type specimens, components)

**If creating visual artifacts** (slides, mocks, throwaway prototypes):
copy assets out of this folder rather than referencing them across projects,
and use the CSS variables from `colors_and_type.css` so every surface stays
on-system. Build new components in the style of `ui_kits/web_app/*.jsx` —
small, cosmetic, no production logic.

**If working on production code**: lift the CSS variables and the rules in
`README.md` (Content Fundamentals, Visual Foundations, Iconography). The JSX
in `ui_kits/web_app/` is a visual contract, not a component library — use
your own Headless UI primitives underneath.

**Core rules** — never break these without asking:
1. The background is `#0E0E0E`. There is no other canvas color.
2. Every "surface" is white-on-carbon at low alpha (the `--white-*` tokens).
   No solid greys for cards.
3. Inter for everything. JetBrains Mono only for code/math.
4. Heroicons only. No emoji. No Unicode glyphs as icons.
5. The accent (`--accent` warm gold) is a single, deliberate note — avatar
   ring only by default. Do not paint buttons with it.
6. No outer drop shadows on cards. Depth comes from translucency stacking
   and the inner-top highlight on pressable surfaces.
7. No gradients, no grain, no decorative imagery. The user's own avatar +
   uploaded thumbnails are the only imagery in the system.
8. Title Case for nouns and actions, Sentence case for help text. No
   exclamation marks. No marketing voice.

If the user invokes this skill without other guidance, ask them what they
want to build or design, ask the relevant questions (audience, surface,
size, variations), and act as an expert designer who outputs either HTML
artifacts or production code, depending on the need.
