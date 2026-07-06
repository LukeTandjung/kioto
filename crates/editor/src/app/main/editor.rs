use crate::core::cursor::Cursor;
use crate::core::editable_buffer::EditableBuffer;
use crate::core::history::{Edit, History};
use crate::core::mode::{EditorAction, EditorMode, Input};
use crate::core::motion::{self, Motion};
use crate::core::position::{Position, Range};
use crate::core::selection::Selection;
use crate::port::clipboard::Clipboard;

/// The concrete editor: canonical document, cursors, selections, the modal
/// state machine, and the undo history. Pure except for the clipboard,
/// which crosses a port: modes return yank/paste as data and `apply`
/// interprets them against the port the caller passes in — GPUI's clipboard
/// in the app, an in-memory one in tests.
///
/// Single-cursor for now: `cursors[0]` is the primary; the `Vec`s exist so
/// multi-cursor lands without an API break.
pub struct Editor<D: EditableBuffer> {
    pub document: D,
    pub cursors: Vec<Cursor>,
    pub selections: Vec<Selection>,
    pub mode: EditorMode,
    history: History,
    last_search: Option<String>,
}

impl<D: EditableBuffer> Editor<D> {
    pub fn new(document: D) -> Self {
        Self {
            document,
            cursors: vec![Cursor::new(Position(0))],
            selections: Vec::new(),
            mode: EditorMode::normal(),
            history: History::default(),
            last_search: None,
        }
    }

    pub fn cursor(&self) -> Position {
        self.cursors[0].position
    }

    /// The primary selection as a normalized byte range, if any.
    pub fn selected_range(&self) -> Option<Range> {
        self.selections
            .first()
            .map(|selection| Range::new(selection.from_position, selection.to_position))
            .filter(|range| !range.is_empty())
    }

    /// Dispatches one input through the active mode and applies the
    /// resulting action. Returns the action so the view can react to the
    /// ones that need UI (opening the search bar).
    pub fn handle_input(&mut self, input: Input, clipboard: &mut dyn Clipboard) -> EditorAction {
        let action = self.mode.handle_input(input);
        self.apply(action.clone(), clipboard);
        action
    }

    pub fn apply(&mut self, action: EditorAction, clipboard: &mut dyn Clipboard) {
        match action {
            EditorAction::None => {}
            EditorAction::InsertText(text) => self.insert_text(&text),
            EditorAction::DeleteBackward => self.delete_motion(Motion::Left),
            EditorAction::DeleteForward => self.delete_motion(Motion::Right),
            EditorAction::Move(motion) => {
                self.history.break_group();
                self.selections.clear();
                let target = motion::resolve(self.document.text(), self.cursor(), motion);
                self.cursors[0].set(target);
            }
            EditorAction::Extend(motion) => self.extend(motion),
            EditorAction::EnterInsert { after_cursor } => {
                self.history.break_group();
                if after_cursor {
                    let target =
                        motion::resolve(self.document.text(), self.cursor(), Motion::Right);
                    self.cursors[0].set(target);
                }
                self.selections.clear();
                self.mode = EditorMode::insert();
            }
            EditorAction::EnterNormal => {
                self.history.break_group();
                self.selections.clear();
                self.mode = EditorMode::normal();
            }
            EditorAction::EnterVisual => {
                let cursor = self.cursor();
                self.selections = vec![Selection::new(cursor, cursor)];
                self.mode = EditorMode::visual();
            }
            EditorAction::SelectLine => self.select_line(),
            EditorAction::DeleteSelection => self.delete_selection(),
            EditorAction::Yank => self.yank(clipboard),
            EditorAction::Paste => self.paste(clipboard),
            EditorAction::Undo => self.undo(),
            // Needs a query first; the view opens its search input and
            // calls `search` with what the user types.
            EditorAction::StartSearch => {}
            EditorAction::NextMatch => self.jump_to_match(Direction::Forward),
            EditorAction::PreviousMatch => self.jump_to_match(Direction::Backward),
        }
    }

    /// Committed text insertion — the shared path for mode-dispatched chars
    /// and platform (IME) input. Replaces the selection when one exists.
    pub fn insert_text(&mut self, text: &str) {
        let range = self
            .selected_range()
            .unwrap_or_else(|| Range::caret(self.cursor()));
        self.selections.clear();
        self.edit(range, text);
    }

    /// Sets a new search query and jumps to its first match at or after the
    /// cursor (wrapping). The query sticks around for `n`/`N`.
    pub fn search(&mut self, query: &str) {
        if query.is_empty() {
            return;
        }
        self.last_search = Some(query.to_string());
        let text = self.document.text();
        let cursor = self.cursor().0;
        let target = find_match(text, query, cursor, Direction::Forward);
        if let Some(target) = target {
            self.history.break_group();
            self.selections.clear();
            self.cursors[0].set(Position(target));
        }
    }

    /// The current search query, if one was set.
    #[cfg(test)]
    pub fn search_query(&self) -> Option<&str> {
        self.last_search.as_deref()
    }

    /// Ends the current undo grouping run — call on cursor placement that
    /// bypasses the modal path (mouse clicks).
    pub fn break_undo_group(&mut self) {
        self.history.break_group();
    }

