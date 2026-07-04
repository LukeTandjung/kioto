use crate::core::actions::CoreActions;
use crate::core::cursor::Cursor;
use crate::core::editable_buffer::EditableBuffer;
use crate::core::mode::{EditorAction, EditorMode, Input};
use crate::core::motion::{self, Motion};
use crate::core::position::{Position, Range};
use crate::core::selection::Selection;

/// The concrete editor: canonical document, cursors, selections, and the
/// modal state machine. Pure — no GPUI, no ports — so the whole modal flow
/// unit-tests directly. Actions that need ports (yank/paste/undo/search)
/// are accepted but deferred to milestone 5.
///
/// Single-cursor for now: `cursors[0]` is the primary; the `Vec`s exist so
/// multi-cursor lands without an API break.
pub struct Editor<D: EditableBuffer> {
    pub document: D,
    pub cursors: Vec<Cursor>,
    pub selections: Vec<Selection>,
    pub mode: EditorMode,
}

impl<D: EditableBuffer> Editor<D> {
    pub fn new(document: D) -> Self {
        Self {
            document,
            cursors: vec![Cursor::default()],
            selections: Vec::new(),
            mode: EditorMode::normal(),
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

    pub fn handle_input(&mut self, input: Input) {
        let action = self.mode.handle_input(input);
        self.apply(action);
    }

    pub fn apply(&mut self, action: EditorAction) {
        match action {
            EditorAction::None => {}
            EditorAction::InsertText(text) => self.insert_text(&text),
            EditorAction::DeleteBackward => self.delete_motion(Motion::Left),
            EditorAction::DeleteForward => self.delete_motion(Motion::Right),
            EditorAction::Move(motion) => {
                self.selections.clear();
                let target = motion::resolve(self.document.text(), self.cursor(), motion);
                self.cursors[0].set(target);
            }
            EditorAction::Extend(motion) => self.extend(motion),
            EditorAction::EnterInsert { after_cursor } => {
                if after_cursor {
                    let target =
                        motion::resolve(self.document.text(), self.cursor(), Motion::Right);
                    self.cursors[0].set(target);
                }
                self.selections.clear();
                self.mode = EditorMode::insert();
            }
            EditorAction::EnterNormal => {
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
            // Deferred to milestone 5: ports (clipboard), core history, search.
            EditorAction::Yank
            | EditorAction::Paste
            | EditorAction::Undo
            | EditorAction::StartSearch
            | EditorAction::NextMatch
            | EditorAction::PreviousMatch => {}
        }
    }

    /// Committed text insertion — the shared path for mode-dispatched chars
    /// and platform (IME) input. Replaces the selection when one exists.
    pub fn insert_text(&mut self, text: &str) {
        let range = self
            .selected_range()
            .unwrap_or_else(|| Range::caret(self.cursor()));
        self.selections.clear();
        CoreActions::replace_chars(&mut self.document, range, text);
        self.cursors[0].set(Position(range.start.0 + text.len()));
    }

    fn delete_motion(&mut self, motion: Motion) {
        if let Some(range) = self.selected_range() {
            self.selections.clear();
            CoreActions::delete_chars(&mut self.document, range);
            self.cursors[0].set(range.start);
            return;
        }
        let cursor = self.cursor();
        let target = motion::resolve(self.document.text(), cursor, motion);
        let range = Range::new(cursor, target);
        CoreActions::delete_chars(&mut self.document, range);
        self.cursors[0].set(range.start);
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
            CoreActions::delete_chars(&mut self.document, range);
            self.cursors[0].set(range.start);
        } else if !self.mode.is_insert() {
            // No selection: delete the grapheme under the cursor.
            self.delete_motion(Motion::Right);
        }
        if self.mode.is_visual() {
            self.mode = EditorMode::normal();
        }
    }
}

#[cfg(test)]
#[path = "editor.test.rs"]
mod tests;
