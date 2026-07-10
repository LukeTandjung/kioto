# AccessKit in GPUI — reference for base_gpui AccessKit follow-ups

This is the authoritative summary of the AccessKit surface available in the pinned
gpui revision (`1d029c5ff5654fb1b1e8caf4462993c8ee13a133`, accesskit `0.24.0`). Every
"AccessKit accessibility follow-up" section in the port issues should be written
against *these* APIs — not against imagined ones. Read this before editing an issue.

Primary gpui sources:
- `crates/gpui/src/_accessibility.rs` — the narrative accessibility guide.
- `crates/gpui/src/elements/div.rs` — the builder methods (`StatefulInteractiveElement`,
  `InteractiveElement`) and `Interactivity::write_a11y_info` (the aria→node mapping).
- `crates/gpui/examples/a11y.rs` — a worked example (heading, spin button, button,
  switch, list) showing the exact call style.
- `crates/gpui/src/window/a11y.rs` — internal tree bookkeeping (no public app API).

## Core model

- An element appears in the accessibility tree **only if it has both** a non-`None`
  `.id(...)` **and** a non-`None` role via `.role(accesskit::Role)`. Nodes with no
  role are not reported. `Role::GenericContainer` is filtered out (asserts in debug).
- The AccessKit `NodeId` is derived from the element's **global** id (composition of
  all ancestor ids). Keep ids stable across frames so AT sees "the same node" rather
  than remove+add. This lines up with the existing keyed-`ElementId` / `use_keyed_state`
  pattern already used across base_gpui components.
- Re-exports (all under `gpui::`): `Role`, `Toggled`, `Orientation`,
  `AccessibleAction` (= `accesskit::Action`), and the whole `accesskit` crate as
  `gpui::accesskit`.

## Builder methods available (on `.id(...)` stateful elements)

Set the role:
- `.role(Role)` — e.g. `Role::Button`, `Role::CheckBox`, `Role::Switch`,
  `Role::Slider`, `Role::SpinButton`, `Role::Tab`, `Role::TabList`, `Role::TabPanel`,
  `Role::Dialog`, `Role::AlertDialog`, `Role::Menu`, `Role::MenuItem`,
  `Role::MenuItemCheckBox`, `Role::MenuItemRadio`, `Role::RadioButton`,
  `Role::RadioGroup`, `Role::ComboBox`, `Role::ListBox`, `Role::ListBoxOption`,
  `Role::List`, `Role::ListItem`, `Role::Heading`, `Role::Separator`,
  `Role::ProgressIndicator`, `Role::Meter`, `Role::Tooltip`, `Role::Toolbar`,
  `Role::MenuBar`, `Role::TextInput`, `Role::Group`, etc. (verify exact variant names
  against accesskit 0.24 `Role` when unsure).

State / properties (each maps to an `accesskit::Node` setter in `write_a11y_info`):
- `.aria_label(impl Into<SharedString>)` → `set_label`
- `.aria_selected(bool)` → `set_selected`
- `.aria_expanded(bool)` → `set_expanded`
- `.aria_toggled(Toggled)` → `set_toggled` (`Toggled::True` / `False` / `Mixed`) —
  the canonical way to express checked/pressed/on states.
- `.aria_numeric_value(f64)` / `.aria_min_numeric_value(f64)` /
  `.aria_max_numeric_value(f64)` → slider/spinbutton/meter/progress values.
- `.aria_orientation(Orientation)` → `Orientation::Horizontal` / `Vertical`.
- `.aria_level(usize)` → heading / tree depth.
- `.aria_position_in_set(usize)` + `.aria_size_of_set(usize)` → item i of N.
- `.aria_row_index` / `.aria_column_index` / `.aria_row_count` /
  `.aria_column_count(usize)` → grid/table geometry.

Actions:
- `.on_a11y_action(accesskit::Action, |Option<&ActionData>, &mut Window, &mut App| ...)`
  — respond to AT-dispatched actions. Common ones: `Action::Click`, `Increment`,
  `Decrement`, `Focus`, `Expand`, `Collapse`, `SetValue`.
- Auto-registered actions: `.on_click(...)` automatically adds an `Action::Click`
  handler; `.track_focus(...)` / `.focusable()` automatically adds `Action::Focus`.
  So a component that already wires click + focus does **not** need to re-add those.

Focus:
- `.focusable()` / `.track_focus(&handle)` / `.tab_stop(bool)` / `.tab_index(isize)`
  are the existing focus primitives; they also drive the `Focus` a11y action. No new
  a11y-specific focus call is needed.

Text labels without duplication:
- `Text::new_inaccessible(...)` creates text with **no** id so it is *not* mirrored
  into the a11y tree. Use it for a control's visible label when you've already set
  `.aria_label(...)` on the parent, to avoid the label appearing twice to a screen
  reader. (`text!(...)` derives an id from source location and *is* accessible.)

## Known gaps in this gpui revision (call these out explicitly)

These Base UI ARIA features have **no direct builder** in this revision. When a
component needs them, the issue must say so and pick a documented fallback rather than
inventing an API:

- **`disabled` / `aria-disabled`**: there is no `.aria_disabled(...)` /
  `.disabled(...)` a11y builder, and `write_a11y_info` never sets a disabled/`Invalid`
  flag. Options to note: (a) omit the interactive role's actions while disabled and
  document the limitation; (b) upstream a `set_disabled`/`aria_disabled` addition to
  gpui; (c) track as blocked. Do **not** claim `.aria_disabled` exists.
- **Relationship props** — `aria-controls`, `aria-labelledby`, `aria-describedby`,
  `aria-activedescendant`, `aria-owns`, `aria-haspopup`: no builders. Labels must go
  through the literal-string `.aria_label(...)`; there is no id-reference wiring.
- **Live regions / announcements** (`aria-live`, toast/alert announcements,
  `role="status"`): no public window or element API to push announcements. Note as a
  gap for Toast/Form-errors/live components.
- **`aria-pressed`** distinct from toggled, **`aria-checked=mixed`** beyond
  `Toggled::Mixed`, `aria-current`, `aria-invalid`, `aria-required`, `aria-readonly`,
  `aria-multiline`, `aria-placeholder`, `aria-valuetext`: no dedicated builders. Use
  the closest available (`aria_toggled`, numeric values) and flag the rest.

## What each follow-up section should contain

Rewrite the issue's `## AccessKit accessibility follow-up` section (keep the heading)
so it is actionable:

1. **Per accessible part**: name the base_gpui layer/element (e.g. `SwitchRoot`,
   `TabsTab`), the `Role` to assign, and every `aria_*` prop it must set with the
   base_gpui state it maps from (e.g. `checked` → `.aria_toggled(Toggled::from(...))`).
2. **Actions**: which `on_a11y_action` handlers to add (skip Click/Focus when
   `on_click`/focus already provide them), routing them into the *same* runtime
   transition the pointer/keyboard path uses.
3. **Labels**: where `.aria_label` comes from, and where to swap visible `text!` for
   `Text::new_inaccessible` to avoid duplication.
4. **Gaps**: explicitly list any Base UI ARIA attribute with no gpui builder (esp.
   disabled + relationship/live props) and the chosen fallback.
5. **Checklist**: convert to `- [ ]` items so the work is trackable, mirroring the
   existing acceptance-criteria style.

Reference the Base UI source's `*DataAttributes.tsx` / `use*.ts` / component `.tsx`
for the authoritative list of roles/ARIA the component emits, then map each onto the
table above.
