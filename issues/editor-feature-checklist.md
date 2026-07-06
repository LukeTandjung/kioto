# Editor feature checklist

Audit of the spike as of 2026-07-06, checked against the actual code
(`app/main/documents/typst.rs`, `core/main/mode.rs`, `app/main/editor.rs`,
`app/main/render/`) and the official Helix tutor (`runtime/tutor`).

## 1. Typst live-preview rendering

### Supported

- [x] Headings (any depth; distinct sizes for levels 1/2/3+, `=` markers hidden)
- [x] Bullet lists (`-` shown as `•`)
- [x] Numbered lists (`+` auto-numbered; counter restarts after a parbreak, matching Typst)
- [x] Code blocks (block raw; fences + language tag hidden, monospace gold)
- [x] Display math — compiled by the real Typst compiler (embedded fonts), rasterized, cached by body hash, centered in the content area; styled raw text as fallback when compilation fails
- [x] Inline math — compiled bitmap in the text flow, chip-styled text as fallback
- [x] Paragraphs (inline runs grouped until parbreak)
- [x] `*strong*` (bold, markers hidden)
- [x] `_emph_` (italic, markers hidden)
- [x] Inline raw (gold chip)
- [x] Verbatim fallback for everything unrecognized — content is never dropped
- [x] Cursor-in-block reveals raw markup (the live-preview rule)
- [x] Per-block source ↔ display offset maps (cursor, mouse, selection geometry)
- [x] Gutter numbers tracking *source* lines, including blank gaps
- [x] Incremental reparse on every edit (`Source::edit`)

### Not yet supported