    /// The single mutation funnel: every buffer edit passes through here so
    /// undo history records all of them. Ranges are clamped exactly the way
    /// `EditableBuffer::replace` clamps, keeping the recorded inverse honest.
    fn edit(&mut self, range: Range, new_text: &str) {
        let clamped = range.byte_range_in(self.document.text());
        let old_text = self.document.text()[clamped.clone()].to_string();
        let cursor_after = Position(clamped.start + new_text.len());
        self.history.record(Edit {
            range: clamped.clone(),
            old_text,
            new_text: new_text.to_string(),
            cursor_before: self.cursor(),
        });
        let edit_range = Range::new(Position(clamped.start), Position(clamped.end));
        self.document.replace(edit_range, new_text);
        self.cursors[0].set(cursor_after);
    }

    fn undo(&mut self) {
        let Some(transaction) = self.history.undo() else {
            return;
        };
        // Reverse each edit newest-first; every inverse is applied against
        // exactly the state its edit produced.
        for edit in transaction.iter().rev() {
            let applied = Range::new(
                Position(edit.range.start),
                Position(edit.range.start + edit.new_text.len()),
            );
            self.document.replace(applied, &edit.old_text);
        }
        self.selections.clear();
        if let Some(first) = transaction.first() {
            self.cursors[0].set(first.cursor_before);
        }
    }

    fn yank(&mut self, clipboard: &mut dyn Clipboard) {
        let Some(range) = self.selected_range() else {
            return;
        };
        let clamped = range.byte_range_in(self.document.text());
        clipboard.write(self.document.text()[clamped.clone()].to_string());
        self.selections.clear();
        self.cursors[0].set(Position(clamped.start));
        if self.mode.is_visual() {
            self.mode = EditorMode::normal();
        }
    }

    /// Pastes clipboard text: over the selection when one exists, otherwise
    /// after the grapheme under the cursor (vim's `p`).
    fn paste(&mut self, clipboard: &mut dyn Clipboard) {
        let Some(text) = clipboard.read() else {
            return;
        };
        self.history.break_group();
        let range = self.selected_range().unwrap_or_else(|| {
            let after = motion::resolve(self.document.text(), self.cursor(), Motion::Right);
            Range::caret(after)
        });
        self.selections.clear();
        self.edit(range, &text);
        if self.mode.is_visual() {
            self.mode = EditorMode::normal();
        }
    }

    fn jump_to_match(&mut self, direction: Direction) {
        let Some(query) = self.last_search.clone() else {
            return;
        };
        // Step past the current match position so repeated `n` advances —
        // by grapheme, not byte, so the slice in `find_match` stays on a
        // char boundary over multi-byte text.
        let start = match direction {
            Direction::Forward => {
                motion::resolve(self.document.text(), self.cursor(), Motion::Right).0
            }
            Direction::Backward => self.cursor().0,
        };
        let text = self.document.text();
        if let Some(target) = find_match(text, &query, start, direction) {
            self.history.break_group();
            self.selections.clear();
            self.cursors[0].set(Position(target));
        }
    }

    fn delete_motion(&mut self, motion: Motion) {
        if let Some(range) = self.selected_range() {
            self.selections.clear();
            self.edit(range, "");
            return;
        }
        let cursor = self.cursor();
        let target = motion::resolve(self.document.text(), cursor, motion);
        self.edit(Range::new(cursor, target), "");
    }

    fn extend(&mut self, motion: Motion) {
        let target = motion::resolve(self.document.text(), self.cursor(), motion);
        match self.selections.first_mut() {
            Some(selection) => selection.set(Some(selection.from_position), Some(target)),
            None => self.selections = vec![Selection::new(self.cursor(), target)],
        }
        self.cursors[0].set(target);
    }

    fn select_line(&mut self) {
        let text = self.document.text();
        let lines = motion::line_ranges(text);
        let cursor = self.cursor().0;
        let row = lines
            .iter()
            .rposition(|line| line.start <= cursor)
            .unwrap_or(0);
        // Select through the line break so `d` removes the whole line.
        let end = lines
            .get(row + 1)
            .map(|next| next.start)
            .unwrap_or(lines[row].end);
        self.selections = vec![Selection::new(Position(lines[row].start), Position(end))];
        self.cursors[0].set(Position(end));
        if !self.mode.is_visual() {
            self.mode = EditorMode::visual();
        }
    }

    fn delete_selection(&mut self) {
        if let Some(range) = self.selected_range() {
            self.selections.clear();
            self.edit(range, "");
        } else if !self.mode.is_insert() {
            // No selection: delete the grapheme under the cursor.
            self.delete_motion(Motion::Right);
        }
        if self.mode.is_visual() {
            self.mode = EditorMode::normal();
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Forward,
    Backward,
}

/// The byte offset of the nearest `query` match from `from` in `direction`,
/// wrapping around the document. Plain case-sensitive substring search.
fn find_match(text: &str, query: &str, from: usize, direction: Direction) -> Option<usize> {
    let from = from.min(text.len());
    match direction {
        Direction::Forward => text[from..]
            .find(query)
            .map(|offset| from + offset)
            .or_else(|| text.find(query)),
        Direction::Backward => text[..from].rfind(query).or_else(|| text.rfind(query)),
    }
}
