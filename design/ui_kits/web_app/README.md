# Concise — Web App UI kit

A click-through recreation of the Concise web app, lifted directly from the
Figma file. Open `index.html` in a browser.

## Components

| File | What it is |
|---|---|
| `App.jsx`           | Top-level shell. Holds active tab + dialog state. |
| `Sidebar.jsx`       | 60px-wide fixed rail. Three tabs, cog, avatar. |
| `Panel.jsx`         | Generic 180px panel chrome — title, search, body, bottom CTA. |
| `ConceptsPanel.jsx` | Concept Store contents — concepts with sub-items. |
| `FragmentsPanel.jsx`| Fragment Store contents — folders + files. |
| `NotesPanel.jsx`    | Notes outline — nested headings. |
| `EmptyComposer.jsx` | Centered hero composer ("What do you want to learn today?"). |
| `MultiSelect.jsx`   | Headless-UI-style trigger + checkbox popover. |
| `Dialog.jsx`        | Floating dialog with scrim, used by Build Concept / Add Fragments. |
| `BuildConceptDialog.jsx`, `AddFragmentsDialog.jsx` | The two real dialogs from the figma. |
| `controls.jsx`      | Button, TextField, Textarea, Search, Checkbox primitives. |
| `Icon.jsx`          | Inline Heroicons. |

## Notes

- **Not production code.** The components are cosmetic and deliberately small.
  There is no real state management, no animations beyond simple CSS, and the
  "send" button just flashes a toast.
- The pixel measurements (sidebar 60px, panel 180px, composer 514px, etc.)
  match the figma exactly. The right-hand canvas is intentionally empty until
  the user starts a thread — that empty space is the design.
- All styling pulls from `colors_and_type.css` at the project root.