- [ ] Nested lists (inner items hit the verbatim fallback, markers visible)
- [ ] Term lists (`/ term: description`)
- [ ] Links / autolinks
- [ ] References (`@label`) and labels (`<label>`)
- [ ] Line breaks (`\`), escapes, shorthands (`--`, `---`, `~`)
- [ ] Smart quotes
- [ ] Comments (`//`, `/* */`) — currently unverified how they render; likely verbatim
- [ ] `#code` expressions, set/show rules — verbatim fallback only, never evaluated
- [ ] Tables, figures, images
- [ ] Footnotes
- [ ] Full-document compile (only math fragments go through the compiler)
- [ ] Markdown / LaTeX document models (architecture supports them; nothing implemented)

## 2. Helix keybindings (vs. the Helix tutor)

### Supported

- [x] `h j k l` + arrow keys (tutor ch. 1)
- [x] `i`, `a` (ch. 1–2)
- [x] `w`, `b`, `e` — **as movements; Helix's select-as-you-move semantics not implemented** (ch. 3)
- [x] `v` select mode (ch. 3)
- [x] `x` select line — partial: does not extend line-wise on repeat like Helix (ch. 3)
- [x] `d` delete selection / char under cursor (ch. 1, 3)
- [x] `u` undo (ch. 4)
- [x] `y`, `p` (ch. 4)
- [x] `/`, `n`, `N` (ch. 4)
- [x] Extras not in the tutor: `gg`, `G` (Vim-style; Helix uses `ge`), `0`/`$` (Vim-style; Helix uses `gh`/`gl`), `Ctrl+S` save

### Not yet supported

- [ ] `:` command mode (`:w`, `:q`, …) (ch. 1)
- [ ] `I`, `A` — insert at line start / end (ch. 2)
- [ ] `o`, `O` — open line below / above (ch. 2)
- [ ] `c` — change selection (ch. 3)
- [ ] `W`, `B`, `E` — WORD motions (ch. 3)
- [ ] `;` — collapse selection to cursor (ch. 3)
- [ ] `U` — redo (ch. 4)
- [ ] `P` — paste before (ch. 4)
- [ ] `Space y` / `Space p` — system-clipboard register (ch. 4; we always use the system clipboard instead)
- [ ] `?` — search backward (ch. 4)
- [ ] Multiple cursors: `C`, `Alt-C`, `s`, `S`, `Alt-s`, `&`, `,`, `(`/`)` cycling (ch. 5, 10) — data model is `Vec<Cursor>`-ready, all logic is single-cursor
- [ ] `f`/`t`/`F`/`T` char finds, `Alt-.` repeat (ch. 6)
- [ ] `r` replace char, `.` repeat insert (ch. 6)
- [ ] `R` replace with yanked, `J` join lines, `>`/`<` indent, `Ctrl-a`/`Ctrl-x` (ch. 7)
- [ ] Registers and macros (`"`, `Q`, `q`) (ch. 8)
- [ ] `*` search-register, jumplist (`Ctrl-i`/`Ctrl-o`), `gw` jump labels (ch. 9)
- [ ] Case ops `~`, `` ` ``, `` Alt-` `` (ch. 10)
- [ ] `Ctrl-c` comment toggle (ch. 11)
- [ ] Match mode `m` (`mm`, `mi(`, `ma(`, `ms`, `md`, `mr`) (ch. 12)
- [ ] Counts (`3w`) and operator-pending state generally
- [ ] Splits/windows, file picker (ch. 13) — app-level, out of editor-crate scope for now

## 3. Bugs

### Confirmed (from code reading)

- [x] **`n`/`N` can panic on multi-byte text.** ~~`jump_to_match` (app/main/editor.rs:220) uses `cursor + 1` as the forward search start; if the cursor sits on a multi-byte char (é, 数, emoji), `text[from..]` in `find_match` slices mid-codepoint and panics.~~ Fixed 2026-07-06: forward step now resolves `Motion::Right` (grapheme-aware); regression test `next_match_from_a_multibyte_char_does_not_panic`.
- [ ] **Cursor on a blank line renders in the wrong place.** `geometry_for_source` (render/layout.rs:159) maps gap offsets to the *previous block's end*, so a cursor on a blank line between paragraphs draws at the end of the paragraph above. Related: `offset_for_point` can never resolve to a gap row, so blank lines are unreachable by mouse click (the gutter number coloring is correct, which makes the mismatch visible).
- [ ] **No redo.** `u` undoes but nothing rebuilds; undone transactions are dropped by `History`.
- [ ] **`x` doesn't extend.** Helix's `x` grows the selection a line per press; ours re-selects only the line under the (moved) cursor, so repeated `x` walks the file selecting one line at a time and loses the previous line.
- [ ] **Line-wise paste is character-wise.** `x y p` pastes the yanked line (with its `\n`) inline after the cursor grapheme instead of on a new line below.
- [ ] **Fragment cache grows without bound.** `FragmentCompiler.cache` (fragments.rs) keeps every math body ever compiled — including failures — for the document's lifetime. Fine for a spike, needs eviction eventually.
- [ ] **No line wrapping or horizontal scroll.** Long lines paint past the right edge and are unreachable.
- [ ] **IME composition preview missing.** `marked_text_range` returns `None`; composition commits directly (noted in code as a known regression).

### Suspected (needs a manual check)

- [ ] **Shifted keys may be wrong on symbols / non-ASCII.** Both `input_for_keystroke` (render/mod.rs:492) and the search bar handler apply `to_ascii_uppercase` when shift is held. Verify that `$` (shift+4) actually reaches the motion table as `Char('$')` and that non-ASCII layouts behave; if GPUI reports the unshifted key, `$`/`_` bindings are dead.
- [ ] **Modifier chords leak into the search bar.** While `/` search is open, `Ctrl`/`Alt` combos whose key is one char (e.g. `Ctrl+X`) fall into the plain-character branch and type the letter.
- [ ] **Inline math is fixed at 10.5 pt.** Inside a level-1 heading (22 px text) the compiled bitmap will look undersized — the fragment template ignores the surrounding block's font size.
- [ ] **Search is case-sensitive plain substring** with silent wrap-around — fine as a spike, but no smart-case and no "wrapped" indicator.
